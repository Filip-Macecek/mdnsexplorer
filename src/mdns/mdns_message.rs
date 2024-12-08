use pnet::packet::Packet;
use pnet::packet::udp::UdpPacket;
use crate::mdns::parser::parse_mdns_message;
use crate::mdns::types::{MDNSAnswer, MDNSMessageHeader, MDNSQuestion};

pub struct MDNSMessage {
    pub header: MDNSMessageHeader,
    pub questions: Vec<MDNSQuestion>,
    pub answers: Vec<MDNSAnswer>
}

impl MDNSMessage {
    pub fn get(udp_packet: &UdpPacket) -> Self
    {
        let udp_payload = udp_packet.payload();
        let mut parser = parse_mdns_message(udp_payload);
        parser.unwrap()
    }
}