
use std::error::Error;
use std::result;

use linked_hash_map::LinkedHashMap;

use parser::ast::*;
use parser::util::*;


#[derive(Debug, Default, PartialEq)]
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

#[derive(Debug, PartialEq)]
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

#[derive(Debug, Clone, PartialEq)]
pub enum ElementOp {
    ElementOpen(String,
                String,
                Option<Vec<Prop>>,
                Option<EventHandlersVec>,
                ElementValueBinding),
    ElementVoid(String,
                String,
                Option<Vec<Prop>>,
                Option<EventHandlersVec>,
                ElementValueBinding),
    ElementClose(String),
    WriteValue(ExprValue, String),
    InstanceComponent(String,
                      String,
                      Option<String>,
                      Option<Vec<PropKey>>,
                      Option<LensExprType>),
    StartBlock(String),
    EndBlock(String),
    MapCollection(String, Option<String>, ExprValue),
}

pub type OpsVec = Vec<ElementOp>;
pub type BlockMap = LinkedHashMap<String, BlockProcessingState>;
pub type ComponentMap = LinkedHashMap<String, Component>;
pub type ReducerKeyMap = LinkedHashMap<String, ReducerKeyData>;
pub type DefaultStateMap = LinkedHashMap<String, (Option<VarType>, Option<ExprValue>)>;

type ElementOpsVec = Vec<ElementOp>;

#[derive(Debug, Clone)]
pub struct Component {
    ty: String,
    ops: Option<ElementOpsVec>,
    // pub uses: Option<Vec<String>>,
    // pub child_map: Option<ComponentMap>,
    // pub symbol_map: SymbolMap,
    // pub props: SymbolMap,
    // pub events: Option<EventsVec>,
}

impl<'a> Component {
    #[allow(dead_code)]
    pub fn new<'ops, I>(ty: &str, ops: Option<I>) -> Self
      where I: IntoIterator<Item = &'ops ElementOp>
    {
        let ops = ops.map(|ops| ops.into_iter().map(|op| op.clone()).collect());
        Component {
            ty: ty.to_owned(),
            ops: ops
        }
    }

    /// Consumes parameters and returns new Component
    pub fn new_with_vec(ty: String, ops: Option<OpsVec>) -> Self {
        Component {
            ty: ty,
            ops: ops
        }
    }

    #[allow(dead_code)]
    pub fn ty(&self) -> &str { &self.ty }

    #[allow(dead_code)]
    pub fn ops_iter(&'a self) -> Option<impl IntoIterator<Item = &'a ElementOp>> {
        self.ops.as_ref().map(|ops| ops.into_iter())
    }
}

// Processing

use parser::token::Error as ParsingError;
use std::io;
use std::fmt;

#[derive(Debug)]
pub enum DocumentProcessingError {
    ParsingError(ParsingError),
    IOError(io::Error),
    FormatError(fmt::Error),
}

impl Error for DocumentProcessingError {
    fn description(&self) -> &str {
        match *self {
            DocumentProcessingError::ParsingError(ref e) => e.description(),
            DocumentProcessingError::IOError(ref e) => e.description(),
            DocumentProcessingError::FormatError(ref e) => e.description()
        }
    }
}

impl fmt::Display for DocumentProcessingError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            DocumentProcessingError::ParsingError(ref e) => e.fmt(f),
            DocumentProcessingError::IOError(ref e) => e.fmt(f),
            DocumentProcessingError::FormatError(ref e) => e.fmt(f)
        }
    }
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

// impl From<result::Result<(), io::Error>> for result::Result<(), DocumentProcessingError> {
//     fn from(r: result::Result<(), io::Error>) -> Self {



//     }
// }

// impl<T, E: Into<DocumentProcessingError>> From<result::Result<T, E>> for DocumentProcessingResult<T> {
//     fn from(r: result::Result<T, E>) -> Self {
//         match r {
//             result::Result::Ok(t) => DocumentProcessingResult::Ok(t),
//             result::Result::Err(e) => DocumentProcessingResult::Err(e)
//         }
//     }
// }

#[derive(Debug, Clone)]
pub struct BlockProcessingState {
    pub block_id: String,
    // pub scope: ElementOpScope,
    pub ops_vec: OpsVec,
    pub events_vec: EventsVec,
}

impl Default for BlockProcessingState {
    fn default() -> Self {
        let block_id = allocate_element_key();
        BlockProcessingState {
            block_id: block_id,
            // scope: Default::default(),
            ops_vec: Default::default(),
            events_vec: Default::default(),
        }
    }
}

#[derive(Debug, Clone, Default)]
pub struct ExprScopeProcessingState {
    pub symbol_map: SymbolMap,
}

#[derive(Debug, Default)]
pub struct DocumentProcessingState {
    root_block: BlockProcessingState,
    pub comp_map: ComponentMap,
    pub block_map: BlockMap,
    pub reducer_key_data: ReducerKeyMap,
    pub default_state_map: DefaultStateMap,
    pub has_default_state_key: bool,
    pub default_state_symbol: Option<Symbol>,
    pub default_reducer_key: Option<String>,
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
    pub default_reducer_key: Option<String>,
}

impl<'inp> DocumentState<'inp> {
    #[allow(dead_code)]
    pub fn resolve_symbol_value(&self, sym: &Symbol) -> Option<ExprValue> {
        match sym.sym_ref() {
            &SymbolReferenceType::ResolvedReference(_, ResolvedSymbolType::ReducerKeyReference(ref reducer_key), _) => {
                if let Some(reducer_data) = self.reducer_key_data.get(reducer_key) {
                    return reducer_data.default_expr.clone();
                };
            }
            _ => {}
        };
        None
    }

    #[allow(dead_code)]
    pub fn reducers_iter<'a>(&'a self) -> impl IntoIterator<Item = (&'a str, &'a ReducerKeyData)> {
        self.reducer_key_data.iter().map(|r| (r.0.as_str(), r.1))
    }
}
