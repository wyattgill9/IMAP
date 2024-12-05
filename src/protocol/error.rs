use thiserror::Error;

#[derive(Error, Debug)]
pub enum ProtocolError {
    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),
    
    #[error("Serialization error: {0}")]
    Serialization(#[from] bincode::Error),
    
    #[error("Connection closed")]
    ConnectionClosed,
    
    #[error("Invalid message format")]
    InvalidMessage,
    
    #[error("Timeout")]
    Timeout,
}

pub type Result<T> = std::result::Result<T, ProtocolError>; 