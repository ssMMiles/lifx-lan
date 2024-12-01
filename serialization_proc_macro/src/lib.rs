pub use lifx_serialization_types::{LifxPayload, LifxDeserializationError};
pub use lifx_serialization_macro::LifxPayload;


#[cfg(feature = "no-std")]
pub fn deserialize_string<const N: usize>(bytes: &[u8], size: u8) -> Result<heapless::String<N>, LifxDeserializationError> {
    let mut string_bytes = heapless::Vec::<u8, size>::new();
    string_bytes.extend_from_slice(&bytes[..32]).unwrap();

    match heapless::String::from_utf8(string_bytes) {
        Ok(string) => Ok(string),
        Err(_) => Err(LifxDeserializationError::InvalidUtf8String),
    }
}

#[cfg(not(feature = "no-std"))]
pub fn deserialize_string(bytes: &[u8]) -> Result<String, LifxDeserializationError> {
    let nul_range_end = bytes.iter()
        .position(|&c| c == b'\0')
        .unwrap_or(bytes.len()); 

    match String::from_utf8(bytes[0..nul_range_end].to_vec()) {
        Ok(string) => Ok(string),
        Err(_) => Err(LifxDeserializationError::InvalidUtf8String),
    }
}