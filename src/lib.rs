#[cfg(feature = "no-std")]
extern crate heapless;

extern crate lifx_serialization;

use lifx_serialization::{LifxDeserializationError, LifxPayload};

pub use header::LifxHeader;
pub use request_options::LifxRequestOptions;
pub use messages::Message;

pub mod header;
pub mod request_options;
pub mod messages;

pub fn serialize_lifx_packet(request_options: &LifxRequestOptions, payload: &messages::Message, buffer: &mut [u8]) {
    let packet_number = payload.packet_number();
    let payload_size = payload.size();

    let mut flags_and_reserved_2: u8 = 0;

    if request_options.ack_required {
        flags_and_reserved_2 |= 0b01000000;
    }

    if request_options.res_required {
        flags_and_reserved_2 |= 0b10000000;
    }

    let header = LifxHeader {
        size: 36 + payload_size as u16,

        packet_number,

        tagged: request_options.tagged,
        source: request_options.source,
        target: request_options.target,

        sequence: request_options.sequence,

        protocol: 1024,
        addressable: true,
        origin: 0,
        
        _reserved_1: [0; 6],
        flags_and_reserved_2,
        _reserved_3: [0; 8],
        _reserved_4: [0; 2],
    };

    header.to_bytes(buffer);
    payload.to_bytes(&mut buffer[36..]);
}

pub fn deserialize_lifx_packet(bytes: &[u8]) -> Result<(LifxHeader, messages::Message), LifxDeserializationError> {
    let header = LifxHeader::from_bytes(&bytes[0..36])?;
    let payload = messages::Message::from_bytes(header.packet_number, &bytes[36..])?;

    Ok((header, payload))
}