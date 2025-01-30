#[derive(Debug, PartialEq, Eq)]
pub(super) enum Token<'a> {
    Identifier(&'a str),
    // Symbols
    LeftParen,
    RightParen,
    LeftBrace,
    RightBrace,
    Colon,
    Equals,
    //
    Keyword(KeyWord),
}

#[derive(Debug, PartialEq, Eq)]
pub(super) enum KeyWord {
    Def,
}
