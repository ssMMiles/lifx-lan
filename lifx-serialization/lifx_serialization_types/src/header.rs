#[derive(Debug, Clone)]
pub struct LifxHeader {
    pub size: u16,           // 16 bits
    pub protocol: u16,       // 16 bits
    pub addressable: bool,   // 1 bit
    pub tagged: bool,        // 1 bit
    pub origin: u8,          // 2 bits
    pub source: u32,         // 32 bits
    pub target: [u8; 8],     // 64 bits (8 bytes)
    pub _reserved_1: [u8; 6], // 48 bits (6 bytes)
    // Combined 8 bits for res_required, ack_required, and reserved_2
    pub flags_and_reserved_2: u8, // 8 bits
    pub sequence: u8,           // 8 bits
    pub _reserved_3: [u8; 8],    // 64 bits (8 bytes)
    pub packet_number: u16,          // 16 bits
    pub _reserved_4: [u8; 2],    // 16 bits (2 bytes)
}

impl LifxHeader {
    pub fn set_ack_required(&mut self, ack_required: bool) -> &mut Self {
        if ack_required {
            self.flags_and_reserved_2 |= 0b01000000;
        } else {
            self.flags_and_reserved_2 &= 0b10111111;
        }

        self
    }

    pub fn set_res_required(&mut self, res_required: bool) -> &mut Self {
        if res_required {
            self.flags_and_reserved_2 |= 0b10000000;
        } else {
            self.flags_and_reserved_2 &= 0b01111111;
        }

        self
    }

    pub fn is_ack_required(&self) -> bool {
        (self.flags_and_reserved_2 & 0b01000000) != 0
    }

    pub fn is_res_required(&self) -> bool {
        (self.flags_and_reserved_2 & 0b10000000) != 0
    }

    pub fn to_bytes(&self, buf: &mut [u8]) {
        if buf.len() < 36 {
            panic!("Buffer too small");
        }

        buf[0..2].copy_from_slice(&self.size.to_le_bytes());

        let mut protocol_and_flags = self.protocol;

        if self.addressable {
            protocol_and_flags += 4096;
        }

        if self.tagged {
            protocol_and_flags += 8192;
        }

        buf[2..4].copy_from_slice(&protocol_and_flags.to_le_bytes());
        buf[4..8].copy_from_slice(&self.source.to_le_bytes());
        buf[8..16].copy_from_slice(&self.target);
        buf[16..22].copy_from_slice(&self._reserved_1);
        buf[22] = self.flags_and_reserved_2;
        buf[23] = self.sequence;
        buf[24..32].copy_from_slice(&self._reserved_3);
        buf[32..34].copy_from_slice(&self.packet_number.to_le_bytes());
        buf[34..36].copy_from_slice(&self._reserved_4);
    }

    pub fn from_bytes(bytes: &[u8]) -> Result<Self, crate::LifxDeserializationError> {
        if bytes.len() < 36 {
            return Err(crate::LifxDeserializationError::InvalidPacketSize);
        }

        Ok(LifxHeader {
            size: u16::from_le_bytes([bytes[0], bytes[1]]),
            protocol: u16::from_le_bytes([bytes[2], bytes[3]]) & 0xFFF,
            addressable: (bytes[3] & 0b10000000) != 0,
            tagged: (bytes[3] & 0b00100000) != 0,
            origin: (bytes[3] & 0b00000011),
            source: u32::from_le_bytes([bytes[4], bytes[5], bytes[6], bytes[7]]),
            target: [
                bytes[8], bytes[9], bytes[10], bytes[11], bytes[12], bytes[13], bytes[14],
                bytes[15],
            ],
            _reserved_1: [
                bytes[16], bytes[17], bytes[18], bytes[19], bytes[20], bytes[21],
            ],
            flags_and_reserved_2: bytes[22],
            sequence: bytes[23],
            _reserved_3: [
                bytes[24], bytes[25], bytes[26], bytes[27], bytes[28], bytes[29], bytes[30],
                bytes[31],
            ],
            packet_number: u16::from_le_bytes([bytes[32], bytes[33]]),
            _reserved_4: [bytes[34], bytes[35]],
        })
    }
}