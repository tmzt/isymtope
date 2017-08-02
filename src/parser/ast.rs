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
    ResolvedReference(String, ResolvedSymbolType)
}

#[derive(Debug, Clone)]
pub enum ResolvedSymbolType {
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
pub struct Symbol(SymbolReferenceType, Option<VarType>, Option<Box<ExprValue>>);
pub type SymbolMap = LinkedHashMap<String, Symbol>;

impl Symbol {
    pub fn unresolved(key: &str) -> Symbol {
        Symbol(SymbolReferenceType::UnresolvedReference(key.to_owned()), None, None)
    }

    pub fn reducer_key(key: &str) -> Symbol {
        let resolved = ResolvedSymbolType::ReducerKeyReference(key.to_owned());
        Symbol(SymbolReferenceType::ResolvedReference(key.to_owned(), resolved), None, None)
    }

    pub fn reducer_key_with_ty(key: &str, ty: Option<&VarType>) -> Symbol {
        let resolved = ResolvedSymbolType::ReducerKeyReference(key.to_owned());
        let ty = ty.map(|ty| ty.to_owned());
        Symbol(SymbolReferenceType::ResolvedReference(key.to_owned(), resolved), ty, None)
    }

    pub fn reducer_key_with_value(key: &str, value: &ExprValue) -> Symbol {
        let resolved = ResolvedSymbolType::ReducerKeyReference(key.to_owned());
        Symbol(SymbolReferenceType::ResolvedReference(key.to_owned(), resolved), None, Some(Box::new(value.clone())))
    }

    pub fn reducer_key_with(key: &str, ty: Option<&VarType>, value: Option<&ExprValue>) -> Symbol {
        let resolved = ResolvedSymbolType::ReducerKeyReference(key.to_owned());
        let value = value.map(|value| Box::new(value.clone()));
        let ty = ty.map(|ty| ty.clone());
        Symbol(SymbolReferenceType::ResolvedReference(key.to_owned(), resolved), ty, value)
    }

    pub fn loop_var(key: &str) -> Symbol {
        let resolved = ResolvedSymbolType::ReducerKeyReference(key.to_owned());
        Symbol(SymbolReferenceType::ResolvedReference(key.to_owned(), resolved), None, None)
    }

    pub fn loop_var_with_value(key: &str, value: &ExprValue) -> Symbol {
        let resolved = ResolvedSymbolType::ReducerKeyReference(key.to_owned());
        let value = Some(Box::new(value.clone()));
        Symbol(SymbolReferenceType::ResolvedReference(key.to_owned(), resolved), None, value)
    }

    pub fn prop(prop_name: &str) -> Symbol {
        let resolved = ResolvedSymbolType::PropReference(prop_name.to_owned());
        Symbol(SymbolReferenceType::ResolvedReference(prop_name.to_owned(), resolved), None, None)
    }

    pub fn prop_with_value(prop_name: &str, value: &ExprValue) -> Symbol {
        let resolved = ResolvedSymbolType::PropReference(prop_name.to_owned());
        let value = Some(Box::new(value.clone()));
        // TODO: peek type
        Symbol(SymbolReferenceType::ResolvedReference(prop_name.to_owned(), resolved), None, value)
    }

    pub fn action_state(ty: Option<&VarType>) -> Symbol {
        let resolved = ResolvedSymbolType::ActionStateReference(ty.map(|ty| ty.to_owned()));
        Symbol(SymbolReferenceType::ResolvedReference("state".to_owned(), resolved), ty.map(|ty| ty.to_owned()), None)
    }

    pub fn sym_ref(&self) -> &SymbolReferenceType {
        &self.0
    }

    pub fn ty(&self) -> Option<&VarType> {
        self.1.as_ref()
    }

    pub fn value(&self) -> Option<&ExprValue> {
        self.2.as_ref().map(|b| b.as_ref())
    }
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
    ForLens(Option<String>, Symbol),
    GetLens(Symbol)
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
