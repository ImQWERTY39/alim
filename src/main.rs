use std::fs::read_to_string;
mod tokeniser;

fn main() {
    let buffer = read_to_string("./tests/test4.alim").unwrap();
    let tokens = tokeniser::tokeniser(&buffer);
    println!("{tokens:#?}")
}

#[cfg(test)]
mod test {
    use crate::tokeniser::{
        brackets::Bracket, keywords::Keyword, literals::Literal, operators::Operator,
        symbols::Symbol, tokens::Token,
    };

    use super::*;
    use std::fs::read_to_string;

    #[test]
    fn basic() {
        let buffer = read_to_string("./tests/test1.alim").unwrap();
        let tokens = tokeniser::tokeniser(&buffer);

        assert_eq!(
            tokens,
            vec![
                Token::KeywordToken(Keyword::SignedInteger32),
                Token::LiteralToken(Literal::Identifier(Box::from("main"))),
                Token::BracketToken(Bracket::RoundOpen),
                Token::BracketToken(Bracket::RoundClose),
                Token::BracketToken(Bracket::CurlyOpen),
                Token::KeywordToken(Keyword::SignedInteger8),
                Token::LiteralToken(Literal::Identifier(Box::from("a"))),
                Token::OperatorToken(Operator::Assign),
                Token::LiteralToken(Literal::Integer(1)),
                Token::SymbolToken(Symbol::End),
                Token::KeywordToken(Keyword::SignedInteger16),
                Token::LiteralToken(Literal::Identifier(Box::from("b"))),
                Token::OperatorToken(Operator::Assign),
                Token::LiteralToken(Literal::Integer(-3)),
                Token::SymbolToken(Symbol::End),
                Token::KeywordToken(Keyword::SignedInteger32),
                Token::LiteralToken(Literal::Identifier(Box::from("c"))),
                Token::SymbolToken(Symbol::End),
                Token::KeywordToken(Keyword::SignedInteger64),
                Token::LiteralToken(Literal::Identifier(Box::from("d"))),
                Token::SymbolToken(Symbol::End),
                Token::KeywordToken(Keyword::UnsignedInteger8),
                Token::LiteralToken(Literal::Identifier(Box::from("e"))),
                Token::OperatorToken(Operator::Assign),
                Token::LiteralToken(Literal::Integer(1)),
                Token::SymbolToken(Symbol::End),
                Token::KeywordToken(Keyword::UnsignedInteger16),
                Token::LiteralToken(Literal::Identifier(Box::from("f"))),
                Token::SymbolToken(Symbol::End),
                Token::KeywordToken(Keyword::UnsignedInteger32),
                Token::LiteralToken(Literal::Identifier(Box::from("g"))),
                Token::SymbolToken(Symbol::End),
                Token::KeywordToken(Keyword::UnsignedInteger64),
                Token::LiteralToken(Literal::Identifier(Box::from("h"))),
                Token::SymbolToken(Symbol::End),
                Token::KeywordToken(Keyword::Float32),
                Token::LiteralToken(Literal::Identifier(Box::from("i"))),
                Token::OperatorToken(Operator::Assign),
                Token::LiteralToken(Literal::Float(7.8)),
                Token::SymbolToken(Symbol::End),
                Token::KeywordToken(Keyword::Float64),
                Token::LiteralToken(Literal::Identifier(Box::from("j"))),
                Token::OperatorToken(Operator::Assign),
                Token::LiteralToken(Literal::Float(-4.5)),
                Token::SymbolToken(Symbol::End),
                Token::KeywordToken(Keyword::Character),
                Token::LiteralToken(Literal::Identifier(Box::from("k"))),
                Token::OperatorToken(Operator::Assign),
                Token::LiteralToken(Literal::Character('\n')),
                Token::SymbolToken(Symbol::End),
                Token::KeywordToken(Keyword::Character),
                Token::LiteralToken(Literal::Identifier(Box::from("l"))),
                Token::OperatorToken(Operator::Assign),
                Token::LiteralToken(Literal::Character('r')),
                Token::SymbolToken(Symbol::End),
                Token::KeywordToken(Keyword::Boolean),
                Token::LiteralToken(Literal::Identifier(Box::from("m"))),
                Token::OperatorToken(Operator::Assign),
                Token::KeywordToken(Keyword::True),
                Token::SymbolToken(Symbol::End),
                Token::KeywordToken(Keyword::Boolean),
                Token::LiteralToken(Literal::Identifier(Box::from("n"))),
                Token::OperatorToken(Operator::Assign),
                Token::KeywordToken(Keyword::False),
                Token::SymbolToken(Symbol::End),
                Token::BracketToken(Bracket::CurlyClose),
            ]
        );
    }

