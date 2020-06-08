use super::super::parser;
use super::lexer;

#[test]
fn parse_empty() {
    let src = "";

    let lexer = lexer::Lexer::new(src);
    let mut parser = parser::Parser::new(lexer);

    parser.parse().expect_err("input should not be empty");
}
