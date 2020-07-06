extern crate tcp_echo_server;

use std::env;
use std::process;

fn main() {
    if let Err(err) = tcp_echo_server::run(env::args().skip(1)) {
        eprintln!("failed to run server: {}", err);
        process::exit(1);
    }
}
