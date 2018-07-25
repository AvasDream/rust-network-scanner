use std::thread;
use std::net::*;
use std::sync::mpsc::{channel,Sender}; //channel


#[allow(dead_code)]
pub fn port_open_tcp_full(addr: String, port:usize,  tx: Sender<(usize,bool)>) {
    thread::spawn(move || {
        let addr = addr;
        if let Ok(stream) = TcpStream::connect(addr) {
            tx.send((port, true))
        } else {
            tx.send((port, false))
        }
    });
}
/*

*/
pub fn tcp_full(addr: String, port:usize)-> Option<usize> {
    let addr = addr;
    println!("{}:{}",addr,port);
    if let Ok(stream) = TcpStream::connect(addr) {
        return Some(port);
    }
    None
}