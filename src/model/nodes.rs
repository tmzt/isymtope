use model::*;


#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct UseStmtType {
    pub package: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum NodeType {
    UseStmtNode(UseStmtType),
    ComponentDefinitionNode(ComponentDefinitionType),
    StoreNode(Vec<DefaultScopeNodeType>),
    ContentNode(ContentNodeType),
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum ContentNodeType {
    ElementNode(ElementType),
    ExpressionValueNode(ExprValue, String),
    ForNode(Option<String>, ExprValue, Option<Vec<ContentNodeType>>),
}
