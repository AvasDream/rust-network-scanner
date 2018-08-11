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
use scoped_threadpool::Pool;

use ScanConfig;
use ScanResult;
use ScanType;
use utility;

pub fn tcp_scan(scanconfig: ScanConfig)-> Vec<ScanResult> {
    let mut results: Vec<ScanResult> = Vec::new();
    let mut pool = Pool::new(4);
    pool.scoped(|scoped| {
        for ip in scanconfig.ips {
            let mut openports = Vec::new();
            for port in scanconfig.start_port..scanconfig.end_port {
                let ip = utility::prep_ip(ip.to_string(), port);
                let check = tcp_full(ip);
                if check {
                    openports.push(port);
                };
            }
            let mut scanresult = ScanResult {
                ports: openports,
                ip: ip,
                scantype: ScanType::TcpFull,
                is_up: false,
            };
            results.push(scanresult);
        }
    });
    results
}
fn tcp_full(addr: String)-> bool {
    println!("{}",addr);
    let addr = addr;
    if let Ok(stream) = TcpStream::connect(addr) {
        return true;
    } else {
        return false;
    }
}

