use pnet::packet::Packet;
use pnet::packet::udp::UdpPacket;
use crate::mdns::types::{MDNSAnswer, MDNSMessageHeader, MDNSQuestion};
use crate::mdns::parser::{parse_mdns_answers, parse_mdns_header, parse_mdns_questions};

pub struct MDNSMessage {
    pub header: MDNSMessageHeader,
    pub questions: Vec<MDNSQuestion>,
    pub answers: Vec<MDNSAnswer>
}

impl MDNSMessage {
    pub fn get(udp_packet: &UdpPacket) -> Self
    {
        const QUESTIONS_START_INDEX: usize = 12;
        let udp_payload = udp_packet.payload();
        let header = parse_mdns_header(&udp_packet.payload()).expect("Failed to parse MDNS Message Header.");
        let questions = parse_mdns_questions(header.question_count as usize, udp_payload, QUESTIONS_START_INDEX);
        let answers = parse_mdns_answers(header.answer_count, udp_payload, QUESTIONS_START_INDEX + questions.0);
        Self {
            header: header,
            questions: questions.1,
            answers: answers.1,
        }
    }
}