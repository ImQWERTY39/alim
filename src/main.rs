enum Keyword {
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
    Character,
    Boolean,

    // conditional
    If,
    Else,
    ElseIf,
    While,
    For,
    In,
    Break,
    Continue,
}

enum Operator {
    Assign,

    Add,
    AddAssign,

    Subtract,
    SubAssing,

    Multiply,
    MulAssign,

    Divide,
    DivAssign,

    Modulus,
    ModAssign,

    BitAnd,
    BitAndAssign,

    BitOr,
    BitOrAssign,

    BitNot,
    BitNotAssign,

    And,
    Or,
    Not,

    Equal,
    NotEqual,
    LessThan,
    LessThanEqual,
    GreaterThan,
    GreaterThanEqual,
}

enum Bracket {
    RoundOpen,
    RoundClose,
    SquareOpen,
    SquareClose,
    CurlyOpen,
    CurlyClose,
    AngleOpen,
    AngleClose,
}

enum Literal {
    Integer(i32),
    BigInteger(i128),
    Float(f32),
    BigFloat(f64),
    Character(char),
    Boolean(bool),
    Array(Vec<Literal>),
}

enum Tokens {
    KeywordToken(Keyword),
    OperatorToken(Operator),
    BracketToken(Bracket),
    LiteralToken(Literal),
}

fn main() {
    println!("Hello, world!");
}
