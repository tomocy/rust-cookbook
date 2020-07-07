use std::error;
use std::io::{Read, Write};
use std::net;
use std::str;
use std::thread;

pub fn run<T: Iterator<Item = String>>(args: T) -> Result<(), Box<dyn error::Error>> {
    let config = Config::new(args)?;
    let server = Server::new(config.address);
    server.listen_and_serve()
}

struct Config {
    address: String,
}

impl Config {
    fn new<T: Iterator<Item = String>>(mut args: T) -> Result<Self, Box<dyn error::Error>> {
        match args.next() {
            Some(address) => Ok(Self { address }),
            None => Err("address is not specified".into()),
        }
    }
}

struct Server {
    address: String,
}

impl Server {
    const BUF_SIZE: usize = 1024;

    fn new(address: String) -> Self {
        Self { address }
    }

    fn listen_and_serve(&self) -> Result<(), Box<dyn error::Error>> {
        let listener = net::TcpListener::bind(&self.address)?;

        Self::log_info(&format!("listen on {}", self.address));

        for stream in listener.incoming() {
            let stream = stream?;
            let work = Self::spawn_worker();

            thread::spawn(move || {
                if let Err(err) = work(stream) {
                    Self::log_error(&format!("failed to serve: {}", err));
                }
            });
        }

        Ok(())
    }

    fn spawn_worker() -> Box<dyn Fn(net::TcpStream) -> Result<(), Box<dyn error::Error>> + Send> {
        Box::new(|mut stream| {
            let mut read_req = Vec::new();
            loop {
                let mut req = [0; Self::BUF_SIZE];
                let n = stream.read(&mut req)?;
                req[0..n].iter().for_each(|&b| read_req.push(b));

                match HTTP0_9Parser.parse(&read_req) {
                    ParseResult::Ok(req) => {
                        stream.write(req.into())?;
                        return Ok(());
                    }
                    ParseResult::Continuing => continue,
                    ParseResult::Err(err) => return Err(err),
                }
            }
        })
    }

    fn log_info(msg: &str) {
        Self::log("INFO", msg);
    }

    fn log_error(msg: &str) {
        Self::log("ERROR", msg);
    }

    fn log(level: &str, msg: &str) {
        println!("[{}] {}", level, msg);
    }
}

trait RequestParser {
    fn parse<'req>(&self, req: &'req [u8]) -> ParseResult<Request<'req>, Box<dyn error::Error>>;
}

struct HTTP0_9Parser;

impl RequestParser for HTTP0_9Parser {
    fn parse<'req>(&self, req: &'req [u8]) -> ParseResult<Request<'req>, Box<dyn error::Error>> {
        if !Self::ends_with_crlf(req) {
            return ParseResult::Continuing;
        }

        let method = Self::parse_method(req);
        if !Self::supports_method(method) {
            return ParseResult::Err(From::from("unsupported method"));
        }

        let req = Self::trim_trailing_crlf(req);

        str::from_utf8(req).map(Request).into()
    }
}

impl HTTP0_9Parser {
    const CRLF: &'static [u8] = b"\r\n";

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

impl<T, E> From<Result<T, E>> for ParseResult<T, Box<dyn error::Error>>
where
    E: Into<Box<dyn error::Error>>,
{
    fn from(result: Result<T, E>) -> Self {
        match result {
            Ok(ok) => ParseResult::Ok(ok),
            Err(err) => ParseResult::Err(err.into()),
        }
    }
}

#[derive(Debug, PartialEq)]
struct Request<'a>(&'a str);

impl<'a> From<Request<'a>> for &'a [u8] {
    fn from(req: Request<'a>) -> Self {
        req.0.as_bytes()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn http0_9parser_succeed_to_parse() {
        let req = "GET /a/b\r\n".as_bytes();
        let res = HTTP0_9Parser.parse(req);

        match res {
            ParseResult::Ok(_) => assert!(true),
            _ => assert!(false),
        }
    }

    #[test]
    fn http0_9parser_continue_parsing() {
        let req = "GET /\r".as_bytes();
        let res = HTTP0_9Parser.parse(req);

        match res {
            ParseResult::Continuing => assert!(true),
            _ => assert!(false),
        }
    }

    #[test]
    fn http0_9parser_failed_to_parse() {
        let req = "POST /\r\n".as_bytes();
        let res = HTTP0_9Parser.parse(req);

        match res {
            ParseResult::Err(_) => assert!(true),
            _ => assert!(false),
        }
    }
}
