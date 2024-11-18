use transport::connection::Connection;
use transport::packet::Packet;

fn main() {
    // Simulate listening for incoming packets (your custom transport layer)
    let connection = Connection::new();

    // Simulate receiving a packet
    match connection.receive_packet() {
        Ok(packet) => {
            // Assuming `packet` is of type `Packet`
            println!("Received packet: {:?}", packet);

            // Process the packet (you can add more logic here)
            let example_hash = [1u8; 32]; // Example response hash
            let msg_type = 1u8; // Example message type (response)
            let payload = b"Hello, client!".to_vec(); // Response data

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
