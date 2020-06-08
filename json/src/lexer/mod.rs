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
        self.skip_whitespaces();

        match self.current_char() {
            "\x00" => self.compose(Token::EOF),
            "\"" => self.compose_string(),
            ":" => self.compose(Token::Colon),
            _ if self.do_have_number() => self.compose_number(),
            _ => self.compose_unknown(),
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

    fn compose_string(&mut self) -> Token {
        Token::String(self.read_quoted_string())
    }

    fn read_quoted_string(&mut self) -> String {
        debug_assert_eq!(r#"""#, self.current_char());
        self.read_char();

        let s = self.read_string();

        debug_assert_eq!(r#"""#, self.current_char());
        self.read_char();

        s
    }

    fn read_string(&mut self) -> String {
        let begin = self.index;
        while self.do_have_letter() {
            self.read_char();
        }

        self.src[begin..self.index].to_string()
    }

    fn do_have_letter(&self) -> bool {
        let c = self.current_char();
        "a" <= c && c <= "z" || "A" <= c && c <= "Z"
    }

    fn compose_unknown(&mut self) -> Token {
        Token::Unknown(self.read_unknown())
    }

    fn read_unknown(&mut self) -> String {
        let c = self.current_char().to_string();
        self.read_char();

        c
    }

    fn skip_whitespaces(&mut self) {
        while self.do_have_whitespace() {
            self.read_char()
        }
    }

    fn do_have_whitespace(&self) -> bool {
        let c = self.current_char();
        c == " " || c == "\t" || c == "\r" || c == "\n"
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
    EOF,
    Colon,
    Number(u32),
    String(String),
    Unknown(String),
}
