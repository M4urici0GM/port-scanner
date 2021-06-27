mod scanner;
use std::env;
use std::time::Instant;
use scanner::argument_parser::ArgumentParser;
use scanner::port_scanner::PortScanner;

fn main() {
    let args: Vec<String> = env::args().collect();
    let argument_parser = ArgumentParser::new(args);
    let parsed_arguments = argument_parser.parse();
    let current_time = Instant::now();

    
    match parsed_arguments {
        Err(err) => println!("{}", err),
        Ok(arguments) => {
            let argument_obj = arguments.to_obj().unwrap();
            let available_ports = PortScanner::start(&argument_obj);
            
            println!("\nFound {} ports within {}ms: ", available_ports.len(), current_time.elapsed().as_millis());
            for port in available_ports {
                println!("{}", port);
            }
        }
    }
}
