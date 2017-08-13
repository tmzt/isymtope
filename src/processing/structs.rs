
use std::result;

use linked_hash_map::LinkedHashMap;

use processing::scope::*;
use parser::ast::*;
use parser::store::*;
use parser::util::*;

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
    ElementOpen(String, Option<String>, Option<Vec<Prop>>, Option<EventHandlersVec>, ElementValueBinding),
    ElementVoid(String, Option<String>, Option<Vec<Prop>>, Option<EventHandlersVec>, ElementValueBinding),
    ElementClose(String),
    WriteValue(ExprValue, Option<String>),
    InstanceComponent(String, Option<String>, Option<Vec<Prop>>, Option<LensExprType>),
    StartBlock(String),
    EndBlock(String),
    MapCollection(String, Option<String>, ExprValue),
}

// #[derive(Debug, Clone)]
// pub enum PrimitiveVarType {
//     StringVar,
//     Number,
//     Expr,
// }

// #[derive(Debug, Clone)]
// pub enum VarType {
//     ArrayVar(Option<Box<VarType>>),
//     Primitive(PrimitiveVarType),
// }

pub type OpsVec = Vec<ElementOp>;
pub type BlockMap = LinkedHashMap<String, BlockProcessingState>;
pub type ComponentMap = LinkedHashMap<String, Component>;
pub type ReducerKeyMap = LinkedHashMap<String, ReducerKeyData>;
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
    pub symbol_map: SymbolMap,
    pub props: SymbolMap,
    pub events: Option<EventsVec>
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

pub type ProcessingScope = (Option<String>, Option<String>, Option<Symbol>);

#[derive(Debug)]
pub struct BlockProcessingState {
    pub block_id: String,
    pub scope: DocumentProcessingScope,
    pub ops_vec: OpsVec,
    pub events_vec: EventsVec,
}

impl Default for BlockProcessingState {
    fn default() -> Self {
        let block_id = allocate_element_key();
        BlockProcessingState {
            block_id: block_id,
            scope: Default::default(),
            ops_vec: Default::default(),
            events_vec: Default::default()
        }
    }
}

#[derive(Debug, Clone, Default)]
pub struct ExprScopeProcessingState {
    pub symbol_map: SymbolMap
}

// impl ExprScopeProcessingState {
//     pub fn with_symbol(mut self, var_name: &str, sym: SymbolReferenceType, ty: Option<&VarType>) -> Self {
//         self.symbol_map.insert(var_name.to_owned(), Symbol(sym, ty.map(Clone::clone), None));
//         self
//     }
// }

#[derive(Debug, Default)]
pub struct DocumentProcessingState {
    root_block: BlockProcessingState,
    pub comp_map: ComponentMap,
    pub block_map: BlockMap,
    pub reducer_key_data: ReducerKeyMap,
    pub default_state_map: DefaultStateMap,
    pub has_default_state_key: bool,
    pub default_state_symbol: Option<Symbol>,
    pub default_reducer_key: Option<String>
}

#[derive(Debug)]
pub struct DocumentState<'inp> {
    pub ast: &'inp Template,
    pub root_block: BlockProcessingState,
    pub comp_map: ComponentMap,
    pub block_map: BlockMap,
    pub reducer_key_data: ReducerKeyMap,
    pub default_state_map: DefaultStateMap,
    pub default_state_symbol: Option<Symbol>,
    pub default_reducer_key: Option<String>
}

impl<'inp> DocumentState<'inp> {
    pub fn resolve_symbol_value(&self, sym: &Symbol) -> Option<ExprValue> {
        match sym.sym_ref() {
            &SymbolReferenceType::ResolvedReference(_, ResolvedSymbolType::ReducerKeyReference(ref reducer_key)) => {
                if let Some(reducer_data) = self.reducer_key_data.get(reducer_key) {
                    return reducer_data.default_expr.clone();
                };
            }
            _ => {}
        };
        None
    }
}