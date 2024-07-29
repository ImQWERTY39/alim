mod parser;
mod tokeniser;

fn main() {}

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
                Token::Keyword(Keyword::SignedInteger32),
                Token::Literal(Literal::Identifier(Box::from("main"))),
                Token::Bracket(Bracket::RoundOpen),
                Token::Bracket(Bracket::RoundClose),
                Token::Bracket(Bracket::CurlyOpen),
                Token::Keyword(Keyword::SignedInteger8),
                Token::Literal(Literal::Identifier(Box::from("a"))),
                Token::Operator(Operator::Assign),
                Token::Literal(Literal::Integer(1)),
                Token::Symbol(Symbol::End),
                Token::Keyword(Keyword::SignedInteger16),
                Token::Literal(Literal::Identifier(Box::from("b"))),
                Token::Operator(Operator::Assign),
                Token::Literal(Literal::Integer(-3)),
                Token::Symbol(Symbol::End),
                Token::Keyword(Keyword::SignedInteger32),
                Token::Literal(Literal::Identifier(Box::from("c"))),
                Token::Symbol(Symbol::End),
                Token::Keyword(Keyword::SignedInteger64),
                Token::Literal(Literal::Identifier(Box::from("d"))),
                Token::Symbol(Symbol::End),
                Token::Keyword(Keyword::UnsignedInteger8),
                Token::Literal(Literal::Identifier(Box::from("e"))),
                Token::Operator(Operator::Assign),
                Token::Literal(Literal::Integer(1)),
                Token::Symbol(Symbol::End),
                Token::Keyword(Keyword::UnsignedInteger16),
                Token::Literal(Literal::Identifier(Box::from("f"))),
                Token::Symbol(Symbol::End),
                Token::Keyword(Keyword::UnsignedInteger32),
                Token::Literal(Literal::Identifier(Box::from("g"))),
                Token::Symbol(Symbol::End),
                Token::Keyword(Keyword::UnsignedInteger64),
                Token::Literal(Literal::Identifier(Box::from("h"))),
                Token::Symbol(Symbol::End),
                Token::Keyword(Keyword::Float32),
                Token::Literal(Literal::Identifier(Box::from("i"))),
                Token::Operator(Operator::Assign),
                Token::Literal(Literal::Float(7.8)),
                Token::Symbol(Symbol::End),
                Token::Keyword(Keyword::Float64),
                Token::Literal(Literal::Identifier(Box::from("j"))),
                Token::Operator(Operator::Assign),
                Token::Literal(Literal::Float(-4.5)),
                Token::Symbol(Symbol::End),
                Token::Keyword(Keyword::Character),
                Token::Literal(Literal::Identifier(Box::from("k"))),
                Token::Operator(Operator::Assign),
                Token::Literal(Literal::Character('\n')),
                Token::Symbol(Symbol::End),
                Token::Keyword(Keyword::Character),
                Token::Literal(Literal::Identifier(Box::from("l"))),
                Token::Operator(Operator::Assign),
                Token::Literal(Literal::Character('r')),
                Token::Symbol(Symbol::End),
                Token::Keyword(Keyword::Boolean),
                Token::Literal(Literal::Identifier(Box::from("m"))),
                Token::Operator(Operator::Assign),
                Token::Keyword(Keyword::True),
                Token::Symbol(Symbol::End),
                Token::Keyword(Keyword::Boolean),
                Token::Literal(Literal::Identifier(Box::from("n"))),
                Token::Operator(Operator::Assign),
                Token::Keyword(Keyword::False),
                Token::Symbol(Symbol::End),
                Token::Bracket(Bracket::CurlyClose),
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
                Token::Literal(Literal::Identifier(Box::from("a"))),
                Token::Operator(Operator::Assign),
                Token::Literal(Literal::Identifier(Box::from("b"))),
                Token::Literal(Literal::Identifier(Box::from("a"))),
                Token::Operator(Operator::LessThan),
                Token::Literal(Literal::Identifier(Box::from("b"))),
                Token::Literal(Literal::Identifier(Box::from("a"))),
                Token::Operator(Operator::LessThanEqual),
                Token::Literal(Literal::Identifier(Box::from("b"))),
                Token::Literal(Literal::Identifier(Box::from("a"))),
                Token::Operator(Operator::GreaterThan),
                Token::Literal(Literal::Identifier(Box::from("b"))),
                Token::Literal(Literal::Identifier(Box::from("a"))),
                Token::Operator(Operator::GreaterThanEqual),
                Token::Literal(Literal::Identifier(Box::from("b"))),
                Token::Literal(Literal::Identifier(Box::from("a"))),
                Token::Operator(Operator::Equal),
                Token::Literal(Literal::Identifier(Box::from("b"))),
                Token::Literal(Literal::Identifier(Box::from("a"))),
                Token::Operator(Operator::NotEqual),
                Token::Literal(Literal::Identifier(Box::from("b"))),
                Token::Literal(Literal::Identifier(Box::from("a"))),
                Token::Operator(Operator::Add),
                Token::Literal(Literal::Identifier(Box::from("b"))),
                Token::Literal(Literal::Identifier(Box::from("a"))),
                Token::Operator(Operator::Subtract),
                Token::Literal(Literal::Identifier(Box::from("b"))),
                Token::Literal(Literal::Identifier(Box::from("a"))),
                Token::Operator(Operator::AddAssign),
                Token::Literal(Literal::Identifier(Box::from("b"))),
                Token::Literal(Literal::Identifier(Box::from("a"))),
                Token::Operator(Operator::SubAssign),
                Token::Literal(Literal::Identifier(Box::from("b"))),
                Token::Literal(Literal::Identifier(Box::from("a"))),
                Token::Operator(Operator::Multiply),
                Token::Literal(Literal::Identifier(Box::from("b"))),
                Token::Literal(Literal::Identifier(Box::from("a"))),
                Token::Operator(Operator::MulAssign),
                Token::Literal(Literal::Identifier(Box::from("b"))),
                Token::Literal(Literal::Identifier(Box::from("a"))),
                Token::Operator(Operator::Divide),
                Token::Literal(Literal::Identifier(Box::from("b"))),
                Token::Literal(Literal::Identifier(Box::from("a"))),
                Token::Operator(Operator::DivAssign),
                Token::Literal(Literal::Identifier(Box::from("b"))),
                Token::Literal(Literal::Identifier(Box::from("a"))),
                Token::Operator(Operator::Modulus),
                Token::Literal(Literal::Identifier(Box::from("b"))),
                Token::Literal(Literal::Identifier(Box::from("a"))),
                Token::Operator(Operator::ModAssign),
                Token::Literal(Literal::Identifier(Box::from("b"))),
                Token::Literal(Literal::Identifier(Box::from("a"))),
                Token::Operator(Operator::BitAnd),
                Token::Literal(Literal::Identifier(Box::from("b"))),
                Token::Literal(Literal::Identifier(Box::from("a"))),
                Token::Operator(Operator::BitAndAssign),
                Token::Literal(Literal::Identifier(Box::from("b"))),
                Token::Literal(Literal::Identifier(Box::from("a"))),
                Token::Operator(Operator::BitOr),
                Token::Literal(Literal::Identifier(Box::from("b"))),
                Token::Literal(Literal::Identifier(Box::from("a"))),
                Token::Operator(Operator::BitOrAssign),
                Token::Literal(Literal::Identifier(Box::from("b"))),
                Token::Literal(Literal::Identifier(Box::from("a"))),
                Token::Operator(Operator::And),
                Token::Literal(Literal::Identifier(Box::from("b"))),
                Token::Literal(Literal::Identifier(Box::from("a"))),
                Token::Operator(Operator::Or),
                Token::Literal(Literal::Identifier(Box::from("b"))),
                Token::Operator(Operator::Not),
                Token::Literal(Literal::Identifier(Box::from("a"))),
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
                Token::Bracket(Bracket::RoundOpen),
                Token::Bracket(Bracket::RoundClose),
                Token::Bracket(Bracket::SquareOpen),
                Token::Bracket(Bracket::SquareClose),
                Token::Bracket(Bracket::CurlyOpen),
                Token::Bracket(Bracket::CurlyClose),
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
                Token::Keyword(Keyword::Import),
                Token::Literal(Literal::Identifier(Box::from("math"))),
                Token::Symbol(Symbol::End),
                Token::Keyword(Keyword::Struct),
                Token::Literal(Literal::Identifier(Box::from("Person"))),
                Token::Bracket(Bracket::CurlyOpen),
                Token::Keyword(Keyword::Character),
                Token::Bracket(Bracket::SquareOpen),
                Token::Literal(Literal::Integer(40)),
                Token::Bracket(Bracket::SquareClose),
                Token::Literal(Literal::Identifier(Box::from("name"))),
                Token::Symbol(Symbol::End),
                Token::Keyword(Keyword::UnsignedInteger8),
                Token::Literal(Literal::Identifier(Box::from("age"))),
                Token::Symbol(Symbol::End),
                Token::Bracket(Bracket::CurlyClose),
                Token::Keyword(Keyword::SignedInteger32),
                Token::Literal(Literal::Identifier(Box::from("main"))),
                Token::Bracket(Bracket::RoundOpen),
                Token::Bracket(Bracket::RoundClose),
                Token::Bracket(Bracket::CurlyOpen),
                Token::Literal(Literal::Identifier(Box::from("Person"))),
                Token::Literal(Literal::Identifier(Box::from("p"))),
                Token::Operator(Operator::Assign),
                Token::Literal(Literal::Identifier(Box::from("Person"))),
                Token::Bracket(Bracket::CurlyOpen),
                Token::Literal(Literal::Identifier(Box::from("name"))),
                Token::Operator(Operator::Assign),
                Token::Literal(Literal::String(Box::from("abc"))),
                Token::Symbol(Symbol::Seperator),
                Token::Literal(Literal::Identifier(Box::from("age"))),
                Token::Operator(Operator::Assign),
                Token::Literal(Literal::Integer(10)),
                Token::Symbol(Symbol::Seperator),
                Token::Bracket(Bracket::CurlyClose),
                Token::Symbol(Symbol::End),
                Token::Bracket(Bracket::CurlyClose),
            ]
        );
    }
}
