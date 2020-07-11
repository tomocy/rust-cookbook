use std::error;

pub fn run<T: Iterator<Item = String>>(_: T) -> Result<(), Box<dyn error::Error>> {
    Err("not implemented".into())
}

struct Parser<'src> {
    lexer: Lexer<'src>,
    curr_token: Token,
    reading_token: Token,
}

impl<'src> Parser<'src> {
    fn new(lexer: Lexer<'src>) -> Self {
        let mut parser = Self {
            lexer,
            curr_token: Token::EOF,
            reading_token: Token::EOF,
        };

        parser.read();
        parser.read();

        parser
    }

    fn parse(&mut self) -> Result<Program, Box<dyn error::Error>> {
        let mut program = Vec::new();

        while !self.have_token(Token::EOF) {
            program.push(self.parse_statement()?);
        }

        Ok(program)
    }

    fn parse_statement(&mut self) -> Result<Statement, Box<dyn error::Error>> {
        let stat = self.parse_expression_statement()?;
        self.read();

        Ok(stat)
    }

    fn parse_expression_statement(&mut self) -> Result<Statement, Box<dyn error::Error>> {
        Ok(Statement::Expression(
            self.parse_expression(InfixOperatorPrecedence::Lowest)?,
        ))
    }

    fn parse_expression<P: Into<InfixOperatorPrecedence>>(
        &mut self,
        prec: P,
    ) -> Result<Expression, Box<dyn error::Error>> {
        let prec = prec.into();
        let mut exp = self.parse_prefix_expression()?;
        while prec < self.reading_token.clone().into() {
            self.read();
            exp = self.parse_infix_expression(exp)?;
        }

        Ok(exp)
    }

    fn parse_prefix_expression(&self) -> Result<Expression, Box<dyn error::Error>> {
        match self.curr_token.clone() {
            Token::Int(x) => Ok(Expression::Int(x)),
            Token::String(x) => Ok(Expression::String(x)),
            _ => Err("invalid tokenen".into()),
        }
    }

    fn parse_infix_expression(
        &mut self,
        left: Expression,
    ) -> Result<Expression, Box<dyn error::Error>> {
        let operator = self.parse_infix_operator()?;
        self.read();

        let right = self.parse_expression(operator)?;

        Ok(Expression::Infix {
            left: Box::new(left),
            operator,
            right: Box::new(right),
        })
    }

    fn parse_infix_operator(&self) -> Result<InfixOperator, Box<dyn error::Error>> {
        match self.curr_token {
            Token::Plus => Ok(InfixOperator::Plus),
            _ => Err("invalid tokenen".into()),
        }
    }

    fn have_token(&self, curr_token: Token) -> bool {
        self.curr_token == curr_token
    }

    fn read(&mut self) {
        self.curr_token = self.reading_token.clone();
        self.reading_token = self.lexer.read();
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

#[derive(Debug, PartialEq, Clone, Copy)]
enum InfixOperator {
    Plus,
}

impl From<InfixOperator> for InfixOperatorPrecedence {
    fn from(op: InfixOperator) -> Self {
        match op {
            InfixOperator::Plus => Self::Additive,
            _ => Self::Lowest,
        }
    }
}

#[derive(Debug, PartialEq, PartialOrd, Clone, Copy)]
enum InfixOperatorPrecedence {
    Lowest,
    Additive,
}

struct Lexer<'src> {
    src: &'src str,
    curr_pos: usize,
    reading_pos: usize,
}

impl<'src> Lexer<'src> {
    const EOF: u8 = 0;

    fn new(src: &'src str) -> Self {
        Self {
            src,
            curr_pos: 0,
            reading_pos: 0,
        }
    }

    fn read(&mut self) -> Token {
        self.read_char();

        self.skip_whitespaces();

        let ch = self.char();
        match ch {
            Self::EOF => Token::EOF,
            b'+' => Token::Plus,
            b';' => Token::Semicolon,
            b'"' => Token::String(self.read_string()),
            _ if self.have_digit() => Token::Int(self.read_number()),
            _ => Token::Illegal(String::from_utf8(vec![ch]).unwrap()),
        }
    }

    fn skip_whitespaces(&mut self) {
        while self.have_whitespace() {
            self.read_char();
        }
    }

    fn read_string(&mut self) -> String {
        debug_assert_eq!(b'"', self.char());
        self.read_char();

        let begin = self.curr_pos;

        while self.have_letter() {
            self.read_char();
        }

        let end = self.curr_pos;

        debug_assert_eq!(b'"', self.char());
        self.read_char();

        self.src[begin..end].into()
    }

    fn read_number(&mut self) -> i32 {
        let begin = self.curr_pos;

        while self.have_digit() {
            self.read_char();
        }

        self.src[begin..self.curr_pos].parse().unwrap()
    }

    fn read_char(&mut self) {
        if self.curr_pos >= self.src.len() {
            return;
        }

        self.curr_pos = self.reading_pos;
        self.reading_pos += 1;
    }

    fn have_whitespace(&self) -> bool {
        let ch = self.char();
        ch == b' ' || ch == b'\t' || ch == b'\r' || ch == b'\n'
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
        if self.curr_pos >= self.src.len() {
            Self::EOF
        } else {
            self.src.as_bytes()[self.curr_pos]
        }
    }
}

#[derive(Debug, PartialEq, Clone)]
enum Token {
    Illegal(String),
    EOF,
    Semicolon,
    Plus,
    Int(i32),
    String(String),
}

impl From<Token> for InfixOperatorPrecedence {
    fn from(token: Token) -> Self {
        match token {
            Token::Plus => Self::Additive,
            _ => Self::Lowest,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parser_parses() {
        let src = r#"1 + 2"#;
        let lexer = Lexer::new(src);
        let mut parser = Parser::new(lexer);

        let expected: Program = vec![Statement::Expression(Expression::Infix {
            left: Box::new(Expression::Int(1)),
            operator: InfixOperator::Plus,
            right: Box::new(Expression::Int(2)),
        })];
        let actual = parser.parse().unwrap();

        assert_eq!(expected, actual);
    }

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
    fn lexer_reads() {
        let src = r#"1 + 2"#;
        let mut lexer = Lexer::new(src);

        let expected = vec![Token::Int(1), Token::Plus, Token::Int(2), Token::EOF];

        for expected in expected.into_iter() {
            let actual = lexer.read();
            assert_eq!(expected, actual);
        }
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
    fn lexer_reads_semicolon() {
        let src = ";";
        let mut lexer = Lexer::new(src);

        let expected = vec![Token::Semicolon, Token::EOF];

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
