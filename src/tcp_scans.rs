use std::net::*;

use std::sync::mpsc::{channel,Sender}; //channel
use std::thread;

use ScanConfig;
use ScanResult;
use ScanType;
use utility;

pub fn tcp_scan(scanconfig: ScanConfig)-> Vec<ScanResult> {
    let mut results: Vec<ScanResult> = Vec::new();
        for ip in scanconfig.ips {
            println!("Scanning {}",ip);
            let start = scanconfig.start_port;
            let end = scanconfig.end_port;
                let mut openports = Vec::new();
                for port in start..end {
                    let ip = utility::prep_ip(ip.to_string(), port);
                    let check = tcp_full(ip);
                    if check {
                        openports.push(port);
                    };
                }
                let mut scanresult = ScanResult {
                    ports: openports.clone(),
                    ip: ip,
                    scantype: ScanType::TcpFull,
                    is_up: false,
                };
                results.push(scanresult);
        }
    results
}
fn tcp_full(addr: String)-> bool {
        let addr = addr;
        if let Ok(_stream) = TcpStream::connect(addr) {
            return true;
        } else {
            return false;
        }
}

