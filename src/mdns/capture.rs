use crate::mdns::mdns_message::{MDNSMessage, MDNSMessageReceivedEvent};
use pnet::datalink::{channel, Channel, NetworkInterface};
use pnet::packet::ethernet::EthernetPacket;
use pnet::packet::ip::IpNextHeaderProtocols;
use pnet::packet::ipv4::Ipv4Packet;
use pnet::packet::udp::UdpPacket;
use pnet::packet::Packet;
use time::{OffsetDateTime, PrimitiveDateTime};

pub fn start<F>(interface: &NetworkInterface, callback: F)
where
    F: Fn(&MDNSMessageReceivedEvent)
{
    // Create a channel to listen for packets
    let (_, mut rx) = match channel(&interface, Default::default()) {
        Ok(Channel::Ethernet(tx, rx)) => (tx, rx),
        _ => panic!("Failed to create datalink channel"),
    };

    println!("Chosen interface name: {}", &interface.description);

    loop {
        // There is actually no way to make the call to rx.next() non-blocking on Windows.
        let packet = rx.next().unwrap();
        let now = OffsetDateTime::now_utc();
        let ethernet_packet = EthernetPacket::new(packet).unwrap();
        let ipv4_packet = handle_ethernet_packet(&ethernet_packet);
        match ipv4_packet {
            Some(p) => {
                let source_ip = p.get_source();
                let destination_ip = p.get_destination();
                let mdns_packet = handle_ipv4_packet(&p);
                match mdns_packet {
                    Some(m) => {
                        callback(&MDNSMessageReceivedEvent {
                            received_datetime: PrimitiveDateTime::new(now.date(), now.time()),
                            message: m,
                            source_ip: source_ip,
                            destination_ip: destination_ip,
                        });
                    },
                    None => {}
                }
            }
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
            if udp_packet.get_source() == 5353 || udp_packet.get_destination() == 5353
            {
                Some(MDNSMessage::get(&udp_packet))
            } else {
                None
            }
        }
        _ => None
    }
}
