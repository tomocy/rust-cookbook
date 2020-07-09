extern crate monkey;

use std::env;
use std::process;

fn main() {
    if let Err(err) = monkey::run(env::args().skip(1)) {
        eprintln!("failed to run: {}", err)
        process::exit(1);
    }
}
