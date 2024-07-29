use crate::tokeniser::tokens::Token;

mod types;
pub use types::{Identifier, Type, Value, VarsNameType, VarsNameValue};

mod storage;

pub struct Scope {
    external: bool,
    defined_globvars_structs: VarsNameType,
    declared_vars_structs: VarsNameValue,
}

pub struct Block(Vec<Instruction>);

pub enum Evaluation {
    Result(Value),
    Get(Value, Identifier), // get from struct/module
    Index(Value, usize),    // get from array/string
    Not(Box<Evaluation>),
    Negate(Box<Evaluation>),
    Add(Box<Evaluation>, Box<Evaluation>),
    Subtract(Box<Evaluation>, Box<Evaluation>),
    Multiply(Box<Evaluation>, Box<Evaluation>),
    Divide(Box<Evaluation>, Box<Evaluation>),
    Modulus(Box<Evaluation>, Box<Evaluation>),
    And(Box<Evaluation>, Box<Evaluation>),
    Or(Box<Evaluation>, Box<Evaluation>),
}

pub struct Function {
    name: Identifier,
    params: VarsNameType,
    instruction: Block,
}

pub enum Instruction {
    Define(Identifier, Type, Option<Evaluation>),
}

pub struct Modules {
    modvars: Scope,
    modfns: Vec<Function>,
}

pub fn parse(tokens: Vec<Token>) -> (Vec<Modules>, Scope, Vec<Function>) {
    todo!()
}
