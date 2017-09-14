
use std::io;
use std::fmt;
use std::error::Error;
use std::result;
use std::collections::hash_map::{HashMap, Entry};

use linked_hash_map::LinkedHashMap;

use parser::*;
use parser::token::Error as ParsingError;
use processing::*;


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

pub type ComponentKeyMapping = (String, String);
pub type ComponentKeyMappingVec = Vec<ComponentKeyMapping>;

#[derive(Debug, Clone)]
pub struct Component {
    ty: String,
    block: Block,
    formal_props: Option<FormalPropVec>
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
}

#[derive(Debug)]
pub enum DocumentTypeError {
    TypeError(String),
    MismatchActionParam(String, String, VarType, VarType)
}

impl DocumentTypeError {
    pub fn mismatch_action_param(complete_key: &str, param_key: &str, existing_ty: &VarType, ty: &VarType) -> DocumentTypeError {
        DocumentTypeError::MismatchActionParam(complete_key.to_owned(), param_key.to_owned(), existing_ty.to_owned(), ty.to_owned())
    }
}

impl Error for DocumentTypeError {
    fn description(&self) -> &str {
        match self {
            &DocumentTypeError::TypeError(..) => "Type error in document",
            &DocumentTypeError::MismatchActionParam(..) => "Type error in document: reducer action param has different type than previous dispatch of this action."
        }
    }
}

impl fmt::Display for DocumentTypeError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            &DocumentTypeError::TypeError(ref s) => f.write_str(s),
            &DocumentTypeError::MismatchActionParam(ref complete_key, ref param_key, ref previous_ty, ref new_ty) => {
                write!(f, "Type error: reducer action ({}) has existing type for param ({}) of ({:?}), attempting to dispatch the action again with a different type ({:?}) for this parameter.",
                    complete_key, param_key, previous_ty, new_ty)
            }
        }
    }
}

#[derive(Debug)]
pub enum DocumentProcessingError {
    ParsingError(ParsingError),
    IOError(io::Error),
    FormatError(fmt::Error),
    TypeError(DocumentTypeError)
}

impl Error for DocumentProcessingError {
    fn description(&self) -> &str {
        match self {
            &DocumentProcessingError::ParsingError(ref e) => e.description(),
            &DocumentProcessingError::IOError(ref e) => e.description(),
            &DocumentProcessingError::FormatError(ref e) => e.description(),
            &DocumentProcessingError::TypeError(ref e) => e.description()
        }
    }
}

impl fmt::Display for DocumentProcessingError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            DocumentProcessingError::ParsingError(ref e) => e.fmt(f),
            DocumentProcessingError::IOError(ref e) => e.fmt(f),
            DocumentProcessingError::FormatError(ref e) => e.fmt(f),
            DocumentProcessingError::TypeError(ref e) => e.fmt(f)
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
impl From<DocumentTypeError> for DocumentProcessingError {
    fn from(err: DocumentTypeError) -> Self {
        DocumentProcessingError::TypeError(err)
    }
}

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
    pub reducer_key_data: ReducerKeyProcessingMap,
    pub default_state_map: DefaultStateMap,
    pub has_default_state_key: bool,
    pub default_state_symbol: Option<Symbol>,
    pub default_reducer_key: Option<String>,
    pub action_tys: ReducerActionTypeMap
}

impl DocumentProcessingState {
    pub fn insert_prop_type(&mut self, complete_key: &str, param_key: &str, ty: &VarType) -> Result {
        let action_ty_data = self.action_tys.entry(complete_key.to_owned()).or_insert(Default::default());

        match action_ty_data.entry(param_key.to_owned()) {
            Entry::Occupied(o) => {
                if let &Some(ref existing_ty) = o.get() {
                    if existing_ty != ty {
                        return Err(DocumentTypeError::mismatch_action_param(complete_key, param_key, &existing_ty, ty).into());
                    };
                };
            },
            Entry::Vacant(v) => {
                v.insert(Some(ty.to_owned()));
            }
        };
        Ok(())
    }
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
    pub action_tys: ReducerActionTypeMap
}

impl<'inp> Into<Document> for DocumentProcessingState {
    fn into(self) -> Document {
        let has_comp_map = self.comp_map.len() > 0;
        let comp_map = if has_comp_map { Some(self.comp_map) } else { None };
        let reducer_key_data: ReducerKeyMap = self.reducer_key_data.into_iter().map(|d| (d.0, d.1.into())).collect();

        Document {
            root_block: self.root_block.into(),
            comp_map: comp_map,
            reducer_key_data: reducer_key_data,
            default_state_map: self.default_state_map,
            default_state_symbol: self.default_state_symbol,
            default_reducer_key: self.default_reducer_key,
            action_tys: self.action_tys
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
    pub fn get_action_param_ty(&mut self, reducer_Key: &str, action_key: &str, param_key: &str) -> Option<&VarType> {
        if let &Some(action_ty_data) = self.action_tys.get(reducer_key)) {
            if let Some(param_ty_data) = action_ty_data.get(action_key)
        }
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