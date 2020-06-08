#[cfg(test)]
mod tests;

use super::lexer;
use std::error;

#[derive(Debug)]
pub struct Parser {
    lexer: lexer::Lexer,
    curr_token: lexer::Token,
    next_token: lexer::Token,
}

impl Parser {
    pub fn new(lexer: lexer::Lexer) -> Self {
        let mut p = Self {
            lexer,
            curr_token: lexer::Token::Unknown("".to_string()),
            next_token: lexer::Token::Unknown("".to_string()),
        };

        p.read_token();
        p.read_token();

        p
    }

    pub fn parse(&mut self) -> Result<Value, Box<dyn error::Error>> {
        match &self.curr_token {
            lexer::Token::EOF => Err(From::from("input should not be empty")),
            lexer::Token::Number(_) => Ok(self.parse_number()),
            lexer::Token::String(_) => Ok(self.parse_string()),
            lexer::Token::Unknown(s) => Err(From::from(format!("token '{}' is unknown", s))),
            _ => Err(From::from("not implemented")),
        }
    }

    fn parse_number(&mut self) -> Value {
        let n = self.read_number();
        self.read_token();

        n
    }

    fn read_number(&self) -> Value {
        match &self.curr_token {
            lexer::Token::Number(n) => Value::Number(*n),
            _ => panic!("current token should be number"),
        }
    }

    fn parse_string(&mut self) -> Value {
        let s = self.read_string();
        self.read_token();

        s
    }

    fn read_string(&self) -> Value {
        match &self.curr_token {
            lexer::Token::String(s) => Value::String(s.clone()),
            _ => panic!("current token should be string"),
        }
    }

    fn read_token(&mut self) {
        self.curr_token = self.next_token.clone();
        self.next_token = self.lexer.read_token();
    }
}

#[derive(Debug, PartialEq)]
pub enum Value {
    Number(u32),
    String(String),
}
