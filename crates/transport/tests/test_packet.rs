#[cfg(test)]
mod tests {
    use std::net::{TcpListener, TcpStream};
    use std::thread;
    use transport::connection::Connection;
    use transport::packet::Packet;

    #[test]
    fn test_packet_serialization() {
        let example_hash = [0u8; 32]; // Example hash
        let msg_type = 0u8; // Example message type
        let payload = b"Test Payload".to_vec(); // Example payload
        let packet = Packet::new(example_hash, msg_type, payload.clone());

        // Serialize the packet
        let serialized_packet = packet.serialize();

        // Deserialize it back
        let deserialized_packet = Packet::deserialize(&serialized_packet).expect("Failed to deserialize packet");

        // Assert equality of original and deserialized packet
        assert_eq!(packet.hash, deserialized_packet.hash, "Hash mismatch during serialization test");
        assert_eq!(packet.length, deserialized_packet.length, "Length mismatch during serialization test");
        assert_eq!(packet.msg_type, deserialized_packet.msg_type, "Message type mismatch during serialization test");
        assert_eq!(packet.payload, deserialized_packet.payload, "Payload mismatch during serialization test");
    }

    #[test]
    fn test_send_receive_packet() {
        // Create a server listener for testing
        let listener = TcpListener::bind("127.0.0.1:7880").expect("Failed to bind server socket");

        // Spawn a client in a separate thread
        thread::spawn(move || {
            let stream = TcpStream::connect("127.0.0.1:7880").expect("Failed to connect to server");
            let mut client_connection = Connection::new(stream);

            let example_hash = [0u8; 32];
            let msg_type = 0u8; // Request
            let payload = b"Request Payload".to_vec();
            let packet = Packet::new(example_hash, msg_type, payload);

            // Client sends a packet
            client_connection.send_packet(&packet).expect("Failed to send packet");
        });

        // Accept a connection from the client
        let (stream, _addr) = listener.accept().expect("Failed to accept client connection");
        let mut server_connection = Connection::new(stream);

        // Server receives the packet
        let received_packet = server_connection.receive_packet().expect("Failed to receive packet");

        // Define the expected packet
        let expected_hash = [0u8; 32];
        let expected_msg_type = 0u8;
        let expected_payload = b"Request Payload".to_vec();

        // Assert that the received packet matches expectations
        assert_eq!(received_packet.hash, expected_hash, "Hash mismatch during send/receive test");
        assert_eq!(received_packet.msg_type, expected_msg_type, "Message type mismatch during send/receive test");
        assert_eq!(received_packet.payload, expected_payload, "Payload mismatch during send/receive test");
    }
}
