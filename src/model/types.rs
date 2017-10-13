

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum PrimitiveVarType {
    StringVar,
    Number,
    Bool
    // Expr,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum VarType {
    ArrayVar(Option<Box<VarType>>),
    ObjectVar,
    Primitive(PrimitiveVarType),
}

impl VarType {
    #[allow(dead_code)]
    pub fn string() -> VarType { VarType::Primitive(PrimitiveVarType::StringVar) }
    pub fn number() -> VarType { VarType::Primitive(PrimitiveVarType::Number) }
    pub fn boolean() -> VarType { VarType::Primitive(PrimitiveVarType::Bool) }

    #[allow(dead_code)]
    pub fn string_array() -> VarType { VarType::ArrayVar(Some(Box::new(VarType::Primitive(PrimitiveVarType::StringVar)))) }

    pub fn array_of(ty: VarType) -> VarType { VarType::ArrayVar(Some(Box::new(ty))) }
}
