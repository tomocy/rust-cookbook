use std::error;

pub fn run() -> Result<(), Box<dyn error::Error>> {
    Err(From::from("not implemented"))
}

enum ParseResult<'a, E> {
    Ok(Request<'a>),
    Continuing,
    Err(E),
}

struct Request<'a>(&'a str);
