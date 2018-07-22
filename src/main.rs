extern crate threadpool;
extern crate pnet;
extern crate rayon;


use std::sync::mpsc::{channel,Sender}; //channel
use std::collections::HashMap;
use rayon::prelude::*;

mod utility;
mod tcp_scans;
mod icmp_scan;

pub enum ScanType{
     TcpFull,
     TcpSyn,
     TcpNull,
     Udp,
    Ping,
    OsDetection
 }


fn main() {

    let arguments: HashMap<String,String> = utility::parse_arguments();
    let mut scan_result = Vec::new();
    /*
    println!("{},{}",arguments.contains_key("scantype"),arguments.contains_key("host"));
    for a in arguments {
        println!("{} => {}",a.0,a.1);
    }
    */
    if arguments.contains_key("scantype") && arguments.contains_key("host") {
        let scantype: ScanType = parse_scan_type(arguments.get("scantype").unwrap().to_string());
        let ip = arguments.get("host").unwrap();
        let start_port = arguments.get("start").unwrap().parse::<usize>().unwrap_or(0);
        let end_port = arguments.get("end").unwrap().parse::<usize>().unwrap_or(0);
        scan_result = scan_ports(&ip,start_port,end_port,scantype);
    } else {
        utility::exit_on_error()
    }
    /*
    for port in scan_result {
        println!("Port {} is open", port)
    }
    */
    scan_result.par_iter().map(|a| println!("Port {} is open",a));

}


fn scan_ports(ip: &str, port_beginn: usize, port_end: usize, scan_type:ScanType)-> Vec<usize> {
    let mut open_ports: Vec<usize> = vec![];
    let (tx,rx) = channel();
    for port in port_beginn..port_end {
        match scan_type {
            ScanType::TcpFull => {
                tcp_scans::port_open_tcp_full(utility::prep_ip(ip.to_string(),port), port, tx.clone())
            },
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
        "P" => {
            scantype = ScanType::Ping;
        },
        "O" => {
            scantype = ScanType::OsDetection;
        },
        "TF" => {
            scantype = ScanType::TcpFull;
        },
        "TS" => {
            scantype = ScanType::TcpSyn;
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






