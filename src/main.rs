use pnet::datalink::{interfaces, channel, Channel};
use pnet::packet::{ethernet::EthernetPacket, Packet};

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
        println!("Captured packet: {:?}", ethernet_packet);
    }
}