use std::error;
use std::io;

pub fn run<T: Iterator<Item = String>>(_: T) -> Result<(), Box<dyn error::Error>> {
    Err("not implemented".into())
}

struct Lexer<'src> {
    src: &'src str,
}

impl<'src> Lexer<'src> {
    fn new(src: &'src str) -> Self {
        Self { src }
    }
}

#[derive(Debug)]
enum Token {
    Illegal(String),
    EOF,
}

#[cfg(test)]
mod tests {}
