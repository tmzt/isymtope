
use std::collections::hash_map::HashMap;
use parser::ast::*;
use parser::store::*;

#[derive(Debug, Default)]
pub struct ReducerKeyData {
    pub reducer_key: String,
    pub default_expr: Option<ExprValue>,
    pub actions: Option<Vec<ReducerActionData>>,
}

impl ReducerKeyData {
    pub fn from_name(reducer_key: &str) -> ReducerKeyData {
        ReducerKeyData {
            reducer_key: String::from(reducer_key),
            default_expr: None,
            actions: Some(Vec::new()),
        }
    }
}

#[derive(Debug)]
pub struct ReducerActionData {
    pub action_type: String,
    pub state_expr: Option<ActionStateExprType>,
}

impl ReducerActionData {
    pub fn from_name(action_name: &str) -> ReducerActionData {
        let action_type = action_name.to_uppercase();

        ReducerActionData {
            action_type: String::from(action_type),
            state_expr: None,
        }
    }
}

#[derive(Debug, Clone)]
pub enum ElementOp {
    ElementOpen(String, Option<String>, Option<Vec<Prop>>, Option<EventHandlersVec>),
    ElementVoid(String, Option<String>, Option<Vec<Prop>>, Option<EventHandlersVec>),
    ElementClose(String),
    WriteValue(ExprValue, Option<String>),
    InstanceComponent(String, Option<String>, Option<Vec<Prop>>, Option<String>),
    StartBlock(String),
    EndBlock(String),
    MapCollection(String, Option<String>, ExprValue)
}

#[allow(dead_code)]
#[derive(Debug, Clone)]
pub enum PrimitiveVarType {
    StringVar,
    Number,
    Expr
}

#[derive(Debug, Clone)]
pub enum VarType {
    ArrayVar(Option<Box<VarType>>),
    Primitive(PrimitiveVarType)
}

pub type OpsVec = Vec<ElementOp>;
pub type ComponentMap<'inp> = HashMap<&'inp str, Component<'inp>>;
pub type ReducerKeyMap<'inp> = HashMap<&'inp str, ReducerKeyData>;
pub type DefaultStateMap<'inp> = HashMap<&'inp str, (Option<VarType>, Option<ExprValue>)>;

#[derive(Debug, Clone, Default)]
pub struct ResolveVars<'inp> { var_prefix: Option<&'inp str>, default_var: Option<&'inp str>, default_scope: Option<&'inp str> }

#[derive(Debug, Clone)]
pub struct Component<'input> {
    pub name: &'input str,
    pub ops: Option<OpsVec>,
    pub uses: Option<Vec<&'input str>>,
    pub child_map: Option<ComponentMap<'input>>,
}

#[derive(Debug, Default)]
pub struct BlockProcessingState {
    pub ops_vec: OpsVec,
    pub events_vec: EventsVec,
}

#[derive(Debug, Default)]
pub struct DocumentProcessingState<'inp> {
    pub keys_vec: Vec<String>,
    pub comp_map: ComponentMap<'inp>,
    pub reducer_key_data: ReducerKeyMap<'inp>,
    pub default_state_map: DefaultStateMap<'inp>
}

/*
impl<'inp> Default for DocumentProcessingState<'inp> {
    fn default() -> DocumentProcessingState<'inp> {
        DocumentProcessingState {
            keys_vec: Default::default(),
            comp_map: Default::default(),
            reducer_key_data: Default::default(),
            default_state_map: Default::default()
        }
    }
}*/

#[derive(Debug)]
pub struct DocumentState<'doc, 'inp: 'doc> {
    pub ops_vec: &'doc OpsVec,
    pub comp_map: &'doc ComponentMap<'inp>,
    pub reducer_key_data: &'doc ReducerKeyMap<'inp>,
    pub default_state_map: &'doc DefaultStateMap<'inp>
}
