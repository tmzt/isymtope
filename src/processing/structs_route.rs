use model::*;
use processing::*;


#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Route (String, Option<FormalPropVec>, RouteActionType);

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum RouteActionType {
    Content(Block),
    Actions(Option<Vec<ActionOpNode>>)
}

impl Route {
    pub fn content(pattern: String, formal_props: Option<FormalPropVec>, block: Block) -> Self {
        Route (pattern, formal_props, RouteActionType::Content(block))
    }

    pub fn actions(pattern: String, formal_props: Option<FormalPropVec>, action_ops: Option<Vec<ActionOpNode>>) -> Self {
        Route (pattern, formal_props, RouteActionType::Actions(action_ops))
    }

    pub fn pattern(&self) -> &str { &self.0 }

    pub fn action_ops_iter<'a>(&'a self) -> Option<impl Iterator<Item = &'a ActionOpNode>> {
        match self.2 {
            RouteActionType::Actions(ref action_ops) => action_ops.as_ref().map(|v| v.iter()),
            _ => None
        }
    }

    pub fn function_key(&self) -> String {
        (&self.0).replace("*", "_2a_").replace("/", "_2f_")
    }
}