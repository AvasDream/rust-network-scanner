use std::env;
use std::process; //exit(0)
use std::collections::HashMap;
use clap::*;

#[allow(dead_code)]
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
TS         Tcp Syn scan
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

pub fn prepare_arguments(arguments: ArgMatches) -> HashMap<String,String> {
    let mut args_map: HashMap<String,String> = HashMap::new();
    /*
    let ports = parse_ports(arguments.value_of("PORTS").unwrap().to_string());
    let ip = arguments.value_of("IP").unwrap().to_string();
    let scantype = arguments.value_of("SCANTYPE").unwrap().to_string();
    let port_beginn = ports[0];
    let port_end = ports[1];
    args_map.insert("ip",ip);
    args_map.insert("port_beginn",port_beginn.to_string());
    args_map.insert("port_end", port_end.clone());
    args_map.insert("scantype", scantype.clone().to_string());
    */
    args_map
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
    //print_usgae();
    process::exit(0);
}
#[allow(dead_code)]
pub fn print_usgae() {
    println!("
 _____         _      _____     _                 _      _____
| __  |_ _ ___| |_   |   | |___| |_ _ _ _ ___ ___| |_   |   __|___ ___ ___ ___ ___ ___
|    -| | |_ -|  _|  | | | | -_|  _| | | | . |  _| '_|  |__   |  _| .'|   |   | -_|  _|
|__|__|___|___|_|    |_|___|___|_| |_____|___|_| |_,_|  |_____|___|__,|_|_|_|_|___|_|

Scan Types:
P          Ping scan
TF         Tcp full scan
TS         Tcp Syn scan
TN         Tcp Null scan
U          Udp scan
O          Host operation system detection

Options:
-iL <file>  Input from list of hosts
-o <file>   Save output to file

Example:
./rns -t P -i 127.0.0.1
./rns -t TF -i 127.0.0.1 -p 1-100
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

/*
if env::args().len() < 5 {
        exit_on_error()
    }
    println!("Argument Count: {}", env::args().len());
    let mut args_map: HashMap<String,String> = HashMap::new();
    let args: Vec<String> = env::args().collect();
    match args.len() {
        5 => {
            args_map.insert("scantype".to_string(), args.iter().nth(2).unwrap().to_string());
            args_map.insert("host".to_string(), args.iter().nth(4).unwrap().to_string());
            if args_map.get("scantype").unwrap() != "P" {
                println!("No Ports given.");
                exit_on_error()
            }
        },
        7 => {
            println!("Match in parse argumetns");
            args_map.insert("scantype".to_string(), args.iter().nth(2).unwrap().to_string());
            args_map.insert("host".to_string(), args.iter().nth(4).unwrap().to_string());
            let ports = parse_ports(args.iter().nth(6).unwrap().to_string());
            args_map.insert("start".to_string(), ports.iter().nth(0).unwrap().to_string());
            args_map.insert("end".to_string(), ports.iter().nth(1).unwrap().to_string());
        },
        _ => exit_on_error()
    }
    args_map

*/