use std::env;
use std::process; //exit(0)

#[allow(dead_code)]
fn parse_arguments()-> Vec<String> {
    let args: Vec<String> = env::args().collect();
    let mut ret: Vec<String> = vec![];

    ret
}

pub fn prep_ip (ip: String, port: usize) -> String {
    let addr = ip.to_string() + ":" + &port.to_string();
    addr
}

#[allow(dead_code)]
pub fn help() {
    println!("
_____         _      _____     _                 _      _____
| __  |_ _ ___| |_   |   | |___| |_ _ _ _ ___ ___| |_   |   __|___ ___ ___ ___ ___ ___
|    -| | |_ -|  _|  | | | | -_|  _| | | | . |  _| '_|  |__   |  _| .'|   |   | -_|  _|
|__|__|___|___|_|    |_|___|___|_| |_____|___|_| |_,_|  |_____|___|__,|_|_|_|_|___|_|

Usage:
./rns <Scan Type> <Port-Range> <IP>

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
./rns 127.0.0.1 1-100
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