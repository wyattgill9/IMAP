use imap::protocol::{
    connection::{Server, Connection},
    error::ProtocolError,
};
use tokio;
use std::time::Duration;
use tokio::sync::mpsc;

const BATCH_SIZE: usize = 100;
const TIMEOUT_DURATION: Duration = Duration::from_secs(30);

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let server = Server::new("127.0.0.1:8080").await?;
    println!("Server listening on 127.0.0.1:8080");

    loop {
        match server.accept().await {
            Ok(connection) => {
                tokio::spawn(async move {
                    let (tx, mut rx) = mpsc::channel(BATCH_SIZE);
                    let (tx_conn, mut rx_conn) = mpsc::channel(1);
                    
                    // Spawn message receiver
                    let receiver_handle = tokio::spawn(async move {
                        let mut connection: Connection = rx_conn.recv().await.unwrap();
                        let mut batch = Vec::with_capacity(BATCH_SIZE);
                        
                        while let Some(message) = rx.recv().await {
                            batch.push(message);
                            
                            if batch.len() >= BATCH_SIZE {
                                // Process batch
                                for msg in batch.drain(..) {
                                    if let Err(e) = connection.send(&msg).await {
                                        eprintln!("Error sending response: {:?}", e);
                                        return;
                                    }
                                }
                                if let Err(e) = connection.flush().await {
                                    eprintln!("Error flushing connection: {:?}", e);
                                    return;
                                }
                            }
                        }
                    });

                    // Send connection to receiver task
                    let _ = tx_conn.send(connection).await;

                    // Message processor
                    let mut consecutive_errors = 0;
                    
                    while consecutive_errors < 3 {
                        tokio::time::sleep(Duration::from_millis(100)).await;
                        consecutive_errors += 1;
                    }
                    
                    drop(tx); // Signal processor to finish
                    let _ = receiver_handle.await;
                });
            }
            Err(e) => eprintln!("Error accepting connection: {:?}", e),
        }
    }
}
