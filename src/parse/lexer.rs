use std::{
    iter::Peekable,
    str::{CharIndices, Chars},
};

use super::token::Token;

pub(super) struct Lexer<'a> {
    input: &'a str,
    chars: Peekable<CharIndices<'a>>,
}

impl<'a> Lexer<'a> {
    pub(super) fn new(input: &'a str) -> Self {
        Self {
            input,
            chars: input.char_indices().peekable(),
        }
    }

    fn handle_colon(&self) -> Token<'a> {
        todo!()
    }
}

impl<'a> Iterator for Lexer<'a> {
    type Item = Token<'a>;

    fn next(&mut self) -> Option<Self::Item> {
        if let Some((i, chr)) = self.chars.next() {
            Some(match chr {
                '(' => Token::LeftParen,
                ')' => Token::RightParen,
                ':' => self.handle_colon(),
                '=' => Token::Equals,
                _ => todo!(),
            })
        } else {
            None
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::parse::{
        lexer::Lexer,
        token::{KeyWord, Token},
    };

    fn test_identity_function_signature() {
        let identity_definition = "
        def identity{A : U }(x:A) -> A := 
            intro x
            x";
        let mut lexer = Lexer::new(&identity_definition);
        assert_eq!(lexer.next(), Some(Token::Keyword(KeyWord::Def)));
        todo!()
    }
}
