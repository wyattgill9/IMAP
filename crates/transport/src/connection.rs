use std::sync::Arc;
use tokio::net::TcpStream;
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use crate::packet::Packet;
use tokio::sync::Mutex;

#[derive(Debug)]
pub struct Connection {
    stream: TcpStream,
    buffer_pool: Arc<Mutex<Vec<Vec<u8>>>>, // Buffer pool to reuse memory
}

impl Connection {
    // Constructor to initialize a new connection
    pub fn new(stream: TcpStream, buffer_pool: Arc<Mutex<Vec<Vec<u8>>>>) -> Self {
        Connection { stream, buffer_pool }
    }

    // Optimized async send for a batch of packets
    pub async fn send_packets(&mut self, packets: Vec<Packet>) -> Result<(), String> {
        let total_size: usize = packets.iter().map(|p| 37 + p.payload.len()).sum();
        let mut buffer = Vec::with_capacity(total_size);

        // Serialize packets and append to the buffer
        for packet in packets {
            buffer.extend(packet.serialize());
        }

        self.stream
            .write_all(&buffer)
            .await
            .map_err(|e| format!("Failed to send batch of packets: {}", e))?;

        // Avoid flushing on every send unless necessary
        self.stream.flush().await.map_err(|e| format!("Failed to flush packet: {}", e))?;
        Ok(())
    }

    // Optimized async receive packet with zero-copy
    pub async fn receive_packet(&mut self) -> Result<Packet, String> {
        let mut buffer = self.get_buffer_for_read().await;
        let bytes_read = self.stream
            .read(&mut buffer)
            .await
            .map_err(|e| format!("Failed to read from stream: {}", e))?;

        buffer.truncate(bytes_read); // Shrink the buffer to the actual data size
        Packet::deserialize(&buffer).map_err(|e| format!("Failed to deserialize packet: {}", e))
    }

    // Get a reusable buffer from the pool with minimal locking
    async fn get_buffer_for_read(&self) -> Vec<u8> {
        let mut pool = self.buffer_pool.lock().await;

        // If there's a reusable buffer, pop it from the pool; otherwise, create a new one
        if let Some(buffer) = pool.pop() {
            buffer
        } else {
            // Allocate a new buffer if necessary
            vec![0; 2048] // Default buffer size; can be adjusted
        }
    }

    // Return buffer to the pool after reading, only if the buffer is large enough
    pub async fn return_buffer_to_pool(&self, buffer: Vec<u8>) {
        // Only return buffers to the pool that are not too large to avoid growing the pool unnecessarily
        if buffer.len() <= 2048 {
            let mut pool = self.buffer_pool.lock().await;
            pool.push(buffer);
        }
    }
}
