
use pnet::packet::{Packet, MutablePacket, icmp};
use pnet::packet::icmp::echo_request::MutableEchoRequestPacket;
use pnet::packet::icmp::echo_reply::MutableEchoReplyPacket;
use pnet::packet::ip::IpNextHeaderProtocols;
use pnet::transport::{icmp_packet_iter, transport_channel};
use pnet::transport::TransportProtocol::Ipv4;
use pnet::transport::TransportChannelType::Layer4;
use pnet::util::checksum;

use rand::{self, Rng};

use std::net::IpAddr;
/*
    Editor needs to run as root when opening raw sockets!
*/
pub fn icmp_scan(dest_ip: &IpAddr) -> bool {
    let protocol = Layer4(Ipv4(IpNextHeaderProtocols::Icmp));
    let (mut tx, mut rx) = transport_channel(256, protocol).unwrap();
    let mut echo_request_buffer = [0u8; 64];
    let mut echo_request_packet = MutableEchoRequestPacket::new(&mut echo_request_buffer).unwrap();
    echo_request_packet.set_icmp_type(icmp::IcmpType(8));
    let mut rng = rand::thread_rng();
    echo_request_packet.set_identifier(rng.gen::<u16>());
    echo_request_packet.set_sequence_number(1);
    let checksum = checksum(echo_request_packet.packet(), 1);
    echo_request_packet.set_checksum(checksum);

    println!("Sending echo request: {:?}", echo_request_packet);
    tx.send_to(echo_request_packet, *dest_ip).unwrap();

    let mut iter = icmp_packet_iter(&mut rx);
    loop {
        let (rx_packet, addr) = iter.next().unwrap();
        if addr == *dest_ip && rx_packet.get_icmp_type() == icmp::IcmpType(0) {
            let mut echo_reply_buffer = [0u8; 64];
            let mut echo_reply_packet = MutableEchoReplyPacket::new(&mut echo_reply_buffer).unwrap();
            echo_reply_packet.clone_from(&rx_packet);
            println!("Received echo reply, host is alive: {:?}", echo_reply_packet);
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

pub fn ping_scan(ip: String)-> bool  {
    true
}