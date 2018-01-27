
use std::marker::PhantomData;

use error::*;
use traits::*;
use expressions::*;
use objects::*;
use ast::*;


#[allow(dead_code)]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum ElementAttr<T> {
    ForLens(Option<String>, Box<ExpressionValue<T>>, PhantomData<T>),
    GetLens(Option<String>, Box<ExpressionValue<T>>, PhantomData<T>),
    QueryLens(Option<String>, String, PhantomData<T>),
    ElementPropValue(Box<ElementPropValue<T>>, PhantomData<T>)
}