extern crate threadpool;




use std::sync::mpsc::{channel,Sender}; //channel



mod utility;
mod tcp_scans;

pub enum ScanType{
     TcpFull,
     TcpSyn,
     TcpNull,
     Udp,
    Ping,
    OsDetection
 }


fn main() {

    let arguments: Vec<String> = utility::parse_arguments();
    //let ip = &arguments[1];
    let scantype: ScanType = parse_scan_type(arguments.iter().nth(0).unwrap().to_string());
    for arg in arguments {
        println!("{}",arg);
    }


}


fn scan_ports(ip: &str, port_beginn: usize, port_end: usize, scan_type:ScanType)-> Vec<usize> {
    let mut open_ports: Vec<usize> = vec![];
    let (tx,rx) = channel();
    for port in port_beginn..port_end {
        match scan_type {
            ScanType::TcpFull => tcp_scans::port_open_tcp_full(utility::prep_ip(ip.to_string(),port), port, tx.clone()),
            ScanType::Udp => {
                unimplemented!()
            },
            ScanType::TcpSyn => {
                unimplemented!()
            },
            ScanType::TcpNull => {
                unimplemented!()
            },
            ScanType::Udp => {
                unimplemented!()
            },
            ScanType::Ping => {
                unimplemented!()
            },
            ScanType::OsDetection => {
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


fn parse_scan_type(string: String) -> ScanType {
    let mut scantype: ScanType = ScanType::Ping;
    match string.as_ref() {
        "-P" => {
            scantype = ScanType::Ping;
        },
        "-O" => {
            scantype = ScanType::OsDetection;
        },
        "-TF" => {
            scantype = ScanType::TcpFull;
        },
        "-TS" => {
            scantype = ScanType::TcpSyn;
        },
        "-TN" => {
            scantype = ScanType::TcpNull;
        },
        "-U" => {
            scantype = ScanType::Udp;
        },
        _ => {
            println!("Error while parsing scan type");
            utility::exit_on_error();
        },
    }
    return scantype
}






