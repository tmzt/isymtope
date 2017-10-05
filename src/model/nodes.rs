use model::*;


#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct UseStmtType {
    pub package: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum NodeType {
    UseStmtNode(UseStmtType),
    ComponentDefinitionNode(ComponentDefinitionType),
    RouteDefinitionNode(RouteDefinitionType),
    StoreNode(Vec<DefaultScopeNodeType>),
    ContentNode(ContentNodeType),
    QueryNode(QueryDefinition)
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum ContentNodeType {
    ElementNode(ElementType),
    ExpressionValueNode(ExprValue, String),
    ForNode(Option<String>, ExprValue, Option<Vec<ContentNodeType>>),
}
