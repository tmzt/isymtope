#![allow(dead_code)]

use linked_hash_map::LinkedHashMap;
use parser::store::DefaultScopeNodeType;
use parser::loc::Loc;


#[derive(Debug, Default)]
pub struct Template {
    pub children: Vec<Loc<NodeType, (usize, usize)>>,
}

#[derive(Debug, Clone, PartialEq)]
pub enum NodeType {
    UseStmtNode(UseStmtType),
    ComponentDefinitionNode(ComponentDefinitionType),
    StoreNode(Vec<DefaultScopeNodeType>),
    ContentNode(ContentNodeType),
}

#[derive(Debug, Clone, PartialEq)]
pub enum ContentNodeType {
    ElementNode(ElementType),
    ExpressionValueNode(ExprValue),
    ForNode(Option<String>, ExprValue, Option<Vec<ContentNodeType>>),
}

/// Operators
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum ExprOp {
    Add,
    Sub,
    Mul,
    Div,
}

#[derive(Debug, Clone, PartialEq)]
pub enum ExprApplyOp {
    JoinString(Option<String>),
    Sum
}

#[derive(Debug, Clone, PartialEq)]
pub enum PrimitiveVarType {
    StringVar,
    Number,
    Expr,
}

#[derive(Debug, Clone, PartialEq)]
pub enum VarType {
    ArrayVar(Option<Box<VarType>>),
    Primitive(PrimitiveVarType),
}

#[derive(Debug, Clone, PartialEq)]
pub enum SymbolReferenceType {
    UnresolvedReference(String),
    ResolvedReference(String, ResolvedSymbolType, Option<String>),
}

#[derive(Debug, Clone, PartialEq)]
pub enum KeyReferenceType {
    ReducerKey(String),
    FuncParam(i32),
    FuncParamObjectProp(i32, String),
    UnboundFormalParam,
    BlockMapIndex,
    BlockMapObjectProp,
    ComponentProp(String),
    InvocationProp(Option<Box<ExprValue>>),
    CurrentElementProp(String),
    CurrentReducerActionParam(String),
    CurrentReducerActionState
}

#[derive(Debug, Clone, PartialEq)]
pub enum ResolvedSymbolType {
    ReferenceToKeyInScope(KeyReferenceType, Option<String>),

    ReducerKeyReference(String),
    ParameterReference(String),
    LocalVarReference(String),
    GlobalVarReference(String),
    ActionStateReference(Option<VarType>),
    ActionParamReference(String),
    LoopVarReference(String),
    LoopIndexReference(String, String),
    BlockParamReference(String),
    PropReference(String),
    LensPropReference(String, Box<LensExprType>),
    ForLensElementReference(String),
    ElementValueReference(String),
}

#[derive(Debug, Clone, PartialEq)]
pub struct Symbol(SymbolReferenceType, Option<VarType>, Option<Box<ExprValue>>);
pub type SymbolMap = LinkedHashMap<String, Symbol>;
pub type ValueMap = LinkedHashMap<String, ExprValue>;

impl Symbol {
    pub fn unresolved(key: &str) -> Symbol {
        Symbol(SymbolReferenceType::UnresolvedReference(key.to_owned()),
               None,
               None)
    }

    pub fn ref_key_in_scope(key: &str, key_ref: KeyReferenceType, scope_id: Option<&str>) -> Symbol {
        let resolved = ResolvedSymbolType::ReferenceToKeyInScope(key_ref, scope_id.map(|s| s.to_owned()));
        Symbol(SymbolReferenceType::ResolvedReference(key.to_owned(), resolved, None),
               None,
               None)
    }

    pub fn ref_prop_in_scope(key: &str, prop_key: &str, scope_id: Option<&str>) -> Symbol {
        let key_ref = KeyReferenceType::ComponentProp(prop_key.to_owned());
        let resolved = ResolvedSymbolType::ReferenceToKeyInScope(key_ref, scope_id.map(|s| s.to_owned()));
        Symbol(SymbolReferenceType::ResolvedReference(key.to_owned(), resolved, None),
               None,
               None)
    }

    pub fn element_prop(key: &str, prop_key: &str, scope_id: Option<&str>) -> Symbol {
        let key_ref = KeyReferenceType::CurrentElementProp(prop_key.to_owned());
        Self::ref_key_in_scope(key, key_ref, scope_id)
    }

    pub fn component_prop(key: &str, prop_key: &str, scope_id: Option<&str>) -> Symbol {
        let key_ref = KeyReferenceType::ComponentProp(prop_key.to_owned());
        Self::ref_key_in_scope(key, key_ref, scope_id)
    }

    /// Defines a value for a given formal parameter on the current component invocation scope.
    pub fn invocation_prop(key: &str, expr: Option<&ExprValue>) -> Symbol {
        let key_ref = KeyReferenceType::InvocationProp(expr.map(|e| Box::new(e.to_owned())));
        Self::ref_key_in_scope(key, key_ref, None)
    }

