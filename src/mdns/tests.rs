#[cfg(test)]
mod tests {
    use crate::mdns::types::MDNSMessageHeader;
    use crate::mdns::parser::{parse_mdns_answers, parse_mdns_header, parse_mdns_questions};

    // id: 0, flags: 0, question_count: 1, answer_count: 0, authority_count: 0, additional_count: 0, label: _spotify-connect_tcplocal
    const RESOLVE_SPOTIFY_MDNS_PAYLOAD: [u8; 45] = [0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 16, 95, 115, 112, 111, 116, 105, 102, 121, 45, 99, 111, 110, 110, 101, 99, 116, 4, 95, 116, 99, 112, 5, 108, 111, 99, 97, 108, 0, 0, 12, 0, 1];

    const ANSWER_MACHINE1_MDNS_PAYLOAD: [u8; 114] = [0, 0, 132, 0, 0, 0, 0, 1, 0, 0, 0, 4, 5, 95, 104, 116, 116, 112, 4, 95, 116, 99, 112, 5, 108, 111, 99, 97, 108, 0, 0, 12, 0, 1, 0, 0, 17, 148, 0, 6, 3, 104, 117, 98, 192, 12, 192, 40, 0, 47, 128, 1, 0, 0, 0, 120, 0, 8, 192, 40, 0, 4, 0, 0, 0, 8, 192, 40, 0, 1, 128, 1, 0, 0, 0, 120, 0, 4, 192, 168, 100, 24, 192, 40, 0, 33, 128, 1, 0, 0, 0, 120, 0, 8, 0, 0, 0, 0, 216, 71, 192, 40, 192, 40, 0, 16, 128, 1, 0, 0, 17, 148, 0, 0];

    const ANSWER_MACHINE1_MDNS_PAYLOAD_ANSWERS_PART: [u8; 34] = [5, 95, 104, 116, 116, 112, 4, 95, 116, 99, 112, 5, 108, 111, 99, 97, 108, 0, 0, 12, 0, 1, 0, 0, 17, 148, 0, 6, 2, 104, 117, 2, 104, 117];

    const ANSWER_MACHINE1_MDNS_PAYLOAD_ANSWERS_WITH_REFERENCE_PART: [u8; 34] = [5, 95, 104, 116, 116, 112, 4, 95, 116, 99, 112, 5, 108, 111, 99, 97, 108, 0, 0, 12, 0, 1, 0, 0, 17, 148, 0, 6, 3, 104, 117, 98, 192, 0];
    // 3, 104, 117, 98, 192, 12, 192, 40, 0, 47, 128, 1, 0, 0, 0, 120, 0, 8, 192, 40, 0, 4, 0, 0, 0, 8, 192, 40, 0, 1, 128, 1, 0, 0, 0, 120, 0, 4, 192, 168, 100, 24, 192, 40, 0, 33, 128, 1, 0, 0, 0, 120, 0, 8, 0, 0, 0, 0, 216, 71, 192, 40, 192, 40, 0, 16, 128, 1, 0, 0, 17, 148, 0, 0];

    #[test]
    fn test_mdns_question_creation() {
        let mdns_packet = parse_mdns_header(&RESOLVE_SPOTIFY_MDNS_PAYLOAD);

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
        let mdns_questions = parse_mdns_questions(1, &RESOLVE_SPOTIFY_MDNS_PAYLOAD, 12);

        assert_eq!(mdns_questions.1.len(), 1);
        let labels = &mdns_questions.1.first().unwrap().labels;
        assert_eq!(labels.len(), 3);
        assert_eq!(labels.get(0).unwrap(), "_spotify-connect");
        assert_eq!(labels.get(1).unwrap(), "_tcp");
        assert_eq!(labels.get(2).unwrap(), "local");
    }

    #[test]
    fn test_mdns_answer_parsing() {
        let mdns_answers = parse_mdns_answers(1, &ANSWER_MACHINE1_MDNS_PAYLOAD_ANSWERS_PART, 0);
        let byte_reads = mdns_answers.0;
        assert_eq!(byte_reads, 34);

        assert_eq!(mdns_answers.1.len(), 1);
        let answer = mdns_answers.1.first().unwrap();
        assert_eq!(answer.answer_type, 0x000C); // PTR
        assert_eq!(answer.answer_class, 0x0001); // IN (Internet)
        assert_eq!(answer.ttl, 4500); // 75 minutes
        assert_eq!(answer.rd_length, 6);
        let rdata_labels = &answer.rdata_labels;
        assert!(rdata_labels.is_some());
        if let Some(l) = &rdata_labels {
            assert_eq!(l.get(0).unwrap(), "hu");
            assert_eq!(l.get(1).unwrap(), "hu");
        }

        let labels = &answer.labels;
        assert_eq!(labels.len(), 3);
        assert_eq!(labels.get(0).unwrap(), "_http");
        assert_eq!(labels.get(1).unwrap(), "_tcp");
        assert_eq!(labels.get(2).unwrap(), "local");
    }

    #[test]
    fn test_mdns_answer_parsing_with_rdata_compression() {
        let mdns_answers = parse_mdns_answers(1, &ANSWER_MACHINE1_MDNS_PAYLOAD_ANSWERS_WITH_REFERENCE_PART, 0);
        let byte_reads = mdns_answers.0;
        assert_eq!(byte_reads, 34);

        assert_eq!(mdns_answers.1.len(), 1);
        let answer = mdns_answers.1.first().unwrap();
        assert_eq!(answer.answer_type, 0x000C); // PTR
        assert_eq!(answer.answer_class, 0x0001); // IN (Internet)
        assert_eq!(answer.ttl, 4500); // 75 minutes
        assert_eq!(answer.rd_length, 6);
        let rdata_labels = &answer.rdata_labels;
        assert!(rdata_labels.is_some());
        if let Some(l) = &rdata_labels {
            assert_eq!(l.get(0).unwrap(), "hub");
            assert_eq!(l.get(1).unwrap(), "_http");
        }

        let labels = &answer.labels;
        assert_eq!(labels.len(), 3);
        assert_eq!(labels.get(0).unwrap(), "_http");
        assert_eq!(labels.get(1).unwrap(), "_tcp");
        assert_eq!(labels.get(2).unwrap(), "local");
    }
}