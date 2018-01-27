#[allow(dead_code)]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum UnaryOpType {
    Negate
}

#[allow(dead_code)]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum BinaryOpType {
    Add,
    Sub,
    Mul,
    Div,
    EqualTo,
    NotEqualTo,
    LessThan,
    GreaterThan,
    LessThanOrEqualTo,
    GreaterThanOrEqualTo
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum ApplyOpType {
    JoinString(Option<String>),
    // Sum
}