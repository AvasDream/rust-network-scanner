extern crate threadpool;




use std::sync::mpsc::{channel,Sender}; //channel



mod utility;
mod tcp_scans;

pub enum ScanTypePorts {
     TcpFull,
     TcpSyn,
     TcpNull,
     Udp,
 }
pub enum ScanTypeHosts {
    Ping,
    OsDetection
}
fn main() {
    utility::help();
}


fn scan_ports(ip: &str, port_beginn: usize, port_end: usize, scan_type:ScanTypePorts)-> Vec<usize> {
    let mut open_ports: Vec<usize> = vec![];
    let (tx,rx) = channel();
    for port in port_beginn..port_end {
        match scan_type {
            ScanTypePorts::TcpFull => tcp_scans::port_open_tcp_full(utility::prep_ip(ip.to_string(),port), port, tx.clone()),
            ScanTypePorts::Udp => {
                unimplemented!()
            },
            ScanTypePorts::TcpSyn => {
                unimplemented!()
            },
            ScanTypePorts::TcpNull => {
                unimplemented!()
            },
            ScanTypePorts::Udp => {
                unimplemented!()
            },
        }
        println!("Scanning Port {} on {}",port,ip);
    }
    for value in rx.iter().take(port_end - port_beginn) {
        if value.1 {
            open_ports.push(value.0)
        }
    }
    open_ports
}









