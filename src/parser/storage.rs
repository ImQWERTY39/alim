use super::{Identifier, Type, VarsNameValue};

// (name, (field name, type of field))
pub type StructDefinition = Box<[(Identifier, (Identifier, Type))]>;

pub struct Scope {
    is_external_scope: bool,
    struct_definition: StructDefinition,
    vars: VarsNameValue,
}
