use clap::Parser;
use std::fs::OpenOptions;
use std::io::Write;
use std::net::IpAddr;
use std::net::TcpStream;

pub fn scan_port(ip: IpAddr, port: u32) -> Option<u32> {
  if let Ok(_stream) = TcpStream::connect(format!("{}:{}", ip, port)) {
    Some(port)
  } else {
    None
  }
}

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

pub fn scan(args: Args) {
  let Args {
    target,
    quick,
    condensed,
    output,
  } = args;
  let port_max: u32 = if quick { 10000 } else { 65536 };
  let mut opened_ports: Vec<String> = Vec::new();
  for port in 1..port_max {
    let result = scan_port(target, port);

    match result {
      Some(r) => {
        opened_ports.push(r.to_string());
      }
      None => (),
    }
  }

  log(
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

pub fn log(content: String, filename: &str) {
  println!("{}", content);

  if !filename.is_empty() {
    let mut file = OpenOptions::new()
      .create(true)
      .append(true)
      .open(filename)
      .unwrap();

    if let Err(e) = writeln!(file, "{}", content) {
      eprintln!("Couldn't write to file: {}", e);
    }
  }
}
