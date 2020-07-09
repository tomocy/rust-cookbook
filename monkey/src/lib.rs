use std::error;

pub fn run<T: Iterator<Item = String>>(_: T) -> Result<(), Box<dyn error::Error>> {
    Err("not implemented".into())
}

enum Token {
    Illegal(String),
    EOF,
}

#[cfg(test)]
mod tests {}
