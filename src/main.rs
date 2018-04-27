use std::net::*;
use std::env;
use std::process;
use std::sync::mpsc;
use std::thread;
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

    let (sender, receiver) = mpsc::channel();


    for port in port_beginn..port_end {
        if scan_port(ip, port) {
            open_ports.push(port);
        }
    }



/*

    thread::spawn(move || {
            match sender.send(port_open(ip,&port.to_string())) {
                Ok(True) => {}, // everything good
                Err(_) => {}, // we have been released, don't panic
            }
        });
        match receiver.try_recv() {
            Ok(True) => open_ports.push(port), // we have a connection
            Err(_) => {
                drop(receiver);
            }
        }
    */


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

fn scan_port(host: &str, port: u32) -> bool {
    let host = host.to_string();
    let port = port;
    let (sender: Result<Some()>, receiver: Result<Some()>) = mpsc::channel();
    let t = thread::spawn(move || {
        let mut addr = host.to_string();
        addr.push_str(":");
        addr.push_str(&port.to_string());
        match sender.send(TcpStream::connect(addr)) {
            Ok(()) => {}, // everything good
            Err(_) => {}, // we have been released, don't panic
        }
    });

    thread::sleep(std::time::Duration::new(5, 0));

    match receiver.try_recv() {
        Ok(Ok(handle)) => true, // we have a connection
        Ok(Err(_)) => false, // connecting failed
        Err(mpsc::TryRecvError::Empty) => {
            drop(receiver);
            drop(t);
            // connecting took more than 5 seconds
            false
        },
        Err(mpsc::TryRecvError::Disconnected) => unreachable!(),
    }
}