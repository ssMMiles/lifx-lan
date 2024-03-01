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

    let mut serialization = Vec::<proc_macro2::TokenStream>::new();
    let mut deserialization = Vec::<proc_macro2::TokenStream>::new();

    let mut map_variant_to_packet_number = Vec::<proc_macro2::TokenStream>::new();
    let mut map_variant_to_size = Vec::<proc_macro2::TokenStream>::new();

    for message in data.variants.iter() {
        let variant_name = &message.ident;
        let packet_number: u16 = message.attrs.iter().find_map(|attr| {
            if attr.meta.path().is_ident("packet_number") {
                let lit: syn::LitInt = attr.parse_args().expect("Packet number must be a u16");

                return Some(lit.base10_parse().expect("Packet number must be a u16"));
            }

            None
        }).expect("Packet number is required for each variant.");


        let mut variant_current_size: usize = 0;

        match &message.fields {
            Fields::Named(fields) => {
                let mut variant_field_serialization = Vec::<proc_macro2::TokenStream>::new();
                let mut variant_field_deserialization = Vec::<proc_macro2::TokenStream>::new();

                for field in fields.named.iter() {
                    let field_name = field.ident.as_ref().unwrap();
                    let field_size: usize;

                    let field_serialization: proc_macro2::TokenStream;
                    let field_deserialization: proc_macro2::TokenStream;

                    match &field.ty {
                        syn::Type::Path(path) => {
                            let path = &path.path;
                            let base_type =  path.segments[0].ident.to_string();

                            if base_type == "String" {
                                field_serialization = quote! {
                                    buffer[buffer_index..buffer_index+32].copy_from_slice(#field_name.clone().into_bytes().as_slice());
                                };

                                field_deserialization = quote! { 
                                    lifx_serialization::deserialize_string(&bytes[#variant_current_size..#variant_current_size + 32])?
                                };

                                field_size = 32;
                            } else {
                                match base_type.as_str() {
                                    "u8" => {
                                        field_deserialization = quote! { bytes[#variant_current_size] };

                                        field_size = 1;
                                    },
                                    "u16" => {
                                        field_deserialization = quote! { u16::from_le_bytes([bytes[#variant_current_size], bytes[#variant_current_size + 1]]) };

                                        field_size = 2;
                                    },
                                    "u32" => {
                                        field_deserialization = quote! { u32::from_le_bytes([bytes[#variant_current_size], bytes[#variant_current_size + 1], bytes[#variant_current_size + 2], bytes[#variant_current_size + 3]]) };

                                        field_size = 4;
                                    },
                                    "u64" => {
                                        field_deserialization = quote! { u64::from_le_bytes([bytes[#variant_current_size], bytes[#variant_current_size + 1], bytes[#variant_current_size + 2], bytes[#variant_current_size + 3], bytes[#variant_current_size + 4], bytes[#variant_current_size + 5], bytes[#variant_current_size + 6], bytes[#variant_current_size + 7]]) };

                                        field_size = 8;
                                    },
                                    "f32" => {
                                        field_deserialization = quote! { f32::from_le_bytes([bytes[#variant_current_size], bytes[#variant_current_size + 1], bytes[#variant_current_size + 2], bytes[#variant_current_size + 3]]) };

                                        field_size = 4;
                                    },
                                    "i16" => {
                                        field_deserialization = quote! { i16::from_le_bytes([bytes[#variant_current_size], bytes[#variant_current_size + 1]]) };

                                        field_size = 2;
                                    },
                                    _ => panic!("Unsupported type: {}", base_type),
                                };

                                field_serialization = quote! {
                                    buffer[buffer_index..buffer_index+#field_size].copy_from_slice(&#field_name.to_le_bytes());
                                };
                            }
                        },
                        syn::Type::Array(array) => {
                            field_size = match &array.len {
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

                            match *array.elem {
                                syn::Type::Path(ref path) => {
                                    let path = &path.path;
                                    let base_type =  path.segments[0].ident.to_string();

                                    let mut bytes = Vec::<proc_macro2::TokenStream>::new();

                                    for byte in 0..field_size {
                                        bytes.push(quote! { bytes[#variant_current_size + #byte] });
                                    }

                                    if base_type == "u8" {
                                        field_serialization = quote! {
                                            buffer[buffer_index..buffer_index+#field_size].copy_from_slice(#field_name);
                                        };
                                        
                                        field_deserialization = quote! {
                                            [#(#bytes),*]
                                        };
                                    } else {
                                        panic!("Array must contain only u8 elements.");
                                    }
                                },
                                _ => {
                                    panic!("Unsupported array type");
                                }
                            }
                        }
                        _ => {
                            panic!("Only u16, u32 u64, f32, String and arrays of u8 are supported");
                        },
                    };

                    variant_current_size += field_size as usize;

                    variant_field_serialization.push(quote! {
                        #field_serialization
                        buffer_index += #field_size;
                    });

                    variant_field_deserialization.push(quote! {
                        #field_name: #field_deserialization
                    });                    
                }

                let field_names = fields.named.iter().map(|f| &f.ident);

                serialization.push(quote! {
                    #name::#variant_name { #( #field_names ),* } => {
                        let mut buffer_index: usize = 0;

                        #( #variant_field_serialization )*
                        
                        buffer_index
                    }
                });

                deserialization.push(quote! {
                    #packet_number => {
                        Ok(#name::#variant_name {
                            #( #variant_field_deserialization ),*
                        })
                    }
                });
            },
            Fields::Unit => {
                serialization.push(quote! {
                    #name::#variant_name => {
                        0
                    }
                });

                deserialization.push(quote! {
                    #packet_number => {
                        Ok(#name::#variant_name)
                    }
                });
            }
            _ => {
                panic!("LifxPayload can only be derived for enums with named fields (tuples)")
            },
        };

        map_variant_to_size.push(quote! {
            #name::#variant_name { .. } => #variant_current_size
        });

        map_variant_to_packet_number.push(quote! {
            #name::#variant_name { .. } => #packet_number
        });
    }

    let expanded = quote! {
        impl LifxPayload for #name {
            fn from_bytes(packet_number: u16, bytes: &[u8]) -> Result<Self, lifx_serialization::LifxDeserializationError> {
                match packet_number {
                    #( #deserialization ),*
                    _ => {
                        Err(lifx_serialization::LifxDeserializationError::InvalidPacketNumber)
                    },
                }
            }

            fn to_bytes(&self, buffer: &mut [u8]) -> usize {
                match self {
                    #( #serialization ),*
                }
            }

            fn packet_number(&self) -> u16 {
                match self {
                    #( #map_variant_to_packet_number ),*
                }
            }

            fn size(&self) -> usize {
                match self {
                    #( #map_variant_to_size ),*
                }
            }
        }
    };

    TokenStream::from(expanded)
}