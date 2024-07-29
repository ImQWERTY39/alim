use super::{CodeBlock, Identifier, Type, VarsNameType, VarsNameValue};

// (name, (field name, type of field))
pub type StructDefinition = Box<[(Identifier, (Identifier, Type))]>;

pub struct Scope {
    is_external_scope: bool,
    struct_definition: StructDefinition,
    vars: VarsNameValue,
}

pub struct Modules {
    modvars: Scope,
    modfns: Vec<Function>,
}

pub struct Function {
    name: Identifier,
    params: VarsNameType,
    instruction: CodeBlock,
}
