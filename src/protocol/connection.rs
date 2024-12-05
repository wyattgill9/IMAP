use tokio::net::{TcpStream, TcpListener};
use tokio::io::{BufReader, BufWriter, AsyncReadExt, AsyncWriteExt};
use tokio::net::tcp::{OwnedReadHalf, OwnedWriteHalf};
use super::error::Result;
use super::message::Message;
use bincode;
use std::io::Result as IoResult;
use super::transport::{Transport, TcpTransport};

pub struct Connection {
    pub(crate) transport: TcpTransport,
}

impl Connection {
    pub fn new(stream: TcpStream) -> IoResult<Self> {
        Ok(Self {
            transport: TcpTransport::new(stream)?
        })
    }

    pub async fn send(&mut self, message: &Message) -> Result<()> {
        self.transport.send_message(message).await
    }

    pub async fn receive(&mut self) -> Result<Message> {
        self.transport.receive_message().await
    }

    pub async fn flush(&mut self) -> Result<()> {
        self.transport.flush().await
    }
}

pub struct Server {
    listener: TcpListener,
}

impl Server {
    pub async fn new(addr: &str) -> Result<Self> {
        let listener = TcpListener::bind(addr).await?;
        Ok(Self { listener })
    }

    pub async fn accept(&self) -> Result<Connection> {
        let (socket, _) = self.listener.accept().await?;
        socket.set_nodelay(true)?;
        // Use `?` to unwrap the `IoResult` from `Connection::new`
        Connection::new(socket).map_err(|e| e.into())
    }
}

pub struct Client;

impl Client {
    pub async fn connect(addr: &str) -> Result<Connection> {
        let socket = TcpStream::connect(addr).await?;
        socket.set_nodelay(true)?;
        // Use `?` to unwrap the `IoResult` from `Connection::new`
        Connection::new(socket).map_err(|e| e.into())
    }
}
