#[cfg(test)]
mod tests;

use super::lexer;
use std::error;

#[derive(Debug)]
pub struct Parser {
    lexer: lexer::Lexer,
    token: lexer::Token,
}

impl Parser {
    pub fn new(lexer: lexer::Lexer) -> Self {
        let mut p = Self {
            lexer,
            token: lexer::Token::Unknown,
        };

        p.read_token();

        p
    }

    pub fn parse(&mut self) -> Result<Value, Box<dyn error::Error>> {
        match self.token {
            lexer::Token::EOF => Err(From::from("input should not be empty")),
            _ => Err(From::from("unimplemented")),
        }
    }

    fn read_token(&mut self) {
        self.token = self.lexer.read_token()
    }
}

#[derive(Debug, PartialEq)]
pub enum Value {}
