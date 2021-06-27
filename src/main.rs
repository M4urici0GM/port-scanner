use std::io;
use std::fmt;
use std::env;
use std::thread;
use std::io::Write;
use std::str::FromStr;
use std::time::Duration;
use std::net::SocketAddr;
use std::sync::mpsc::channel;
use std::collections::HashMap;
use std::time::{ Instant };
use std::net::{ IpAddr, TcpStream };
use std::sync::mpsc::{ Sender, Receiver };

const MAX_PORT: u16 = 65535;

struct Arguments {
    host: IpAddr,
    threads: u16,
    timeout: u64
}

#[derive(Debug)]
struct PortScannerError {
    details: String
}

impl PortScannerError {
    fn new(msg: &str) -> PortScannerError {
        return PortScannerError {
            details: msg.to_string()
        };
    }
}

impl fmt::Display for PortScannerError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        return write!(f, "{}", self.details); 
    }
}

impl Arguments {
    fn is_valid_flag(argument: &String) -> bool {
        return argument.starts_with("--");
    }

    fn is_valid_argument(argument: &String) -> bool {
        let is_valid_argument = argument.starts_with("-");
        return is_valid_argument;
    }

    fn is_valid_argument_value(value: &String) -> bool {
        return !Arguments::is_valid_flag(value) && !Arguments::is_valid_argument(value);
    }

    fn parse(args: &[String]) -> Result<Arguments, PortScannerError> {
        let vec_copy = args.to_owned();
        let mut arguments_arr: HashMap<&str, &str> = HashMap::new();

        if vec_copy.len() == 0 {
            return Err(PortScannerError::new("Missing arguments"));
        }

        let mut key = 1;
        while key < args.len() {
            let argument_name = &args[key];
            if (key + 1) == args.len() {
                let msg = format!("Missing value for argument {}", argument_name);
                return Err(PortScannerError::new(&msg));
            }

            let argument_value = &args[key + 1];
            if Arguments::is_valid_argument(argument_value) {
                let msg = format!("Invalid value for argument {}", argument_name);
                return Err(PortScannerError::new(&msg))
            }


            let is_valid_argument = Arguments::is_valid_argument(argument_name);
            let is_valid_argument_value = Arguments::is_valid_argument_value(argument_value);
            if !is_valid_argument || !is_valid_argument_value {
                return Err(PortScannerError::new("Invalid Arguments"));
            }
            
            arguments_arr.insert(&argument_name, &argument_value);
            key += 2;
        }

        let host_str = arguments_arr.get("-host").unwrap();
        let timeout_str = arguments_arr
            .get("-timeout")
            .unwrap_or(&"500")
            .to_owned();

        let threads_str = arguments_arr
            .get("-threads")
            .unwrap_or(&"100")
            .to_owned();

        return Ok(Arguments {
            host: IpAddr::from_str(host_str).expect("Invalid IP Address!"),
            threads: u16::from_str(threads_str).unwrap(),
            timeout: u64::from_str(timeout_str).unwrap()
        });
    }
}

fn scan_ports(sender: Sender<u16>, port: u16, ip_addr: IpAddr, thread_count: u16, timeout: u64) {
    let mut scan_port: u16 = port + 1;
    let socket_address = SocketAddr::from((ip_addr, scan_port));
    loop {
        match TcpStream::connect_timeout(&socket_address, Duration::from_millis(timeout)) {
            Ok(_) => {
                print!(".");
                io::stdout().flush().unwrap();
                sender.send(scan_port).unwrap();
            },
            Err(_) => {}
        }

        if (MAX_PORT - scan_port) <= thread_count {
            break;
        }

        scan_port += thread_count;
    }
}

fn spawn_threads(ip_addr: IpAddr, thread_count: u16, timeout: u64) -> Receiver<u16> {
    let mut children_threads = Vec::new();
    let (sender, receiver) = channel::<u16>();

    for i in  0..thread_count {
        let tx = sender.clone();
        let child = thread::spawn(move || {
            scan_ports(tx, i, ip_addr, thread_count, timeout);
        });

        children_threads.push(child);
    }

    // Waits for children thread finish.
    for child in children_threads {
        child
            .join()
            .expect("Children thread panicked.");
    }

    return receiver;
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let parsed_arguments = Arguments::parse(&args);
    match parsed_arguments {
        Err(err) => println!("{}", err),
        Ok(arguments) => {
            let threads = arguments.threads;
            let host = arguments.host;
            let timeout = arguments.timeout;
            let now = Instant::now();

            println!("Running with {} threads on host {}", arguments.threads, arguments.host);
            let receiver = spawn_threads(host, threads, timeout);

            println!("Ports found in {}ms: ", now.elapsed().as_millis());
            let port_vec: Vec<u16> = receiver.iter().map(|x|  x).collect();
            for port in port_vec {
                println!("{}", port);
            }
        }
    }
}
