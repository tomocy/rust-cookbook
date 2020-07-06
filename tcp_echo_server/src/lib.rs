use std::error;
use std::io::{Read, Write};
use std::net;
use std::thread;

pub fn run() -> Result<(), Box<dyn error::Error>> {
    let server = Server::new("localhost:8080".to_string());
    server.listen_and_serve()
}

struct Server {
    address: String,
}

impl Server {
    fn new(address: String) -> Self {
        Self { address }
    }

    fn listen_and_serve(&self) -> Result<(), Box<dyn error::Error>> {
        let listener = net::TcpListener::bind(&self.address)?;
        println!("listening on {}", self.address);

        for stream in listener.incoming() {
            let stream = match stream {
                Ok(stream) => stream,
                Err(err) => {
                    eprintln!("failed to accept a stream: {}", err);
                    continue;
                }
            };

            let work = self.spawn_worker();
            thread::spawn(move || {
                if let Err(err) = work(stream) {
                    eprintln!("faield to serve a stream: {}", err);
                }
            });
        }

        Ok(())
    }

    fn spawn_worker(
        &self,
    ) -> Box<dyn Fn(net::TcpStream) -> Result<(), Box<dyn error::Error>> + Send> {
        Box::new(|mut stream: net::TcpStream| loop {
            let mut buf = [0; 1024];
            let n = stream.read(&mut buf)?;
            stream.write(&buf[0..n])?;
        })
    }
}
