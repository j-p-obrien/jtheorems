use super::lexer::Lexer;

pub struct Parser<'a> {
    lexer: Lexer<'a>,
}

impl<'a> Parser<'a> {
    pub fn new(input: &'a str) -> Self {
        Self {
            lexer: Lexer::new(input),
        }
    }

    // This is just here so I won't get fined (by Rust Analyzer for not using certain values).
    pub fn inc(&mut self) {
        self.lexer.next();
    }
}
