extern crate adder;

use std::env;
use std::process;

fn main() {
    let config = adder::Config::new(&mut env::args()).unwrap_or_else(|err| {
        eprintln!("failed to parse arguments: {}", err);
        process::exit(1);
    });
    if let Err(err) = adder::run(&config) {
        eprintln!("{}", err);
        process::exit(1);
    };
}
