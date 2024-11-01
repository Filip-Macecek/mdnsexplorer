use std::fmt::{Debug, Display, Formatter};
use std::net::IpAddr;
use std::str::{from_utf8_mut, Utf8Error};
use pnet::datalink::{interfaces, channel, Channel};
use pnet::packet::{ethernet::EthernetPacket, Packet};
use pnet::packet::ip::IpNextHeaderProtocols;
use pnet::packet::ipv4::Ipv4Packet;
use pnet::packet::tcp::TcpPacket;
use pnet::packet::udp::UdpPacket;

fn main() {
    // Get the list of available network interfaces
    let interfaces = interfaces();

    for interface in &interfaces {
        println!("{}", interface.description);
    }

    // Search for the default interface - the one that is
    // up, not loopback and has an IP.
    let default_interface_option = interfaces
        .iter()
        .find(|e| e.description == "Intel(R) Wi-Fi 6E AX211 160MHz");

    // Create a channel to listen for packets
    let default_interface = match default_interface_option {
        Some(interface) => interface,
        None => panic!("Default interface not found."),
    };

    // Create a channel to listen for packets
    let (_, mut rx) = match channel(&default_interface, Default::default()) {
        Ok(Channel::Ethernet(tx, rx)) => (tx, rx),
        _ => panic!("Failed to create datalink channel"),
    };

    println!("Chosen interface name: {}", default_interface.description);

    // Start capturing packets
    loop {
        let packet = rx.next().unwrap();
        let ethernet_packet = EthernetPacket::new(packet).unwrap();
        handle_ethernet_packet(&ethernet_packet);
    }
}

fn handle_ethernet_packet(eth_packet: &EthernetPacket) {
    // Check if the packet is an IPv4 packet
    if eth_packet.get_ethertype() == pnet::packet::ethernet::EtherTypes::Ipv4 {
        let ipv4_packet = Ipv4Packet::new(eth_packet.payload()).unwrap();
        handle_ipv4_packet(&ipv4_packet);
    } else {
        // println!("Not an IPv4 packet");
    }
}

fn handle_ipv4_packet(ipv4_packet: &Ipv4Packet) {
    // Extract source and destination IP addresses
    let src_ip = IpAddr::V4(ipv4_packet.get_source());
    let dst_ip = IpAddr::V4(ipv4_packet.get_destination());

    // println!("IPv4 Packet: {} -> {}", src_ip, dst_ip);

    match ipv4_packet.get_next_level_protocol() {
        IpNextHeaderProtocols::Tcp => {
            if let Some(tcp_packet) = TcpPacket::new(ipv4_packet.payload()) {
                handle_tcp_packet(&tcp_packet);
            }
        }
        IpNextHeaderProtocols::Udp => {
            if let Some(udp_packet) = UdpPacket::new(ipv4_packet.payload()) {
                handle_udp_packet(&udp_packet);
            }
        },
        _ => {}
    }
}

fn handle_tcp_packet(tcp_packet: &TcpPacket) {
    let src_port = tcp_packet.get_source();
    let dst_port = tcp_packet.get_destination();

    // println!("TCP Packet: {} -> {}", src_port, dst_port);

    // Infer application protocol based on port numbers
    // match dst_port {
    //     80 => println!("HTTP traffic"),
    //     443 => println!("HTTPS traffic"),
    //     21 => println!("FTP traffic"),
    //     25 => println!("SMTP traffic"),
    //     _ => println!("Other TCP protocol"),
    // }
}

struct MDNSPacket{
    query_identifier: u16,
    flags: u16,
    question_count: u16,
    answer_count: u16,
    authority_count: u16,
    additional_count: u16,
    questions_answers: Vec<u8>,
    questions: Vec<MDNSQuestion>
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
    fn parse_mdns_questions(&self) -> Vec<MDNSQuestion>
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

    fn parse_udp_packet(udp_packet: &UdpPacket) -> Option<MDNSPacket>
    {
        if udp_packet.get_source() != 5353 || udp_packet.get_destination() != 5353 {
            return None
        }

        let payload = udp_packet.payload();
        let query_identifier = ((payload[0] as u16) << 8) | payload[1] as u16;
        let flags = ((payload[2] as u16) << 8) | payload[3] as u16;
        let question_count = ((payload[4] as u16) << 8) | payload[5] as u16;
        let answer_count = ((payload[6] as u16) << 8) | payload[7] as u16;
        let authority_count = ((payload[8] as u16) << 8) | payload[9] as u16;
        let additional_count = ((payload[10] as u16) << 8) | payload[11] as u16;

        let packet = MDNSPacket {
            query_identifier: query_identifier,
            flags: flags,
            question_count: question_count,
            answer_count: answer_count,
            authority_count: authority_count,
            additional_count: additional_count,
            questions_answers: payload[12..payload.len()].to_vec(),
            questions: vec![]
        };
        let questions = packet.parse_mdns_questions();
        for q in &questions{
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
}

struct MDNSQuestion{
    labels: Vec<Vec<u8>>,
    question_type: u16, // first byte is terminator for labels and as such is always 0
    question_class: u16
}

fn handle_udp_packet(udp_packet: &UdpPacket) {
    let mdns_packet = MDNSPacket::parse_udp_packet(udp_packet);
    match (mdns_packet)
    {
        Some(p) => println!("{}", p),
        None => {}
    }
}
