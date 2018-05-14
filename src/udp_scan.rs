use std::thread;
use std::net::*;
use std::sync::mpsc::{channel,Sender}; //channel

#[allow(dead_code)]
pub fn port_open_udp(ip: String, port: usize, tx: Sender<(usize,bool)>, socket: UdpSocket) {
    unimplemented!();
    /*
        thread::spawn(move || {
        let mut addr = prep_ip(ip,port);


        let bytes_amount = match socket.send_to(&[0;63], addr) {
            Ok(res) => res,
            Err(_)    => 666,
        };
        println!("{}",bytes_amount);
        match bytes_amount.cmp(&0) {
            cmp::Ordering::tx.send((port, true)),
            _ => ()
        }

        });
    */
}