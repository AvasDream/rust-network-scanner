extern crate threadpool;
extern crate pnet;
extern crate clap;
extern crate rand;
use std::sync::mpsc::channel;
use threadpool::ThreadPool;

use std::net::IpAddr;
use std::net::Ipv4Addr;
use std::path::Path;

mod utility;
mod tcp_scans;
mod icmp_scan;
mod iana_mapping;

#[derive(PartialEq)]
pub enum ScanType{
     TcpFull,
     Udp,
     Ping,
 }
pub struct ScanResult {
    ports: Vec<u16>,
    ip: Ipv4Addr,
    scantype: ScanType,
    is_up: bool
}
pub struct ScanConfig {
    ips: Vec<Ipv4Addr>,
    start_port: u16,
    end_port: u16,
    scantype: ScanType,
    to_file: String,
}
/*
    Bugs:
    - Programm not exiting after run with threadpool.
    - Cant compile when using pnet::datalink
*/

fn main() {
    let scanconfig = utility::get_config();
    println!("{:?}",scanconfig.ips);
    println!("{:?}",scanconfig.start_port);
    println!("{:?}",scanconfig.end_port);
    println!("{:?}",scanconfig.to_file);
}
fn threaded_scan(ip: &str, port_beginn: u16, port_end: u16, scan_type:ScanType, threads: usize) -> Vec<u16> {
    let n_workers = threads;
    let pool = ThreadPool::new(n_workers);

    let (tx, rx) = channel();

    for port in port_beginn..port_end {
        let tx = tx.clone();
        let ip = utility::prep_ip(ip.to_string(),port);


        pool.execute(move|| {
            let p = tcp_scans::tcp_full(ip,port);
            if p != None {
                tx.send(p).expect("error while sending port");
            } else {
                tx.send(Some(0)).expect("mimimi");
            }
        });
    }
    println!("Active count: {}",pool.active_count());
    let mut open_ports: Vec<u16> = Vec::new();
    for received in rx.iter() {
        let value = received.unwrap();
        if value != 0  {
            open_ports.push(received.unwrap());
            println!("Port open: {:?}",received.unwrap());
            println!("Active count: {}",pool.active_count());
        }
    }


    open_ports
}










