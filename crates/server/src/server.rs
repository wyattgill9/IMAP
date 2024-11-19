use std::net::{TcpListener, TcpStream};
use std::thread;
use transport::connection::Connection;
use transport::packet::Packet;

fn handle_client(mut connection: Connection) {
    match connection.receive_packet() {
        Ok(packet) => {
            println!("Received packet: {:?}", packet);

            // Prepare a response packet
            let example_hash = [1u8; 32]; // Response hash
            let msg_type = 1u8; // Message type (response)
            let payload = b"Hello, client!".to_vec(); // Payload data

            let response_packet = Packet::new(example_hash, msg_type, payload);

            // Send a response back to the client
            match connection.send_packet(&response_packet) {
                Ok(()) => println!("Response sent successfully!"),
                Err(e) => eprintln!("Failed to send response: {}", e),
            }
        }
        Err(e) => eprintln!("Failed to receive packet: {}", e),
    }
}

fn main() {
    let listener = TcpListener::bind("127.0.0.1:7878").expect("Failed to bind to address");

    println!("Server listening on 127.0.0.1:7878...");

    // Accept incoming connections
    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                let connection = Connection::new(stream);
                thread::spawn(move || {
                    handle_client(connection);
                });
            }
            Err(e) => eprintln!("Failed to accept connection: {}", e),
        }
    }
}
