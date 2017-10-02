
use std::io;
use std::fmt;
use std::error::Error;
use std::result;
use std::collections::hash_map::{HashMap, Entry};

use linked_hash_map::LinkedHashMap;

use model::*;
use parser::*;
use processing::*;


#[derive(Debug, Clone, PartialEq)]
pub struct BoundEvent {
    instance_key: String,
    element_key: String,
    handler: EventHandler,
    props: Option<Vec<Prop>>
}

impl BoundEvent {
    pub fn bind<'a, I: IntoIterator<Item = &'a PropRef<'a>>>(instance_key: &str, event: &EventsItem, props: Option<I>) -> Self {
        let instance_key = instance_key.to_owned();
        let element_key = event.0.to_owned();
        let handler = event.2.to_owned();

        let props: Option<PropVec> = props.map(|props| props.into_iter().map(|p| (p.0.to_owned(), p.1.map(|p| p.to_owned()))).collect());

        BoundEvent {
            instance_key: instance_key,
            element_key: element_key,
            handler: handler,
            props: props
        }
    }

    pub fn instance_key(&self) -> &str {
        self.instance_key.as_str()
    }

    pub fn element_key(&self) -> &str {
        self.element_key.as_str()
    }

    pub fn complete_key(&self) -> String {
        format!("{}.{}", self.instance_key(), self.element_key())
    }

    pub fn props<'a>(&'a self) -> Option<impl IntoIterator<Item = &'a Prop>> {
        self.props.as_ref().map(|props| props.into_iter())
    }

    pub fn event_item(&self) -> EventsItem {
        (self.element_key.to_owned(), "".into(), self.handler.to_owned())
    }
}

#[derive(Debug, Default, PartialEq)]
pub struct BlockEvents {
    events: EventsWithData
}

impl BlockEvents {
    #[allow(dead_code)]
    pub fn event<'a, I: IntoIterator<Item = &'a PropRef<'a>>>(&mut self, event: &EventsItem, props: I) -> Result {
        Ok(())
    }

}
