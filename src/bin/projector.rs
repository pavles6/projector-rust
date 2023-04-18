use clap::Parser;
use projector_rust::opts::Opts;

fn main() {
    let opts = Opts::parse();
    println!("{:?}", opts)
}
