use model::*;


#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct RouteDefinitionType (String, Option<PropTypeVec>, RouteDefinitionActionType);

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum RouteDefinitionActionType {
    Content(Option<Vec<NodeType>>),
    Actions(Option<Vec<ActionOpNode>>)
}

impl RouteDefinitionType {
    pub fn content(pattern: String, inputs: Option<PropTypeVec>, children: Option<Vec<NodeType>>) -> Self {
        RouteDefinitionType (pattern, inputs, RouteDefinitionActionType::Content(children))
    }

    pub fn actions(pattern: String, inputs: Option<PropTypeVec>, action_ops: Option<Vec<ActionOpNode>>) -> Self {
        RouteDefinitionType (pattern, inputs, RouteDefinitionActionType::Actions(action_ops))
    }

    pub fn pattern(&self) -> &str { &self.0 }

    pub fn formals_iter<'a>(&'a self) -> Option<impl Iterator<Item = &'a PropType>> {
        self.1.as_ref().map(|formals| formals.iter())
    }

    pub fn action_ref(&self) -> &RouteDefinitionActionType { &self.2 }
}