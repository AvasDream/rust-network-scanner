use std::thread;
use std::net::*;
use std::net::IpAddr;
use std::net::Ipv4Addr;

use pnet::packet::{Packet, MutablePacket, tcp};
use pnet::packet::ip::IpNextHeaderProtocols;
use pnet::transport::{ipv4_packet_iter, transport_channel};
use pnet::transport::TransportProtocol::Ipv4;
use pnet::transport::TransportChannelType::Layer4;
use pnet::packet::tcp::MutableTcpPacket;
use pnet::packet::tcp::Tcp;
use pnet::packet::ipv4::MutableIpv4Packet;
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

pub fn tcp_null(addr: &Ipv4Addr, port: usize)-> Option<usize> {
    let protocol = Layer4(Ipv4(IpNextHeaderProtocols::Ipv4));
    let (mut tx, mut rx) = transport_channel(256, protocol).unwrap();
    let mut tcp_request_buffer = [0u8;128];
    let mut tcp_packet = MutableTcpPacket::new(&mut tcp_request_buffer).unwrap();
    let tcp_struct = Tcp {
        source: 65002,
        destination: port as u16,
        sequence: 0,
        acknowledgement: 0,
        data_offset: 0,
        reserved: 0,
        flags: 0,
        window: 0,
        checksum: 0,
        urgent_ptr: 0,
        options: Vec::new(),
        payload: vec![0; 40],
    };

    println!("\n\nTcp struct: {:?}\n\n", tcp_struct);

    tcp_packet.populate(&tcp_struct);
    println!("Tcp packet{:?}\n\n", tcp_packet);

    let mut ipv4_packet = MutableIpv4Packet::new( tcp_packet.packet_mut()).unwrap();
    ipv4_packet.set_version(4);
    let source = Ipv4Addr::new(127, 0, 0, 1);
    ipv4_packet.set_source(source);
    ipv4_packet.set_destination(*addr);

    println!("Ipv4 Packet:{:?}\n\n",ipv4_packet);

    let send = tx.send_to(ipv4_packet, IpAddr::V4(*addr));


    println!("Send: {:?}\n\n", send);
    let mut iter = ipv4_packet_iter(&mut rx);
    loop {
        let (rx_packet, addr) = iter.next().unwrap();
        println!("Received Packet: {:?}\n\n", rx_packet);
        return Some(port);
    }
    Some(0)
}
