use std::fmt;

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

    // Serialize function (basic)
    pub fn serialize(&self) -> Vec<u8> {
        let mut buffer = Vec::new();
        buffer.extend_from_slice(&self.hash);
        buffer.extend_from_slice(&self.length.to_be_bytes());
        buffer.push(self.msg_type);
        buffer.extend_from_slice(&self.payload);
        buffer
    }

    // Deserialize function (basic)
    pub fn deserialize(buffer: &[u8]) -> Option<Self> {
        if buffer.len() < 37 {
            eprintln!("Buffer too small for deserialization: {:?}", buffer);
            return None; // Minimum size check (32 + 4 + 1)
        }
        let hash = <[u8; 32]>::try_from(&buffer[0..32]).ok()?;
        let length = u32::from_be_bytes(buffer[32..36].try_into().ok()?);
        let msg_type = buffer[36];
        let payload = buffer[37..].to_vec();
        
        Some(Packet {
            hash,
            length,
            msg_type,
            payload,
        })
    }
}
impl fmt::Debug for Packet {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.debug_struct("Packet")
            .field("hash", &self.hash)
            .field("length", &self.length)
            .field("msg_type", &self.msg_type)
            .field("payload", &self.payload)
            .finish()
    }
}
