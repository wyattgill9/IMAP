#[cfg(test)]
mod tests {
    use std::net::{TcpListener, TcpStream};
    use std::thread;
    use transport::connection::Connection;
    use transport::packet::Packet;

    #[test]
    fn test_server_receive_and_respond() {
        // Create a server that listens for incoming connections
        let listener = TcpListener::bind("127.0.0.1:7879").expect("Failed to bind to address");

        // Spawn a separate thread to simulate a client connecting to the server
        thread::spawn(move || {
            let stream = TcpStream::connect("127.0.0.1:7879").expect("Failed to connect to server");
            let mut client_connection = Connection::new(stream);

            let example_hash = [0u8; 32];
            let msg_type = 0u8; // Request message type
            let payload = b"Server Test Payload".to_vec();
            let packet = Packet::new(example_hash, msg_type, payload);

            // Client sends a packet to the server
            client_connection.send_packet(&packet).expect("Failed to send packet");
        });

        // Accept a connection from the client (simulated above)
        let (stream, _addr) = listener.accept().expect("Failed to accept client connection");
        let mut server_connection = Connection::new(stream);

        // Server receives the packet
        let received_packet = server_connection.receive_packet().expect("Failed to receive packet");

        // Validate the packet's integrity
        assert_eq!(received_packet.hash, [0u8; 32]);
        assert_eq!(received_packet.msg_type, 0u8);
        assert_eq!(received_packet.payload, b"Server Test Payload".to_vec());

        // Example server logic: prepare a response packet
        let response_hash = [1u8; 32]; // Example response hash
        let response_msg_type = 1u8; // Response type
        let response_payload = b"Server Response Payload".to_vec();
        let response_packet = Packet::new(response_hash, response_msg_type, response_payload);

        // Server sends a response back to the client (if desired)
        server_connection.send_packet(&response_packet).expect("Failed to send response packet");
    }
}
