use crate::parse::parser::Parser;

use super::{context::LocalContext, term_arena::TermArena};

pub struct Terminal {
    parser: Parser,
    term_arena: TermArena,
    context: LocalContext,
}

impl Terminal {
    pub fn new() -> Self {
        Self {
            parser: Parser::new(),
            term_arena: TermArena::new(),
            context: LocalContext::new(),
        }
    }

    pub fn parse(&mut self, input: &str) {
        self.parser
            .parse(input, &mut self.context, &mut self.term_arena);
        todo!()
    }
}
