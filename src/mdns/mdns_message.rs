use std::net::{IpAddr, Ipv4Addr};
use crate::mdns::parser::parse_mdns_message;
use crate::mdns::types::{MDNSAnswer, MDNSMessageHeader, MDNSQuestion};
use pnet::packet::udp::UdpPacket;
use pnet::packet::Packet;
use time::PrimitiveDateTime;

pub struct MDNSMessageReceivedEvent {
    pub received_datetime: PrimitiveDateTime,
    pub message: MDNSMessage,
    pub source_ip: Ipv4Addr,
    pub destination_ip: Ipv4Addr
}

#[derive(Clone)]
pub struct MDNSMessage {
    pub header: MDNSMessageHeader,
    pub questions: Vec<MDNSQuestion>,
    pub answers: Vec<MDNSAnswer>
}

impl MDNSMessage {
    pub fn get(udp_packet: &UdpPacket) -> Self
    {
        let udp_payload = udp_packet.payload();
        let parser = parse_mdns_message(udp_payload);
        parser.unwrap()
    }
}