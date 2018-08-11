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
use std::time::{Duration, Instant};

use ScanConfig;
use ScanResult;
use ScanType;


pub fn ping_scan(scanconfig: ScanConfig)-> Vec<ScanResult> {
    let mut results: Vec<ScanResult> = Vec::new();
        for ip in  scanconfig.ips {
            let mut scanresult = ScanResult {
                ports: Vec::new(),
                ip: ip,
                scantype: ScanType::Ping,
                is_up: icmp_scan(&ip),
            };
            results.push(scanresult);
        }
    results
}
fn icmp_scan(dest_ip: &Ipv4Addr) -> bool {
    let protocol = Layer4(Ipv4(IpNextHeaderProtocols::Icmp));
    let (mut tx, mut rx) = transport_channel(256, protocol).unwrap();
    let mut req_buffer = [0u8; 64];
    let mut req_packet = MutableEchoRequestPacket::new(&mut req_buffer).unwrap();
    let mut req_packet = configure_icmp_packet(req_packet);
    tx.send_to(req_packet, IpAddr::V4(*dest_ip)).unwrap();
    let mut iter = icmp_packet_iter(&mut rx);
    loop {
            let (rx_packet, addr) = iter.next().unwrap();
            //println!("{:?}",rx_packet);
            if addr == *dest_ip && rx_packet.get_icmp_type() == icmp::IcmpType(0) {
                let mut echo_reply_buffer = [0u8; 64];
                let mut echo_reply_packet = MutableEchoReplyPacket::new(&mut echo_reply_buffer).unwrap();
                //println!("{:?}",echo_reply_packet);
                echo_reply_packet.clone_from(&rx_packet);
                return true
            } else {
                return false
            }
    }
    false
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