use crate::scanner::structs::{PortScannerError, ScannerArguments};
use std::collections::HashMap;
use std::net::IpAddr;
use std::str::FromStr;
use std::{u16, u64};

pub struct ArgumentParser {
  raw_arguments: Vec<String>,
  parsed_arguments: HashMap<String, String>
}

impl ArgumentParser {
    pub fn new(args: Vec<String>) -> ArgumentParser {
      return ArgumentParser {
        raw_arguments: args,
        parsed_arguments: HashMap::new(),
      }
    }

    pub fn is_valid_flag(argument: &String) -> bool {
        return argument.starts_with("--");
    }

    pub fn is_valid_argument(argument: &String) -> bool {
        let is_valid_argument = argument.starts_with("-");
        return is_valid_argument;
    }

    fn is_valid_argument_value(value: &String) -> bool {
        return !ArgumentParser::is_valid_flag(value) && !ArgumentParser::is_valid_argument(value);
    }

    pub fn to_obj(&self) -> Result<ScannerArguments, PortScannerError> {
        let host_str = self.parsed_arguments.get("-host").unwrap();
        let timeout_str = self.parsed_arguments.get("-timeout").unwrap_or(&String::from("500")).to_owned();
        let threads_str = self.parsed_arguments.get("-threads").unwrap_or(&String::from("100")).to_owned();

        let host = IpAddr::from_str(host_str).expect("Missing argument for Host");
        let threads = u16::from_str(&threads_str.to_owned()).unwrap();
        let timeout = u64::from_str(&timeout_str.to_owned()).unwrap();

        return Ok(ScannerArguments {
            host,
            threads,
            timeout,
        });
    }

    pub fn parse(&self) -> Result<ArgumentParser, PortScannerError> {
        if self.raw_arguments.len() == 0 {
          return Err(PortScannerError::new("Missing arguments"))
        }
        
        let mut parsed_arguments: HashMap<String, String> = HashMap::new();
        let vec_copy = self.raw_arguments.to_owned();
        let mut key = 1;
        while key < vec_copy.len() {
            let argument_name = &vec_copy[key];
            if (key + 1) == vec_copy.len() {
                let msg = format!("Missing value for argument {}", argument_name);
                return Err(PortScannerError::new(&msg));
            }

            let argument_value = &vec_copy[key + 1];
            if ArgumentParser::is_valid_argument(argument_value) {
                let msg = format!("Invalid value for argument {}", argument_name);
                return Err(PortScannerError::new(&msg));
            }

            let is_valid_argument = ArgumentParser::is_valid_argument(argument_name);
            let is_valid_argument_value = ArgumentParser::is_valid_argument_value(argument_value);
            if !is_valid_argument || !is_valid_argument_value {
                return Err(PortScannerError::new("Invalid Arguments"));
            }

            parsed_arguments.insert(argument_name.to_string(), argument_value.to_string());
            key += 2;
        }

        return Ok(ArgumentParser {
          parsed_arguments,
          raw_arguments: self.raw_arguments.clone(),
        });
    }
}
