use std::env;
use std::process; //exit(0)

#[allow(dead_code)]
pub fn parse_arguments()-> Vec<String> {
    if env::args().len() == 0 || env::args().len() == 1 {
        exit_on_error()
    }
    let args: Vec<String> = env::args().collect();
    let mut ret: Vec<String> = vec![];
    println!("Argument Count: {}", args.len());
    match args.len() {
        3 => {
            // ./rns -P 127.0.0.1
            if !args.iter().nth(1).unwrap().to_string().eq("-P") {
                exit_on_error()
            }
            // Scan Type
            ret.push(args.iter().nth(1).unwrap().to_string());
            // IP
            ret.push(args.iter().nth(2).unwrap().to_string());
        },
        4 => {
            // ./rns -TF 127.0.0.1 20-60
            // Scan Type
            ret.push(args.iter().nth(1).unwrap().to_string());
            // IP
            ret.push(args.iter().nth(2).unwrap().to_string());
            // Ports
            let ports = parse_ports(args.iter().nth(3).unwrap().to_string());
            ret.push(ports.iter().nth(0).unwrap().to_string());
            ret.push(ports.iter().nth(1).unwrap().to_string());
        },
        5 => {
            // ./rns -U -iL /home/ips.txt 1-1000
            // Scan Type
            ret.push(args.iter().nth(1).unwrap().to_string());
            // IP input file
            ret.push(args.iter().nth(3).unwrap().to_string());
            // Ports
            let ports = parse_ports(args.iter().nth(4).unwrap().to_string());
            ret.push(ports.iter().nth(0).unwrap().to_string());
            ret.push(ports.iter().nth(1).unwrap().to_string());
        },
        7 => {
            // ./rns -U -iL /home/ips.txt 1-1000 -o /root/Desktop/scan.txt
            // Scan Type
            ret.push(args.iter().nth(1).unwrap().to_string());
            // IP input file
            ret.push(args.iter().nth(3).unwrap().to_string());
            // Ports
            let ports = parse_ports(args.iter().nth(4).unwrap().to_string());
            ret.push(ports.iter().nth(0).unwrap().to_string());
            ret.push(ports.iter().nth(1).unwrap().to_string());
            // output file
            ret.push(args.iter().nth(6).unwrap().to_string());
        },
        _ => {
            exit_on_error()
        }
    }

    ret
}

pub fn prep_ip (ip: String, port: usize) -> String {
    let addr = ip.to_string() + ":" + &port.to_string();
    addr
}

pub fn parse_ports (ports: String) -> Vec<String> {
    if !ports.contains("-") {
        println!("Error while parsing ports.");
        exit_on_error()
    }
    let res: Vec<String> = ports.split("-").map(|s| s.to_string()).collect();
    if res[0].parse::<usize>().is_err() || res[1].parse::<usize>().is_err() || res.len() < 2 {
        exit_on_error();
    }
    res
}

pub fn str_to_usize(string: String) -> Option<usize> {
    let ret = string.parse::<usize>().unwrap();
    Some(ret)
}
pub fn exit_on_error(){
    help();
    process::exit(0);
}
#[allow(dead_code)]
pub fn help() {
    println!("
 _____         _      _____     _                 _      _____
| __  |_ _ ___| |_   |   | |___| |_ _ _ _ ___ ___| |_   |   __|___ ___ ___ ___ ___ ___
|    -| | |_ -|  _|  | | | | -_|  _| | | | . |  _| '_|  |__   |  _| .'|   |   | -_|  _|
|__|__|___|___|_|    |_|___|___|_| |_____|___|_| |_,_|  |_____|___|__,|_|_|_|_|___|_|

Usage:
./rns <Scan Type> <IP> <Port-Range>
./rns <Scan Type> -iL <input file> <Port-Range>
./rns <Scan Type> -iL <input file> <Port-Range> -o <output file>

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
./rns -P 127.0.0.1
./rns -TF 127.0.0.1 20-60
./rns -U -iL /home/ips.txt 1-1000
./rns -U -iL /home/ips.txt 1-1000 -o /root/Desktop/scan.txt
");
}



#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_prep_ip() {
        assert_eq!(prep_ip("1.3.3.7".to_string(), 1337),"1.3.3.7:1337");
    }
}