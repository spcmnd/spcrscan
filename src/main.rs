use std::env;
use std::process;

mod lib;

fn main() {
    let args: Vec<String> = env::args().collect();
    
    if let Ok(target) = lib::handle_args(args) {
        for port in 1..65536 {
            lib::scan_port(target.ip, port);
        }
    } else {
        println!("IP Address is not valid");
        process::exit(1);
    }
}
