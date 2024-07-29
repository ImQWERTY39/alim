use crate::tokeniser::tokens::Token;

mod types;
pub use types::{Identifier, Type, Value, VarsNameType, VarsNameValue};

mod storage;
pub use storage::{Function, Modules, Scope};

mod instructions;
pub use instructions::CodeBlock;

mod evalable;

pub fn parse(tokens: Vec<Token>) -> (Vec<Modules>, Scope, Vec<Function>) {
    todo!()
}
