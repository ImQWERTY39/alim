#[derive(Debug, PartialEq)]
pub enum Literal {
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
