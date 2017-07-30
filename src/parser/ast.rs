#![allow(dead_code)]

use parser::store::DefaultScopeNodeType;
use parser::loc::Loc;

#[derive(Debug, Default)]
pub struct Template {
    pub children: Vec<Loc<NodeType, (usize, usize)>>
}

#[derive(Debug, Clone)]
pub enum NodeType {
    UseStmtNode(UseStmtType),
    ComponentDefinitionNode(ComponentDefinitionType),
    StoreNode(Vec<DefaultScopeNodeType>),
    ContentNode(ContentNodeType)
}

#[derive(Debug, Clone)]
pub enum ContentNodeType {
    ElementNode(ElementType),
    ExpressionValueNode(ExprValue),
    ForNode(Option<String>, ExprValue, Option<Vec<ContentNodeType>>)
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

#[derive(Debug, Clone)]
pub enum PrimitiveVarType {
    StringVar,
    Number,
    Expr,
}

#[derive(Debug, Clone)]
pub enum VarType {
    ArrayVar(Option<Box<VarType>>),
    Primitive(PrimitiveVarType),
}

#[derive(Debug, Clone)]
pub enum SymbolReferenceType {
    ReducerKeyReference(String),
    ParameterReference(String),
    LocalVarReference(String),
    ActionStateReference(Option<VarType>),
    LoopVarReference(String)
}
pub type SymbolRefType = Option<SymbolReferenceType>;
pub type Symbol = (SymbolRefType, Option<VarType>);

/// Simple expression (parameter value)
#[derive(Debug, Clone)]
pub enum ExprValue {
    LiteralNumber(i32),
    LiteralString(String),
    LiteralArray(Option<Vec<ExprValue>>),
    VariableReference(String),
    DefaultVariableReference,
    SymbolReference(Symbol),
    Expr(ExprOp, Box<ExprValue>, Box<ExprValue>),
    ContentNode(Box<ContentNodeType>),
    DefaultAction(Option<Vec<String>>, Option<Vec<ActionOpNode>>),
    Action(String, Option<Vec<String>>, Option<Vec<ActionOpNode>>)
}

#[derive(Debug, Clone)]
pub enum ElementExpr {
    Element(String,Option<String>,Option<Vec<Box<ExprValue>>>),
    Value(ExprValue)
}

#[derive(Debug, Clone)]
pub enum LensExprType {
    ForLens(Option<String>, ExprValue),
    GetLens(ExprValue)
}

#[derive(Debug, Clone)]
pub enum ActionOpNode {
    DispatchAction(String, Option<Vec<(String, ExprValue)>>)
}

#[derive(Debug, Clone)]
pub struct UseStmtType {
    pub package: String
}

#[derive(Debug, Clone)]
pub struct ComponentDefinitionType {
    pub name: String,
    pub inputs: Option<Vec<String>>,
    pub children: Option<Vec<NodeType>>
}

pub type EventHandler = (Option<String>,Option<EventHandlerParams>,Option<EventHandlerActionOps>);
pub type EventHandlerParams = Vec<String>;
pub type EventHandlerActionOps = Vec<ActionOpNode>;
pub type EventHandlersVec = Vec<EventHandler>;
pub type EventsItem = (String,Option<String>,Option<EventHandlerParams>,Option<EventHandlerActionOps>,Option<String>);
pub type EventsVec = Vec<EventsItem>;
pub type PropVec = Vec<Prop>;

pub type Prop = (String,Option<ExprValue>);

#[derive(Debug, Clone)]
pub struct ElementType {
    pub element_ty: String,
    pub element_key: Option<String>,
    pub attrs: Option<PropVec>,
    pub lens: Option<LensExprType>,
    pub children: Option<Vec<ContentNodeType>>,
    pub events: Option<Vec<(Option<String>,Option<EventHandlerParams>,Option<EventHandlerActionOps>)>>
}
