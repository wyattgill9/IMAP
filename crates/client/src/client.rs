use std::net::TcpStream;
use transport::connection::Connection;
use transport::packet::Packet;

fn main() {
    // Connect to the server at the specified address
    match TcpStream::connect("127.0.0.1:7878") {
        Ok(stream) => {
            let mut connection = Connection::new(stream);

            // Define the packet with example data
            let example_hash = [0u8; 32]; // Placeholder hash
            let msg_type = 0u8; // Example message type (e.g., request)
            let payload = b"Hello, server!".to_vec(); // Payload data

            let packet = Packet::new(example_hash, msg_type, payload);

            // Send the packet
            match connection.send_packet(&packet) {
                Ok(()) => println!("Packet sent successfully!"),
                Err(e) => eprintln!("Failed to send packet: {}", e),
            }

            // Optionally, receive a response from the server
            match connection.receive_packet() {
                Ok(response) => println!("Received response: {:?}", response),
                Err(e) => eprintln!("Failed to receive response: {}", e),
            }
        }
        Err(e) => eprintln!("Failed to connect to server: {}", e),
    }
}
