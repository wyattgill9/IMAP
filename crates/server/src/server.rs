use transport::connection::Connection;
use transport::packet::Packet;

fn main() {
    
    let connection = Connection::new();

    
    match connection.receive_packet() {
        Ok(packet) => {
            
            println!("Received packet: {:?}", packet);

            
            let example_hash = [1u8; 32]; // response hash
            let msg_type = 1u8; // message type (response)
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
