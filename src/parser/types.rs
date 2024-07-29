use std::{collections::HashMap, rc::Rc};

pub type Identifier = Rc<str>;
pub type VarsNameType = Rc<[(Identifier, Type)]>;
pub type VarsNameValue = HashMap<Identifier, Value>;

#[derive(Clone)]
pub enum Type {
    SignedInteger8,
    SignedInteger16,
    SignedInteger32,
    SignedInteger64,
    UnsignedInteger8,
    UnsignedInteger16,
    UnsignedInteger32,
    UnsignedInteger64,
    Character,
    Boolean,
    Array(Box<Type>, usize),
    Struct(Identifier),
}

#[derive(Clone)]
pub enum Value {
    Null(Type),
    I8(i8),
    I16(i16),
    I32(i32),
    I64(i64),
    U8(u8),
    U16(u16),
    U32(u32),
    U64(u64),
    Char(char),
    Bool(bool),
    String(Box<str>),
    Array(Type, Vec<Value>),
    Struct(Identifier, VarsNameValue),
}

impl Value {
    pub fn type_of(&self) -> Type {
        match self {
            Value::Null(i) => i.clone(),
            Value::I8(_) => Type::SignedInteger8,
            Value::I16(_) => Type::SignedInteger16,
            Value::I32(_) => Type::SignedInteger32,
            Value::I64(_) => Type::SignedInteger64,
            Value::U8(_) => Type::UnsignedInteger8,
            Value::U16(_) => Type::UnsignedInteger16,
            Value::U32(_) => Type::UnsignedInteger32,
            Value::U64(_) => Type::UnsignedInteger64,
            Value::Char(_) => Type::Character,
            Value::Bool(_) => Type::Boolean,
            Value::String(i) => Type::Array(Box::new(Type::Character), i.len()),
            Value::Array(t, arr) => Type::Array(Box::new(t.clone()), arr.len()),
            Value::Struct(name, _) => Type::Struct(name.clone()),
        }
    }
}
