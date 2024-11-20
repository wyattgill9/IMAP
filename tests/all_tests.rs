use std::sync::Arc;
use tokio::sync::{Mutex, Semaphore};
use tokio::net::{TcpListener, TcpStream};
use tokio::task::spawn;
use transport::connection::Connection;
use transport::packet::Packet;

#[tokio::test]
async fn ultra_optimized_test_multiple_client_requests() {
    // Create a buffer pool (mocked for now)
    let buffer_pool = Arc::new(Mutex::new(Vec::new()));

    // Semaphore to limit the number of concurrent connections
    let semaphore = Arc::new(Semaphore::new(100)); // limit concurrency to 100 simultaneous clients

    // Bind the server to the desired address and port
    let listener = TcpListener::bind("127.0.0.1:8000")
        .await
        .expect("Failed to bind server socket");

    // Spawn a server loop to handle requests concurrently
    let server_handle = spawn({
        let buffer_pool = Arc::clone(&buffer_pool); // Clone the Arc reference here
        async move {
            loop {
                match listener.accept().await {
                    Ok((stream, _)) => {
                        // Spawn a new task to handle the connection
                        spawn(handle_client(stream, buffer_pool.clone()));
                    }
                    Err(e) => {
                        eprintln!("Failed to accept connection: {}", e);
                        break;
                    }
                }
            }
        }
    });

    const NUM_CLIENTS: usize = 2000;
    let mut client_tasks = Vec::with_capacity(NUM_CLIENTS);

    // Create client tasks with a Semaphore to control the concurrency
    for i in 0..NUM_CLIENTS {
        let semaphore = Arc::clone(&semaphore); // Clone the semaphore for each client
        let buffer_pool = Arc::clone(&buffer_pool); // Clone the Arc reference for each client

        client_tasks.push(spawn({
            let _permit = semaphore.acquire().await.unwrap(); // acquire a permit to limit concurrency
            async move {
                // Connect the client to the server
                let stream = TcpStream::connect("127.0.0.1:8000")
                    .await
                    .expect("Failed to connect to server");
                let mut connection = Connection::new(stream, buffer_pool.clone());

                // Create a unique request packet for each client
                let example_hash = [i as u8; 32];
                let msg_type = 0u8;
                let payload = format!("Request payload from client {}", i).into_bytes();
                let packet = Packet::new(example_hash, msg_type, payload);

                // Send the packet to the server
                if let Err(e) = connection.send_packets(vec![packet]).await {
                    eprintln!("Client {} failed to send packet: {}", i, e);
                }

                // Receive the server's response
                if let Ok(_response) = connection.receive_packet().await {
                    // Process the response (for now we just check it was received)
                    // e.g., println!("Client {} received response: {:?}", i, response);
                } else {
                    eprintln!("Client {} failed to receive a response", i);
                }
            }
        }));
    }

    // Await all client tasks to complete
    let results = futures::future::join_all(client_tasks).await;
    for result in results {
        result.expect("Client task failed");
    }

    // Gracefully terminate the server after all tasks complete
    server_handle.abort();
}

// Handler to process client connections
async fn handle_client(stream: TcpStream, buffer_pool: Arc<Mutex<Vec<Vec<u8>>>>) {
    let mut connection = Connection::new(stream, buffer_pool);

    // Attempt to receive the packet from the client
    match connection.receive_packet().await {
        Ok(_packet) => {
            // Process the received packet (for now, just print it)
            // e.g., println!("Server received packet: {:?}", packet);

            // Prepare and send a response packet
            let response_packet = Packet::new([1u8; 32], 1, b"Response".to_vec());
            if let Err(e) = connection.send_packets(vec![response_packet]).await {
                eprintln!("Error sending response: {}", e);
            }
        },
        Err(e) => {
            eprintln!("Failed to receive packet from client: {}", e);
        }
    }
}
