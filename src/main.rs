use std::net::*;

fn main() {
    let remote_target = "141.37.29.215";
    println!("{}",remote_target);
    for port in 0..100 {
        println!("{},{}",port, port_open(remote_target,&port.to_string()))
    }
}

pub fn port_open(ip: &str, port: &str) -> bool {
    let mut addr = ip.to_string();
    addr.push_str(":");
    addr.push_str(port);
    if let Ok(stream) = TcpStream::connect(addr) {
        true
    } else {
        false
    }
}