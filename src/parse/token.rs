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
    // [
    LeftBracket,
    // ]
    RightBracket,
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
    // =>
    FatRightArrow,

    // Keywords
    // def
    Def,
    // Type
    Type,
    // fn
    Fn,

    // Predefined Stuff
    // Nat
    Nat,
    // succ
    Succ,
    // zero
    Zero,
}
