use std::io::Result as IoResult;
use tokio::io::{AsyncReadExt, AsyncWriteExt, BufWriter, BufReader};
use tokio::net::TcpStream;
use tokio::net::tcp::{OwnedReadHalf, OwnedWriteHalf};
use super::error::{Result, ProtocolError};
use super::message::Message;
use super::serialization::{serialize_message, deserialize_message};
use async_trait::async_trait;

const MAX_MESSAGE_SIZE: usize = 1024 * 1024 * 10; // 10MB

#[async_trait]
pub trait Transport: Send + Sync {
    async fn send_message(&mut self, message: &Message) -> Result<()>;
    async fn receive_message(&mut self) -> Result<Message>;
    async fn flush(&mut self) -> Result<()>;
    async fn close(self) -> Result<()>;
}

pub struct TcpTransport {
    reader: BufReader<OwnedReadHalf>,
    writer: BufWriter<OwnedWriteHalf>,
}

impl TcpTransport {
    pub fn new(stream: TcpStream) -> IoResult<Self> {
        let (read_half, write_half) = stream.into_split();
        Ok(Self {
            reader: BufReader::new(read_half),
            writer: BufWriter::new(write_half),
        })
    }
}

#[async_trait]
impl Transport for TcpTransport {
    async fn send_message(&mut self, message: &Message) -> Result<()> {
        let serialized = serialize_message(message)?;
        let len = serialized.len() as u32;
        
        if len as usize > MAX_MESSAGE_SIZE {
            return Err(ProtocolError::InvalidMessage);
        }
        self.writer.write_all(&len.to_be_bytes()).await?;
        self.writer.write_all(&serialized).await?;
        if self.writer.buffer().len() >= 32 * 1024 { // 32KB
            self.writer.flush().await?;
        }
        Ok(())
    }

    async fn receive_message(&mut self) -> Result<Message> {
        let mut len_bytes = [0u8; 4];
        self.reader.read_exact(&mut len_bytes).await?;
        let len = u32::from_be_bytes(len_bytes) as usize;
        
        if len > MAX_MESSAGE_SIZE {
            return Err(ProtocolError::InvalidMessage);
        }
        
        let mut buffer = vec![0u8; len];
        self.reader.read_exact(&mut buffer).await?;
        
        deserialize_message(&buffer)
    }

    async fn flush(&mut self) -> Result<()> {
        self.writer.flush().await?;
        Ok(())
    }

    async fn close(self) -> Result<()> {
        Ok(()) // TCP connection is automatically closed when dropped
    }
} 