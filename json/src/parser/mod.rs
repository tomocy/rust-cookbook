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
            lexer::Token::LBrace => Ok(self.parse_object()),
            lexer::Token::Number(_) => Ok(self.parse_number()),
            lexer::Token::String(_) => Ok(self.parse_string()),
            lexer::Token::Unknown(s) => Err(From::from(format!("token '{}' is unknown", s))),
            _ => Err(From::from("not implemented")),
        }
    }

    fn parse_object(&mut self) -> Value {
        debug_assert_eq!(lexer::Token::LBrace, self.curr_token);
        self.read_token();

        let mut props = Vec::new();

        if self.do_have_token(lexer::Token::RBrace) {
            return Value::Object(props);
        }

        loop {
            let prop = self.parse_property();
            props.push(prop);

            if !self.do_have_token(lexer::Token::Comma) || self.do_have_token(lexer::Token::EOF) {
                break;
            }

            self.read_token();
        }

        Value::Object(props)
    }

    fn parse_property(&mut self) -> Property {
        let key = if let Value::String(s) = self.parse_string() {
            s
        } else {
            panic!("key should be string");
        };

        debug_assert_eq!(lexer::Token::Colon, self.curr_token);
        self.read_token();

        let value = self.parse().unwrap();

        Property::new(&key, value)
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

    fn do_have_token(&self, token: lexer::Token) -> bool {
        self.curr_token == token
    }

    fn read_token(&mut self) {
        self.curr_token = self.next_token.clone();
        self.next_token = self.lexer.read_token();
    }
}

#[derive(Debug, PartialEq)]
pub enum Value {
    Object(Vec<Property>),
    Number(u32),
    String(String),
}

#[derive(Debug, PartialEq)]
pub struct Property {
    key: String,
    value: Value,
}

impl Property {
    pub fn new(key: &str, value: Value) -> Self {
        Self {
            key: key.to_string(),
            value,
        }
    }
}
