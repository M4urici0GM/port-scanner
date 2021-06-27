use std::net::{ IpAddr };
use std::{ fmt, u16, u64 };

#[derive(Clone, Copy)]
pub struct ScannerArguments {
  pub host: IpAddr,
  pub threads: u16,
  pub timeout: u64,
}

#[derive(Debug)]
pub struct PortScannerError {
  details: String
}

impl ScannerArguments {
  pub fn clone(&self) -> ScannerArguments {
    return ScannerArguments {
      host: self.host,
      threads:  self.threads,
      timeout: self.timeout
    };
  }
}

impl PortScannerError {
  pub fn new(msg: &str) -> PortScannerError {
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