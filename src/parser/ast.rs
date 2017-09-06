// #![allow(dead_code)]

use linked_hash_map::LinkedHashMap;
pub use parser::store::*;
pub use parser::loc::Loc;


#[derive(Debug, Default)]
pub struct Template {
    pub children: Vec<Loc<NodeType, (usize, usize)>>,
}

impl Template {
    #[allow(dead_code)]
    pub fn new(children: Vec<Loc<NodeType, (usize, usize)>>) -> Template {
        Template { children: children }
    }
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
    ExpressionValueNode(ExprValue, String),
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
    // Sum
}

#[derive(Debug, Clone, PartialEq)]
pub enum PrimitiveVarType {
    StringVar,
    Number,
    // Expr,
}

#[derive(Debug, Clone, PartialEq)]
pub enum VarType {
    ArrayVar(Option<Box<VarType>>),
    Primitive(PrimitiveVarType),
}

impl VarType {
    #[allow(dead_code)]
    pub fn string() -> VarType { VarType::Primitive(PrimitiveVarType::StringVar) }

    #[allow(dead_code)]
    pub fn string_array() -> VarType { VarType::ArrayVar(Some(Box::new(VarType::Primitive(PrimitiveVarType::StringVar)))) }
}

#[derive(Debug, Clone, PartialEq)]
pub enum SymbolReferenceType {
    UnresolvedReference(String),
    Binding(BindingType),
    ResolvedReference(String, ResolvedSymbolType, Option<String>),
}

#[derive(Debug, Clone, PartialEq)]
pub enum KeyReferenceType {
    UnboundFormalParam,
    ComponentProp(String),
    InvocationProp(Option<Box<ExprValue>>),
    CurrentElementProp(String),
    UnboundReducerActionParam(String)
}

#[derive(Debug, Clone, PartialEq)]
pub enum ResolvedSymbolType {
    ReferenceToKeyInScope(KeyReferenceType, Option<String>),
    ReducerKeyReference(String),
    ParameterReference(String),
    PropReference(String),
}

#[derive(Debug, Clone, PartialEq)]
pub struct Symbol(SymbolReferenceType, Option<VarType>, Option<Box<ExprValue>>);
pub type SymbolMap = LinkedHashMap<String, Symbol>;
// pub type ValueMap = LinkedHashMap<String, ExprValue>;

impl Symbol {
    pub fn unresolved(key: &str) -> Symbol {
        Symbol(SymbolReferenceType::UnresolvedReference(key.to_owned()),
               None,
               None)
    }

    pub fn with_value(self, expr: ExprValue) -> Self {
        let mut sym = self;
        sym.2 = Some(Box::new(expr));
        sym
    }

    pub fn binding(binding: &BindingType) -> Symbol {
        Symbol(SymbolReferenceType::Binding(binding.to_owned()), None, None)
    }

    pub fn typed_binding(binding: &BindingType, ty: &VarType) -> Symbol {
        Symbol(SymbolReferenceType::Binding(binding.to_owned()), Some(ty.to_owned()), None)
    }

