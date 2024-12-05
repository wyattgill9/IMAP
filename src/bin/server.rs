use tokio;
use imap::protocol::connection::Server;
use std::sync::atomic::{AtomicUsize, Ordering};
use std::sync::Arc;

#[tokio::main(worker_threads = 1)]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let server = Server::new("127.0.0.1:8080").await?;
    println!("Server listening on 127.0.0.1:8080");

    let message_count = Arc::new(AtomicUsize::new(0));

    loop {
        match server.accept().await {
            Ok(mut connection) => {
                println!("New client connected");
                let count = Arc::clone(&message_count);
                
                tokio::spawn(async move {
                    loop {
                        match connection.receive().await {
                            Ok(message) => {
                                let current_count = count.fetch_add(1, Ordering::SeqCst);
                                if let Err(_) = connection.send(&message).await {
                                    // Client disconnected - this is normal behavior
                                    break;
                                }
                                if current_count >= 10_000 {
                                    println!("Benchmark complete, shutting down...");
                                    std::process::exit(0);
                                }
                            }
                            Err(_) => {
                                // Client disconnected - this is normal behavior
                                break;
                            }
                        }
                    }
                });
            }
            Err(e) => {
                eprintln!("Failed to accept connection: {:?}", e);
            }
        }
    }
}
