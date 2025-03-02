#[derive(Debug, PartialEq, Eq)]
pub(super) enum Token<'a> {
    // Term identifiers
    Identifier(&'a str),
    // Number Literals
    // TODO: How do I want to represent numbers?
    Number(&'a str),

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
    // *
    Star,
    // /
    Slash,
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
    NatType,
    // succ
    Succ,
    // zero
    Zero,
    // Empty
    EmptyType,
    // Unit
    UnitType,
    // refl
    Refl,
}
