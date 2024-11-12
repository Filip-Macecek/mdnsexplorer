#[cfg(test)]
mod tests {
    use crate::mdns::mdns::MDNSPacket;

    // id: 0, flags: 0, question_count: 1, answer_count: 0, authority_count: 0, additional_count: 0, label: _spotify-connect_tcplocal
    const RESOLVE_SPOTIFY_MDNS_PAYLOAD: [u8; 89] = [0,0,0,0,0,1,0,0,0,0,0,0,1,6,9,5,1,1,5,1,1,2,1,1,1,1,1,6,1,0,5,1,0,2,1,2,1,4,5,9,9,1,1,1,1,1,0,1,1,0,1,0,1,9,9,1,1,6,4,9,5,1,1,6,9,9,1,1,2,5,1,0,8,1,1,1,9,9,9,7,1,0,8,0,0,1,2,0,1];

    #[test]
    fn test_mdns_question_creation() {
        let mdns_packet = MDNSPacket::parse_mdns_packet(&RESOLVE_SPOTIFY_MDNS_PAYLOAD);

        assert!(mdns_packet.is_some());
        let mdns_packet_unwrapped = mdns_packet.unwrap();
        assert_eq!(mdns_packet_unwrapped.query_identifier, 0);
        assert_eq!(mdns_packet_unwrapped.question_count, 1);
        assert_eq!(mdns_packet_unwrapped.answer_count, 0);
        assert_eq!(mdns_packet_unwrapped.authority_count, 0);
        assert_eq!(mdns_packet_unwrapped.additional_count, 0);
    }
}