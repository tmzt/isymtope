
use std::result;

use linked_hash_map::LinkedHashMap;

use parser::ast::*;
use parser::store::*;

#[derive(Debug, Default)]
pub struct ReducerKeyData {
    pub reducer_key: String,
    pub default_expr: Option<ExprValue>,
    pub ty: Option<VarType>,
    pub actions: Option<Vec<ReducerActionData>>,
}

impl ReducerKeyData {
    pub fn from_name(reducer_key: &str, ty: Option<VarType>) -> ReducerKeyData {
        ReducerKeyData {
            reducer_key: String::from(reducer_key),
            default_expr: None,
            ty: ty,
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
pub type ComponentMap = LinkedHashMap<String, Component>;
pub type ReducerKeyMap<'inp> = LinkedHashMap<String, ReducerKeyData>;
pub type DefaultStateMap = LinkedHashMap<String, (Option<VarType>, Option<ExprValue>)>;

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

#[derive(Debug, Clone)]
pub struct Component {
    pub name: String,
    pub ops: Option<OpsVec>,
    pub uses: Option<Vec<String>>,
    pub child_map: Option<ComponentMap>,
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

#[derive(Debug)]
pub enum SymbolReferenceType {
    ReducerKeyReference(String),
    ParameterReference(String),
    LocalVarReference(String)
}
pub type SymbolRefType = Option<SymbolReferenceType>;

pub type Symbol = (SymbolRefType, Option<VarType>);
pub type SymbolMap = LinkedHashMap<String, Symbol>;

pub type ProcessingScope = (Option<String>, Option<String>, SymbolRefType);

#[derive(Debug, Default)]
pub struct BlockProcessingState {
    pub symbol_map: SymbolMap, 
    pub ops_vec: OpsVec,
    pub events_vec: EventsVec,
}

#[derive(Debug, Default)]
pub struct ExprScopeProcessingState {
    pub symbol_map: SymbolMap
}

#[derive(Debug, Default)]
pub struct DocumentProcessingState<'inp> {
    root_block: BlockProcessingState,
    pub comp_map: ComponentMap,
    pub reducer_key_data: ReducerKeyMap<'inp>,
    pub default_state_map: DefaultStateMap,
    pub has_default_state_key: bool,
    pub default_state_key: Cell<Option<&'inp str>>,
}

#[derive(Debug)]
pub struct DocumentState<'inp> {
    pub ast: &'inp Template,
    pub root_block: BlockProcessingState,
    pub comp_map: ComponentMap,
    pub reducer_key_data: ReducerKeyMap<'inp>,
    pub default_state_map: DefaultStateMap,
    pub default_reducer_key: Option<&'inp str>
}