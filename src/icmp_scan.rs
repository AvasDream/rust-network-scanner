use pnet::packet::{Packet, MutablePacket, icmp};
use pnet::packet::ip::IpNextHeaderProtocols;
use pnet::util::checksum;
use rand::{self, Rng};
use pnet::packet::icmp::echo_request::MutableEchoRequestPacket;
use pnet::packet::icmp::echo_reply::MutableEchoReplyPacket;
use std::net::Ipv4Addr;
use std::net::IpAddr;
use pnet::transport::{icmp_packet_iter, transport_channel};
use pnet::transport::TransportProtocol::Ipv4;
use pnet::transport::TransportChannelType::Layer4;
/*
    Editor needs to run as root when opening raw sockets!
*/
pub fn icmp_scan(dest_ip: &Ipv4Addr) -> bool {
    let protocol = Layer4(Ipv4(IpNextHeaderProtocols::Icmp));
    let (mut tx, mut rx) = transport_channel(256, protocol).unwrap();
    let mut req_buffer = [0u8; 64];
    let mut req_packet = MutableEchoRequestPacket::new(&mut req_buffer).unwrap();
    let mut req_packet = configure_icmp_packet(req_packet);
    //println!("Sending echo request: {:?}", req_packet);
    tx.send_to(req_packet, IpAddr::V4(*dest_ip)).unwrap();

    let mut iter = icmp_packet_iter(&mut rx);
    loop {
        let (rx_packet, addr) = iter.next().unwrap();
        if addr == *dest_ip && rx_packet.get_icmp_type() == icmp::IcmpType(0) {
            let mut echo_reply_buffer = [0u8; 64];
            let mut echo_reply_packet = MutableEchoReplyPacket::new(&mut echo_reply_buffer).unwrap();
            echo_reply_packet.clone_from(&rx_packet);
            //println!("Received echo reply, host is alive: {:?}", echo_reply_packet);
            return true
        } else {
            if rx_packet.get_icmp_type() == icmp::IcmpType(3) {
                println!("ICMP reply: DESTINATION UNREACHABLE - {:?}", rx_packet);
            } else {
                println!("ICMP reply: {:?}", rx_packet);
            }
            return false
        }
    }
}
fn configure_icmp_packet(mut req_packet: MutableEchoRequestPacket)-> MutableEchoRequestPacket {
    req_packet.set_icmp_type(icmp::IcmpType(8));
    let mut rng = rand::thread_rng();
    req_packet.set_identifier(rng.gen::<u16>());
    req_packet.set_sequence_number(1);
    let checksum = checksum(req_packet.packet(), 1);
    req_packet.set_checksum(checksum);
    req_packet
}