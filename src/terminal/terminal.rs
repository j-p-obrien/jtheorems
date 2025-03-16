use super::context::Context;

use super::parse::parser::Parser;

pub struct Terminal {
    parser: Parser,
    context: Context,
}

impl Terminal {
    pub fn new() -> Self {
        Self {
            parser: Parser::new(),
            context: Context::new(),
        }
    }

    pub fn parse(&mut self, input: &str) {
        self.parser.parse(input, &mut self.context);
        todo!()
    }
}
