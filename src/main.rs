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
mod udp_scan;
mod iana_mapping;

#[derive(PartialEq)]
#[derive(Clone, Copy)]
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
impl Clone for ScanConfig {
    fn clone(&self) -> ScanConfig {
        let clone = ScanConfig {
            ips: self.ips.clone(),
            start_port: self.start_port.clone(),
            end_port: self.end_port.clone(),
            scantype: self.scantype,
            to_file: self.to_file.clone(),
        };
        clone
    }
}
/*
    Bugs:
    - Programm not exiting after run with threadpool.
    - Cant compile when using pnet::datalink
*/

fn main() {
    /*
    let scanconfig = utility::get_config();

    let to_file = scanconfig.to_file.clone();
    let mut output = "".to_string();
    match scanconfig.scantype {
        ScanType::TcpFull => {

        },
        ScanType::Udp => {

        },
        ScanType::Ping => {
            let results = icmp_scan::ping_scan(scanconfig.clone());
            output = utility::prepare_output(results);
        },
    }
    if to_file != "" {
        println!("{}",output);
        utility::write_to_file(scanconfig.to_file, output);
    } else {
        println!("{}",output);
    }
    */
    let result = udp_scan::udp("192.168.0.1",0,500);
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




/* Code to reuse

    println!("{:?}",scanconfig.ips);
    println!("{:?}",scanconfig.start_port);
    println!("{:?}",scanconfig.end_port);
    println!("{:?}",scanconfig.to_file);

*/





