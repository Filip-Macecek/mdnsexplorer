mod tests;

pub mod mdns {
    use std::fmt::{Display, Formatter};
    use std::str::from_utf8_mut;
    use pnet::packet::udp::UdpPacket;

    pub struct MDNSQuestion{
        pub labels: Vec<Vec<u8>>,
        pub question_type: u16, // first byte is terminator for labels and as such is always 0
        pub question_class: u16
    }
    
    pub struct MDNSPacket{
        pub query_identifier: u16,
        pub flags: u16,
        pub question_count: u16,
        pub answer_count: u16,
        pub authority_count: u16,
        pub additional_count: u16,
        pub questions_answers: Vec<u8>,
        pub questions: Vec<MDNSQuestion>
    }

    impl Display for MDNSPacket {
        fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
            write!(f, "id: {}, flags: {}, question_count: {}, answer_count: {}, authority_count: {}, additional_count: {}",
                   self.query_identifier,
                   self.flags,
                   self.question_count,
                   self.answer_count,
                   self.authority_count,
                   self.additional_count
            )
        }
    }

    impl MDNSPacket {
        pub fn is_mdns_packet(udp_packet: &UdpPacket) -> bool {
            udp_packet.get_source() == 5353 || udp_packet.get_destination() == 5353
        }

        pub fn parse_mdns_packet(udp_payload: &[u8]) -> Option<MDNSPacket> {
            let query_identifier = ((udp_payload[0] as u16) << 8) | udp_payload[1] as u16;
            let flags = ((udp_payload[2] as u16) << 8) | udp_payload[3] as u16;
            let question_count = ((udp_payload[4] as u16) << 8) | udp_payload[5] as u16;
            let answer_count = ((udp_payload[6] as u16) << 8) | udp_payload[7] as u16;
            let authority_count = ((udp_payload[8] as u16) << 8) | udp_payload[9] as u16;
            let additional_count = ((udp_payload[10] as u16) << 8) | udp_payload[11] as u16;

            let packet = MDNSPacket {
                query_identifier: query_identifier,
                flags: flags,
                question_count: question_count,
                answer_count: answer_count,
                authority_count: authority_count,
                additional_count: additional_count,
                questions_answers: udp_payload[12..udp_payload.len()].to_vec(),
                questions: vec![]
            };
            let questions = packet.parse_mdns_questions();
            for q in &questions {
                for label in &q.labels {
                    let a: &mut Vec<u8> = &mut label.clone();
                    let s = from_utf8_mut(a);
                    match s {
                        Ok(ok) => {
                            print!("{}", ok);
                        }
                        Err(_) => {
                            print!("Error parsing label.");
                        }
                    }
                }
                println!()
            }
            Some(packet)
        }

        pub fn parse_mdns_questions(&self) -> Vec<MDNSQuestion>
        {
            let mut questions = Vec::with_capacity(self.question_count as usize);
            let mut byte_index = 0;
            let label_length = 0;
            let label = 1;
            for i in 0..self.question_count
            {
                let mut current_byte = self.questions_answers[byte_index];
                let mut state = label_length;
                let mut labels = Vec::new();
                let mut label_array: Vec<u8> = Vec::with_capacity(0);
                while current_byte != 0 {
                    if state == label_length {
                        let length = current_byte;
                        label_array = Vec::with_capacity(length as usize);
                        state = label;
                    }
                    else if state == label {
                        label_array.push(current_byte);
                        if label_array.capacity() == label_array.len(){
                            labels.push(label_array.clone());
                            state = label_length;
                        }
                    }
                    byte_index += 1;
                    current_byte = self.questions_answers[byte_index];
                }

                let question_type = ((current_byte as u16) << 8) | (self.questions_answers[byte_index + 1] as u16);
                let question_class = ((self.questions_answers[byte_index + 2] as u16) << 8) | (self.questions_answers[byte_index + 3] as u16);

                questions.push(MDNSQuestion{
                    labels: labels.clone(),
                    question_type: question_type,
                    question_class: question_class
                });
                byte_index += 4;
            }
            return questions;
        }
    }
}