    pub fn ref_key_in_scope(key: &str, key_ref: KeyReferenceType, scope_id: Option<&str>) -> Symbol {
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

    pub fn reducer_key(key: &str) -> Symbol {
        let resolved = ResolvedSymbolType::ReducerKeyReference(key.to_owned());
        Symbol(SymbolReferenceType::ResolvedReference(key.to_owned(), resolved, None),
               None,
               None)
    }

    pub fn reducer_key_with(key: &str, ty: Option<&VarType>, value: Option<&ExprValue>) -> Symbol {
        let resolved = ResolvedSymbolType::ReducerKeyReference(key.to_owned());
        let value = value.map(|value| Box::new(value.clone()));
        let ty = ty.map(|ty| ty.clone());
        Symbol(SymbolReferenceType::ResolvedReference(key.to_owned(), resolved, None),
               ty,
               value)
    }

    pub fn action_param(key: &str, _scope_id: &str) -> Symbol {
        Symbol::binding(&BindingType::ActionParamBinding(key.to_owned()))
    }

    pub fn param(key: &str, scope_id: &str) -> Symbol {
        let resolved = ResolvedSymbolType::ParameterReference(key.to_owned());
        Symbol(SymbolReferenceType::ResolvedReference(key.to_owned(), resolved, Some(scope_id.to_owned())),
               None,
               None)
    }

    pub fn prop(prop_name: &str, scope_id: &str) -> Symbol {
        let resolved = ResolvedSymbolType::PropReference(prop_name.to_owned());
        Symbol(SymbolReferenceType::ResolvedReference(prop_name.to_owned(), resolved, Some(scope_id.to_owned())),
               None,
               None)
    }

    /// References used within reducers and actions

    pub fn unbound_action_param(key: &str, scope_id: Option<&str>) -> Symbol {
        // Symbol::binding(&BindingType::UnboundActionParam(key.to_owned()))
        let key_ref = KeyReferenceType::UnboundReducerActionParam(key.to_owned());
        Self::ref_key_in_scope(key, key_ref, scope_id)
    }

    /// Accessors

    pub fn sym_ref(&self) -> &SymbolReferenceType {
        &self.0
    }

    pub fn ty(&self) -> Option<&VarType> {
        self.1.as_ref()
    }

    #[allow(dead_code)]
    pub fn value(&self) -> Option<&ExprValue> {
        self.2.as_ref().map(|b| b.as_ref())
    }
}

/// Bindings
#[derive(Debug, Clone, PartialEq)]
pub enum BindingType {
    ExpressionBinding(Box<ExprValue>),
    KeyInSymbolsBinding(String, String),
    // ReducerPathBinding(String, Option<Vec<String>>),
    ReducerPathBinding(String),
    MapItemBinding,
    MapIndexBinding,
    // LoopIndexBinding,
    ActionStateBinding,
    ActionParamBinding(String),
    ComponentKeyBinding,
    ComponentPropBinding(String),
    DOMElementAttributeBinding(String, String),
    DOMInputElementValueBinding(String)
}

pub type BindingMap = LinkedHashMap<String, BindingType>;

/// Simple expression (parameter value)
#[derive(Debug, Clone, PartialEq)]
pub enum ExprValue {
    LiteralNumber(i32),
    LiteralString(String),
    LiteralArray(Option<Vec<ExprValue>>),
    DefaultVariableReference,
    SymbolReference(Symbol),
    SymbolPathReference(Vec<Symbol>),
    Binding(BindingType),
    TypedBinding(BindingType, VarType),

    Expr(ExprOp, Box<ExprValue>, Box<ExprValue>),
    Apply(ExprApplyOp, Option<Vec<Box<ExprValue>>>),
    ContentNode(Box<ContentNodeType>),
    // DefaultAction(Option<Vec<String>>, Option<Vec<ActionOpNode>>),
    // Action(String, Option<Vec<String>>, Option<Vec<ActionOpNode>>),
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

    #[inline]
    pub fn is_array(&self) -> bool {
        match self {
            &ExprValue::LiteralArray(..) => true,
            _ => false,
        }
    }

    #[inline]
    #[allow(dead_code)]
    pub fn is_literal_string(&self) -> bool {
        match self {
            &ExprValue::LiteralString(..) => true,
            _ => false,
        }
    }

    #[allow(dead_code)]
    pub fn peek_ty(&self) -> Option<VarType> {
        match self {
            &ExprValue::LiteralNumber(..) => {
                return Some(VarType::Primitive(PrimitiveVarType::Number));
            }

            &ExprValue::LiteralString(..) => {
                return Some(VarType::Primitive(PrimitiveVarType::StringVar));
            }

            &ExprValue::LiteralArray(Some(ref items)) => {
                if !items.is_empty() {
                    if let Some(ref first_item) = items.get(0) {
                        if let Some(var_ty) = first_item.peek_ty() {
                            return Some(VarType::ArrayVar(Some(Box::new(var_ty))));
                        }
                        return Some(VarType::ArrayVar(None));
                    };
                };
                return Some(VarType::ArrayVar(None));
            }

            &ExprValue::SymbolReference(ref sym) => {
                if let Some(ty) = sym.ty() { return Some(ty.to_owned()); }
            }

            _ => {}
        };
        None
    }

