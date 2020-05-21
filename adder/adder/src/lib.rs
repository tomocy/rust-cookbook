extern crate add_one;
extern crate add_two;

use std::env;

pub fn run(config: &Config) -> Result<(), &'static str> {
    println!("{} + 1 = {}", config.value, add_one::add_one(config.value));
    println!("{} + 2 = {}", config.value, add_two::add_two(config.value));
    Ok(())
}

pub struct Config {
    value: i32,
}

impl Config {
    pub fn new(args: &mut env::Args) -> Result<Config, &'static str> {
        args.next();

        let v = match args.next() {
            Some(v) => v,
            None => return Err("failed to get value"),
        };
        let v: i32 = match v.parse() {
            Ok(v) => v,
            Err(_) => return Err("failed to parse value to number"),
        };

        Ok(Config { value: v })
    }
}
