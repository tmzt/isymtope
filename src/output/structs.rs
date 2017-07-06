
use std::collections::hash_map::HashMap;
use parser::ast::*;
use parser::store::*;

#[derive(Debug)]
pub struct ReducerKeyData {
    pub reducer_key: String,
    pub default_expr: Option<ExprValue>,
    pub actions: Option<Vec<ReducerActionData>>
}

impl ReducerKeyData {
    pub fn from_name(reducer_key: &str) -> ReducerKeyData {
        ReducerKeyData {
            reducer_key: String::from(reducer_key),
            default_expr: None,
            actions: Some(Vec::new())
        }
    }
}

#[derive(Debug)]
pub struct ReducerActionData {
    pub action_type: String,
    pub state_expr: Option<ActionStateExprType>
}

impl ReducerActionData {
    pub fn from_name(action_name: &str) -> ReducerActionData {
        let action_type = action_name.to_uppercase();

        ReducerActionData {
            action_type: String::from(action_type),
            state_expr: None
        }
    }
}

#[derive(Debug, Clone)]
pub enum ElementOp {
    ElementOpen(String, Option<String>, Option<Vec<(String, ExprValue)>>, Option<EventHandlersVec>),
    ElementVoid(String, Option<String>, Option<Vec<(String, ExprValue)>>, Option<EventHandlersVec>),
    ElementClose(String),
    WriteValue(ExprValue, Option<String>),
    InstanceComponent(String, Option<String>, Option<Vec<(String, ExprValue)>>)
}

pub type OpsVec = Vec<ElementOp>;
pub type ComponentMap<'inp> = HashMap<&'inp str, Component<'inp>>;

#[derive(Debug, Clone)]
pub struct Component<'input> {
    pub name: &'input str,
    pub ops: Option<OpsVec>,
    pub uses: Option<Vec<&'input str>>,
    pub child_map: Option<ComponentMap<'input>>,
}

/*
pub struct FlattenContent;

impl FlattenContent {
    pub fn iter_content<'input>(&self, content: &'input Iterator<Item = &'input ContentNodeType>) -> Box<Iterator<Item = &'input ElementOp> + 'input> {
        Box::new(&content.flat_map(move |x| {
            if let &ContentNodeType::ElementNode(ref element_data) = x {
                if let Some(ref children) = element_data.children {
                    return self.iter_content(&children.iter())
                }
            };
            Box::new(iter::empty())
        }) as Box<Iterator<Item = &'input ElementOp> + 'input>)
    }
}
*/

/*
#[derive(Debug)]
pub struct ContentIter<I, U: iter::IntoIterator> {
    iter: I,
    src: U::IntoIter
}
*/

/*
impl<I: Iterator, U: IntoIterator> Iterator for ContentIter<I, U> {
    type Item = U::Item;

    #[inline]
    fn next(&mut self) -> Option<U::Item> {
        loop {
            if let &ContentNodeType::ElementNode(ref element_data) = x {
                if let Some(ref children) = element_data.children {
                    return self.iter_content(&children.iter())
                }
            };
        }
    }
}
*/
