use super::super::parser;
use super::lexer;

#[test]
fn parse_empty() {
    let src = "";

    let lexer = lexer::Lexer::new(src);
    let mut parser = parser::Parser::new(lexer);

    parser.parse().expect_err("input should not be empty");
}

#[test]
fn parse_unknown() {
    let src = "@";

    let lexer = lexer::Lexer::new(src);
    let mut parser = parser::Parser::new(lexer);

    parser.parse().expect_err("token '@' is unknown");
}

#[test]
fn parse_number() {
    let src = "12345";
    let expected = vec![parser::Value::Number(12345)];

    let lexer = lexer::Lexer::new(src);
    let mut parser = parser::Parser::new(lexer);

    for expected in expected {
        let actual = parser.parse().expect("should have succeeded to parse");

        assert_eq!(actual, expected);
    }
}

#[test]
fn parse_string() {
    let src = "\"aiueo\"";
    let expected = vec![parser::Value::String("aiueo".to_string())];

    let lexer = lexer::Lexer::new(src);
    let mut parser = parser::Parser::new(lexer);

    for expected in expected {
        let actual = parser.parse().expect("should have succeeded to parse");

        assert_eq!(actual, expected);
    }
}
