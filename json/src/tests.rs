use super::lexer;
use super::*;

#[test]
fn parse_empty() {
    let src = "";

    let lexer = lexer::Lexer::new(src);
    let mut parser = Parser::new(lexer);

    parser.parse().expect_err("input should not be empty");
}

#[test]
fn parse_unknown() {
    let src = "@";

    let lexer = lexer::Lexer::new(src);
    let mut parser = Parser::new(lexer);

    parser.parse().expect_err("token '@' is unknown");
}

#[test]
fn parse_number() {
    let src = "12345";
    let expected = vec![Value::Number(12345)];

    let lexer = lexer::Lexer::new(src);
    let mut parser = Parser::new(lexer);

    for expected in expected {
        let actual = parser.parse().expect("should have succeeded to parse");

        assert_eq!(actual, expected);
    }
}

#[test]
fn parse_string() {
    let src = "\"aiueo\"";
    let expected = vec![Value::String("aiueo".to_string())];

    let lexer = lexer::Lexer::new(src);
    let mut parser = Parser::new(lexer);

    for expected in expected {
        let actual = parser.parse().expect("should have succeeded to parse");

        assert_eq!(actual, expected);
    }
}

#[test]
fn parse_empty_object() {
    let src = "{}";
    let expected = vec![Value::Object(vec![])];

    let lexer = lexer::Lexer::new(src);
    let mut parser = Parser::new(lexer);

    for expected in expected {
        let actual = parser.parse().expect("should have succeeded to parse");

        assert_eq!(actual, expected);
    }
}

#[test]
fn parse_object_with_single_property() {
    let src = r#"{"aiueo": 12345}"#;
    let expected = vec![Value::Object(vec![Property::new(
        "aiueo",
        Value::Number(12345),
    )])];

    let lexer = lexer::Lexer::new(src);
    let mut parser = Parser::new(lexer);

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
    let expected = vec![Value::Object(vec![
        Property::new("aaa", Value::Number(111)),
        Property::new("bbb", Value::Number(222)),
        Property::new("ccc", Value::Number(333)),
    ])];

    let lexer = lexer::Lexer::new(src);
    let mut parser = Parser::new(lexer);

    for expected in expected {
        let actual = parser.parse().expect("should have succeeded to parse");

        assert_eq!(actual, expected);
    }
}
