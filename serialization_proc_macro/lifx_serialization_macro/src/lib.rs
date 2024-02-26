extern crate proc_macro;
extern crate alloc;

use alloc::string::ToString;
use proc_macro::TokenStream;
use quote::quote;

use syn::{parse_macro_input, Data, DeriveInput, Fields};

#[proc_macro_derive(LifxPayload, attributes(packet_number))]
pub fn from_bytes_derive(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);

    let name = input.ident;
    let data = match input.data {
        Data::Enum(data) => data,
        _ => panic!("LifxPayload can only be derived for enums"),
    };

    let mut message_parsing = Vec::<proc_macro2::TokenStream>::new();
    let mut message_sizing = Vec::<proc_macro2::TokenStream>::new();

    let mut map_packet_number_to_message_types = Vec::<proc_macro2::TokenStream>::new();

    for message in data.variants.iter() {
        let message_type = &message.ident;
        let packet_number: u16 = message.attrs.iter().find_map(|attr| {
            if attr.meta.path().is_ident("packet_number") {
                let lit: syn::LitInt = attr.parse_args().expect("Packet number must be a u16");

                return Some(lit.base10_parse().expect("Packet number must be a u16"));
            }

            None
        }).expect("Packet number is required for each variant.");

        let mut byte_offset: usize = 0;
        let parse_message_fields = match &message.fields {
            Fields::Named(fields) => {
                Some(fields.named.iter().map(|field|{
                    let field_name = field.ident.as_ref().unwrap();

                    let parsing = match &field.ty {
                        syn::Type::Path(path) => {
                            let path = &path.path;

                            let base_type =  path.segments[0].ident.to_string();

                            if base_type == "String" {
                                let parsing = quote! { lifx_serialization::deserialize_string(&bytes[#byte_offset..#byte_offset + 32])? };

                                byte_offset += 32;
                                parsing
                            } else {
                                let field_parsing: proc_macro2::TokenStream;

                                match base_type.as_str() {
                                    "u8" => {
                                        field_parsing = quote! { bytes[#byte_offset] };

                                        byte_offset += 1;
                                    },
                                    "u16" => {
                                        field_parsing = quote! { u16::from_le_bytes([bytes[#byte_offset], bytes[#byte_offset + 1]]) };

                                        byte_offset += 2;
                                    },
                                    "u32" => {
                                        field_parsing = quote! { u32::from_le_bytes([bytes[#byte_offset], bytes[#byte_offset + 1], bytes[#byte_offset + 2], bytes[#byte_offset + 3]]) };

                                        byte_offset += 4;
                                    },
                                    "u64" => {
                                        field_parsing = quote! { u64::from_le_bytes([bytes[#byte_offset], bytes[#byte_offset + 1], bytes[#byte_offset + 2], bytes[#byte_offset + 3], bytes[#byte_offset + 4], bytes[#byte_offset + 5], bytes[#byte_offset + 6], bytes[#byte_offset + 7]]) };

                                        byte_offset += 8;
                                    },
                                    "f32" => {
                                        field_parsing = quote! { f32::from_le_bytes([bytes[#byte_offset], bytes[#byte_offset + 1], bytes[#byte_offset + 2], bytes[#byte_offset + 3]]) };

                                        byte_offset += 4;
                                    },
                                    "i16" => {
                                        field_parsing = quote! { i16::from_le_bytes([bytes[#byte_offset], bytes[#byte_offset + 1]]) };

                                        byte_offset += 2;
                                    },
                                    _ => panic!("Unsupported type: {}", base_type),
                                };

                                field_parsing
                            }
                        },
                        syn::Type::Array(array) => {
                            let len = match &array.len {
                                syn::Expr::Lit(lit) => {
                                    match &lit.lit {
                                        syn::Lit::Int(int) => {
                                            let len: usize = int.base10_parse().expect("Array length must be a number");
                                            len
                                        },
                                        _ => {
                                            panic!("Array length must be a number");
                                        }
                                    }
                                },
                                _ => {
                                    panic!("Array length must be a fixed value.");
                                }
                            };
                            
                            let field_parsing: proc_macro2::TokenStream;

                            match *array.elem {
                                syn::Type::Path(ref path) => {
                                    let path = &path.path;
                                    let base_type =  path.segments[0].ident.to_string();

                                    let mut bytes = Vec::<proc_macro2::TokenStream>::new();

                                    for byte in 0..len {
                                        bytes.push(quote! { bytes[#byte_offset + #byte] });
                                    }

                                    if base_type == "u8" {
                                        field_parsing = quote! {
                                            [#(#bytes),*]
                                        };

                                        byte_offset += len;
                                    } else {
                                        panic!("Array must contain only u8 elements.");
                                    }
                                },
                                _ => {
                                    panic!("Unsupported array type");
                                }
                            }

                            field_parsing
                        }
                        _ => {
                            panic!("Only u16, u32 u64, f32, String and arrays of u8 are supported");
                        },
                    };
                    
                    quote! {
                        #field_name: #parsing
                    }
                }))
            },
            Fields::Unit => {
                None
            }
            _ => {
                panic!("LifxPayload can only be derived for enums with named fields (tuples)")
            },
        };

        let parsing = if let Some(parsing) = parse_message_fields {
            quote! {
                #packet_number => {
                    Ok(#name::#message_type {
                        #( #parsing ),*
                    })
                }
            }
        } else {
            quote! {
                #packet_number => {
                    Ok(#name::#message_type)
                }
            }
        };

        message_parsing.push(parsing);
        message_sizing.push(quote! {
            #name::#message_type { .. } => #byte_offset
        });

        map_packet_number_to_message_types.push(quote! {
            #name::#message_type { .. } => #packet_number
        });
    }

    let serialization = data.variants.iter().map(|variant| {
        let variant_name = &variant.ident;

        match &variant.fields {
            Fields::Named(ref fields) => {
                let field_names = fields.named.iter().map(|f| &f.ident);

                let field_serialization = fields.named.iter().map(|f| {
                    let field_name = &f.ident;

                    match &f.ty {
                        syn::Type::Path(path) => {
                            let path = &path.path;

                            let base_type =  path.segments[0].ident.to_string();

                            if base_type == "String" {
                                quote! {
                                    bytes.extend(#field_name.clone().into_bytes());
                                }
                            } else {
                                quote! {
                                    bytes.extend(#field_name.to_le_bytes());
                                }
                            }
                        },
                        syn::Type::Array(array) => {
                            match *array.elem {
                                syn::Type::Path(ref path) => {
                                    let path = &path.path;
                                    let base_type =  path.segments[0].ident.to_string();

                                    if base_type == "u8" {
                                        quote! {
                                            bytes.extend(*#field_name);
                                        }
                                    } else {
                                        panic!("Array must contain only u8 elements.");
                                    }
                                },
                                _ => {
                                    panic!("Unsupported array type");
                                }
                            }
                        },
                        _ => {
                            panic!("Only u16, u32 u64, f32, String and arrays of u8 are supported");
                        },
                    }
                });

                quote! {
                    #name::#variant_name { #( #field_names ),* } => {
                        let mut bytes = heapless::Vec::<u8, 64>::new();

                        #( #field_serialization )*

                        let size = bytes.len();
                        buffer[..size].copy_from_slice(&bytes);
                        
                        size
                    }
                }
            },
            Fields::Unit => {
                quote! {
                    #name::#variant_name => {
                        0
                    }
                }
            },
            _ => {
                panic!("LifxSerialize can only be derived for enums with named fields (tuples)")
            },
        }
    });

    let expanded = quote! {
        impl LifxPayload for #name {
            fn from_bytes(packet_number: u16, bytes: &[u8]) -> Result<Self, lifx_serialization::LifxDeserializationError> {
                match packet_number {
                    #( #message_parsing ),*
                    _ => {
                        Err(lifx_serialization::LifxDeserializationError::InvalidPacketNumber)
                    },
                }
            }

            fn to_bytes(&self, buffer: &mut [u8]) -> usize {
                let payload_size = self.size();

                if buffer.len() < payload_size {
                    panic!("Buffer too small");
                }
                
                match self {
                    #( #serialization ),*
                }
            }

            fn packet_number(&self) -> u16 {
                match self {
                    #( #map_packet_number_to_message_types ),*
                }
            }

            fn size(&self) -> usize {
                match self {
                    #( #message_sizing ),*
                }
            }
        }
    };

    TokenStream::from(expanded)
}