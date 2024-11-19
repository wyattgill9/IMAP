use std::io::{Read, Write};
use std::net::TcpStream;
use crate::packet::Packet;

pub struct Connection {
    stream: TcpStream,
}

impl Connection {
    pub fn new(stream: TcpStream) -> Self {
        Connection { stream }
    }

    pub fn send_packet(&mut self, packet: &Packet) -> Result<(), String> {
        let serialized_packet = packet.serialize();
        self.stream
            .write_all(&serialized_packet)
            .map_err(|e| format!("Failed to send packet: {}", e))?;
        Ok(())
    }

    pub fn receive_packet(&mut self) -> Result<Packet, String> {
        let mut buffer = vec![0; 1024]; // Adjust buffer size as needed
        let bytes_read = self
            .stream
            .read(&mut buffer)
            .map_err(|e| format!("Failed to read from stream: {}", e))?;
        buffer.truncate(bytes_read);
        Packet::deserialize(&buffer).ok_or_else(|| "Failed to deserialize packet".into())
    }
}
