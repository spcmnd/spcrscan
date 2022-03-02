use clap::Parser;
use std::net::IpAddr;

mod lib;
mod logger;

#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
pub struct Args {
    #[clap(short, long)]
    pub target: IpAddr,

    #[clap(short, long)]
    pub output: Option<String>,

    #[clap(short, long)]
    pub quick: bool,

    #[clap(short, long)]
    pub condensed: bool,
}

fn main() {
    let args = Args::parse();
    let target = args.target;
    let quick = args.quick;
    let condensed = args.condensed;
    let output = args.output;

    let port_max = if quick { 10000 } else { 65536 };
    let mut opened_ports: Vec<String> = Vec::new();

    for port in 1..port_max {
        let result = lib::scan_port(target, port);

        match result {
            Some(r) => {
                opened_ports.push(r.to_string());

                if !condensed {
                    logger::log(
                        format!("Port {} is opened", r),
                        match &output {
                            None => "",
                            Some(x) => x,
                        },
                    );
                }
            }
            None => (),
        }
    }

    if condensed {
        logger::log(
            format!("{}", opened_ports.join(",")),
            match &output {
                None => "",
                Some(x) => x,
            },
        );
    }
}
