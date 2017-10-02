use model::*;


#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct ComponentDefinitionType {
    pub name: String,
    pub inputs: Option<Vec<String>>,
    pub children: Option<Vec<NodeType>>,
}
