#[cfg(test)]
mod tests;

use super::lexer;

pub struct Parser {
    lexer: lexer::Lexer,
}

impl Parser {
    pub fn new(lexer: lexer::Lexer) -> Self {
        Self { lexer }
    }
}