    #[test]
    fn operators() {
        let buffer = read_to_string("./tests/test2.alim").unwrap();
        let tokens = tokeniser::tokeniser(&buffer);

        assert_eq!(
            tokens,
            vec![
                Token::LiteralToken(Literal::Identifier(Box::from("a"))),
                Token::OperatorToken(Operator::Assign),
                Token::LiteralToken(Literal::Identifier(Box::from("b"))),
                Token::LiteralToken(Literal::Identifier(Box::from("a"))),
                Token::OperatorToken(Operator::LessThan),
                Token::LiteralToken(Literal::Identifier(Box::from("b"))),
                Token::LiteralToken(Literal::Identifier(Box::from("a"))),
                Token::OperatorToken(Operator::LessThanEqual),
                Token::LiteralToken(Literal::Identifier(Box::from("b"))),
                Token::LiteralToken(Literal::Identifier(Box::from("a"))),
                Token::OperatorToken(Operator::GreaterThan),
                Token::LiteralToken(Literal::Identifier(Box::from("b"))),
                Token::LiteralToken(Literal::Identifier(Box::from("a"))),
                Token::OperatorToken(Operator::GreaterThanEqual),
                Token::LiteralToken(Literal::Identifier(Box::from("b"))),
                Token::LiteralToken(Literal::Identifier(Box::from("a"))),
                Token::OperatorToken(Operator::Equal),
                Token::LiteralToken(Literal::Identifier(Box::from("b"))),
                Token::LiteralToken(Literal::Identifier(Box::from("a"))),
                Token::OperatorToken(Operator::NotEqual),
                Token::LiteralToken(Literal::Identifier(Box::from("b"))),
                Token::LiteralToken(Literal::Identifier(Box::from("a"))),
                Token::OperatorToken(Operator::Add),
                Token::LiteralToken(Literal::Identifier(Box::from("b"))),
                Token::LiteralToken(Literal::Identifier(Box::from("a"))),
                Token::OperatorToken(Operator::Subtract),
                Token::LiteralToken(Literal::Identifier(Box::from("b"))),
                Token::LiteralToken(Literal::Identifier(Box::from("a"))),
                Token::OperatorToken(Operator::AddAssign),
                Token::LiteralToken(Literal::Identifier(Box::from("b"))),
                Token::LiteralToken(Literal::Identifier(Box::from("a"))),
                Token::OperatorToken(Operator::SubAssign),
                Token::LiteralToken(Literal::Identifier(Box::from("b"))),
                Token::LiteralToken(Literal::Identifier(Box::from("a"))),
                Token::OperatorToken(Operator::Multiply),
                Token::LiteralToken(Literal::Identifier(Box::from("b"))),
                Token::LiteralToken(Literal::Identifier(Box::from("a"))),
                Token::OperatorToken(Operator::MulAssign),
                Token::LiteralToken(Literal::Identifier(Box::from("b"))),
                Token::LiteralToken(Literal::Identifier(Box::from("a"))),
                Token::OperatorToken(Operator::Divide),
                Token::LiteralToken(Literal::Identifier(Box::from("b"))),
                Token::LiteralToken(Literal::Identifier(Box::from("a"))),
                Token::OperatorToken(Operator::DivAssign),
                Token::LiteralToken(Literal::Identifier(Box::from("b"))),
                Token::LiteralToken(Literal::Identifier(Box::from("a"))),
                Token::OperatorToken(Operator::Modulus),
                Token::LiteralToken(Literal::Identifier(Box::from("b"))),
                Token::LiteralToken(Literal::Identifier(Box::from("a"))),
                Token::OperatorToken(Operator::ModAssign),
                Token::LiteralToken(Literal::Identifier(Box::from("b"))),
                Token::LiteralToken(Literal::Identifier(Box::from("a"))),
                Token::OperatorToken(Operator::BitAnd),
                Token::LiteralToken(Literal::Identifier(Box::from("b"))),
                Token::LiteralToken(Literal::Identifier(Box::from("a"))),
                Token::OperatorToken(Operator::BitAndAssign),
                Token::LiteralToken(Literal::Identifier(Box::from("b"))),
                Token::LiteralToken(Literal::Identifier(Box::from("a"))),
                Token::OperatorToken(Operator::BitOr),
                Token::LiteralToken(Literal::Identifier(Box::from("b"))),
                Token::LiteralToken(Literal::Identifier(Box::from("a"))),
                Token::OperatorToken(Operator::BitOrAssign),
                Token::LiteralToken(Literal::Identifier(Box::from("b"))),
                Token::LiteralToken(Literal::Identifier(Box::from("a"))),
                Token::OperatorToken(Operator::And),
                Token::LiteralToken(Literal::Identifier(Box::from("b"))),
                Token::LiteralToken(Literal::Identifier(Box::from("a"))),
                Token::OperatorToken(Operator::Or),
                Token::LiteralToken(Literal::Identifier(Box::from("b"))),
                Token::OperatorToken(Operator::Not),
                Token::LiteralToken(Literal::Identifier(Box::from("a"))),
            ]
        );
    }

