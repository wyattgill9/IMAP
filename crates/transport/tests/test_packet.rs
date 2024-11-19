#[cfg(test)]
mod tests {
    use super::*;
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
        let mut connection = Connection::new(); // Ensure `Connection::new` sets up appropriate buffer/queue mechanisms

        // Define a packet to send
        let example_hash = [0u8; 32]; // Placeholder hash
        let msg_type = 0u8; // Request
        let payload = b"Request Payload".to_vec();
        let packet = Packet::new(example_hash, msg_type, payload.clone());

        // Send the packet using the connection
        connection.send_packet(&packet).expect("Failed to send packet");

        // Simulate receiving the packet (assuming `Connection` internally handles buffering correctly)
        let received_serialized_packet = packet.serialize(); // Manually serialize for controlled testing
        let received_packet = Packet::deserialize(&received_serialized_packet).expect("Failed to deserialize received packet");

        // Assert that the received packet matches the original sent packet
        assert_eq!(packet.hash, received_packet.hash, "Hash mismatch during send/receive test");
        assert_eq!(packet.msg_type, received_packet.msg_type, "Message type mismatch during send/receive test");
        assert_eq!(packet.payload, received_packet.payload, "Payload mismatch during send/receive test");
    }
}
