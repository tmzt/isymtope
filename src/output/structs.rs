
use std::result;
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
    pub state_ty: Option<VarType>,
    pub default_scope_key: Option<String>,
}

impl ReducerActionData {
    pub fn from_name(action_name: &str, default_scope_key: Option<&str>) -> ReducerActionData {
        let action_type = action_name.to_uppercase();
        let default_scope_key = default_scope_key.map(String::from);

        ReducerActionData {
            action_type: String::from(action_type),
            state_expr: None,
            state_ty: None,
            default_scope_key: default_scope_key,
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
    MapCollection(String, Option<String>, ExprValue),
}

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

pub type OpsVec = Vec<ElementOp>;
pub type ComponentMap<'inp> = HashMap<&'inp str, Component<'inp>>;
pub type ReducerKeyMap<'inp> = HashMap<&'inp str, ReducerKeyData>;
pub type DefaultStateMap<'inp> = HashMap<&'inp str, (Option<VarType>, Option<ExprValue>)>;

#[derive(Debug, Clone, Default)]
pub struct ResolveVars {
    pub expr_context: ExpressionContext,
    pub cur_block_id: Option<String>,
    pub cur_state_key: Option<String>,
    pub cur_scope: Option<String>,
    pub default_var: Option<String>,
}

#[derive(Debug, Clone)]
pub enum ExpressionContext {
    Normal,
    ActionResult,
}
impl Default for ExpressionContext {
    fn default() -> Self {
        ExpressionContext::Normal
    }
}

impl ResolveVars {
    pub fn default_resolver() -> ResolveVars {
        ResolveVars {
            expr_context: ExpressionContext::Normal,
            cur_block_id: None,
            cur_state_key: None,
            cur_scope: None,
            default_var: None,
        }
    }

    pub fn with_state_key(&self, scope: &str) -> ResolveVars {
        ResolveVars {
            expr_context: self.expr_context.clone(),
            cur_block_id: self.cur_block_id.clone(),
            cur_state_key: Some(String::from(scope)),
            cur_scope: self.cur_scope.clone(),
            default_var: self.default_var.clone(),
        }
    }

    pub fn action_result(&self, state_key: &str) -> ResolveVars {
        ResolveVars {
            expr_context: ExpressionContext::ActionResult,
            cur_block_id: None,
            cur_state_key: Some(String::from(state_key)),
            cur_scope: None,
            default_var: None,
        }
    }

    pub fn block_scope(&self, block_id: &str, default_var: Option<&str>) -> ResolveVars {
        ResolveVars {
            expr_context: ExpressionContext::Normal,
            cur_block_id: Some(String::from(block_id)),
            cur_state_key: self.cur_state_key.clone(),
            cur_scope: self.cur_scope.clone(),
            default_var: default_var.map(String::from),
        }
    }

    #[allow(dead_code)]
    #[inline]
    pub fn state_key_only(&self) -> Option<String> {
        let state_key_part = self.cur_state_key.as_ref().map_or("".into(), |s| format!("{}", s));
        let scope_part = self.cur_scope.as_ref().map_or("".into(), |s| format!("{}", s));

        // (cur_state_key.cur_scope | cur_state_key | cur_scope)
        if self.cur_state_key.is_some() && self.cur_scope.is_some() {
            Some(format!("{}.{}", state_key_part, scope_part))
        } else if self.cur_state_key.is_some() || self.cur_scope.is_some() {
            Some(format!("{}{}", state_key_part, scope_part))
        } else {
            None
        }
    }

    #[inline]
    pub fn state_lookup_key(&self, var_name: Option<&str>) -> Option<String> {
        // (cur_state_key.cur_scope | cur_state_key | cur_scope) | var_name
        self.state_key_only().or_else(|| var_name.map(String::from))
    }

    #[inline]
    pub fn var_key(&self, var_name: Option<&str>) -> Option<String> {
        let state_key_only = self.state_key_only();

        // state_key_only.var_name | var_name
        if let (Some(ref state_key_part), Some(var_part)) = (state_key_only, var_name) {
            Some(format!("{}.{}", state_key_part, var_part))
        } else {
            var_name.map(String::from)
        }
    }

    #[inline]
    pub fn action_type(&self, action_name: &str) -> String {
        self.var_key(Some(action_name)).map(|s| s.to_uppercase()).map_or("".into(), |s| s)
    }

    #[inline]
    pub fn var_reference(&self, is_scope_var: bool, var_name: Option<&str>) -> String {
        let var_key = self.var_key(var_name);

        let scope_prefix = match self.expr_context {
            ExpressionContext::ActionResult => Some("state"),
            _ if is_scope_var => Some("store.getState()"),
            _ => None,
        };

        let scope_part = scope_prefix.map_or("", |s| s);
        let var_part = var_key.as_ref().map_or("", |s| s);

        if scope_prefix.is_some() && var_key.is_some() {
            format!("{}.{}", scope_part, var_part)
        } else {
            format!("{}{}", scope_part, var_part)
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

// Processing

use parser::token::Error as ParsingError;
use std::io;
use std::fmt;
use std::cell::Cell;

#[derive(Debug)]
pub enum DocumentProcessingError {
    ParsingError(ParsingError),
    IOError(io::Error),
    FormatError(fmt::Error),
}

pub type DocumentProcessingResult<T> = result::Result<T, DocumentProcessingError>;
pub type Result = DocumentProcessingResult<()>;

impl From<ParsingError> for DocumentProcessingError {
    fn from(err: ParsingError) -> Self {
        DocumentProcessingError::ParsingError(err)
    }
}
impl From<fmt::Error> for DocumentProcessingError {
    fn from(err: fmt::Error) -> Self {
        DocumentProcessingError::FormatError(err)
    }
}
impl From<io::Error> for DocumentProcessingError {
    fn from(err: io::Error) -> Self {
        DocumentProcessingError::IOError(err)
    }
}

#[derive(Debug, Default)]
pub struct BlockProcessingState {
    pub ops_vec: OpsVec,
    pub events_vec: EventsVec,
}

#[derive(Debug, Default)]
pub struct DocumentProcessingState<'inp> {
    root_block: BlockProcessingState,
    pub comp_map: ComponentMap<'inp>,
    pub reducer_key_data: ReducerKeyMap<'inp>,
    pub default_state_map: DefaultStateMap<'inp>,
    pub has_default_state_key: bool,
    pub default_state_key: Cell<Option<&'inp str>>,
}

#[derive(Debug)]
pub struct DocumentState<'inp> {
    pub ast: &'inp Template,
    pub root_block: BlockProcessingState,
    pub comp_map: ComponentMap<'inp>,
    pub reducer_key_data: ReducerKeyMap<'inp>,
    pub default_state_map: DefaultStateMap<'inp>,
}
