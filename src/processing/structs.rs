
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

pub type ComponentKeyMapping = (String, String);
pub type ComponentKeyMappingVec = Vec<ComponentKeyMapping>;

#[derive(Debug, Clone)]
pub struct Component {
    ty: String,
    block: Block,
    formal_props: Option<FormalPropVec>
    // ops: Option<ElementOpsVec>,
    // pub uses: Option<Vec<String>>,
    // pub child_map: Option<ComponentMap>,
    // pub symbol_map: SymbolMap,
    // pub props: SymbolMap,
    // events: Option<EventsVec>,
}

impl Component {
    /// Consumes parameters and returns new Component
    pub fn new(ty: String, block: Block, formal_props: Option<FormalPropVec>) -> Self {
        Component {
            ty: ty,
            block: block,
            formal_props: formal_props
            // events: events
        }
    }

    #[allow(dead_code)]
    pub fn ty(&self) -> &str { &self.ty }

    #[allow(dead_code)]
    pub fn root_block<'a>(&'a self) -> &'a Block { &self.block }

    pub fn formal_props_iter<'a>(&'a self) -> Option<impl IntoIterator<Item = FormalPropRef<'a>>> {
        self.formal_props.as_ref().map(|v| v.into_iter().map(|s| (s.as_str())))
    }

    // pub fn actual_props_iter<'a>(&'a self) -> Option<impl IntoIterator<Item = ActualPropRef<'a>>> {
    //     self.formal_props.as_ref().map(|v| v.into_iter().map(|p| (p.0.as_str(), p.1.as_ref().map(|e| e)))
    // }

    // #[allow(dead_code)]
    // pub fn ops_iter<'a>(&'a self) -> Option<impl IntoIterator<Item = &'a ElementOp>> {
    //     self.block.ops_iter()
    // }

    // #[allow(dead_code)]
    // pub fn events_iter<'a>(&'a self) -> Option<impl IntoIterator<Item = &'a EventsItem>> {
    //     self.block.events_iter()
    // }

    // #[allow(dead_code)]
    // pub fn componentkey_mappings_iter<'a>(&'a self) -> Option<impl IntoIterator<Item = &'a ComponentKeyMapping>> {
    //     self.block.componentkey_mappings_iter()
    // }
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
    pub child_blocks: Vec<Box<BlockProcessingState>>,
    // pub scope: ElementOpScope,
    pub ops_vec: OpsVec,
    pub events_vec: EventsVec,
    pub compkey_mappings: ComponentKeyMappingVec
}

impl Default for BlockProcessingState {
    fn default() -> Self {
        let block_id = allocate_element_key();
        BlockProcessingState {
            block_id: block_id,
            child_blocks: Default::default(),
            // scope: Default::default(),
            ops_vec: Default::default(),
            events_vec: Default::default(),
            compkey_mappings: Default::default()
        }
    }
}

#[derive(Debug, Clone)]
pub struct Block {
    block_id: String,
    child_blocks: Option<BlockVec>,
    ops: Option<OpsVec>,
    events: Option<EventsVec>,
    compkey_mappings: Option<ComponentKeyMappingVec>
}

pub type BlockVec = Vec<Block>;

impl Into<Block> for BlockProcessingState {
    fn into(self) -> Block {
        let has_ops = self.ops_vec.len() > 0;
        let has_events = self.events_vec.len() > 0;
        let has_compkey_mappings = self.compkey_mappings.len() > 0;
        let has_child_blocks = self.child_blocks.len() > 0;

        let ops = if has_ops { Some(self.ops_vec) } else { None };
        let events = if has_events { Some(self.events_vec) } else { None };
        let compkey_mappings = if has_compkey_mappings { Some(self.compkey_mappings) } else { None };

        let child_blocks: Option<BlockVec> = if has_child_blocks {
            Some(self.child_blocks.into_iter().map(|child_block| {
                let box child_block = child_block;
                child_block.into()
            }).collect())
        } else {
            None
        };
        
        Block {
            block_id: self.block_id,
            child_blocks: child_blocks,
            ops: ops,
            events: events,
            compkey_mappings: compkey_mappings
        }
    }
}

impl Block {
    pub fn id(&self) -> &str { &self.block_id }

    #[allow(dead_code)]
    pub fn blocks_iter<'a>(&'a self) -> Option<impl IntoIterator<Item = &'a Block>> {
        self.child_blocks.as_ref().map(|child_blocks| child_blocks.into_iter())
    }

    #[allow(dead_code)]
    pub fn ops_iter<'a>(&'a self) -> Option<impl IntoIterator<Item = &'a ElementOp>> {
        self.ops.as_ref().map(|ops| ops.into_iter())
    }

    #[allow(dead_code)]
    pub fn events_iter<'a>(&'a self) -> Option<impl IntoIterator<Item = &'a EventsItem>> {
        self.events.as_ref().map(|events| events.into_iter())
    }

    #[allow(dead_code)]
    pub fn componentkey_mappings_iter<'a>(&'a self) -> Option<impl IntoIterator<Item = &'a ComponentKeyMapping>> {
        self.compkey_mappings.as_ref().map(|compkey_mappings| compkey_mappings.into_iter())
    }
}

#[derive(Debug, Default)]
pub struct DocumentProcessingState {
    pub root_block: BlockProcessingState,
    pub comp_map: ComponentMap,
    // pub block_map: BlockMap,
    pub reducer_key_data: ReducerKeyMap,
    pub default_state_map: DefaultStateMap,
    pub has_default_state_key: bool,
    pub default_state_symbol: Option<Symbol>,
    pub default_reducer_key: Option<String>,
}

#[derive(Debug)]
pub struct Document {
    root_block: Block,
    comp_map: Option<ComponentMap>,
    // pub block_map: BlockMap,
    pub reducer_key_data: ReducerKeyMap,
    pub default_state_map: DefaultStateMap,
    pub default_state_symbol: Option<Symbol>,
    pub default_reducer_key: Option<String>,
}

impl<'inp> Into<Document> for DocumentProcessingState {
    fn into(self) -> Document {
        let has_comp_map = self.comp_map.len() > 0;
        let comp_map = if has_comp_map { Some(self.comp_map) } else { None };

        Document {
            root_block: self.root_block.into(),
            comp_map: comp_map,
            reducer_key_data: self.reducer_key_data,
            default_state_map: self.default_state_map,
            default_state_symbol: self.default_state_symbol,
            default_reducer_key: self.default_reducer_key,
        }
    }
}

impl Document {
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

    #[allow(dead_code)]
    pub fn root_block<'a>(&'a self) -> &'a Block {
        &self.root_block
    }

    #[allow(dead_code)]
    pub fn get_component_definitions<'a>(&'a self) -> Option<impl Iterator<Item = (&'a str, &'a Component)>> {
        self.comp_map.as_ref().map(|comp_map| comp_map.iter().map(|c| (c.0.as_str(), c.1)))
    }

    #[allow(dead_code)]
    pub fn get_component<'a>(&'a self, component_ty: &str) -> Option<&'a Component> {
        self.comp_map.as_ref().map_or(None, |comp_map| comp_map.get(component_ty))
    }
}
