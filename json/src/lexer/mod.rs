#[cfg(test)]
mod tests;

#[derive(Debug)]
pub struct Lexer {
    src: String,
    index: usize,
}

impl Lexer {
    pub fn new(src: &str) -> Self {
        Self {
            src: src.to_string(),
            index: 0,
        }
    }

    pub fn read_token(&mut self) -> Token {
        match self.current_char() {
            "\x00" => self.compose(Token::EOF),
            _ => self.compose(Token::Unknown),
        }
    }

    pub fn compose(&mut self, token: Token) -> Token {
        self.read_char();
        token
    }

    fn current_char(&self) -> &str {
        if self.index >= self.src.len() {
            "\x00"
        } else {
            &self.src.as_str()[self.index..self.index + 1]
        }
    }

    fn read_char(&mut self) {
        if self.index >= self.src.len() {
            return;
        }

        self.index += 1;
    }
}

#[derive(Debug, PartialEq)]
pub enum Token {
    Unknown,
    EOF,
}
