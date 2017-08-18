#![allow(dead_code)]

use parser::api::ApiNodeType;
use parser::ast::ExprValue;

// Scope: store

#[derive(Debug, Clone)]
pub enum ActionStateExprType {
    SimpleReducerKeyExpr(ExprValue),
}

#[derive(Debug, Clone)]
pub enum DefaultScopeNodeType {
    LetNode(String, Option<ExprValue>),
    ApiRootNode(String, Option<Vec<ApiNodeType>>),
    ScopeNode(String, Vec<ScopeNodeType>),
}

#[derive(Debug, Clone)]
pub enum ScopeNodeType {
    LetNode(String, Option<ExprValue>),
    ActionNode(String, Option<ActionStateExprType>, Option<Vec<String>>),
    ApiNode(Vec<ApiNodeType>),
    ScopeNode(String, Vec<ScopeNodeType>),
}
