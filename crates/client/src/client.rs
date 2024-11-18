use transport::connection::Connection;
use transport::packet::Packet;

fn main() {
    // Simulate establishing a connection (using your custom transport layer)
    let connection = Connection::new();

    // Define the packet with some example data
    let example_hash = [0u8; 32]; // Placeholder hash (replace with actual logic)
    let msg_type = 0u8; // Example message type (e.g., request)
    let payload = b"Hello, server!".to_vec(); // Payload data

    let packet = Packet::new(example_hash, msg_type, payload);

    // Send the packet
    match connection.send_packet(&packet) {
        Ok(()) => println!("Packet sent successfully!"),
        Err(e) => eprintln!("Failed to send packet: {}", e),
    }

    // Optionally, receive a response (just for testing)
    match connection.receive_packet() {
        Ok(response) => println!("Received response: {:?}", response),
        Err(e) => eprintln!("Failed to receive response: {}", e),
    }
}