    #[test]
    fn brackets() {
        let buffer = read_to_string("./tests/test3.alim").unwrap();
        let tokens = tokeniser::tokeniser(&buffer);

        assert_eq!(
            tokens,
            vec![
                Token::BracketToken(Bracket::RoundOpen),
                Token::BracketToken(Bracket::RoundClose),
                Token::BracketToken(Bracket::SquareOpen),
                Token::BracketToken(Bracket::SquareClose),
                Token::BracketToken(Bracket::CurlyOpen),
                Token::BracketToken(Bracket::CurlyClose),
            ]
        );
    }

    #[test]
    fn struct_and_string() {
        let buffer = read_to_string("./tests/test4.alim").unwrap();
        let tokens = tokeniser::tokeniser(&buffer);

        assert_eq!(
            tokens,
            vec![
                Token::KeywordToken(Keyword::Import),
                Token::LiteralToken(Literal::Identifier(Box::from("math"))),
                Token::SymbolToken(Symbol::End),
                Token::KeywordToken(Keyword::Struct),
                Token::LiteralToken(Literal::Identifier(Box::from("Person"))),
                Token::BracketToken(Bracket::CurlyOpen),
                Token::KeywordToken(Keyword::Character),
                Token::BracketToken(Bracket::SquareOpen),
                Token::LiteralToken(Literal::Integer(40)),
                Token::BracketToken(Bracket::SquareClose),
                Token::LiteralToken(Literal::Identifier(Box::from("name"))),
                Token::SymbolToken(Symbol::End),
                Token::KeywordToken(Keyword::UnsignedInteger8),
                Token::LiteralToken(Literal::Identifier(Box::from("age"))),
                Token::SymbolToken(Symbol::End),
                Token::BracketToken(Bracket::CurlyClose),
                Token::KeywordToken(Keyword::SignedInteger32),
                Token::LiteralToken(Literal::Identifier(Box::from("main"))),
                Token::BracketToken(Bracket::RoundOpen),
                Token::BracketToken(Bracket::RoundClose),
                Token::BracketToken(Bracket::CurlyOpen),
                Token::LiteralToken(Literal::Identifier(Box::from("Person"))),
                Token::LiteralToken(Literal::Identifier(Box::from("p"))),
                Token::OperatorToken(Operator::Assign),
                Token::LiteralToken(Literal::Identifier(Box::from("Person"))),
                Token::BracketToken(Bracket::CurlyOpen),
                Token::LiteralToken(Literal::Identifier(Box::from("name"))),
                Token::OperatorToken(Operator::Assign),
                Token::LiteralToken(Literal::Array(vec![
                    Literal::Character('a'),
                    Literal::Character('b'),
                    Literal::Character('c'),
                ])),
                Token::SymbolToken(Symbol::Seperator),
                Token::LiteralToken(Literal::Identifier(Box::from("age"))),
                Token::OperatorToken(Operator::Assign),
                Token::LiteralToken(Literal::Integer(10)),
                Token::SymbolToken(Symbol::Seperator),
                Token::BracketToken(Bracket::CurlyClose),
                Token::SymbolToken(Symbol::End),
                Token::BracketToken(Bracket::CurlyClose),
            ]
        );
    }
}
