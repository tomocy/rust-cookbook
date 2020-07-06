use std::error;

pub fn run() -> Result<(), Box<dyn error::Error>> {
    Err(From::from("not implemented"))
}

struct Server {
    address: String,
}

impl Server {
    fn new(address: String) -> Self {
        Self { address }
    }
}
