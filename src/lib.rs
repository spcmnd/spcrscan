use std::net::IpAddr;
use std::net::TcpStream;

pub fn scan_port(ip: IpAddr, port: i32) -> Option<i32> {
  if let Ok(_stream) = TcpStream::connect(format!("{}:{}", ip, port)) {
    Some(port)
  } else {
    None
  }
}
