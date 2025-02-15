use std::{iter::Peekable, str::CharIndices};

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

    fn handle_colon(&mut self) -> Token<'a> {
        if self.chars.next_if(|&(_, chr)| chr == '=').is_some() {
            Token::ColonEquals
        } else {
            Token::Colon
        }
    }

    fn handle_minus(&self) -> Token<'a> {
        todo!("Need to implement lexing after a '-' is encountered.")
    }

    fn handle_d(&self) -> Token<'a> {
        todo!("Need to implement lexing after a 'd' is encountered.")
    }

    fn next_nonwhitespace_char(&mut self) -> Option<(usize, char)> {
        self.chars.find(|&(_, chr)| !chr.is_ascii_whitespace())
    }

    fn handle_T(&mut self) -> Token<'a> {
        todo!("Need to implement lexing after a 'T' is encountered.")
    }
}

impl<'a> Iterator for Lexer<'a> {
    type Item = Token<'a>;

    fn next(&mut self) -> Option<Self::Item> {
        if let Some((i, chr)) = self.next_nonwhitespace_char() {
            Some(match chr {
                '(' => Token::LeftParen,
                ')' => Token::RightParen,
                '{' => Token::LeftBrace,
                '}' => Token::RightBrace,
                ':' => self.handle_colon(),
                '=' => Token::Equals,
                '+' => Token::Plus,
                '-' => self.handle_minus(),
                '<' => Token::LessThan,
                '>' => Token::GreaterThan,
                'd' => self.handle_d(),
                'T' => self.handle_T(),
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

    #[test]
    fn test_identity_function_signature() {
        let identity_definition = "def identity : {A:Type} -> A -> A :=";
        let mut lexer = Lexer::new(&identity_definition);
        assert_eq!(lexer.next(), Some(Token::Keyword(KeyWord::Def)));
        assert_eq!(lexer.next(), Some(Token::Identifier("identity")));
        assert_eq!(lexer.next(), Some(Token::Colon));
        assert_eq!(lexer.next(), Some(Token::LeftBrace));
        assert_eq!(lexer.next(), Some(Token::Identifier("A")));
        assert_eq!(lexer.next(), Some(Token::Colon));
        assert_eq!(lexer.next(), Some(Token::Keyword(KeyWord::Type)));
        assert_eq!(lexer.next(), Some(Token::RightBrace));
        assert_eq!(lexer.next(), Some(Token::RightArrow));
        assert_eq!(lexer.next(), Some(Token::Identifier("A")));
        assert_eq!(lexer.next(), Some(Token::RightArrow));
        assert_eq!(lexer.next(), Some(Token::Identifier("A")));
        assert_eq!(lexer.next(), Some(Token::ColonEquals));
        todo!()
    }
}
