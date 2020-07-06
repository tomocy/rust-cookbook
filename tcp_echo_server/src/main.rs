extern crate tcp_echo_server;

use std::process;

fn main() {
    if let Err(err) = tcp_echo_server::run() {
        eprintln!("failed to run server: {}", err);
        process::exit(1);
    }
}
