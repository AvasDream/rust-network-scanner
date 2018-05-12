extern crate threadpool;

use std::net::*;
use std::env;
use std::process; //exit(0)
use std::sync::mpsc::{channel,Sender}; //channel
use std::thread;
use std::cmp;

pub enum ScanType {
     TCP_FULL,
     UDP
 }
fn main() {
    let remote_target = "192.168.0.1";
    help();
    let open_ports = scan(remote_target,0,10000, ScanType::UDP);
    for port in open_ports {
        println!("{} is open",port)
    }

}


pub fn scan(ip: &str, port_beginn: usize, port_end: usize, scan_type:ScanType)-> Vec<usize> {

    let mut open_ports: Vec<usize> = vec![];

    let (tx,rx) = channel();
    let mut socket = UdpSocket::bind("0.0.0.0:0").unwrap();
    for port in port_beginn..port_end {
        match scan_type {
            ScanType::TCP_FULL => port_open_tcp_full(ip.to_string(), port, tx.clone()),
            ScanType::UDP => {
                port_open_udp(ip.to_string(), port, tx.clone(), socket.try_clone().unwrap())
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

pub fn port_open_udp(ip: String, port: usize, tx: Sender<(usize,bool)>, socket: UdpSocket) {
    thread::spawn(move || {
        let mut addr = prep_ip(ip,port);


        let bytes_amount = match socket.send_to(&[0;63], addr) {
            Ok(res) => res,
            Err(_)    => 666,
        };
        println!("{}",bytes_amount);
        /*
        match bytes_amount.cmp(&0) {
            cmp::Ordering::Greater => tx.send((port, true)),
            _ => ()
        }
        */
    });
}
pub fn port_open_tcp_full(ip: String, port: usize,  tx: Sender<(usize,bool)>) {
    thread::spawn(move || {
        let addr = prep_ip(ip,port);
        if let Ok(stream) = TcpStream::connect(addr) {
            tx.send((port, true))
        } else {
            tx.send((port, false))
        }
    });
}
pub fn prep_ip (ip: String, port: usize) -> String {
    let addr = ip.to_string() + ":" + &port.to_string();
    addr
}

 #[allow(dead_code)]
 fn help() {
     println!("
 _____         _      _____     _                 _      _____
| __  |_ _ ___| |_   |   | |___| |_ _ _ _ ___ ___| |_   |   __|___ ___ ___ ___ ___ ___
|    -| | |_ -|  _|  | | | | -_|  _| | | | . |  _| '_|  |__   |  _| .'|   |   | -_|  _|
|__|__|___|___|_|    |_|___|___|_| |_____|___|_| |_,_|  |_____|___|__,|_|_|_|_|___|_|

    Usage:
    ./rns <Scan Type> <Port-Range> <IP>

    Scan Types:
    -P          Ping scan
    -TF         Tcp full scan
    -TS         Tcp Syn scan
    -TN         Tcp Null scan
    -U          Udp scan
    -O          Host operation system detection

    Options:
    -iL <file>  Input from list of hosts
    -o <file>   Save output to file

    Example:
    ./rns 127.0.0.1 1-100
");
 }

 #[allow(dead_code)]
 fn parse_arguments()-> Vec<String> {
     let args: Vec<String> = env::args().collect();
     let mut ret: Vec<String> = vec![];
     println!("{}",args.len());
     match args.len() {
         1 => {
             help();
             process::exit(0);
         },
         2 => {
             help();
             process::exit(0);
         },
         3 => {
             let ip = args[1].to_string();
             let ports: Vec<&str > = args[2].split('-').collect();
             ret.push(ip);
             ret.push(ports[0].to_string());
             ret.push(ports[1].to_string());
         },
         _ => {
             help();
             process::exit(0);
         }
     }
     ret
 }
