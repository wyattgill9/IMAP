#[cfg(test)]
mod tests {
    use transport::connection::Connection;
    use transport::packet::Packet;

    #[test]
    fn test_server_receive_and_respond() {
        let mut connection = Connection::new(); // Assuming `Connection::new` creates a connection with a send/receive queue
        let example_hash = [0u8; 32];
        let msg_type = 0u8; // Represents a "Request"
        let payload = b"Server Test Payload".to_vec();
        let packet = Packet::new(example_hash, msg_type, payload.clone());

        // Simulate sending a packet
        connection.send_packet(&packet).expect("Failed to send packet");

        let serialized_packet = packet.serialize(); 
        let received_packet = Packet::deserialize(&serialized_packet).expect("Failed to deserialize packet");

        // Validate the packet was correctly received
        assert_eq!(packet.hash, received_packet.hash);
        assert_eq!(packet.msg_type, received_packet.msg_type);
        assert_eq!(packet.payload, received_packet.payload);

        // Server logic would generate a response here (no implemnt in this test)
    }
}
