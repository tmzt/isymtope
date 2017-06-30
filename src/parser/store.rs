#![allow(dead_code)]

use parser::api::ApiNodeType;
use parser::ast::ExprValue;

// Scope: store

#[derive(Debug)]
pub enum ScopeNodeType {
    LetNode(String, Option<ExprValue>),
    ActionNode(String),
    ApiNode(Vec<ApiNodeType>),
    ScopeNode(String, Vec<ScopeNodeType>)
}
