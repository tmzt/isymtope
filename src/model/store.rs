#![allow(dead_code)]

use model::*;
use parser::*;


// Scope: store

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum ActionStateExprType {
    SimpleReducerKeyExpr(ExprValue),
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum DefaultScopeNodeType {
    LetNode(String, Option<ExprValue>),
    ApiRootNode(String, Option<Vec<ApiNodeType>>),
    ScopeNode(String, Vec<ScopeNodeType>),
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum ScopeNodeType {
    LetNode(String, Option<ExprValue>),
    ActionNode(String, Option<ActionStateExprType>, Option<Vec<String>>),
    ApiNode(Vec<ApiNodeType>),
    ScopeNode(String, Vec<ScopeNodeType>),
}