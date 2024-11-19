#[cfg(test)]
mod tests {
    use transport::connection::Connection;
    use transport::packet::Packet;

    #[test]
    fn test_client_send_packet() {
        let mut connection = Connection::new(); // Assuming `Connection::new` creates a connection
        let example_hash = [0u8; 32];
        let msg_type = 0u8; // Request message type
        let payload = b"Client Test Payload".to_vec();
        let packet = Packet::new(example_hash, msg_type, payload.clone());

        // Send packet
        connection.send_packet(&packet).expect("Failed to send packet");

        // Serialize the packet for consistency check (similar to server test)
        let serialized_packet = packet.serialize();
        let received_packet = Packet::deserialize(&serialized_packet).expect("Failed to deserialize packet");

        // Validate the packet's integrity
        assert_eq!(packet.hash, received_packet.hash);
        assert_eq!(packet.msg_type, received_packet.msg_type);
        assert_eq!(packet.payload, received_packet.payload);

        // Additional debug info (optional, for troubleshooting)
        println!("Client packet serialized successfully and matched after deserialization.");
    }
}
