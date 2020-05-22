extern crate hello_http_server;

use std::process;

fn main() {
    if let Err(err) = hello_http_server::run("localhost:7878") {
        eprintln!("failed to run http server: {}", err);
        process::exit(1);
    }
}
