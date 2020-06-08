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
