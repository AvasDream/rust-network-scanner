extern crate threadpool;
extern crate pnet;
extern crate clap;

use std::sync::mpsc::{channel,Sender}; //channel
use std::collections::HashMap;

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

    let arguments = utility::parse_arguments();
    let ports = utility::parse_ports(arguments.value_of("PORTS").unwrap().to_string());
    let ip = arguments.value_of("IP").unwrap();
    let scantype = parse_scan_type(arguments.value_of("SCANTYPE").unwrap().to_string());
    let port_beginn = ports[0].parse::<usize>().unwrap_or(0);
    let port_end = ports[1].parse::<usize>().unwrap_or(0);
    /*let scan_result = scan_ports(ip,port_beginn,port_end,scantype);
    for port in scan_result {
        println!("Port {} is open", port)
    }
    */
    let Threadpool = utility::Threadpool::new(4);
    for port in port_beginn..port_end {
        Threadpool.execute(tcp_scans::tcp_full(ip.to_string(), port))
    }
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






