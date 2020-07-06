use std::error;
use std::io::{Read, Write};
use std::net;
use std::thread;

pub fn run<T: Iterator<Item = String>>(args: T) -> Result<(), Box<dyn error::Error>> {
    let config = Config::new(args)?;
    let server = Server::new(config.address);
    server.listen_and_serve()
}

#[derive(Debug, PartialEq)]
struct Config {
    address: String,
}

impl Config {
    fn new<T: Iterator<Item = String>>(mut args: T) -> Result<Self, Box<dyn error::Error>> {
        let address = match args.next() {
            Some(address) => address,
            None => return Err(From::from("address is not specified")),
        };

        Ok(Self { address })
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
        Self::log_info(&format!("listening on {}", self.address));

        for stream in listener.incoming() {
            let stream = match stream {
                Ok(stream) => stream,
                Err(err) => {
                    Self::log_error(&format!("failed to accept a stream: {}", err));
                    continue;
                }
            };

            Self::log_debug("accept a stream");

            let work = Self::spawn();
            thread::spawn(move || {
                if let Err(err) = work(stream) {
                    Self::log_error(&format!("failed to work: {}", err));
                }

                Self::log_debug("succeed to work");
            });

            Self::log_debug("accept a stream");
        }

        Ok(())
    }

    fn spawn() -> Box<dyn Fn(net::TcpStream) -> Result<(), Box<dyn error::Error>> + Send> {
        Box::new(|mut stream: net::TcpStream| loop {
            let mut buf = [0; Self::BUF_SIZE];
            let n = stream.read(&mut buf)?;
            stream.write(&buf[0..n])?;
            if n < Self::BUF_SIZE {
                return Ok(());
            }
        })
    }

    fn log_info(msg: &str) {
        Self::log("INFO", msg);
    }

    fn log_debug(msg: &str) {
        Self::log("DEBUG", msg);
    }

    fn log_error(msg: &str) {
        Self::log("ERROR", msg);
    }

    fn log(level: &str, msg: &str) {
        println!("[{}] {}", level, msg);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn new_config() {
        let expected = Config {
            address: "127.0.0.1:8080".to_string(),
        };

        let actual = Config::new(vec!["127.0.0.1:8080"].iter().map(|s| s.to_string())).unwrap();

        assert_eq!(expected, actual);
    }

    #[test]
    #[should_panic]
    fn new_config_without_address() {
        let args: Vec<String> = Vec::new();
        Config::new(args.into_iter()).unwrap();
    }
}
