use std::error;
use std::str;

pub fn run() -> Result<(), Box<dyn error::Error>> {
    Err(From::from("not implemented"))
}

struct HTTP0_9Parser;

impl HTTP0_9Parser {
    const CRLF: &'static [u8] = b"\r\n";

    fn parse(req: &[u8]) -> ParseResult<Request, Box<dyn error::Error>> {
        if !Self::ends_with_crlf(req) {
            return ParseResult::Continuing;
        }

        let method = Self::parse_method(req);
        if !Self::supports_method(method) {
            return ParseResult::Err(From::from("unsupported method"));
        }

        let req = Self::trim_trailing_crlf(req);

        str::from_utf8(req).map(Request).map_err(From::from).into()
    }

    fn parse_method(buf: &[u8]) -> &[u8] {
        buf.split(|&b| b == b' ').next().unwrap()
    }

    fn supports_method(method: &[u8]) -> bool {
        vec![b"GET"].iter().any(|&supported| method == supported)
    }

    fn trim_trailing_crlf(buf: &[u8]) -> &[u8] {
        if !Self::ends_with_crlf(buf) {
            buf
        } else {
            &buf[0..buf.len() - Self::CRLF.len()]
        }
    }

    fn ends_with_crlf(buf: &[u8]) -> bool {
        buf.ends_with(Self::CRLF)
    }
}

#[derive(Debug)]
enum ParseResult<T, E> {
    Ok(T),
    Continuing,
    Err(E),
}

impl<T, E> From<Result<T, E>> for ParseResult<T, E> {
    fn from(result: Result<T, E>) -> Self {
        match result {
            Ok(ok) => ParseResult::Ok(ok),
            Err(err) => ParseResult::Err(err),
        }
    }
}

struct Error;

impl From<Error> for Box<dyn error::Error> {
    fn from(err: Error) -> Self {
        From::from("")
    }
}

impl From<str::Utf8Error> for Error {
    fn from(err: str::Utf8Error) -> Self {
        Error
    }
}

#[derive(Debug, PartialEq)]
struct Request<'a>(&'a str);

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn http0_9parser_succeed_to_parse() {
        let req = "GET /a/b\r\n".as_bytes();
        let res = HTTP0_9Parser::parse(req);

        match res {
            ParseResult::Ok(_) => assert!(true),
            _ => assert!(false),
        }
    }

    #[test]
    fn http0_9parser_continue_parsing() {
        let req = "GET /\r".as_bytes();
        let res = HTTP0_9Parser::parse(req);

        match res {
            ParseResult::Continuing => assert!(true),
            _ => assert!(false),
        }
    }

    #[test]
    fn http0_9parser_failed_to_parse() {
        let req = "POST /\r\n".as_bytes();
        let res = HTTP0_9Parser::parse(req);

        match res {
            ParseResult::Err(_) => assert!(true),
            _ => assert!(false),
        }
    }
}
