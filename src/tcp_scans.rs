use std::thread;
use std::net::*;
use std::net::IpAddr;

use pnet::packet::{Packet, MutablePacket, tcp};
use pnet::packet::ip::IpNextHeaderProtocols;
use pnet::transport::{icmp_packet_iter, transport_channel};
use pnet::transport::TransportProtocol::Ipv4;
use pnet::transport::TransportChannelType::Layer4;
use pnet::packet::tcp::TcpPacket;
use pnet::util::checksum;

pub fn tcp_full(addr: String, port:usize)-> Option<usize> {
    println!("{}",addr);
    let addr = addr;
    if let Ok(stream) = TcpStream::connect(addr) {
        return Some(port);
    } else {
        None
    }
}

pub fn tcp_null(addr: &IpAddr, port: usize)-> Option<usize> {
    let protocol = Layer4(Ipv4(IpNextHeaderProtocols::Tcp));
    let (mut tx, mut rx) = transport_channel(256, protocol).unwrap();
    let mut tcp_request_buffer = [0u8;64];
    let mut packet = TcpPacket::new(&mut tcp_request_buffer).unwrap();
    //let mut p2 = MutablePacket::packet();
    println!("{:?}",packet);
    //tx.send_to(packet, *addr).unwrap();
    Some(0)
}