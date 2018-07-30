use std::thread;
use std::net::*;
use std::sync::mpsc::{channel,Sender}; //channel



pub fn tcp_full(addr: String, port:usize)-> Option<usize> {
    println!("{}",addr);
    let addr = addr;
    if let Ok(stream) = TcpStream::connect(addr) {
        return Some(port);
    } else {
        None
    }
}