    #[inline]
    #[allow(dead_code)]
    pub fn peek_is_array(&self) -> bool {
        match self {
            &ExprValue::LiteralString(..) => true,

            &ExprValue::Expr(_, box ref l_expr, box ref r_expr) => {
                l_expr.peek_is_array() || r_expr.peek_is_array()
            }

            &ExprValue::SymbolReference(ref sym) => {
                if let Some(&VarType::ArrayVar(..)) = sym.ty() { true } else { false }
            }

            _ => false
        }
    }

    #[inline]
    #[allow(dead_code)]
    pub fn peek_is_string(&self) -> bool {
        match self {
            &ExprValue::LiteralString(..) => true,

            &ExprValue::Expr(_, box ref l_expr, box ref r_expr) => {
                l_expr.peek_is_string() || r_expr.peek_is_string()
            }

            &ExprValue::SymbolReference(ref sym) if sym.ty() == Some(&VarType::string()) => true,

            _ => false
        }
    }
}

#[allow(dead_code)]
#[derive(Debug, Clone, PartialEq)]
pub enum ElementExpr {
    Element(String, Option<String>, Option<Vec<Box<ExprValue>>>),
    Value(ExprValue),
}

#[derive(Debug, Clone, PartialEq)]
pub enum LensExprType {
    ForLens(Option<String>, ExprValue),
    GetLens(ExprValue),
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

#[derive(Debug, Clone, PartialEq)]
pub enum EventHandler {
    Event(String, Option<EventHandlerParams>, Option<EventHandlerActionOps>),
    DefaultEvent(Option<EventHandlerParams>, Option<EventHandlerActionOps>)
}

pub type EventHandlerParams = Vec<String>;
pub type EventHandlerActionOps = Vec<ActionOpNode>;
pub type EventHandlersVec = Vec<EventHandler>;
pub type EventsItem = (String, String, EventHandler);
pub type EventsVec = Vec<EventsItem>;
pub type PropVec = Vec<Prop>;

impl EventHandler {
    // pub fn event_name(&self) -> &str { &self.0 }
    // pub fn params_iter<'a>(&'a self) -> Option<impl IntoIterator<Item = &'a str>> { self.1.as_ref().map(|v| v.iter().map(|s| s.as_str())) }
    // pub fn action_ops_iter<'a>(&'a self) -> Option<impl IntoIterator<Item = &'a ActionOpNode>> { self.2.as_ref().map(|v| v.iter()) }

    // pub fn new(event_name: &str, params: Option<EventHandlerParams>, action_ops: Option<EventHandlerActionOps>) -> Self {
    //     EventHandler(event_name.to_owned(), params, action_ops)
    // }

    pub fn create_event(&self, element_key: &str, scope_id: &str) -> EventsItem {
        (element_key.to_owned(), scope_id.to_owned(), self.clone())
    }
}

// pub type ElementEventBinding = (Option<String>, Option<Vec<String>>, Option<Vec<ActionOpNode>>);
pub type ElementValueBinding = Option<String>;

#[derive(Debug, Clone, PartialEq)]
pub enum ElementBindingNodeType {
    ElementEventBindingNode(EventHandler),
    ElementValueBindingNode(String),
}

pub type PropKey = String;
pub type Prop = (String, Option<ExprValue>);

#[derive(Debug, Clone, PartialEq)]
pub struct ElementType {
    pub element_ty: String,
    pub element_key: String,
    pub attrs: Option<PropVec>,
    pub lens: Option<LensExprType>,
    pub children: Option<Vec<ContentNodeType>>,
    pub bindings: Option<Vec<ElementBindingNodeType>>,
}
