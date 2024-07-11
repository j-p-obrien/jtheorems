pub struct ParseError;

pub struct Parser;

pub struct ParsedInput;

impl Parser {
    pub fn new() -> Self {
        Self
    }

    pub fn parse(&self, input: &str) -> Result<ParsedInput, ParseError> {
        todo!()
    }
}
