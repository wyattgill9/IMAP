#[cfg(test)]
mod tests {
    use transport::connection::Connection;
    use transport::packet::Packet;

    #[test]
    fn test_server_receive_and_respond() {
        let connection = Connection::new();
        let example_hash = [0u8; 32];
        let msg_type = 0u8; // Request
        let payload = b"Server Test Payload".to_vec();
        let packet = Packet::new(example_hash, msg_type, payload);

        // Simulate receiving a packet
        connection.send_packet(&packet).unwrap();

        // Simulate the server receiving the packet and responding
        let received_packet = connection.receive_packet().unwrap();
        assert_eq!(packet.hash, received_packet.hash);
        assert_eq!(packet.payload, received_packet.payload);

        // Server logic would generate a response here
    }
}
