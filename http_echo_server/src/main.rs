extern crate http_echo_server;

use std::process;

fn main() {
    if let Err(err) = http_echo_server::run() {
        eprintln!("failed to run: {}", err);
        process::exit(1);
    }
}
