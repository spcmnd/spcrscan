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
    let Args {
        target,
        quick,
        condensed,
        output,
    } = Args::parse();

    let port_max = if quick { 10000 } else { 65536 };
    let mut opened_ports: Vec<String> = Vec::new();

    for port in 1..port_max {
        let result = lib::scan_port(target, port);

        match result {
            Some(r) => {
                opened_ports.push(r.to_string());
            }
            None => (),
        }
    }

    logger::log(
        match condensed {
            true => format!("{}", opened_ports.join(",")),
            false => opened_ports
                .into_iter()
                .map(|o| format!("Port {} is opened", o))
                .collect::<Vec<String>>()
                .join("\n"),
        },
        match &output {
            None => "",
            Some(x) => x,
        },
    );
}