    /// Defines an unbound formal parameter on the current component definition.
    ///
    /// Retains the current scope id.
    pub fn unbound_formal_param(key: &str, scope_id: Option<&str>) -> Symbol {
        let key_ref = KeyReferenceType::UnboundFormalParam;
        Self::ref_key_in_scope(key, key_ref, scope_id)
    }

    // /// Creates a reference to an unbound formal parameter the given scope.
    // ///
    // /// Typically will be used to reference an unbound formal parameter in the currently
    // /// defined component and refers to the closest enclosing component definition.
    // pub fn unbound_formal_param_ref(key: &str, scope_id: &str) -> Symbol {
    //     // let key_ref = KeyReferenceType::UnboundFormalParamRef(key.to_owned());
    //     let key_ref = KeyReferenceType::UnboundFormalParam;
    //     Self::ref_key_in_scope(key, key_ref, Some(scope_id))
    // }

    pub fn reducer_key(key: &str) -> Symbol {
        let resolved = ResolvedSymbolType::ReducerKeyReference(key.to_owned());
        Symbol(SymbolReferenceType::ResolvedReference(key.to_owned(), resolved, None),
               None,
               None)
    }

    pub fn reducer_key_with_ty(key: &str, ty: Option<&VarType>) -> Symbol {
        let resolved = ResolvedSymbolType::ReducerKeyReference(key.to_owned());
        let ty = ty.map(|ty| ty.to_owned());
        Symbol(SymbolReferenceType::ResolvedReference(key.to_owned(), resolved, None),
               ty,
               None)
    }

    pub fn reducer_key_with_value(key: &str, value: &ExprValue) -> Symbol {
        let resolved = ResolvedSymbolType::ReducerKeyReference(key.to_owned());
        Symbol(SymbolReferenceType::ResolvedReference(key.to_owned(), resolved, None),
               None,
               Some(Box::new(value.clone())))
    }

    pub fn reducer_key_with(key: &str, ty: Option<&VarType>, value: Option<&ExprValue>) -> Symbol {
        let resolved = ResolvedSymbolType::ReducerKeyReference(key.to_owned());
        let value = value.map(|value| Box::new(value.clone()));
        let ty = ty.map(|ty| ty.clone());
        Symbol(SymbolReferenceType::ResolvedReference(key.to_owned(), resolved, None),
               ty,
               value)
    }

    pub fn action_param(key: &str, scope_id: &str) -> Symbol {
        let resolved = ResolvedSymbolType::ActionParamReference(key.to_owned());
        Symbol(SymbolReferenceType::ResolvedReference(key.to_owned(), resolved, Some(scope_id.to_owned())),
               None,
               None)
    }

    pub fn loop_idx(key: &str, block_id: &str, scope_id: &str) -> Symbol {
        let var_key = format!("__{}_{}", key, block_id);
        let resolved = ResolvedSymbolType::LoopIndexReference(key.to_owned(), block_id.to_owned());
        Symbol(SymbolReferenceType::ResolvedReference(var_key.clone(), resolved, Some(scope_id.to_owned())),
               None,
               None)
    }

    pub fn loop_var(key: &str, scope_id: &str) -> Symbol {
        let resolved = ResolvedSymbolType::LoopVarReference(key.to_owned());
        Symbol(SymbolReferenceType::ResolvedReference(key.to_owned(), resolved, Some(scope_id.to_owned())),
               None,
               None)
    }

    pub fn param(key: &str, scope_id: &str) -> Symbol {
        let resolved = ResolvedSymbolType::ParameterReference(key.to_owned());
        Symbol(SymbolReferenceType::ResolvedReference(key.to_owned(), resolved, Some(scope_id.to_owned())),
               None,
               None)
    }

    pub fn block_param(key: &str, scope_id: &str) -> Symbol {
        let resolved = ResolvedSymbolType::BlockParamReference(key.to_owned());
        Symbol(SymbolReferenceType::ResolvedReference(key.to_owned(), resolved, Some(scope_id.to_owned())),
               None,
               None)
    }

    // pub fn loop_var_with_value(key: &str, value: &ExprValue) -> Symbol {
    //     let resolved = ResolvedSymbolType::ReducerKeyReference(key.to_owned());
    //     let value = Some(Box::new(value.clone()));
    //     Symbol(SymbolReferenceType::ResolvedReference(key.to_owned(), resolved),
    //            None,
    //            value)
    // }

    pub fn prop(prop_name: &str, scope_id: &str) -> Symbol {
        let resolved = ResolvedSymbolType::PropReference(prop_name.to_owned());
        Symbol(SymbolReferenceType::ResolvedReference(prop_name.to_owned(), resolved, Some(scope_id.to_owned())),
               None,
               None)
    }

