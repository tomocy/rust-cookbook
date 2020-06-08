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

#[test]
fn parse_empty_object() {
    let src = "{}";
    let expected = vec![parser::Value::Object(vec![])];

    let lexer = lexer::Lexer::new(src);
    let mut parser = parser::Parser::new(lexer);

    for expected in expected {
        let actual = parser.parse().expect("should have succeeded to parse");

        assert_eq!(actual, expected);
    }
}

#[test]
fn parse_object_with_single_property() {
    let src = r#"{"aiueo": 12345}"#;
    let expected = vec![parser::Value::Object(vec![parser::Property::new(
        "aiueo",
        parser::Value::Number(12345),
    )])];

    let lexer = lexer::Lexer::new(src);
    let mut parser = parser::Parser::new(lexer);

    for expected in expected {
        let actual = parser.parse().expect("should have succeeded to parse");

        assert_eq!(actual, expected);
    }
}

#[test]
fn parse_object_with_multiple_properties() {
    let src = r#"{
    "aaa": 111,
    "bbb": 222,
    "ccc": 333
}"#;
    let expected = vec![parser::Value::Object(vec![
        parser::Property::new("aaa", parser::Value::Number(111)),
        parser::Property::new("bbb", parser::Value::Number(222)),
        parser::Property::new("ccc", parser::Value::Number(333)),
    ])];

    let lexer = lexer::Lexer::new(src);
    let mut parser = parser::Parser::new(lexer);

    for expected in expected {
        let actual = parser.parse().expect("should have succeeded to parse");

        assert_eq!(actual, expected);
    }
}
