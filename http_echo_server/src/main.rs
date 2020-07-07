extern crate http_echo_server;

use std::env;
use std::process;

fn main() {
    if let Err(err) = http_echo_server::run(env::args().skip(1)) {
        eprintln!("failed to run: {}", err);
        process::exit(1);
    }
}
