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

    fn handle_minus(&mut self) -> Token<'a> {
        if self.chars.next_if(|&(_, chr)| chr == '>').is_some() {
            Token::RightArrow
        } else {
            Token::Minus
        }
    }

    fn handle_d(&mut self, start_index: usize) -> Token<'a> {
        let string = self.get_alphanumeric_starting_at(start_index);
        if string == "def" {
            Token::Def
        } else {
            Token::Identifier(string)
        }
    }

    fn next_nonwhitespace_char(&mut self) -> Option<(usize, char)> {
        self.chars.find(|&(_, chr)| !chr.is_whitespace())
    }

    fn handle_T(&mut self, start_index: usize) -> Token<'a> {
        let string = self.get_alphanumeric_starting_at(start_index);
        if string == "Type" {
            Token::Type
        } else {
            Token::Identifier(string)
        }
    }

    fn get_alphanumeric_starting_at(&mut self, start_index: usize) -> &'a str {
        let mut end_index = start_index;
        while let Some((i, _)) = self.chars.next_if(|&(_, chr)| chr.is_alphanumeric()) {
            end_index = i;
        }
        &self.input[start_index..=end_index]
    }

    fn handle_N(&mut self, index: usize) -> Token<'a> {
        todo!("Implement 'handle_N()' function.")
    }
}

impl<'a> Iterator for Lexer<'a> {
    type Item = Token<'a>;

    fn next(&mut self) -> Option<Self::Item> {
        let (index, chr) = self.next_nonwhitespace_char()?;
        Some(match chr {
            '(' => Token::LeftParen,
            ')' => Token::RightParen,
            '{' => Token::LeftBrace,
            '}' => Token::RightBrace,
            '[' => Token::LeftBracket,
            ']' => Token::RightBracket,
            ':' => self.handle_colon(),
            '=' => Token::Equals,
            '+' => Token::Plus,
            '-' => self.handle_minus(),
            '<' => Token::LessThan,
            '>' => Token::GreaterThan,
            'd' => self.handle_d(index),
            'T' => self.handle_T(index),
            'N' => self.handle_N(index),
            chr if chr.is_alphabetic() => {
                Token::Identifier(self.get_alphanumeric_starting_at(index))
            }
            chr => todo!("Unrecognized character '{chr}'."),
        })
    }
}

#[cfg(test)]
mod tests {
    use crate::parse::{lexer::Lexer, token::Token};

    #[test]
    fn test_identity_function_signature() {
        let identity_definition = "def identity : {A:Type} -> A -> A :=";
        let lexer = Lexer::new(&identity_definition);
        let tokens: Vec<_> = lexer.collect();
        assert_eq!(
            tokens,
            vec![
                Token::Def,
                Token::Identifier("identity"),
                Token::Colon,
                Token::LeftBrace,
                Token::Identifier("A"),
                Token::Colon,
                Token::Type,
                Token::RightBrace,
                Token::RightArrow,
                Token::Identifier("A"),
                Token::RightArrow,
                Token::Identifier("A"),
                Token::ColonEquals
            ]
        )
    }

    #[test]
    fn test_nat_add_function_signature() {
        let add_definition = "def add : Nat -> Nat -> Nat :=";
        let lexer = Lexer::new(&add_definition);
        let tokens: Vec<_> = lexer.collect();
        assert_eq!(
            tokens,
            vec![
                Token::Def,
                Token::Identifier("add"),
                Token::Colon,
                Token::Nat,
                Token::RightArrow,
                Token::Nat,
                Token::RightArrow,
                Token::Nat,
                Token::ColonEquals,
            ]
        )
    }
}
