#[cfg(test)]
mod tests {
    use crate::mdns::mdns::{MDNSPacket, MDNSQuestion};

    // id: 0, flags: 0, question_count: 1, answer_count: 0, authority_count: 0, additional_count: 0, label: _spotify-connect_tcplocal
    const RESOLVE_SPOTIFY_MDNS_PAYLOAD: [u8; 45] = [0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 16, 95, 115, 112, 111, 116, 105, 102, 121, 45, 99, 111, 110, 110, 101, 99, 116, 4, 95, 116, 99, 112, 5, 108, 111, 99, 97, 108, 0, 0, 12, 0, 1];

    const ANSWER_MACHINE1_MDNS_PAYLOAD: [u8; 114] = [0, 0, 132, 0, 0, 0, 0, 1, 0, 0, 0, 4, 5, 95, 104, 116, 116, 112, 4, 95, 116, 99, 112, 5, 108, 111, 99, 97, 108, 0, 0, 12, 0, 1, 0, 0, 17, 148, 0, 6, 3, 104, 117, 98, 192, 12, 192, 40, 0, 47, 128, 1, 0, 0, 0, 120, 0, 8, 192, 40, 0, 4, 0, 0, 0, 8, 192, 40, 0, 1, 128, 1, 0, 0, 0, 120, 0, 4, 192, 168, 100, 24, 192, 40, 0, 33, 128, 1, 0, 0, 0, 120, 0, 8, 0, 0, 0, 0, 216, 71, 192, 40, 192, 40, 0, 16, 128, 1, 0, 0, 17, 148, 0, 0];

    const ANSWER_MACHINE1_MDNS_PAYLOAD_ANSWERS_PART: [u8; 34] = [5, 95, 104, 116, 116, 112, 4, 95, 116, 99, 112, 5, 108, 111, 99, 97, 108, 0, 0, 12, 0, 1, 0, 0, 17, 148, 0, 6, 3, 104, 117, 98, 192, 12];
    // 3, 104, 117, 98, 192, 12, 192, 40, 0, 47, 128, 1, 0, 0, 0, 120, 0, 8, 192, 40, 0, 4, 0, 0, 0, 8, 192, 40, 0, 1, 128, 1, 0, 0, 0, 120, 0, 4, 192, 168, 100, 24, 192, 40, 0, 33, 128, 1, 0, 0, 0, 120, 0, 8, 0, 0, 0, 0, 216, 71, 192, 40, 192, 40, 0, 16, 128, 1, 0, 0, 17, 148, 0, 0];

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

    #[test]
    fn test_mdns_question_parsing() {
        let mdns_packet = MDNSPacket::parse_mdns_packet(&RESOLVE_SPOTIFY_MDNS_PAYLOAD);

        let mdns_questions = mdns_packet.unwrap().parse_mdns_questions();

        assert_eq!(mdns_questions.1.len(), 1);
        let labels = &mdns_questions.1.first().unwrap().labels;
        assert_eq!(labels.len(), 3);
        assert_eq!(labels.get(0).unwrap(), "_spotify-connect");
        assert_eq!(labels.get(1).unwrap(), "_tcp");
        assert_eq!(labels.get(2).unwrap(), "local");
    }

    #[test]
    fn test_mdns_answer_parsing() {
        let mdns_answers = MDNSPacket::parse_mdns_answers(1, &ANSWER_MACHINE1_MDNS_PAYLOAD_ANSWERS_PART, 0);

        assert_eq!(mdns_answers.1.len(), 1);
        let answer = mdns_answers.1.first().unwrap();
        assert_eq!(answer.answer_type, 0x000C); // PTR
        assert_eq!(answer.answer_class, 0x0001); // IN (Internet)
        assert_eq!(answer.ttl, 4500); // 75 minutes



        let labels = &answer.labels;
        assert_eq!(labels.len(), 3);
        assert_eq!(labels.get(0).unwrap(), "_http");
        assert_eq!(labels.get(1).unwrap(), "_tcp");
        assert_eq!(labels.get(2).unwrap(), "local");
    }
}