use crate::{
    parse::token::Token,
    term::terms::Term,
    terminal::{context::LocalContext, term_arena::TermArena},
};

use super::lexer::Lexer;

pub(crate) enum ParseOk {
    Ok,
    ContinueDefinition,
}

#[derive(Debug)]
pub(crate) enum ParseError {
    General(&'static str),
    ExpectedIdentifier,
}

type ParseResult = Result<ParseOk, ParseError>;

#[derive(Debug)]
pub(crate) enum Expression {
    ConstantName(String),
}

#[derive(Debug)]
enum ParseState {
    Init,
    ConstantDefinition,
}

#[derive(Debug)]
pub(crate) struct Parser {
    state: ParseState,
}

impl Parser {
    pub(crate) fn new() -> Self {
        Self {
            state: ParseState::Init,
        }
    }

    pub(crate) fn parse(
        &mut self,
        input: &str,
        context: &mut LocalContext,
        term_arena: &mut TermArena,
    ) -> ParseResult {
        let mut lexer = Lexer::new(input);
        match self.state {
            ParseState::Init => match lexer.next() {
                Some(Token::Def) => self.parse_definition(&mut lexer, context, term_arena),
                None => Ok(ParseOk::Ok),
                _ => todo!("Decide which tokens are okay in the parsing Init state."),
            },
            ParseState::ConstantDefinition => todo!(),
        }
    }

    pub(crate) fn parse_definition(
        &mut self,
        lexer: &mut Lexer,
        context: &mut LocalContext,
        term_arena: &mut TermArena,
    ) -> ParseResult {
        if let Some(token) = lexer.next() {
            if let Token::Identifier(name) = token {
                self.parse_definition_with_name(name, lexer, context, term_arena)
            } else {
                Err(ParseError::ExpectedIdentifier)
            }
        } else {
            Ok(ParseOk::ContinueDefinition)
        }
    }

    fn parse_definition_with_name(
        &mut self,
        name: &str,
        lexer: &mut Lexer<'_>,
        context: &mut LocalContext,
        term_arena: &mut TermArena,
    ) -> ParseResult {
        todo!()
    }
}

#[cfg(test)]
mod tests {
    use crate::terminal::{context::LocalContext, term_arena::TermArena};

    use super::Parser;

    #[test]
    fn test_identity_function_parsing() {
        let identity_signature = "def identity : {A:Type} -> A -> A :=";
        let mut parser = Parser::new();
        let mut context = &mut LocalContext::new();
        let mut term_arena = &mut TermArena::new();
        parser.parse(&identity_signature, context, term_arena);
    }
}
