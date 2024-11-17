mod tests;

pub mod mdns {
    use std::fmt::{Display, Formatter};
    use std::str::{from_utf8, from_utf8_mut};
    use pnet::packet::udp::UdpPacket;

    pub struct MDNSQuestion{
        pub labels_raw: Vec<Vec<u8>>,
        pub labels: Vec<String>,
        pub question_type: u16, // first byte is terminator for labels and as such is always 0
        pub question_class: u16
    }

    pub struct MDNSAnswer{
        pub labels_raw: Vec<Vec<u8>>,
        pub labels: Vec<String>,
        pub answer_type: u16, // first byte is terminator for labels and as such is always 0
        pub answer_class: u16,
        pub ttl: u32,
        pub rd_length: u16,
        pub rdata_raw: Vec<u8>
    }
    
    pub struct MDNSPacket{
        pub query_identifier: u16,
        pub flags: u16,
        pub question_count: u16,
        pub answer_count: u16,
        pub authority_count: u16,
        pub additional_count: u16,
        pub questions_answers: Vec<u8>,
        pub questions: Vec<MDNSQuestion>,
        pub answers: Vec<MDNSAnswer>
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
                questions: vec![],
                answers: vec![]
            };
            // let (last_questions_byte, questions) = packet.parse_mdns_questions();
            // for q in &questions {
            //     for label in &q.labels_raw {
            //         let a: &mut Vec<u8> = &mut label.clone();
            //         let s = from_utf8_mut(a);
            //         match s {
            //             Ok(ok) => {
            //                 print!("{}", ok);
            //             }
            //             Err(_) => {
            //                 print!("Error parsing label.");
            //             }
            //         }
            //     }
            //     println!()
            // }
            // let (last_answers_byte, answers) = MDNSPacket::parse_mdns_answers(packet.answer_count, &packet.questions_answers, last_questions_byte + 1);
            Some(packet)
        }

        pub fn parse_mdns_questions(&self) -> (usize, Vec<MDNSQuestion>)
        {
            let mut questions = Vec::with_capacity(self.question_count as usize);
            let mut byte_index: usize = 0;
            let label_length = 0;
            let label = 1;
            for i in 0..self.question_count
            {
                let mut current_byte = self.questions_answers[byte_index];
                let mut state = label_length;
                let mut labels_raw: Vec<Vec<u8>> = Vec::new();
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
                            labels_raw.push(label_array.clone());
                            state = label_length;
                        }
                    }
                    byte_index += 1;
                    current_byte = self.questions_answers[byte_index];
                }

                let mut labels_str = Vec::new();
                for label_raw in labels_raw.clone() {
                    let s = from_utf8(&label_raw).unwrap();
                    labels_str.push(s.to_string())
                }

                let question_type = ((current_byte as u16) << 8) | (self.questions_answers[byte_index + 1] as u16);
                let question_class = ((self.questions_answers[byte_index + 2] as u16) << 8) | (self.questions_answers[byte_index + 3] as u16);

                questions.push(MDNSQuestion{
                    labels_raw: labels_raw.clone(),
                    labels: labels_str.iter().map(|l| l.to_string()).collect(),
                    question_type: question_type,
                    question_class: question_class
                });
                byte_index += 4;
            }
            return (byte_index - 1, questions);
        }

        pub fn parse_mdns_answers(answer_count: u16, bytes: &[u8], start_byte: usize) -> (usize, Vec<MDNSAnswer>)
        {
            let mut answers = Vec::with_capacity(answer_count as usize);
            let mut byte_index: usize = start_byte;
            let label_length = 0;
            let label = 1;
            for i in 0..answer_count
            {
                let mut current_byte = bytes[byte_index];
                let mut state = label_length;
                let mut labels_raw: Vec<Vec<u8>> = Vec::new();
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
                            labels_raw.push(label_array.clone());
                            state = label_length;
                        }
                    }
                    byte_index += 1;
                    current_byte = bytes[byte_index];
                }

                byte_index += 1; // skip terminating 0

                let mut labels_str = Vec::new();
                for label_raw in labels_raw.clone() {
                    let s = from_utf8(&label_raw).unwrap();
                    labels_str.push(s.to_string())
                }

                let answer_type = ((current_byte as u16) << 8) | (bytes[byte_index + 1] as u16);
                let answer_class = ((bytes[byte_index + 2] as u16) << 8) | (bytes[byte_index + 3] as u16);
                let ttl: u32 = (((bytes[byte_index + 4] as u32) << 24)
                    | ((bytes[byte_index + 5] as u32) << 16)
                    | (bytes[byte_index + 6] as u32) << 8)
                    | (bytes[byte_index + 7] as u32);
                let rd_length: u16 = ((bytes[byte_index + 8] as u16) << 8) | (bytes[byte_index + 9] as u16);
                let mut rdata_raw: Vec<u8> = Vec::with_capacity(rd_length as usize);
                
                byte_index += 10;
                for i in 0..rd_length {
                    rdata_raw.push(bytes[byte_index+i as usize])    
                }
                
                answers.push(MDNSAnswer{
                    labels_raw: labels_raw.clone(),
                    labels: labels_str.iter().map(|l| l.to_string()).collect(),
                    answer_type: answer_type,
                    answer_class: answer_class,
                    ttl: ttl,
                    rd_length: rd_length,
                    rdata_raw: rdata_raw
                });
                byte_index = (rd_length + 1) as usize;
            }
            return (byte_index - 1, answers);
        }

        // pub fn parse_mdns_answers(&self) -> Vec<MDNSAnswer>
        // {
        //     let mut questions = Vec::with_capacity(self.question_count as usize);
        //     let mut byte_index = 0;
        //     let label_length = 0;
        //     let label = 1;
        //     for i in 0..self.question_count
        //     {
        //         let mut current_byte = self.questions_answers[byte_index];
        //         let mut state = label_length;
        //         let mut labels_raw: Vec<Vec<u8>> = Vec::new();
        //         let mut label_array: Vec<u8> = Vec::with_capacity(0);
        //         while current_byte != 0 {
        //             if state == label_length {
        //                 let length = current_byte;
        //                 label_array = Vec::with_capacity(length as usize);
        //                 state = label;
        //             }
        //             else if state == label {
        //                 label_array.push(current_byte);
        //                 if label_array.capacity() == label_array.len(){
        //                     labels_raw.push(label_array.clone());
        //                     state = label_length;
        //                 }
        //             }
        //             byte_index += 1;
        //             current_byte = self.questions_answers[byte_index];
        //         }
        //
        //         let mut labels_str = Vec::new();
        //         for label_raw in labels_raw.clone() {
        //             let s = from_utf8(&label_raw).unwrap();
        //             labels_str.push(s.to_string())
        //         }
        //
        //         let question_type = ((current_byte as u16) << 8) | (self.questions_answers[byte_index + 1] as u16);
        //         let question_class = ((self.questions_answers[byte_index + 2] as u16) << 8) | (self.questions_answers[byte_index + 3] as u16);
        //
        //         questions.push(MDNSQuestion{
        //             labels_raw: labels_raw.clone(),
        //             labels: labels_str.iter().map(|l| l.to_string()).collect(),
        //             question_type: question_type,
        //             question_class: question_class
        //         });
        //         byte_index += 4;
        //     }
        //     return questions;
        // }
    }
}