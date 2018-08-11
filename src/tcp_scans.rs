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
            let (tx,rx) = channel();
            let start = scanconfig.start_port;
            let end = scanconfig.end_port;
                let mut openports = Vec::new();


                for port in start..end {
                    let ip = utility::prep_ip(ip.to_string(), port);
                    println!("{}",ip);
                    port_open_tcp(ip,port, tx.clone());

                }
                for value in rx.iter().take(end as usize - start as usize + 1) {
                    if value.1 {
                        openports.push(value.0)
                    }
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

pub fn port_open_tcp(addr: String,port: u16,  tx: Sender<(u16,bool)>) {
    thread::spawn(move || {
        if let Ok(stream) = TcpStream::connect(addr) {
            tx.send((port, true))
        } else {
            tx.send((port, false))
        }
    });
}
