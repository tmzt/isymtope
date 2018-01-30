#[derive(Debug, Clone, Hash, PartialEq)]
pub enum OutputScopeEnvironment {
    Normal,
    Reducer(String),
    Component,
    ComponentInstance,
    MappedComponentInstance,
    SubComponentInstance,
    MappedSubComponentInstance,
}
