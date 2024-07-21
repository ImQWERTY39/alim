#[derive(Debug, PartialEq, Eq)]
pub enum Keyword {
    // random keywords
    Import,
    Struct,
    Return,

    // types
    SignedInteger8,
    SignedInteger16,
    SignedInteger32,
    SignedInteger64,
    UnsignedInteger8,
    UnsignedInteger16,
    UnsignedInteger32,
    UnsignedInteger64,
    Float32,
    Float64,
    Character,
    Boolean,

    // bool stuff
    True,
    False,

    // conditional
    If,
    Else,
    While,
    For,
    In,
    Break,
    Continue,
}
