use std::io;
use std::io::Write;
use std::net::{SocketAddr, TcpStream};
use std::sync::mpsc::Receiver;
use std::sync::mpsc::{channel, Sender};
use std::thread;
use std::time::Duration;
use std::u16;
use super::structs::ScannerArguments;

const MAX_PORT: u16 = 65535;

pub struct PortScanner {}

impl PortScanner {
    pub fn scan_port(sender: Sender<u16>, arguments: &ScannerArguments, initial_port: u16) {
        let ip_addr = arguments.host;
        let mut scan_port: u16 = initial_port + 1;
        let socket_address = SocketAddr::from((ip_addr, scan_port));
        loop {
            match TcpStream::connect_timeout(
                &socket_address,
                Duration::from_millis(arguments.timeout),
            ) {
                Ok(_) => {
                    print!(".");
                    io::stdout().flush().unwrap();
                    sender.send(scan_port).unwrap();
                }
                Err(_) => {}
            }

            if (MAX_PORT - scan_port) <= arguments.threads {
                break;
            }
            
            scan_port += arguments.threads;
        }
    }

    fn spawn_threads(arguments: &ScannerArguments) -> Receiver<u16>{
        let mut children_threads = Vec::new();
        let arguments_clone = arguments.clone();
        let (sender, receiver) = channel::<u16>();

        for i in 0..arguments.threads {
            let tx = sender.clone();
            let child = thread::spawn(move || {
                PortScanner::scan_port(tx, &arguments_clone, i);
            });

            children_threads.push(child);
        }

        // Waits for children threads finish.
        for child in children_threads {
            child.join().expect("Children thread panicked.");
        }

        return receiver;
    }

    pub fn start(arguments: &ScannerArguments) -> Vec<u16> {
        let receiver = PortScanner::spawn_threads(arguments);
        let available_ports = receiver.iter().map(|x| x).collect();
        return available_ports;
    }
}
