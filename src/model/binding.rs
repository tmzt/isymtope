use linked_hash_map::LinkedHashMap;
use model::*;


/// Bindings
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
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
    // ComponentFormalProp,
    ComponentPropsBinding,
    ComponentPropBinding(String),
    EventElementValueBinding,
    DOMElementBinding(Box<ExprValue>),
    DOMElementAttributeBinding(String, String),
    DOMInputElementValueBinding(String),
    DOMInputCheckboxElementCheckedBinding(Box<ReducedValue>)
}

impl BindingType {
    pub fn is_transparent(&self) -> bool {
        match self {
            &BindingType::MapItemBinding => true,
            _ => false
        }
    }
}

pub type BindingMap = LinkedHashMap<String, BindingType>;
pub type BindingOfTypeMap = LinkedHashMap<BindingType, ExprValue>;
