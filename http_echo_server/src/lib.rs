use std::error;

pub fn run() -> Result<(), Box<dyn error::Error>> {
    Err(From::from("not implemented"))
}

struct Request<'a>(&'a str);
