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
    Increment,

    Subtract,
    SubAssing,
    Decrement,

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

enum Token {
    KeywordToken(Keyword),
    OperatorToken(Operator),
    BracketToken(Bracket),
    LiteralToken(Literal),
    End,
}

impl From<&str> for Token {
    fn from(value: &str) -> Self {
        match value {
            "import" => Self::KeywordToken(Keyword::Import),
            "struct" => Self::KeywordToken(Keyword::Struct),
            "return" => Self::KeywordToken(Keyword::Return),
            "i8" => Self::KeywordToken(Keyword::SignedInteger8),
            "i16" => Self::KeywordToken(Keyword::SignedInteger16),
            "i32" => Self::KeywordToken(Keyword::SignedInteger32),
            "i64" => Self::KeywordToken(Keyword::SignedInteger64),
            "u8" => Self::KeywordToken(Keyword::UnsignedInteger8),
            "u16" => Self::KeywordToken(Keyword::UnsignedInteger16),
            "u32" => Self::KeywordToken(Keyword::UnsignedInteger32),
            "u64" => Self::KeywordToken(Keyword::UnsignedInteger64),
            "char" => Self::KeywordToken(Keyword::Character),
            "bool" => Self::KeywordToken(Keyword::Boolean),
            "if" => Self::KeywordToken(Keyword::If),
            "else" => Self::KeywordToken(Keyword::Else),
            "while" => Self::KeywordToken(Keyword::While),
            "for" => Self::KeywordToken(Keyword::For),
            "in" => Self::KeywordToken(Keyword::In),
            "break" => Self::KeywordToken(Keyword::Break),
            "continue" => Self::KeywordToken(Keyword::Continue),
            "=" => todo!(),
            "+" => todo!(),
            "+=" => todo!(),
            "++" => todo!(),
            "-" => todo!(),
            "-=" => todo!(),
            "--" => todo!(),
            "*" => todo!(),
            "*=" => todo!(),
            "/" => todo!(),
            "/=" => todo!(),
            "%" => todo!(),
            "%=" => todo!(),
            "&" => todo!(),
            "&=" => todo!(),
            "|" => todo!(),
            "|=" => todo!(),
            "~" => todo!(),
            "&&" => todo!(),
            "||" => todo!(),
            "!" => todo!(),
            "==" => todo!(),
            "!=" => todo!(),
            "<" => todo!(),
            ">" => todo!(),
            "<=" => todo!(),
            ">=" => todo!(),
            "(" => todo!(),
            ")" => todo!(),
            "[" => todo!(),
            "]" => todo!(),
            "{" => todo!(),
            "}" => todo!(),
            other => Self::LiteralToken(Literal::from(other)),
        }
    }
}

fn parse(file: &str) -> Vec<Token> {
    let mut tokens = Vec::new();

    tokens
}

fn main() {
    println!("return, world!");
}
