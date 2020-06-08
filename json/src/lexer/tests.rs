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
fn read_unknown() {
    let src = "@";
    let expected = vec![lexer::Token::Unknown("@".to_string()), lexer::Token::EOF];

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

#[test]
fn read_string() {
    let src = "\"aiueo\"";
    let expected = vec![lexer::Token::String("aiueo".to_string()), lexer::Token::EOF];

    let mut lexer = lexer::Lexer::new(src);

    for expected in expected {
        let actual = lexer.read_token();

        assert_eq!(expected, actual);
    }
}

#[test]
fn read_object_with_single_property() {
    let src = "{\"aiueo\": 12345}";
    let expected = vec![
        lexer::Token::LBrace,
        lexer::Token::String("aiueo".to_string()),
        lexer::Token::Colon,
        lexer::Token::Number(12345),
        lexer::Token::RBrace,
        lexer::Token::EOF,
    ];

    let mut lexer = lexer::Lexer::new(src);

    for expected in expected {
        let actual = lexer.read_token();

        assert_eq!(expected, actual);
    }
}

#[test]
fn read_object_with_multiple_properties() {
    let src = "{\"aiueo\": 12345, \"wawonn\": 98765}";
    let expected = vec![
        lexer::Token::LBrace,
        lexer::Token::String("aiueo".to_string()),
        lexer::Token::Colon,
        lexer::Token::Number(12345),
        lexer::Token::Comma,
        lexer::Token::String("wawonn".to_string()),
        lexer::Token::Colon,
        lexer::Token::Number(98765),
        lexer::Token::RBrace,
        lexer::Token::EOF,
    ];

    let mut lexer = lexer::Lexer::new(src);

    for expected in expected {
        let actual = lexer.read_token();

        assert_eq!(expected, actual);
    }
}
