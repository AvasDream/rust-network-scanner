use std::thread;
use std::net::*;
use std::sync::mpsc::{channel,Sender}; //channel

use std::net::UdpSocket;
use std::cmp::Ordering;
use std::str::from_utf8;

use ScanResult;
use ScanConfig;
use utility;

pub fn udp(host: &str, first: u16, last: u16) -> Vec<u32> {
    let mut result = Vec::new();

    let socket = UdpSocket::bind("0.0.0.0:0").unwrap();

    for port_num in first..last {
        let port_str = port_num.to_string();
        let addr = host.to_string() + ":" + &port_str;
        println!("Testing: {}", addr);

        let bytes_amount = match socket.send_to(&[0x01, 0x02, 0x03], addr) {
            Ok(res) => res,
            Err(_)    => continue,
        };
        /*
        let mut buf = [0; 2048];
        loop {
            match socket.recv_from(&mut buf) {
                Ok((amt, src)) => {
                    thread::spawn(move || {
                        println!("amt: {}", amt);
                        println!("src: {}", src);
                        println!("{}", from_utf8(&buf).unwrap_or(""));
                    });
                },
                Err(e) => {
                    println!("couldn't recieve a datagram: {}", e);
                }
            }
        }
        */
        println!("Bytes: {}", bytes_amount);

        match bytes_amount.cmp(&0) {
            Ordering::Greater => {
                result.push(port_num as u32);
            },
            _ => continue,
        }
        // println!("{}", bytes_amount)
    }
    result
}