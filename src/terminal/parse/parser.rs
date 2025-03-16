use crate::{term::terms::Term, terminal::context::Context};

use super::{lexer::Lexer, token::Token};

pub(crate) enum ParseOk {
    Ok,
    ContinueDefinition,
}

#[derive(Debug)]
pub(crate) enum ParseError {
    General(&'static str),
    ExpectedIdentifier,
}

type ParseResult = Result<(), ParseError>;

#[derive(Debug)]
pub(crate) enum Expression {
    ConstantName(String),
}

#[derive(Debug)]
enum ParseState {
    Init,
    ConstantParameters,
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

    pub(crate) fn parse(&mut self, input: &str, context: &mut Context) -> ParseResult {
        let mut lexer = Lexer::new(input);
        match self.state {
            ParseState::Init => match lexer.next() {
                Some(Token::Def) => self.parse_definition(&mut lexer, context),
                None => Ok(()),
                _ => todo!("Decide which tokens are okay in the parsing Init state."),
            },
            ParseState::ConstantParameters => todo!(),
        }
    }

    pub(crate) fn parse_definition(
        &mut self,
        lexer: &mut Lexer,
        context: &mut Context,
    ) -> ParseResult {
        self.state = ParseState::ConstantParameters;
        if let Some(token) = lexer.next() {
            if let Token::Identifier(name) = token {
                self.get_constant_parameters(lexer, context);
                todo!("Decide what to do after parameters for a user-defined constant are set.")
            } else {
                Err(ParseError::ExpectedIdentifier)
            }
        } else {
            Ok(())
        }
    }

    fn get_constant_parameters(
        &mut self,
        lexer: &mut Lexer<'_>,
        context: &mut Context,
    ) -> ParseResult {
        todo!()
    }
}

#[cfg(test)]
mod tests {
    use crate::terminal::context::Context;

    use super::Parser;

    #[test]
    fn test_identity_function_parsing1() {
        let identity_signature1 = "def identity : (A:Type) -> A -> A :=";
        let mut parser = Parser::new();
        let mut context = &mut Context::new();
        parser.parse(&identity_signature1, context);
    }

    #[test]
    fn test_identity_function_parsing2() {
        let identity_signature2 = "def identity(A:Type) : A -> A :=";
        let mut parser = Parser::new();
        let mut context = &mut Context::new();
        parser.parse(&identity_signature2, context);
    }

    #[test]
    fn test_identity_function_parsing3() {
        let identity_signature2 = "def identity(A:Type)(x:A): A :=";
        let mut parser = Parser::new();
        let mut context = &mut Context::new();
        parser.parse(&identity_signature2, context);
    }
}
