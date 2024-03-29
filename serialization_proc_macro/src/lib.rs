pub use lifx_serialization_types::{LifxPayload, LifxDeserializationError};
pub use lifx_serialization_macro::LifxPayload;

#[cfg(feature = "no-std")]
pub fn deserialize_string(bytes: &[u8]) -> Result<heapless::String<32>, LifxDeserializationError> {
    let mut string_bytes = heapless::Vec::<u8, 32>::new();
    string_bytes.extend_from_slice(&bytes[..32]).unwrap();

    match heapless::String::from_utf8(string_bytes) {
        Ok(string) => Ok(string),
        Err(_) => Err(LifxDeserializationError::InvalidUtf8String),
    }
}

#[cfg(not(feature = "no-std"))]
pub fn deserialize_string(bytes: &[u8]) -> Result<String, LifxDeserializationError> {
    match String::from_utf8(bytes.to_vec()) {
        Ok(string) => Ok(string),
        Err(_) => Err(LifxDeserializationError::InvalidUtf8String),
    }
}