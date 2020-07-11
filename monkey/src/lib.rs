use std::error;

pub fn run<T: Iterator<Item = String>>(_: T) -> Result<(), Box<dyn error::Error>> {
    Err("not implemented".into())
}

struct Lexer<'src> {
    src: &'src str,
    pos: usize,
    reading_pos: usize,
}

impl<'src> Lexer<'src> {
    const EOF: u8 = 0;

    fn new(src: &'src str) -> Self {
        Self {
            src,
            pos: 0,
            reading_pos: 0,
        }
    }

    fn read_token(&mut self) -> Token {
        self.read_char();

        let ch = self.char();
        match ch {
            Self::EOF => Token::EOF,
            _ if self.have_digit() => Token::Int(self.read_number()),
            _ => Token::Illegal(String::from_utf8(vec![ch]).unwrap()),
        }
    }

    fn read_number(&mut self) -> i32 {
        let begin = self.pos;

        while self.have_digit() {
            self.read_char();
        }

        self.src[begin..self.pos].parse().unwrap()
    }

    fn read_char(&mut self) {
        if self.pos >= self.src.len() {
            return;
        }

        self.pos = self.reading_pos;
        self.reading_pos += 1;
    }

    fn have_digit(&self) -> bool {
        let ch = self.char();
        b'0' <= ch && ch <= b'9'
    }

    fn char(&self) -> u8 {
        if self.pos >= self.src.len() {
            Self::EOF
        } else {
            self.src.as_bytes()[self.pos]
        }
    }
}

#[derive(Debug, PartialEq)]
enum Token {
    Illegal(String),
    EOF,
    Int(i32),
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn lexer_reads_empty() {
        let src = "";
        let mut lexer = Lexer::new(src);

        let expected = vec![Token::EOF];

        for expected in expected.into_iter() {
            let actual = lexer.read_token();
            assert_eq!(expected, actual);
        }
    }

    #[test]
    fn lexer_reads_int() {
        let src = "12345";
        let mut lexer = Lexer::new(src);

        let expected = vec![Token::Int(12345), Token::EOF];

        for expected in expected.into_iter() {
            let actual = lexer.read_token();
            assert_eq!(expected, actual);
        }
    }
}
