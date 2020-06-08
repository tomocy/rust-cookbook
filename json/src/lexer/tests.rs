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