    // pub fn prop_with_value(prop_name: &str, value: &ExprValue) -> Symbol {
    //     let resolved = ResolvedSymbolType::PropReference(prop_name.to_owned());
    //     let value = Some(Box::new(value.clone()));
    //     // TODO: peek type
    //     Symbol(SymbolReferenceType::ResolvedReference(prop_name.to_owned(), resolved),
    //            None,
    //            value)
    // }

    pub fn action_state(ty: Option<&VarType>, scope_id: &str) -> Symbol {
        let resolved = ResolvedSymbolType::ActionStateReference(ty.map(|ty| ty.to_owned()));
        Symbol(SymbolReferenceType::ResolvedReference("state".to_owned(), resolved, Some(scope_id.to_owned())),
               ty.map(|ty| ty.to_owned()),
               None)
    }

    pub fn for_lens_element_key(key: &str, scope_id: &str) -> Symbol {
        let resolved = ResolvedSymbolType::ForLensElementReference(key.to_owned());
        Symbol(SymbolReferenceType::ResolvedReference(key.to_owned(), resolved, Some(scope_id.to_owned())),
               None,
               None)
    }

    pub fn element_value_binding(key: &str, element_key: &str, scope_id: &str) -> Symbol {
        let resolved = ResolvedSymbolType::ElementValueReference(element_key.to_owned());
        Symbol(SymbolReferenceType::ResolvedReference(key.to_owned(), resolved, Some(scope_id.to_owned())),
               None,
               None)
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
#[derive(Debug, Clone, PartialEq)]
pub enum ExprValue {
    LiteralNumber(i32),
    LiteralString(String),
    LiteralArray(Option<Vec<ExprValue>>),
    DefaultVariableReference,
    SymbolReference(Symbol),
    SymbolPathReference(Vec<Symbol>),
    Expr(ExprOp, Box<ExprValue>, Box<ExprValue>),
    Apply(ExprApplyOp, Option<Vec<Box<ExprValue>>>),
    ContentNode(Box<ContentNodeType>),
    DefaultAction(Option<Vec<String>>, Option<Vec<ActionOpNode>>),
    Action(String, Option<Vec<String>>, Option<Vec<ActionOpNode>>),
    Undefined,
}

impl ExprValue {
    #[inline]
    pub fn is_literal(&self) -> bool {
        match self {
            &ExprValue::LiteralArray(..) |
            &ExprValue::LiteralString(..) |
            &ExprValue::LiteralNumber(..) => true,
            _ => false,
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum ElementExpr {
    Element(String, Option<String>, Option<Vec<Box<ExprValue>>>),
    Value(ExprValue),
}

#[derive(Debug, Clone, PartialEq)]
pub enum LensExprType {
    ForLens(Option<String>, Symbol),
    GetLens(Symbol),
}

#[derive(Debug, Clone, PartialEq)]
pub enum ActionOpNode {
    DispatchAction(String, Option<PropVec>),
}

#[derive(Debug, Clone, PartialEq)]
pub struct UseStmtType {
    pub package: String,
}

#[derive(Debug, Clone, PartialEq)]
pub struct ComponentDefinitionType {
    pub name: String,
    pub inputs: Option<Vec<String>>,
    pub children: Option<Vec<NodeType>>,
}

pub type EventHandler = (Option<String>, Option<EventHandlerParams>, Option<EventHandlerActionOps>);
pub type EventHandlerParams = Vec<String>;
pub type EventHandlerActionOps = Vec<ActionOpNode>;
pub type EventHandlersVec = Vec<EventHandler>;
pub type EventsItem = (String,
                       Option<String>,
                       Option<EventHandlerParams>,
                       Option<EventHandlerActionOps>,
                       Option<String>,
                       Option<String>);
pub type EventsVec = Vec<EventsItem>;
pub type PropVec = Vec<Prop>;

pub type ElementEventBinding = (Option<String>, Option<Vec<String>>, Option<Vec<ActionOpNode>>);
pub type ElementValueBinding = Option<String>;
pub type ElementBindings = (ElementValueBinding, Option<Vec<ElementEventBinding>>);

#[derive(Debug, Clone, PartialEq)]
pub enum ElementBindingNodeType {
    ElementEventBindingNode(ElementEventBinding),
    ElementValueBindingNode(String),
}
pub type ElementBindingNodeVec = Vec<ElementBindingNodeType>;

pub type PropKey = String;
pub type PropList = Vec<PropKey>;
pub type Prop = (String, Option<ExprValue>);

#[derive(Debug, Clone, PartialEq)]
pub struct ElementType {
    pub element_ty: String,
    pub element_key: Option<String>,
    pub attrs: Option<PropVec>,
    pub lens: Option<LensExprType>,
    pub children: Option<Vec<ContentNodeType>>,
    pub bindings: Option<Vec<ElementBindingNodeType>>,
}
