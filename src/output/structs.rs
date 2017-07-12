
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
pub struct ResolveVars { var_prefix: Option<String>, default_var: Option<String>, default_scope: Option<String> }

impl ResolveVars {
    pub fn for_block_scope<'inp>(is_state_varref: bool, block_id: &str, forvar: Option<&'inp str>, parent_scope: &ResolveVars) -> ResolveVars {
        let default_scope = format!("{}", parent_scope.default_scope.as_ref().map_or("", |s| s));

        let scoped_default = format!("{}{}", default_scope, forvar.as_ref().map_or("".into(), |s| format!(".{}", s)));
        let scoped_prefix = format!("{}.", scoped_default);

        let forvar_prefix = format!("__forvar_{}", block_id);
        let forvar_default = format!("__forvar_{}{}", block_id, forvar.as_ref().map_or("", |s| s));

        if is_state_varref {
            ResolveVars {
                var_prefix: Some(scoped_prefix),
                default_var: Some(scoped_default),
                default_scope: Some(default_scope),
            }
        } else {
            // Assume this is a for block parameter for now
            ResolveVars {
                var_prefix: Some(forvar_prefix),
                default_var: Some(forvar_default),
                default_scope: None,
            }
        }
    }
}

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
    root_block: BlockProcessingState,
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
pub struct DocumentState<'inp> {
    pub ast: &'inp Template,
    pub root_block: BlockProcessingState,
    pub comp_map: ComponentMap<'inp>,
    pub reducer_key_data: ReducerKeyMap<'inp>,
    pub default_state_map: DefaultStateMap<'inp>
}
