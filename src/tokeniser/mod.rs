pub mod brackets;
pub mod keywords;
pub mod literals;
pub mod operators;
pub mod symbols;
pub mod tokens;

use brackets::Bracket;
use literals::Literal;
use tokens::Token;

pub fn tokeniser(file: &str) -> Vec<Token> {
    let mut iterator = file.chars().peekable();

    let mut tokens = Vec::new();
    let mut current_token = String::new();

    let mut in_char = false;
    let mut in_str = false;
    let mut prev_chr = '\0';

    // let mut in_line_comment = false;
    // let mut in_mulline_comment = false;

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

            '"' => {
                if in_str && prev_chr != '\\' {
                    tokens.push(current_token.as_str().try_into().unwrap());

                    current_token.clear();
                    in_str = false;
                } else {
                    in_str = true;
                }

                continue;
            }

            '-' => {
                if let (Some(Token::LiteralToken(_)), Some(i)) = (tokens.last(), iterator.peek()) {
                    if i.is_ascii_alphanumeric() {
                        tokens.push(current_token.as_str().try_into().unwrap());
                        current_token.clear();
                        continue;
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
                        continue;
                    }
                }
            }

            _ => (),
        }

        if in_char || in_str {
            continue;
        }

        if Token::try_from(current_token.as_str()).is_err() {
            current_token.pop();

            tokens.push(current_token.as_str().try_into().unwrap());
            current_token.clear();

            if i != ' ' {
                current_token.push(i);
            }
        }

        prev_chr = i;
    }

    if !current_token.is_empty() {
        tokens.push(current_token.as_str().try_into().unwrap());
    }

    tokens
}
