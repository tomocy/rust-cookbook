use std::error;
use std::io;

pub fn run<T: Iterator<Item = String>>(_: T) -> Result<(), Box<dyn error::Error>> {
    Err("not implemented".into())
}

struct Lexer {
    src_reader: Box<dyn io::Read>,
}

impl Lexer {
    fn new(src_reader: Box<dyn io::Read>) -> Self {
        Self { src_reader }
    }
}

#[derive(Debug)]
enum Token {
    Illegal(String),
    EOF,
}

#[cfg(test)]
mod tests {}
