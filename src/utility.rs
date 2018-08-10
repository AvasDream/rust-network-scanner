use std::env;
use std::process; //exit(0)
use clap::*;
use std::net::Ipv4Addr;
use std::collections::HashMap;
use std::io::{self, BufReader};
use std::io::prelude::*;
use std::fs::File;
use ScanType;
use iana_mapping;
use ScanResult;
use ScanConfig;
use std::path::Path;


pub fn prepare_output(results: Vec<ScanResult>) -> String {
    let mut output = "".to_string();

    let portmap = match results[2].scantype {
        ScanType::TcpFull => iana_mapping::get_tcp_map(),
        ScanType::Udp => iana_mapping::get_udp_map(),
        _ => HashMap::new()
    };
    for result in results {
        output += &format!("\nScan result for {}\n",result.ip.to_string());
        /*
        Achtung ausnahmefall ICMP Scan, da keine Ports nötig!
        */
        if portmap.len() == 0 {
            if result.ports.len() == 1 {
                output += &format!("Host is up!\n");
            } else {
                output += &format!("Host is down!\n");
            }
        } else {
            for port in result.ports {
                output += &format!("Port {} | {:?} open.\n",port.to_string(),portmap.get(&(port as u64)).unwrap());
            }

        }
    }

    output
}
pub fn read_from_file(file: String)-> Vec<Ipv4Addr> {
    let mut ipvec = Vec::new();
    let file = match File::open(file.clone()) {
        Err(err) => panic!("Couldn't open file {}", file),
        Ok(file) => file,
    };
    let f = BufReader::new(file);
    for line in f.lines() {
        let ip = line.unwrap();
        let ipv4 = string_to_ipv4(ip);
        ipvec.push(ipv4);
    }
    ipvec
}



fn string_to_ipv4(ip: String)-> Ipv4Addr {
    let mut vals = ip.split(".").collect::<Vec<&str>>();
    let ipv4 = Ipv4Addr::new(vals[0].to_string().parse::<u8>().unwrap(),
                             vals[1].to_string().parse::<u8>().unwrap(),
                             vals[2].to_string().parse::<u8>().unwrap(),
                             vals[3].to_string().parse::<u8>().unwrap());
    ipv4
}


pub fn write_to_file(filename: String, data: String) {
    let mut file = match File::create(filename.clone()) {
        Err(err) => panic!("Couldn't create file {}", filename),
        Ok(file) => file,
    };
    file.write( data.as_bytes()).expect("error while writing to file");
}
pub fn get_config()-> ScanConfig {
    let arguments = parse_arguments();
    let ports = parse_ports(arguments.value_of("PORTS").unwrap().to_string());
    let ip = arguments.value_of("IP").unwrap().to_string();
    let scantype = parse_scan_type(arguments.value_of("SCANTYPE").unwrap().to_string());
    let port_beginn = ports[0].parse::<u16>().unwrap_or(0);
    let port_end = ports[1].parse::<u16>().unwrap_or(0);
    let scanconfig: ScanConfig = ScanConfig {
        ips: vec![Ipv4Addr::new(0,0,0,0)],
        start_port: 0,
        end_port: 0,
        scantype: ScanType::TcpFull,
        to_file: "/root".to_string()
    };
    scanconfig
}
pub fn parse_arguments()-> ArgMatches<'static> {
    let matches = App::new("Rust Network Scanner")
        .version("1.0")
        .author("Vincent Grimmeisen <vincent.grimmeisen@htwg-konstanz.de>")
        .about("
 _____         _      _____     _                 _      _____
| __  |_ _ ___| |_   |   | |___| |_ _ _ _ ___ ___| |_   |   __|___ ___ ___ ___ ___ ___
|    -| | |_ -|  _|  | | | | -_|  _| | | | . |  _| '_|  |__   |  _| .'|   |   | -_|  _|
|__|__|___|___|_|    |_|___|___|_| |_____|___|_| |_,_|  |_____|___|__,|_|_|_|_|___|_|

Scan Types:
P          Ping scan
TF         Tcp full scan
TN         Tcp Null scan
U          Udp scan

            RNS is a free Network Scanner intended to work like the glorious nmap.")
        .arg(Arg::with_name("SCANTYPE")
            .short("s")
            .long("scantype")
            .help("Set the type of your scan")
            .required(true)
            .takes_value(true))
        .arg(Arg::with_name("IP")
            .help("Sets the IP or IP range to use")
            .short("i")
            .long("ip")
            .required(true)
            .takes_value(true))
        .arg(Arg::with_name("PORTS")
            .help("Sets the Port range to use")
            .short("p")
            .long("ports")
            .required(true)
            .takes_value(true))
        .arg(Arg::with_name("OUTPUT")
            .help("Sets the output file")
            .short("o")
            .long("output")
            .required(false)
            .takes_value(true))
        .get_matches();
    matches
}

pub fn prep_ip (ip: String, port: u16) -> String {
    let addr = ip.to_string() + ":" + &port.to_string();
    addr
}

pub fn parse_ports (ports: String) -> Vec<String> {
    if !ports.contains("-") {
        println!("Error while parsing ports.");
        exit_on_error()
    }
    let res: Vec<String> = ports.split("-").map(|s| s.to_string()).collect();
    if res[0].parse::<u16>().is_err() || res[1].parse::<u16>().is_err() || res.len() < 2 {
        exit_on_error();
    }
    res
}

pub fn str_to_u16(string: String) -> Option<u16> {
    let ret = string.parse::<u16>().unwrap();
    Some(ret)
}
pub fn exit_on_error(){
    //print_usgae();
    process::exit(0);
}




#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_prep_ip() {
        assert_eq!(prep_ip("1.3.3.7".to_string(), 1337),"1.3.3.7:1337");
    }
}

fn parse_scan_type(string: String) -> ScanType {
    let mut scantype: ScanType = ScanType::Ping;
    match string.as_ref() {
        "P" => {
            scantype = ScanType::Ping;
        },
        "TF" => {
            scantype = ScanType::TcpFull;
        },
        "TN" => {
            scantype = ScanType::TcpNull;
        },
        "U" => {
            scantype = ScanType::Udp;
        },
        _ => {
            println!("Error while parsing scan type");
            exit_on_error();
        },
    }
    return scantype
}

/*

Code for future tests:



let mut ports: Vec<u16>= Vec::new();
    ports.push(80);
    ports.push(443);
    ports.push(445);
    let mut ips: Vec<Ipv4Addr>= Vec::new();
    ips.push(Ipv4Addr::new(12,23,34,45));
    ips.push(Ipv4Addr::new(12,223,34,45));
    ips.push(Ipv4Addr::new(212,23,34,45));
    ips.push(Ipv4Addr::new(122,23,34,45));
    let scantype: ScanType = ScanType::TcpFull;
    let scantype1: ScanType = ScanType::TcpFull;
    let scantype2: ScanType = ScanType::TcpFull;
    let res1 = ScanResult {
        ports: ports.clone(),
        ip: ips[0],
        scantype: scantype
    };
    let res2 = ScanResult {
        ports: ports.clone(),
        ip: ips[0],
        scantype: scantype1
    };
    let res3 = ScanResult {
        ports: ports.clone(),
        ip: ips[0],
        scantype: scantype2
    };
    let mut results: Vec<ScanResult>= Vec::new();
    results.push(res1);
    results.push(res2);
    results.push(res3);
    let out = utility::prepare_output(results);


*/
