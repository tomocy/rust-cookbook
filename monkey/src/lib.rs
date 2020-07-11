use std::error;

pub fn run<T: Iterator<Item = String>>(_: T) -> Result<(), Box<dyn error::Error>> {
    Err("not implemented".into())
}

struct Parser<'src> {
    lexer: Lexer<'src>,
    tok: Token,
    reading_tok: Token,
}

impl<'src> Parser<'src> {
    fn new(lexer: Lexer<'src>) -> Self {
        let mut parser = Self {
            lexer,
            tok: Token::EOF,
            reading_tok: Token::EOF,
        };

        parser.read();
        parser.read();

        parser
    }

    fn parse(&mut self) -> Result<Program, Box<dyn error::Error>> {
        let mut program = Vec::new();

        while !self.have_token(Token::EOF) {
            program.push(self.parse_statement()?);
            self.read();
        }

        Ok(program)
    }

    fn parse_statement(&self) -> Result<Statement, Box<dyn error::Error>> {
        self.parse_expression_statement()
    }

    fn parse_expression_statement(&self) -> Result<Statement, Box<dyn error::Error>> {
        Ok(Statement::Expression(self.parse_expression()?))
    }

    fn parse_expression(&self) -> Result<Expression, Box<dyn error::Error>> {
        self.parse_prefix_expression()
    }

    fn parse_prefix_expression(&self) -> Result<Expression, Box<dyn error::Error>> {
        let tok = self.tok.clone();
        match tok {
            Token::Int(x) => Ok(Expression::Int(x)),
            Token::String(x) => Ok(Expression::String(x)),
            _ => Err("invalid token".into()),
        }
    }

    fn parse_infix_expression(
        &mut self,
        left: Expression,
    ) -> Result<Expression, Box<dyn error::Error>> {
        let operator = self.parse_infix_operator()?;
        let right = self.parse_expression()?;

        Ok(Expression::Infix {
            left: Box::new(left),
            operator,
            right: Box::new(right),
        })
    }

    fn parse_infix_operator(&mut self) -> Result<InfixOperator, Box<dyn error::Error>> {
        match self.tok {
            Token::Plus => {
                self.read();
                Ok(InfixOperator::Plus)
            }
            _ => Err("invalid token".into()),
        }
    }

    fn have_token(&self, tok: Token) -> bool {
        self.tok == tok
    }

    fn read(&mut self) {
        self.tok = self.reading_tok.clone();
        self.reading_tok = self.lexer.read();
    }
}

type Program = Vec<Statement>;

#[derive(Debug, PartialEq)]
enum Statement {
    Expression(Expression),
}

#[derive(Debug, PartialEq)]
enum Expression {
    Infix {
        left: Box<Expression>,
        operator: InfixOperator,
        right: Box<Expression>,
    },
    Int(i32),
    String(String),
}

#[derive(Debug, PartialEq)]
enum InfixOperator {
    Plus,
}

enum InfixOperatorPrecedence {
    Lowest,
    Additive,
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

    fn read(&mut self) -> Token {
        self.read_char();

        let ch = self.char();
        match ch {
            Self::EOF => Token::EOF,
            b'+' => Token::Plus,
            b'"' => Token::String(self.read_string()),
            _ if self.have_digit() => Token::Int(self.read_number()),
            _ => Token::Illegal(String::from_utf8(vec![ch]).unwrap()),
        }
    }

    fn read_string(&mut self) -> String {
        debug_assert_eq!(b'"', self.char());
        self.read_char();

        let begin = self.pos;

        while self.have_letter() {
            self.read_char();
        }

        let end = self.pos;

        debug_assert_eq!(b'"', self.char());
        self.read_char();

        self.src[begin..end].into()
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

    fn have_letter(&self) -> bool {
        let ch = self.char();
        b'a' <= ch && ch <= b'z' || b'A' <= ch && ch <= b'Z'
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

#[derive(Debug, PartialEq, Clone)]
enum Token {
    Illegal(String),
    EOF,
    Plus,
    Int(i32),
    String(String),
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parser_parses_empty() {
        let src = "";
        let lexer = Lexer::new(src);
        let mut parser = Parser::new(lexer);

        let expected: Program = Vec::new();
        let actual = parser.parse().unwrap();

        assert_eq!(expected, actual);
    }

    #[test]
    fn parser_parses_int() {
        let src = "12345";
        let lexer = Lexer::new(src);
        let mut parser = Parser::new(lexer);

        let expected = vec![Statement::Expression(Expression::Int(12345))];
        let actual = parser.parse().unwrap();

        assert_eq!(expected, actual);
    }

    #[test]
    fn parser_parses_string() {
        let src = r#""string""#;
        let lexer = Lexer::new(src);
        let mut parser = Parser::new(lexer);

        let expected = vec![Statement::Expression(Expression::String("string".into()))];
        let actual = parser.parse().unwrap();

        assert_eq!(expected, actual);
    }

    #[test]
    fn lexer_reads_empty() {
        let src = "";
        let mut lexer = Lexer::new(src);

        let expected = vec![Token::EOF];

        for expected in expected.into_iter() {
            let actual = lexer.read();
            assert_eq!(expected, actual);
        }
    }

    #[test]
    fn lexer_reads_plus() {
        let src = "+";
        let mut lexer = Lexer::new(src);

        let expected = vec![Token::Plus, Token::EOF];

        for expected in expected.into_iter() {
            let actual = lexer.read();
            assert_eq!(expected, actual);
        }
    }

    #[test]
    fn lexer_reads_int() {
        let src = "12345";
        let mut lexer = Lexer::new(src);

        let expected = vec![Token::Int(12345), Token::EOF];

        for expected in expected.into_iter() {
            let actual = lexer.read();
            assert_eq!(expected, actual);
        }
    }

    #[test]
    fn lexer_reads_string() {
        let src = r#""string""#;
        let mut lexer = Lexer::new(src);

        let expected = vec![Token::String("string".into()), Token::EOF];

        for expected in expected.into_iter() {
            let actual = lexer.read();
            assert_eq!(expected, actual);
        }
    }
}
