#[cfg(test)]
pub mod parser_tests {
    use crate::mdns::parser::{parse_mdns_header, parse_mdns_message, parse_name, ByteReader};

    #[test]
    fn parse_header__reads_the_header_and_only_the_header()
    {
        let mut reader = ByteReader {
            bytes: [0, 0, 132, 0, 0, 0, 0, 5, 0, 0, 0, 0, 5, 190].try_into().unwrap(),
            byte_index: 0
        };

        let result = parse_mdns_header(&mut reader);

        assert_eq!(reader.byte_index, 12);
        assert_eq!(result.is_some(), true);
        let header = result.unwrap();
    }

    #[test]
    fn parse_header__returns_empty_when_missing_bytes()
    {
        let mut reader = ByteReader {
            bytes: [0, 0, 132, 0, 0, 0, 0, 5, 0, 0, 0].try_into().unwrap(),
            byte_index: 0
        };

        let result = parse_mdns_header(&mut reader);

        assert_eq!(reader.byte_index, 11);
        assert_eq!(result.is_some(), false);
    }

    #[test]
    fn parse_name__when_multiple_labels__parses_correctly()
    {
        let mut reader = ByteReader {
            bytes: [16, 95, 115, 112, 111, 116, 105, 102, 121, 45, 99, 111, 110, 110, 101, 99, 116, 4, 95, 116, 99, 112, 5, 108, 111, 99, 97, 108, 0].try_into().unwrap(),
            byte_index: 0
        };

        let result = parse_name(&mut reader);

        assert_eq!(reader.byte_index, 29);
        assert_eq!(result, "_spotify-connect._tcp.local");
    }

    #[test]
    fn parse_name__when_single_label__parses_correctly()
    {
        let mut reader = ByteReader {
            bytes: [16, 95, 115, 112, 111, 116, 105, 102, 121, 45, 99, 111, 110, 110, 101, 99, 116, 0].try_into().unwrap(),
            byte_index: 0
        };

        let result = parse_name(&mut reader);

        assert_eq!(reader.byte_index, 18);
        assert_eq!(result, "_spotify-connect");
    }

    #[test]
    fn parse_name__when_pointer_is_present__parses_correctly()
    {
        let mut reader = ByteReader {
            bytes: [16, 95, 115, 112, 111, 116, 105, 102, 121, 45, 99, 111, 110, 110, 101, 99, 116, 0, 192, 0].try_into().unwrap(),
            byte_index: 18
        };

        let result = parse_name(&mut reader);

        assert_eq!(reader.byte_index, 20);
        assert_eq!(result, "_spotify-connect");
    }

    #[test]
    fn parse_name__reads_trailing_0()
    {
        let mut reader = ByteReader {
            bytes: [5, 95, 104, 116, 116, 112, 0].try_into().unwrap(),
            byte_index: 0
        };

        let result = parse_name(&mut reader);

        assert_eq!(reader.byte_index, 7);
    }

    // id: 0, flags: 0, question_count: 1, answer_count: 0, authority_count: 0, additional_count: 0, label: _spotify-connect_tcplocal
    const RESOLVE_SPOTIFY_MDNS_PAYLOAD: [u8; 45] = [0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 16, 95, 115, 112, 111, 116, 105, 102, 121, 45, 99, 111, 110, 110, 101, 99, 116, 4, 95, 116, 99, 112, 5, 108, 111, 99, 97, 108, 0, 0, 12, 0, 1];

    // const ANSWER_MACHINE1_MDNS_PAYLOAD: [u8; 114] = [0, 0, 132, 0, 0, 0, 0, 1, 0, 0, 0, 4, 5, 95, 104, 116, 116, 112, 4, 95, 116, 99, 112, 5, 108, 111, 99, 97, 108, 0, 0, 12, 0, 1, 0, 0, 17, 148, 0, 6, 3, 104, 117, 98, 192, 12, 192, 40, 0, 47, 128, 1, 0, 0, 0, 120, 0, 8, 192, 40, 0, 4, 0, 0, 0, 8, 192, 40, 0, 1, 128, 1, 0, 0, 0, 120, 0, 4, 192, 168, 100, 24, 192, 40, 0, 33, 128, 1, 0, 0, 0, 120, 0, 8, 0, 0, 0, 0, 216, 71, 192, 40, 192, 40, 0, 16, 128, 1, 0, 0, 17, 148, 0, 0];

    const ANSWER_MACHINE1_MDNS_PAYLOAD_ANSWERS_PART: [u8; 34] = [5, 95, 104, 116, 116, 112, 4, 95, 116, 99, 112, 5, 108, 111, 99, 97, 108, 0, 0, 12, 0, 1, 0, 0, 17, 148, 0, 6, 2, 104, 117, 2, 104, 117];

