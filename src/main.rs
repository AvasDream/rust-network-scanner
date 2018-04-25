use std::net::*;
use std::env;
use std::process;
/*
- input validation
*/

fn main() {
    let remote_target = "141.37.29.215";
    let args: Vec<String> = parse_arguments();
    println!("{},{},{}",args[0],args[1],args[2]);
    let open_ports = scan(&args[0],&args[1],&args[2]);
    println!("{}",open_ports[0]);
    for port in open_ports {
        println!("{} is open",port)
    }
}

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

fn help() {
    println!("usage:
              main.rs <IP> <Port-Range>
              Example:
              main.rs 127.0.0.1 1-100");
}

pub fn scan(ip: &str, port_beginn: &str, port_end:&str)-> Vec<u32> {
    let mut open_ports: Vec<u32> = vec![];
    let port_beginn = port_beginn.parse::<u32>().unwrap();
    let port_end = port_end.parse::<u32>().unwrap();
    for port in port_beginn..port_end {
        if port_open(ip, &port.to_string()) {
            open_ports.push(port);
        }
    }
    open_ports
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