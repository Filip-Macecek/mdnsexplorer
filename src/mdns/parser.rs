use crate::mdns::mdns_message::MDNSMessage;
use crate::mdns::types::{MDNSAnswer, MDNSMessageHeader, MDNSQueryClass, MDNSQuestion, MDNSRData, MDNSRecordType};
use pnet::packet::udp::UdpPacket;
use std::fmt::{Debug, Display};
use std::net::{Ipv4Addr, Ipv6Addr};
use std::str::from_utf8;
use std::string::ParseError;

pub struct ByteReader {
    bytes: Vec<u8>,
    byte_index: usize,
}

impl ByteReader {
    fn peak_byte(&mut self) -> Option<u8>
    {
        let byte = self.bytes.get(self.byte_index).map(|b| b.to_owned());
        return byte;
    }

    fn read_byte(&mut self) -> Option<u8>
    {
        let byte = self.bytes.get(self.byte_index).map(|b| b.to_owned());
        if byte.is_some() {
            self.byte_index += 1;
        }
        return byte;
    }

    fn read_u16(&mut self) -> Option<u16>
    {
        let first_byte = self.read_byte().map(|b| b as u16);
        return first_byte.map(|first| {
            let second_byte = self.read_byte().map(|b| b as u16);
            return second_byte.map(|second| first << 8 | second)
        }).flatten();
    }

    fn read_u32(&mut self) -> Option<u32>
    {
        let mut result: u32 = 0;
        for i in (0..4).rev()
        {
            result |= (self.read_byte()? as u32) << (i*8);
        }
        return Some(result);
    }

    fn read_n(&mut self, bytes_count: usize) -> Option<Vec<u8>>
    {
        let mut buffer: Vec<u8> = Vec::with_capacity(bytes_count);
        for _ in 0..bytes_count {
            buffer.push(self.read_byte()?)
        }
        Some(buffer)
    }
}

pub struct MDNSParser {
    reader: ByteReader
}

impl MDNSParser {
    pub fn new(bytes: &[u8]) -> MDNSParser
    {
        Self {
            reader: ByteReader {
                bytes: bytes.to_owned(),
                byte_index: 0
            }
        }
    }

    pub fn parse(&mut self) -> Result<MDNSMessage, ParseError>
    {
        let header = self.parse_mdns_header().expect("Could not parse MDNS header.");
        let questions = self.parse_mdns_questions(header.question_count as usize);
        let answers = self.parse_mdns_answers(header.answer_count as usize);
        Ok(MDNSMessage {
            header,
            questions,
            answers
        })
    }

    fn parse_mdns_header(&mut self) -> Option<MDNSMessageHeader> {
        let query_identifier = self.reader.read_u16().expect("Could not read MDNS query ID.");
        let flags = self.reader.read_u16().expect("Could not read MDNS flags.");
        let question_count = self.reader.read_u16().expect("Could not read MDNS Question Count.");
        let answer_count = self.reader.read_u16().expect("Could not read MDNS Answer Count.");
        let authority_count = self.reader.read_u16().expect("Could not read MDNS Authority Count.");
        let additional_count = self.reader.read_u16().expect("Could not read MDNS Additional Count.");

        let packet = MDNSMessageHeader {
            query_identifier: query_identifier,
            flags: flags,
            question_count: question_count,
            answer_count: answer_count,
            authority_count: authority_count,
            additional_count: additional_count
        };
        Some(packet)
    }

    fn parse_mdns_questions(&mut self, question_count: usize) -> Vec<MDNSQuestion>
    {
        let mut questions = Vec::with_capacity(question_count);
        for _ in 0..question_count
        {
            let name = Self::parse_name(&mut self.reader);

            let question_type = self.reader.read_u16().expect("Could not read question type.");
            // disregard the 3rd byte of the sequence -> class is either 1 or 255 (IN or ANY)
            // The first byte can contain cache flush flag which is not relevant to MDNS as per RFC 6762 - 10.2
            _ = self.reader.read_byte().expect("Could not read next byte.");
            let question_class = self.reader.read_byte().expect("Could not read question class.") as u16;

            questions.push(MDNSQuestion {
                name: name,
                question_type: MDNSRecordType::from_u16(question_type).expect(format!("Could not read question type: {}", question_type).as_str()),
                question_class: MDNSQueryClass::from_u16(question_class).expect(format!("Could not read question class: {}", question_class).as_str()),
            });
        }
        return questions
    }

    fn parse_mdns_answers(&mut self, answer_count: usize) -> Vec<MDNSAnswer>
    {
        let mut answers = Vec::with_capacity(answer_count);
        for i in 0..answer_count
        {
            println!("Parsing answer number {}", i+1);
            let name = Self::parse_name(&mut self.reader);
            println!("Parsing answer name {}", name);
            println!("current byte index: {}", self.reader.byte_index);

            let answer_type = self.reader.read_u16().expect("Could not read answer type.");

            // disregard the 1st byte of the sequence -> class is either 1 or 255 (IN or ANY)
            // The first byte can contain cache flush flag which is not relevant to MDNS as per RFC 6762 - 10.2
            _ = self.reader.read_byte().expect("Could not read next byte.");
            let answer_class = self.reader.read_byte().expect("Could not read question class.") as u16;
            let ttl = self.reader.read_u32().expect("Could not read ttl.");
            let rd_length = self.reader.read_u16().expect("Could not read record length.");

            println!("Parsing answer record length {}", rd_length);
            println!("current byte index {}", self.reader.byte_index);

            let record_type = MDNSRecordType::from_u16(answer_type).expect("Invalid answer type.");
            let rdata = self.parse_rdata(record_type, rd_length);

            answers.push(MDNSAnswer {
                name: name,
                answer_type: record_type,
                answer_class: MDNSQueryClass::from_u16(answer_class).expect("Invalid answer class."),
                ttl_seconds: ttl,
                rd_length: rd_length,
                rdata: rdata
            });
        }
        return answers
    }

