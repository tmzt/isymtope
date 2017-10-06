// #![allow(dead_code)]

use std::collections::HashMap;
use parser::*;
use model::*;


#[derive(Debug, Default)]
pub struct Template {
    pub children: Vec<Loc<NodeType, (usize, usize)>>,
}

impl Template {
    #[allow(dead_code)]
    pub fn new(children: Vec<Loc<NodeType, (usize, usize)>>) -> Template {
        Template { children: children }
    }
}

#[allow(dead_code)]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum ElementExpr {
    Element(String, Option<String>, Option<Vec<Box<ExprValue>>>),
    Value(ExprValue),
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum LensExprType {
    ForLens(Option<String>, ExprValue),
    GetLens(String, ExprValue),
    QueryLens(ExprValue, String)
}

impl LensExprType {
    pub fn expr(&self) -> Option<&ExprValue> {
        match *self {
            LensExprType::GetLens(_, ref expr) |
            LensExprType::ForLens(_, ref expr) |
            LensExprType::QueryLens(ref expr, _) => Some(expr),
            _ => None
        }
    }

    pub fn item_key(&self) -> Option<&str> {
        match *self {
            LensExprType::GetLens(ref key, _) |
            LensExprType::ForLens(Some(ref key), _) |
            LensExprType::QueryLens(_, ref key) => Some(key),
            _ => None
        }
    }
}

pub type PropKey = String;
pub type Prop = (String, Option<ExprValue>);
pub type PropRef<'a> = (&'a str, Option<&'a ExprValue>);

pub type PropVec = Vec<Prop>;
pub type PropMap = HashMap<String, Option<ExprValue>>;

pub type PropType = (String, Option<VarType>);
pub type PropTypeVec = Vec<PropType>;
pub type PropTypeMap = HashMap<String, Option<VarType>>;

pub type FormalProp = (String);
pub type FormalPropVec = Vec<FormalProp>;

pub type FormalPropRef<'a> = (&'a str);

pub type ActualPropRef<'a> = (&'a str, Option<&'a ExprValue>);

// pub trait PropRefs {
//     fn formal_refs<'a>(&'a self) -> impl IntoIterator<Item = FormalPropRef<'a>>;
//     fn actual_refs<'a>(&'a self) -> impl IntoIterator<Item = ActualPropRef<'a>>;
// }

// impl PropRefs for PropVec {
//     fn formal_refs<'a>(&'a self) -> impl IntoIterator<Item = FormalPropRef<'a>> {
//         self.iter().map(|p| (&p.0, p.1.map(|s| &s)))
//     }

//     fn actual_refs<'a>(&'a self) -> impl IntoIterator<Item = ActualPropRef<'a>> {
//         self.iter().map(|p| (&p.0, p.1.map(|s| &s)))
//     }
// }

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct ElementType {
    pub element_ty: String,
    pub element_key: String,
    pub attrs: Option<PropVec>,
    pub lens: Option<LensExprType>,
    pub children: Option<Vec<ContentNodeType>>,
    pub bindings: Option<Vec<ElementBindingNodeType>>,
}
