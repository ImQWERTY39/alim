use super::{Identifier, Type, Value};

pub enum Instruction {
    // Define(Identifier, Type, Option<Box<dyn Evaluation>>),
}

pub struct CodeBlock(Vec<Instruction>);
