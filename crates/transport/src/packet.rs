use std::convert::TryInto;

#[derive(Debug)]
pub struct Packet {
    pub hash: [u8; 32],     // 256-bit hash identifier
    pub length: u32,        // Length of the payload
    pub msg_type: u8,       // Message type (request, response, etc.)
    pub payload: Vec<u8>,   // Actual payload data
}

impl Packet {
    // Constructor for creating a new packet
    pub fn new(hash: [u8; 32], msg_type: u8, payload: Vec<u8>) -> Self {
        let length = payload.len() as u32;
        Packet {
            hash,
            length,
            msg_type,
            payload,
        }
    }

    // Serialize function: Serialize the packet to a byte vector
    pub fn serialize(&self) -> Vec<u8> {
        let mut buffer = Vec::with_capacity(37 + self.payload.len());
        buffer.extend_from_slice(&self.hash); // 32 bytes for the hash
        buffer.extend_from_slice(&self.length.to_be_bytes()); // 4 bytes for length
        buffer.push(self.msg_type); // 1 byte for msg_type
        buffer.extend_from_slice(&self.payload); // Payload data
        buffer
    }

    // Deserialize function: Deserialize a byte buffer into a Packet
    pub fn deserialize(buffer: &[u8]) -> Result<Self, String> {
        if buffer.len() < 37 {
            return Err(format!(
                "Buffer too small for deserialization: {} bytes, expected at least 37",
                buffer.len()
            ));
        }

        let hash = <[u8; 32]>::try_from(&buffer[0..32])
            .map_err(|_| "Failed to read 32-byte hash from buffer")?;
        let length = u32::from_be_bytes(buffer[32..36].try_into().map_err(|_| "Failed to read length")?);
        let msg_type = buffer[36];
        let payload = buffer[37..].to_vec();

        Ok(Packet {
            hash,
            length,
            msg_type,
            payload,
        })
    }
}

