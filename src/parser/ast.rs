#![allow(dead_code)]

#[derive(Debug)]
pub enum NodeType {
    ComponentDefinitionNode(ComponentDefinitionType),
    ElementNode(ElementType),
    ExpressionNode(ExprType)
}

/// Complex expression
#[derive(Debug)]
pub enum ExprType {
    LiteralString(String),
    VariableReference(String)
}

/// Simple expression (parameter value)
#[derive(Debug)]
pub enum ExprValue {
    LiteralString(String),
    VariableReference(String)    
}

#[derive(Debug)]
pub struct ComponentDefinitionType {
    pub name: String,
    pub inputs: Option<Vec<String>>,
    pub children: Option<Vec<NodeType>>
}

#[derive(Debug)]
pub struct ElementType {
    pub element_ty: String,
    pub attrs: Option<Vec<(String, ExprValue)>>,
    pub children: Option<Vec<NodeType>>
}
