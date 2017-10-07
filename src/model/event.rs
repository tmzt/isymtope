use model::*;


#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum ActionOpNode {
    DispatchAction(String, Option<PropVec>),
    DispatchActionTo(String, Option<PropVec>, String),
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum EventHandler {
    Event(String, Option<EventHandlerParams>, Option<EventHandlerActionOps>),
    DefaultEvent(Option<EventHandlerParams>, Option<EventHandlerActionOps>)
}

pub type EventHandlerParams = Vec<String>;
pub type EventHandlerActionOps = Vec<ActionOpNode>;
pub type EventHandlersVec = Vec<EventHandler>;
pub type EventsItem = (String, String, EventHandler);
pub type EventsVec = Vec<EventsItem>;

impl EventHandler {
    // pub fn event_name(&self) -> &str { &self.0 }
    // pub fn params_iter<'a>(&'a self) -> Option<impl IntoIterator<Item = &'a str>> { self.1.as_ref().map(|v| v.iter().map(|s| s.as_str())) }
    // pub fn action_ops_iter<'a>(&'a self) -> Option<impl IntoIterator<Item = &'a ActionOpNode>> { self.2.as_ref().map(|v| v.iter()) }

    // pub fn new(event_name: &str, params: Option<EventHandlerParams>, action_ops: Option<EventHandlerActionOps>) -> Self {
    //     EventHandler(event_name.to_owned(), params, action_ops)
    // }

    pub fn create_event(&self, element_key: &str, scope_id: &str) -> EventsItem {
        (element_key.to_owned(), scope_id.to_owned(), self.clone())
    }
}

pub type EventWithData = (EventsItem, Option<PropVec>);
pub type ElementValueBinding = Option<(String, Symbol, Option<Symbol>)>;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum ElementBindingNodeType {
    ElementEventBindingNode(EventHandler),
    ElementValueBindingNode(String, Symbol),
    ElementValueBindingAsNode(String, Symbol, Symbol),
}