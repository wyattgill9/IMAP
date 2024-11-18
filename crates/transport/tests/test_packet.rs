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
        let packet = Packet::new(example_hash, msg_type, payload);

        // Serialize the packet
        let serialized_packet = packet.serialize();

        // Deserialize it back
        let deserialized_packet = Packet::deserialize(&serialized_packet).unwrap();

        // Assert equality of original and deserialized packet
        assert_eq!(packet.hash, deserialized_packet.hash);
        assert_eq!(packet.length, deserialized_packet.length);
        assert_eq!(packet.msg_type, deserialized_packet.msg_type);
        assert_eq!(packet.payload, deserialized_packet.payload);
    }

    #[test]
    fn test_send_receive_packet() {
        let connection = Connection::new();

        // Define a packet to send
        let example_hash = [0u8; 32]; // Placeholder hash
        let msg_type = 0u8; // Request
        let payload = b"Request Payload".to_vec();
        let packet = Packet::new(example_hash, msg_type, payload);

        // Send the packet
        connection.send_packet(&packet).unwrap();

        // Simulate receiving the packet (from the same connection)
        let received_packet = connection.receive_packet().unwrap();

        // Assert that the received packet is the same as the one sent
        assert_eq!(packet.hash, received_packet.hash);
        assert_eq!(packet.payload, received_packet.payload);
    }
}
