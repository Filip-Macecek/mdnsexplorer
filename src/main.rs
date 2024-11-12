mod mdns;

use crate::mdns::mdns::MDNSPacket;
use pnet::datalink::{channel, interfaces, Channel};
use pnet::packet::ip::IpNextHeaderProtocols;
use pnet::packet::ipv4::Ipv4Packet;
use pnet::packet::udp::UdpPacket;
use pnet::packet::{ethernet::EthernetPacket, Packet};
use std::net::IpAddr;

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
        let ipv4_packet = handle_ethernet_packet(&ethernet_packet);
        let mdns_packet = ipv4_packet.and_then(|p| handle_ipv4_packet(&p));
        match mdns_packet {
            Some(p) => println!("{}", p),
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

fn handle_ipv4_packet(ipv4_packet: &Ipv4Packet) -> Option<MDNSPacket> {
    // Extract source and destination IP addresses
    let src_ip = IpAddr::V4(ipv4_packet.get_source());
    let dst_ip = IpAddr::V4(ipv4_packet.get_destination());

    // println!("IPv4 Packet: {} -> {}", src_ip, dst_ip);

    match ipv4_packet.get_next_level_protocol() {
        IpNextHeaderProtocols::Udp => {
            let udp_packet = UdpPacket::new(ipv4_packet.payload()).unwrap();
            return if MDNSPacket::is_mdns_packet(&udp_packet) {
                for c in udp_packet.payload() {
                    print!("{}", c);
                }
                println!();
                MDNSPacket::parse_mdns_packet(&udp_packet.payload())
            } else {
                None
            }
        }
        _ => None
    }
}