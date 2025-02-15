#[derive(Debug, PartialEq, Eq)]
pub(super) enum Token<'a> {
    Identifier(&'a str),

    // Symbols
    // (
    LeftParen,
    // )
    RightParen,
    // {
    LeftBrace,
    // }
    RightBrace,
    // :
    Colon,
    // =
    Equals,
    // :=
    ColonEquals,
    // +
    Plus,
    // -
    Minus,
    // >
    GreaterThan,
    // <
    LessThan,
    // ->
    RightArrow,

    // Keywords
    Keyword(KeyWord),
}

#[derive(Debug, PartialEq, Eq)]
pub(super) enum KeyWord {
    Def,
    Type,
}
