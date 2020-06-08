use super::super::*;

#[test]
fn read_empty() {
    let src = "";
    let expected = vec![lexer::Token::EOF];

    let mut lexer = lexer::Lexer::new(src);

    for expected in expected {
        let actual = lexer.read_token();

        assert_eq!(expected, actual);
    }
}

#[test]
fn read_number() {
    let src = "12345";
    let expected = vec![lexer::Token::Number(12345), lexer::Token::EOF];

    let mut lexer = lexer::Lexer::new(src);

    for expected in expected {
        let actual = lexer.read_token();

        assert_eq!(expected, actual);
    }
}