    fn parse_rdata(&mut self, record_type: MDNSRecordType, rd_length: u16) -> MDNSRData
    {
        match record_type {
            MDNSRecordType::A => MDNSRData::A {
                ipv4_address: Ipv4Addr::from(self.reader.read_u32().expect("Could not read ipv4 address.")),
            },
            MDNSRecordType::NS => MDNSRData::OTHER {
                raw: self.reader.read_n(rd_length as usize).expect("Could not read other data."),
            },
            MDNSRecordType::CNAME => MDNSRData::CNAME {
                canonical_domain_name: Self::parse_name(&mut self.reader),
            },
            MDNSRecordType::SOA => MDNSRData::OTHER {
                raw: self.reader.read_n(rd_length as usize).expect("Could not read other data."),
            },
            MDNSRecordType::PTR => MDNSRData::PTR {
                domain_name: Self::parse_name(&mut self.reader),
            },
            MDNSRecordType::MX => MDNSRData::OTHER {
                raw: self.reader.read_n(rd_length as usize).expect("Could not read other data."),
            },
            MDNSRecordType::TXT => MDNSRData::TXT {
                text: "".to_string(),
            },
            MDNSRecordType::AAAA => {
                let ip_bytes_dynamic = self.reader.read_n(16).expect("Could not read ipv6 address.");
                let ip_bytes_static: Result<[u8; 16], Vec<u8>> = <[u8; 16]>::try_from(ip_bytes_dynamic);
                MDNSRData::AAAA {
                    ipv6_addr: Ipv6Addr::from(ip_bytes_static.expect("Could not read ipv6 address.")),
                }
            },
            MDNSRecordType::SRV => MDNSRData::SRV {
                priority: self.reader.read_u16().expect("Could not read priority."),
                weight: self.reader.read_u16().expect("Could not read weight."),
                port: self.reader.read_u16().expect("Could not read port."),
                target_domain_name: Self::parse_name(&mut self.reader)
            },
            MDNSRecordType::NSEC | MDNSRecordType::OPT | MDNSRecordType::ANY | MDNSRecordType::AXFR | MDNSRecordType::MAILB | MDNSRecordType::MAILA => MDNSRData::OTHER {
                raw: self.reader.read_n(rd_length as usize).expect("Could not read other data."),
            }
        }
    }

    fn parse_label(reader: &mut ByteReader) -> String
    {
        let length = reader.read_byte().expect("Could not read label length.");
        let label_raw = reader.read_n(length as usize).expect("Could not read label.");
        return from_utf8(&label_raw).expect(format!("Could not read label data: {:?}.", label_raw).as_str()).to_string();
    }

    fn parse_name(reader: &mut ByteReader) -> String
    {
        let mut labels: Vec<String> = vec![];
        let mut peaked_byte = reader.peak_byte();
        let mut referenced_name: Option<String> = None;
        while peaked_byte.is_some() && peaked_byte.unwrap() != 0 && referenced_name.is_none() {
            if Self::is_label_pointer(peaked_byte.unwrap()) {
                let pointer = Self::get_pointer(reader.read_u16().expect("Could not read pointer.")) as usize;
                let mut new_reader = ByteReader {
                    bytes: reader.bytes.to_vec(),
                    byte_index: pointer
                };
                let r_name = Self::parse_name(&mut new_reader);
                println!("Found referenced name: {}", r_name);
                if r_name.is_empty() {
                    panic!("Empty referenced name at pointer {}", pointer);
                }
                referenced_name = Some(r_name);
            }
            else {
                let label = Self::parse_label(reader);
                labels.push(label);
            }
            peaked_byte = reader.peak_byte();
        }

        // When name does not contain a pointer, the name always ends with 0 byte.
        if referenced_name.is_none()
        {
            _ = reader.read_byte().expect("Could not read the trailing 0.");
        }

        let name = match !labels.is_empty() {
            true => {
                if labels.iter().any(|l| l.is_empty()) {
                    panic!("Empty label found.")
                }
                Some(labels.join("."))
            },
            false => None
        };
        let result_name = match referenced_name {
            Some(name_to_join) => match name {
                Some(n) => Some(format!("{}.{}", n, name_to_join)),
                None => Some(name_to_join)
            }
            None => match name {
                Some(n) => Some(n),
                None => None
            }
        };
        return result_name.expect("Could not read name.");
    }

    fn is_label_pointer(byte: u8) -> bool
    {
        return (byte & 0b11000000) == 0b11000000;
    }

    fn get_pointer(double_byte: u16) -> u16
    {
        return double_byte & 0b00111111_11111111;
    }
}
