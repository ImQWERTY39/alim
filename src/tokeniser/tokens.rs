use super::{
    brackets::Bracket, keywords::Keyword, literals::Literal, operators::Operator, symbols::Symbol,
};

#[derive(Debug, PartialEq)]
pub enum Token {
    KeywordToken(Keyword),
    OperatorToken(Operator),
    BracketToken(Bracket),
    LiteralToken(Literal),
    SymbolToken(Symbol),
}

impl TryFrom<&str> for Token {
    type Error = ();

    fn try_from(value: &str) -> Result<Self, ()> {
        match value {
            "import" => Ok(Self::KeywordToken(Keyword::Import)),
            "struct" => Ok(Self::KeywordToken(Keyword::Struct)),
            "return" => Ok(Self::KeywordToken(Keyword::Return)),
            "i8" => Ok(Self::KeywordToken(Keyword::SignedInteger8)),
            "i16" => Ok(Self::KeywordToken(Keyword::SignedInteger16)),
            "i32" => Ok(Self::KeywordToken(Keyword::SignedInteger32)),
            "i64" => Ok(Self::KeywordToken(Keyword::SignedInteger64)),
            "u8" => Ok(Self::KeywordToken(Keyword::UnsignedInteger8)),
            "u16" => Ok(Self::KeywordToken(Keyword::UnsignedInteger16)),
            "u32" => Ok(Self::KeywordToken(Keyword::UnsignedInteger32)),
            "u64" => Ok(Self::KeywordToken(Keyword::UnsignedInteger64)),
            "f32" => Ok(Self::KeywordToken(Keyword::Float32)),
            "f64" => Ok(Self::KeywordToken(Keyword::Float64)),
            "char" => Ok(Self::KeywordToken(Keyword::Character)),
            "bool" => Ok(Self::KeywordToken(Keyword::Boolean)),
            "if" => Ok(Self::KeywordToken(Keyword::If)),
            "else" => Ok(Self::KeywordToken(Keyword::Else)),
            "while" => Ok(Self::KeywordToken(Keyword::While)),
            "for" => Ok(Self::KeywordToken(Keyword::For)),
            "in" => Ok(Self::KeywordToken(Keyword::In)),
            "break" => Ok(Self::KeywordToken(Keyword::Break)),
            "continue" => Ok(Self::KeywordToken(Keyword::Continue)),
            "true" => Ok(Self::KeywordToken(Keyword::True)),
            "false" => Ok(Self::KeywordToken(Keyword::False)),
            "=" => Ok(Self::OperatorToken(Operator::Assign)),
            "+" => Ok(Self::OperatorToken(Operator::Add)),
            "+=" => Ok(Self::OperatorToken(Operator::AddAssign)),
            "-" => Ok(Self::OperatorToken(Operator::Subtract)),
            "-=" => Ok(Self::OperatorToken(Operator::SubAssign)),
            "*" => Ok(Self::OperatorToken(Operator::Multiply)),
            "*=" => Ok(Self::OperatorToken(Operator::MulAssign)),
            "/" => Ok(Self::OperatorToken(Operator::Divide)),
            "/=" => Ok(Self::OperatorToken(Operator::DivAssign)),
            "%" => Ok(Self::OperatorToken(Operator::Modulus)),
            "%=" => Ok(Self::OperatorToken(Operator::ModAssign)),
            "&" => Ok(Self::OperatorToken(Operator::BitAnd)),
            "&=" => Ok(Self::OperatorToken(Operator::BitAndAssign)),
            "|" => Ok(Self::OperatorToken(Operator::BitOr)),
            "|=" => Ok(Self::OperatorToken(Operator::BitOrAssign)),
            "&&" => Ok(Self::OperatorToken(Operator::And)),
            "||" => Ok(Self::OperatorToken(Operator::Or)),
            "!" => Ok(Self::OperatorToken(Operator::Not)),
            "==" => Ok(Self::OperatorToken(Operator::Equal)),
            "!=" => Ok(Self::OperatorToken(Operator::NotEqual)),
            "<" => Ok(Self::OperatorToken(Operator::LessThan)),
            ">" => Ok(Self::OperatorToken(Operator::GreaterThan)),
            "<=" => Ok(Self::OperatorToken(Operator::LessThanEqual)),
            ">=" => Ok(Self::OperatorToken(Operator::GreaterThanEqual)),
            "(" => Ok(Self::BracketToken(Bracket::RoundOpen)),
            ")" => Ok(Self::BracketToken(Bracket::RoundClose)),
            "[" => Ok(Self::BracketToken(Bracket::SquareOpen)),
            "]" => Ok(Self::BracketToken(Bracket::SquareClose)),
            "{" => Ok(Self::BracketToken(Bracket::CurlyOpen)),
            "}" => Ok(Self::BracketToken(Bracket::CurlyClose)),
            ";" => Ok(Self::SymbolToken(Symbol::End)),
            "," => Ok(Self::SymbolToken(Symbol::Seperator)),
            "." => Ok(Self::SymbolToken(Symbol::Get)),
            other => Ok(Self::LiteralToken(Literal::try_from(other)?)),
        }
    }
}
