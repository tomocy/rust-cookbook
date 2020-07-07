use std::error;

pub fn run() -> Result<(), Box<dyn error::Error>> {
    Err(From::from("not implemented"))
}

struct HTTP0_9Parser;

enum ParseResult<'a, E> {
    Ok(Request<'a>),
    Continuing,
    Err(E),
}

impl<'a, E> From<Result<Request<'a>, E>> for ParseResult<'a, E> {
    fn from(result: Result<Request<'a>, E>) -> Self {
        match result {
            Ok(req) => ParseResult::Ok(req),
            Err(err) => ParseResult::Err(err),
        }
    }
}

struct Request<'a>(&'a str);

##[cfg(test)]
mod tests {
    use super::*;
}