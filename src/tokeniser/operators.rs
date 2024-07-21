#[derive(Debug, PartialEq, Eq)]
pub enum Operator {
    Assign,

    Add,
    AddAssign,

    Subtract,
    SubAssign,

    Multiply,
    MulAssign,

    Divide,
    DivAssign,

    Modulus,
    ModAssign,

    BitAnd,
    BitAndAssign,

    BitOr,
    BitOrAssign,

    And,
    Or,
    Not,

    Equal,
    NotEqual,
    LessThan,
    LessThanEqual,
    GreaterThan,
    GreaterThanEqual,
}
