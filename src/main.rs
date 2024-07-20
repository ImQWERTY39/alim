use std::fs::read_to_string;

#[derive(Debug)]
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
    Float32,
    Float64,
    Character,
    Boolean,

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

#[derive(Debug)]
enum Operator {
    Assign,

    Add,
    AddAssign,

    Subtract,
    SubAssign,

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

#[derive(Debug)]
enum Bracket {
    RoundOpen,
    RoundClose,
    SquareOpen,
    SquareClose,
    CurlyOpen,
    CurlyClose,
}

#[derive(Debug)]
enum Literal {
    Integer(i128),
    Float(f64),
    Character(char),
    Array(Vec<Literal>),
    Identifier(Box<str>),
}

impl TryFrom<&str> for Literal {
    type Error = ();

    fn try_from(value: &str) -> Result<Self, ()> {
        if let Ok(i) = value.parse::<i128>() {
            Ok(Self::Integer(i))
        } else if let Ok(i) = value.parse::<f64>() {
            Ok(Self::Float(i))
        } else if value.starts_with('\'') && value.ends_with('\'') {
            if value.len() == 3 {
                Ok(Self::Character(value.chars().nth(1).unwrap()))
            } else {
                if !value.starts_with("'\\") {
                    return Err(());
                }

                match value.chars().nth(2).unwrap() {
                    'n' => Ok(Self::Character('\n')),
                    'r' => Ok(Self::Character('\r')),
                    't' => Ok(Self::Character('\t')),
                    '\\' => Ok(Self::Character('\\')),
                    '0' => Ok(Self::Character('\0')),
                    '\'' => Ok(Self::Character('\'')),
                    '"' => Ok(Self::Character('\"')),
                    _ => Err(()),
                }
            }
        } else if value.starts_with('[') && value.ends_with(']') {
            todo!()
            // parse_array(value[1..value.len() - 1])
        } else {
            let mut valid_first_character: Vec<char> = ('A'..='Z').chain('a'..='z').collect();
            valid_first_character.push('_');

            if !valid_first_character.contains(&value.chars().next().ok_or(())?) {
                return Err(());
            }

            valid_first_character.extend('0'..='9');

            if !value.chars().all(|x| valid_first_character.contains(&x)) {
                return Err(());
            }

            Ok(Self::Identifier(value.into()))
        }
    }
}

#[derive(Debug)]
enum Symbol {
    End,
    Get,
    Seperator,
}

#[derive(Debug)]
enum Token {
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

fn tokeniser(file: &str) -> Vec<Token> {
    let mut iterator = file.chars().peekable();

    let mut tokens = Vec::new();
    let mut current_token = String::new();

    let mut in_char = false;
    let mut prev_chr = '\u{0}';

    while let Some(i) = iterator.next() {
        if i.is_whitespace() && !in_char {
            if !current_token.is_empty() {
                tokens.push(current_token.as_str().try_into().unwrap());
                current_token.clear();
            }

            continue;
        }

        current_token.push(i);

        match i {
            '\'' => {
                if in_char && prev_chr != '\\' {
                    tokens.push(current_token.as_str().try_into().unwrap());

                    current_token.clear();
                    in_char = false;
                } else {
                    in_char = true;
                }

                continue;
            }

            '-' => {
                if let (Some(Token::LiteralToken(_)), Some(i)) = (tokens.last(), iterator.peek()) {
                    if i.is_ascii_alphanumeric() {
                        tokens.push(current_token.as_str().try_into().unwrap());
                        current_token.clear();
                    }
                }
            }

            '.' => {
                if let (Some(i), Some(j)) = (tokens.last(), iterator.peek()) {
                    let valid = match i {
                        Token::BracketToken(Bracket::RoundClose) => true,
                        Token::LiteralToken(Literal::Identifier(_)) => true,
                        _ => false,
                    };

                    if valid && j.is_ascii_alphabetic() {
                        tokens.push(current_token.as_str().try_into().unwrap());
                        current_token.clear();
                    }
                }
            }

            _ => (),
        }

        if !in_char {
            if Token::try_from(current_token.as_str()).is_err() {
                println!("{current_token:?}");
                current_token.pop();

                tokens.push(current_token.as_str().try_into().unwrap());
                current_token.clear();

                if i != ' ' {
                    current_token.push(i);
                }
            }
        }
        prev_chr = i;
    }

    if !current_token.is_empty() {
        tokens.push(current_token.as_str().try_into().unwrap());
    }

    tokens
}

fn main() {
    let buffer = read_to_string("./main.alim").unwrap();
    let tokens = tokeniser(&buffer);
    println!("{:#?}", tokens);
}
