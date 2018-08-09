extern crate threadpool;
extern crate pnet;
extern crate clap;
extern crate rand;
use std::sync::mpsc::channel;
use threadpool::ThreadPool;

use std::net::IpAddr;
use std::net::Ipv4Addr;

mod utility;
mod tcp_scans;
mod icmp_scan;
mod iana_mapping;

pub enum ScanType{
     TcpFull,
     TcpNull,
     Udp,
     Ping,
 }
pub struct ScanResult {
    ports: Vec<usize>,
    ip: Ipv4Addr,
    scantype: ScanType
}
/*
    Bugs:
    - Programm not exiting after run with threadpool.
    - Cant compile when using pnet::datalink
*/

fn main() {
    /*
    let arguments = utility::parse_arguments();
    let ports = utility::parse_ports(arguments.value_of("PORTS").unwrap().to_string());
    let ip = arguments.value_of("IP").unwrap().to_string();
    let scantype = parse_scan_type(arguments.value_of("SCANTYPE").unwrap().to_string());
    let port_beginn = ports[0].parse::<usize>().unwrap_or(0);
    let port_end = ports[1].parse::<usize>().unwrap_or(0);
    let open = threaded_scan(&ip, port_beginn, port_end, scantype, 120);
    for i in 0..1000 {
        let ip = Ipv4Addr::new(127, 0, 0, 1);
        tcp_scans::tcp_null(&ip, 80);
    }
    let file = "C:\\Users\\Tyrell Wellick\\git\\rust-projekt\\src\\test.txt".to_string();
    utility::write_to_file(file, "Hallo Welt".to_string());
    */
    let mut ports: Vec<usize>= Vec::new();
    ports.push(80);
    ports.push(443);
    ports.push(445);
    let mut ips: Vec<Ipv4Addr>= Vec::new();
    ips.push(Ipv4Addr::new(12,23,34,45));
    ips.push(Ipv4Addr::new(12,223,34,45));
    ips.push(Ipv4Addr::new(212,23,34,45));
    ips.push(Ipv4Addr::new(122,23,34,45));
    let scantype: ScanType = ScanType::TcpFull;
    let scantype1: ScanType = ScanType::TcpFull;
    let scantype2: ScanType = ScanType::TcpFull;
    let res1 = ScanResult {
        ports: ports.clone(),
        ip: ips[0],
        scantype: scantype
    };
    let res2 = ScanResult {
        ports: ports.clone(),
        ip: ips[0],
        scantype: scantype1
    };
    let res3 = ScanResult {
        ports: ports.clone(),
        ip: ips[0],
        scantype: scantype2
    };
    let mut results: Vec<ScanResult>= Vec::new();
    results.push(res1);
    results.push(res2);
    results.push(res3);
    let out = utility::prepare_output(results);
    println!("{}",out);
}
fn threaded_scan(ip: &str, port_beginn: usize, port_end: usize, scan_type:ScanType, threads: usize) -> Vec<usize> {
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
    let mut open_ports: Vec<usize> = Vec::new();
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



fn parse_scan_type(string: String) -> ScanType {
    let mut scantype: ScanType = ScanType::Ping;
    match string.as_ref() {
        "P" => {
            scantype = ScanType::Ping;
        },
        "TF" => {
            scantype = ScanType::TcpFull;
        },
        "TN" => {
            scantype = ScanType::TcpNull;
        },
        "U" => {
            scantype = ScanType::Udp;
        },
        _ => {
            println!("Error while parsing scan type");
            utility::exit_on_error();
        },
    }
    return scantype
}