    const ANSWER_MACHINE1_MDNS_PAYLOAD_ANSWERS_WITH_REFERENCE_PART: [u8; 34] = [5, 95, 104, 116, 116, 112, 4, 95, 116, 99, 112, 5, 108, 111, 99, 97, 108, 0, 0, 12, 0, 1, 0, 0, 17, 148, 0, 6, 3, 104, 117, 98, 192, 0];
    // 3, 104, 117, 98, 192, 12, 192, 40, 0, 47, 128, 1, 0, 0, 0, 120, 0, 8, 192, 40, 0, 4, 0, 0, 0, 8, 192, 40, 0, 1, 128, 1, 0, 0, 0, 120, 0, 4, 192, 168, 100, 24, 192, 40, 0, 33, 128, 1, 0, 0, 0, 120, 0, 8, 0, 0, 0, 0, 216, 71, 192, 40, 192, 40, 0, 16, 128, 1, 0, 0, 17, 148, 0, 0];

    const MDNS_ANSWER_1: [u8; 114] = [
        // Header
        0, 0, 132, 0, 0, 0, 0, 5, 0, 0, 0, 0, // 0
        // Answer #1 - PTR Record _http._tcp.local.
        5, 95, 104, 116, 116, 112, 4, 95, 116, 99, 112, 5, 108, 111, 99, 97, 108, 0, // 12
        // info
        0, 12, 0, 1, 0, 0, 17, 148, 0, 6, // 30
        // rdata
        3, 104, 117, 98, 192, 12, // 40
        // Answer #2 - SRV Record
        192, 40, // 46
        // info
        0, 33, 128, 1, 0, 0, 0, 120, 0, 8, // 48
        0, 0, 0, 0,211, 149, 192, 40,
        192, 40, 0, 16, 128, 1, 0, 0, 17, 148, 0, 0, 192, 40,
        0, 47, 128, 1, 0, 0, 0, 120, 0, 8, 192, 40, 0, 4, 0, 0, 0, 8, 192, 40, 0, 1, 128, 1, 0, 0, 0, 120, 0, 4, 192, 168, 100, 25
    ];

    const MDNS_ANSWER_2: [u8; 100] = [
        // Header
        0, 0, 132, 0, 0, 0, 0, 4, 0, 0, 0, 0,
        // first answers labels
        3, 104, 117, 98, 5, 95, 104, 116, 116, 112, 4, 95, 116, 99, 112, 5, 108, 111, 99, 97, 108, 0,
        // first answer data info
        0, 16, 128, 1, 0, 0, 17, 148, 0, 0,
        192, 12,
        // second answer
        0, 1, 128, 1, 0, 0, 0, 120, 0, 4, 192, 168, 100, 24, 192, 12, 0, 47, 128, 1, 0, 0, 0, 120, 0, 8, 192, 12, 0, 4, 0, 0, 0, 8, 192, 12, 0, 33, 128, 1, 0, 0, 0, 120, 0, 8, 0, 0, 0, 0, 227, 155, 192, 12];

    const MDNS_ANSWER_3: [u8; 52] = [
        0, 0, 0, 0, 0, 1, 0, 1, 0, 0, 0, 0,
        5, 95, 104, 116, 116, 112, 4, 95, 116, 99, 112, 5, 108, 111, 99, 97, 108, 0,
        0, 12, 0, 1, 192, 12, 0, 12, 0, 1,
        0, 0, 17, 147, 0, 6, 3, 104, 117, 98, 192, 12];

    #[test]
    fn parse_header_tests() {
        let mdns_message_result = parse_mdns_message(&RESOLVE_SPOTIFY_MDNS_PAYLOAD);

        assert!(mdns_message_result.is_ok());
        let mdns_message = mdns_message_result.unwrap();
        assert_eq!(mdns_message.header.query_identifier, 0);
        assert_eq!(mdns_message.header.question_count, 1);
        assert_eq!(mdns_message.header.answer_count, 0);
        assert_eq!(mdns_message.header.authority_count, 0);
        assert_eq!(mdns_message.header.additional_count, 0);

        assert_eq!(mdns_message.questions.len(), 1);
        assert_eq!(mdns_message.questions.first().unwrap().name, "_spotify-connect._tcp.local");
        // TODO: Question type and class assert.
    }

    #[test]
    fn test_mdns_answer1() {
        let mdns_message = parse_mdns_message(&MDNS_ANSWER_1).unwrap();

        assert_eq!(mdns_message.answers.len(), 5);
        // TODO: Complete the assert section.
    }
}