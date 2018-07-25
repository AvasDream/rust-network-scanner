use std::env;
use std::process; //exit(0)
use std::collections::HashMap;
use clap::*;
use std::thread;
use std::sync::{Arc,Mutex,mpsc};

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

pub struct Threadpool {
    workers: Vec<Worker>,
    sender: mpsc::Sender<Job>,
}
trait FnBox {
    fn call_box(self: Box<Self>);
}
/*
impl<F: FnOnce()> FnBox for F {
    fn call_box(self: Box<F>) {
        (*self)()
    }
}
*/
impl<Option: FnOnce()> FnBox for Option {
    fn call_box(self: Box<Option>) {
        (*self)()
    }
}
type Job = Box<FnBox + Send + 'static>;
impl Threadpool {

    pub fn new(size:usize) -> Threadpool {
        assert!(size > 0);

        let (sender, receiver) = mpsc::channel();

        let receiver = Arc::new(Mutex::new(receiver));

        let mut workers = Vec::with_capacity(size);

        for id in 0..size {
            workers.push(Worker::new(id, Arc::clone(&receiver)));
        }

        Threadpool{
            workers,
            sender
        }
    }

    pub fn execute<F>(&self, f: F)
        where
            F: FnOnce() + Send + 'static
    {
        let job = Box::new(f);

        self.sender.send(job).unwrap();
    }
}

struct Worker {
    id: usize,
    thread: thread::JoinHandle<Arc<Mutex<mpsc::Receiver<Job>>>>,
}
impl Worker {
    fn new(id: usize, receiver: Arc<Mutex<mpsc::Receiver<Job>>>) -> Worker {
        let thread = thread::spawn(move || {
            loop {
                let job = receiver.lock().unwrap().recv().expect("Error in Job thread.");

                println!("Worker {} got a job; executing.", id);

                job.call_box();
            }
        });

        Worker {
            id,
            thread,
        }
    }
}




#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_prep_ip() {
        assert_eq!(prep_ip("1.3.3.7".to_string(), 1337),"1.3.3.7:1337");
    }
}

