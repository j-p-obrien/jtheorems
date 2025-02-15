use crate::parse::parser::Parser;

use super::{context::LocalContext, term_arena::TermArena};

pub struct Terminal {
    term_arena: TermArena,
    context: LocalContext,
}

impl Terminal {
    pub fn new() -> Self {
        Self {
            term_arena: TermArena::new(),
            context: LocalContext::new(),
        }
    }

    pub fn parse(&mut self, input: &str) {
        let mut parser = Parser::new(input);
        parser.inc();
        todo!()
    }
}
