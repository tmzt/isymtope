#![allow(dead_code)]

use linked_hash_map::LinkedHashMap;
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
    UnresolvedReference(String),
    ReducerKeyReference(String),
    ParameterReference(String),
    LocalVarReference(String),
    GlobalVarReference(String),
    ActionStateReference(Option<VarType>),
    LoopVarReference(String),
    PropReference(String),
    LensPropReference(String, Box<LensExprType>)
}

#[derive(Debug, Clone)]
pub struct Symbol(pub SymbolReferenceType, pub Option<VarType>, pub Option<Box<ExprValue>>);
pub type SymbolMap = LinkedHashMap<String, Symbol>;

impl Symbol {
    pub fn unresolved(key: &str) -> Symbol {
        Symbol(SymbolReferenceType::UnresolvedReference(key.to_owned()), None, None)
    }

    pub fn reducer_key(key: &str) -> Symbol {
        Symbol(SymbolReferenceType::ReducerKeyReference(key.to_owned()), None, None)
    }

    pub fn reducer_key_with_value(key: &str, value: &ExprValue) -> Symbol {
        Symbol(SymbolReferenceType::ReducerKeyReference(key.to_owned()), None, Some(Box::new(value.clone())))
    }

    pub fn loop_var(key: &str) -> Symbol {
        Symbol(SymbolReferenceType::LoopVarReference(key.to_owned()), None, None)
    }

    pub fn loop_var_with_value(key: &str, value: &ExprValue) -> Symbol {
        Symbol(SymbolReferenceType::LoopVarReference(key.to_owned()), None, Some(Box::new(value.clone())))
    }

    pub fn prop(prop_name: &str) -> Symbol {
        Symbol(SymbolReferenceType::PropReference(prop_name.to_owned()), None, None)
    }

    pub fn prop_with_value(prop_name: &str, value: &ExprValue) -> Symbol {
        let value = Some(Box::new(value.clone()));
        // TODO: peek type
        Symbol(SymbolReferenceType::LoopVarReference(prop_name.to_owned()), None, value)
    }

    // pub fn with_value(value: &ExprValue) -> Symbol {
    //     Symbol(SymbolReferenceType::UnresolvedReference(key.to_owned()), None, None)
    // }
}

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
