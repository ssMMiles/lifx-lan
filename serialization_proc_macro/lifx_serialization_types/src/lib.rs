#[derive(thiserror::Error, Debug)]
pub enum LifxDeserializationError {
    #[error("Invalid packet number: {0}")]
    InvalidPacketNumber(u16),

    #[error("Invalid UTF-8 string")]
    InvalidUtf8String,

    #[error("Invalid packet size")]
    InvalidPacketSize,
}

pub trait LifxPayload {
    fn from_bytes(payload_number: u16, bytes: &[u8]) -> Result<Self, LifxDeserializationError>
    where
        Self: Sized;
    fn to_bytes(&self, buffer: &mut [u8]) -> usize;

    fn packet_number(&self) -> u16;
    fn size(&self) -> usize;
}
