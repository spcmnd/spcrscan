use clap::Parser;

mod lib;

fn main() {
    let args = lib::Args::parse();
    lib::scan(args);
}
