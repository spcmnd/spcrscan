use std::env;
use std::net::IpAddr;
use std::net::TcpStream;
use std::process;

fn main() {
    let args: Vec<String> = env::args().collect();
    if let Ok(ip) = args[1].clone().parse::<IpAddr>() {
        for port in 1..65536 {
            if let Ok(_stream) = TcpStream::connect(format!("{}:{}", ip, port)) {
                println!("Port {} opened", port);
            }
        }
    } else {
        println!("IP Address is not valid");
        process::exit(1);
    }
}
