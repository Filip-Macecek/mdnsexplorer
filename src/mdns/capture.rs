use std::net::{IpAddr, Ipv4Addr};
use std::time::Duration;
use crate::mdns::mdns_message::MDNSMessage;
use pnet::datalink::{channel, interfaces, Channel, Config};
use pnet::packet::ethernet::EthernetPacket;
use pnet::packet::ip::IpNextHeaderProtocols;
use pnet::packet::ipv4::Ipv4Packet;
use pnet::packet::udp::{MutableUdpPacket, UdpPacket};
use pnet::packet::Packet;

pub fn start<F>(callback: F)
where
    F: Fn(&MDNSMessage)
{
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

    loop {
        // There is actually no way to make the call to rx.next() non-blocking on Windows.
        let packet = rx.next().unwrap();
        let ethernet_packet = EthernetPacket::new(packet).unwrap();
        let ipv4_packet = handle_ethernet_packet(&ethernet_packet);
        let mdns_packet = ipv4_packet.and_then(|p| handle_ipv4_packet(&p));
        match mdns_packet {
            Some(m) => {
                println!("{}", m.header.query_identifier);
                callback(&m);
            },
            None => {}
        }
    }
}

fn handle_ethernet_packet<'a>(eth_packet: &'a EthernetPacket<'a>) -> Option<Ipv4Packet<'a>> {
    // Check if the packet is an IPv4 packet
    let is_ipv4 = eth_packet.get_ethertype() == pnet::packet::ethernet::EtherTypes::Ipv4;
    match is_ipv4 {
        true => Some(Ipv4Packet::new(eth_packet.payload()).unwrap()),
        false => None
    }
}

fn handle_ipv4_packet(ipv4_packet: &Ipv4Packet) -> Option<MDNSMessage> {
    match ipv4_packet.get_next_level_protocol() {
        IpNextHeaderProtocols::Udp => {
            let udp_packet = UdpPacket::new(ipv4_packet.payload()).unwrap();
            return if udp_packet.get_source() == 5353 || udp_packet.get_destination() == 5353
            {
                for c in udp_packet.payload() {
                    print!("{}, ", c);
                }
                println!();
                Some(MDNSMessage::get(&udp_packet))
            } else {
                None
            }
        }
        _ => None
    }
}
