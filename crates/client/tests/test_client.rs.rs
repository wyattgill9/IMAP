#[cfg(test)]
mod tests {
    use transport::{Connection, Packet};

    #[test]
    fn test_client_send_packet() {
        let connection = Connection::new();
        let example_hash = [0u8; 32];
        let msg_type = 0u8; // Request
        let payload = b"Client Test Payload".to_vec();
        let packet = Packet::new(example_hash, msg_type, payload);

        // Send packet
        connection.send_packet(&packet).unwrap();

        // Verify by simulating packet receive
        let received_packet = connection.receive_packet().unwrap();
        assert_eq!(packet.hash, received_packet.hash);
        assert_eq!(packet.payload, received_packet.payload);
    }
}
