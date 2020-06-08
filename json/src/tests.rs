use super::*;

#[test]
fn parse_empty() {
    let src = "";

    parse(src).expect_err("input should not be empty");
}

#[test]
fn parse_unknown() {
    let src = "@";

    parse(src).expect_err("token '@' is unknown");
}

#[test]
fn parse_number() {
    let src = "12345";
    let expected = Value::Number(12345);

    let actual = parse(src).expect("should have succeeded to parse");

    assert_eq!(actual, expected);
}

#[test]
fn parse_string() {
    let src = "\"aiueo\"";
    let expected = Value::String("aiueo".to_string());

    let actual = parse(src).expect("should have succeeded to parse");

    assert_eq!(actual, expected);
}

#[test]
fn parse_true() {
    let src = "true";
    let expected = Value::Bool(true);

    let actual = parse(src).expect("should have succeeded to parse");

    assert_eq!(actual, expected);
}

#[test]
fn parse_false() {
    let src = "false";
    let expected = Value::Bool(false);

    let actual = parse(src).expect("should have succeeded to parse");

    assert_eq!(actual, expected);
}

#[test]
fn parse_null() {
    let src = "null";
    let expected = Value::Null;

    let actual = parse(src).expect("should have succeeded to parse");

    assert_eq!(actual, expected);
}

#[test]
fn parse_empty_object() {
    let src = "{}";
    let expected = Value::Object(vec![]);

    let actual = parse(src).expect("should have succeeded to parse");

    assert_eq!(actual, expected);
}

#[test]
fn parse_object_with_single_property() {
    let src = r#"{"aiueo": 12345}"#;
    let expected = Value::Object(vec![Property::new("aiueo", Value::Number(12345))]);

    let actual = parse(src).expect("should have succeeded to parse");

    assert_eq!(actual, expected);
}

#[test]
fn parse_object_with_multiple_properties() {
    let src = r#"{
    "aaa": 111,
    "bbb": 222,
    "ccc": 333
}"#;
    let expected = Value::Object(vec![
        Property::new("aaa", Value::Number(111)),
        Property::new("bbb", Value::Number(222)),
        Property::new("ccc", Value::Number(333)),
    ]);

    let actual = parse(src).expect("should have succeeded to parse");

    assert_eq!(actual, expected);
}

#[test]
fn parse_empty_array() {
    let src = "[]";
    let expected = Value::Array(vec![]);

    let actual = parse(src).expect("should have succeeded to parse");

    assert_eq!(actual, expected);
}

#[test]
fn parse_array_with_single_element() {
    let src = "[1]";
    let expected = Value::Array(vec![Value::Number(1)]);

    let actual = parse(src).expect("should have succeeded to parse");

    assert_eq!(actual, expected);
}

#[test]
fn parse_array_with_multiple_elements() {
    let src = r#"[1, "two", 3, "four"]"#;
    let expected = Value::Array(vec![
        Value::Number(1),
        Value::String("two".to_string()),
        Value::Number(3),
        Value::String("four".to_string()),
    ]);

    let actual = parse(src).expect("should have succeeded to parse");

    assert_eq!(actual, expected);
}
