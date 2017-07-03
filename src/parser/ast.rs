#![allow(dead_code)]

use parser::store::DefaultScopeNodeType;
use parser::loc::Loc;

#[derive(Debug)]
pub struct Template {
    pub children: Vec<Loc<NodeType, (usize, usize)>>
}

#[derive(Debug)]
pub enum NodeType {
    UseStmtNode(UseStmtType),
    ComponentDefinitionNode(ComponentDefinitionType),
    StoreNode(Vec<DefaultScopeNodeType>),
    ContentNode(ContentNodeType)
}

#[derive(Debug)]
pub enum ContentNodeType {
    ElementNode(ElementType),
    ExpressionValueNode(ExprValue),
}

/// Operators
#[derive(Debug, Clone, Copy)]
pub enum ExprOp {
    Add,
    Sub,
    Mul,
    Div
}

/*
/// Complex expression
#[derive(Debug)]
pub enum ExprType {
    LiteralNumber(i32),
    LiteralString(String),
    VariableReference(String),
    Expr(ExprOp, ExprType, ExprType)
}
*/

/// Simple expression (parameter value)
#[derive(Debug, Clone)]
pub enum ExprValue {
    LiteralNumber(i32),
    LiteralString(String),
    VariableReference(String),
    DefaultVariableReference,
    Expr(ExprOp, Box<ExprValue>, Box<ExprValue>)
}

#[derive(Debug)]
pub struct UseStmtType {
    pub package: String
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
    pub element_key: Option<String>,
    pub attrs: Option<Vec<(String, ExprValue)>>,
    pub children: Option<Vec<ContentNodeType>>
}
