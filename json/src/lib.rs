mod lexer;

#[cfg(test)]
mod tests;

use std::error;

type Result<T> = std::result::Result<T, Box<dyn error::Error>>;

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

    pub fn parse(&mut self) -> Result<Value> {
        match &self.curr_token {
            lexer::Token::EOF => Err(From::from("input should not be empty")),
            lexer::Token::LBrace => self.parse_object(),
            lexer::Token::Number(_) => self.parse_number(),
            lexer::Token::String(_) => self.parse_string(),
            lexer::Token::Unknown(s) => Err(From::from(format!("token '{}' is unknown", s))),
            _ => Err(From::from("not implemented")),
        }
    }

    fn parse_object(&mut self) -> Result<Value> {
        debug_assert_eq!(lexer::Token::LBrace, self.curr_token);
        self.read_token();

        let mut props = Vec::new();

        if self.do_have_token(lexer::Token::RBrace) {
            return Ok(Value::Object(props));
        }

        loop {
            let prop = self.parse_property()?;
            props.push(prop);

            if self.do_have_token(lexer::Token::EOF) {
                return Err(From::from("object should be closed with '}'"));
            }

            if !self.do_have_token(lexer::Token::Comma) {
                break;
            }

            self.read_token();
        }

        Ok(Value::Object(props))
    }

    fn parse_property(&mut self) -> Result<Property> {
        let key = if let Ok(Value::String(s)) = self.parse_string() {
            s
        } else {
            panic!("key should be string");
        };

        if !self.do_have_token(lexer::Token::Colon) {
            return Err(From::from(
                "property should be composed of key, ':', and value",
            ));
        }
        self.read_token();

        let value = self.parse().unwrap();

        Ok(Property::new(&key, value))
    }

    fn parse_number(&mut self) -> Result<Value> {
        let n = self.read_number();
        self.read_token();

        Ok(n)
    }

    fn read_number(&self) -> Value {
        match &self.curr_token {
            lexer::Token::Number(n) => Value::Number(*n),
            _ => panic!("current token should be number"),
        }
    }

    fn parse_string(&mut self) -> Result<Value> {
        let s = self.read_string();
        self.read_token();

        Ok(s)
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
