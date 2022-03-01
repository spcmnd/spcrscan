use std::net::AddrParseError;
use std::net::IpAddr;
use std::net::TcpStream;

pub fn scan_port(ip: IpAddr, port: i32) {
  if let Ok(_stream) = TcpStream::connect(format!("{}:{}", ip, port)) {
    println!("Port {} opened", port);
  }
}

pub struct Target {
  pub ip: IpAddr,
}

pub fn handle_args(args: Vec<String>) -> Result<Target, AddrParseError> {
  let ip = match args[1].clone().parse::<IpAddr>() {
    Ok(ip) => ip,
    Err(e) => return Err(e),
  };

  Ok(Target { ip })
}
