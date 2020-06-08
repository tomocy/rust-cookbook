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
            _ if self.do_have_number() => self.compose_number(),
            _ => self.compose(Token::Unknown),
        }
    }

    pub fn compose(&mut self, token: Token) -> Token {
        self.read_char();
        token
    }

    fn compose_number(&mut self) -> Token {
        Token::Number(self.read_number())
    }

    fn read_number(&mut self) -> u32 {
        let begin = self.index;
        while self.do_have_number() {
            self.read_char();
        }

        self.src[begin..self.index].parse().unwrap()
    }

    fn do_have_number(&self) -> bool {
        let curr = self.current_char();
        "0" <= curr && curr <= "9"
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
    Number(u32),
}
