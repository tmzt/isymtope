use std::marker::PhantomData;
use super::*;

#[allow(dead_code)]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum TemplateNode<T> {
    UseStmt(String),
    ComponentDefinition(ComponentDefinition<T>, PhantomData<T>),
    ExternComponentDefinition(ExternComponentDefinition<T>, PhantomData<T>),
    RouteDefinition(RouteDefinition<T>, PhantomData<T>),
    StoreDefinition(StoreDefinition<T>, PhantomData<T>),
    QueryDefinition(QueryDefinition<T>, PhantomData<T>),
    Content(ContentNode<T>, PhantomData<T>),
}
