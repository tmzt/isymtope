
use std::io;
use std::clone::Clone;
use std::slice::Iter;
use std::borrow::Borrow;
use std::collections::hash_map::HashMap;

use linked_hash_map::LinkedHashMap;

use parser::ast::*;
use parser::util::allocate_element_key;
use parser::store::*;
use super::structs::*;
// use super::client_js::*;
// use super::client_html::*;
use super::client_misc::*;
use super::client_misc_html::*;
use super::client_output::*;
use super::client_ops_writer::*;
use super::structs::*;


pub trait ScopeManagement {
    // fn with_doc(doc: &'input DocumentState<'input>, stream_writer: &'scope ElementOpsStreamWriter) -> Self;

    fn push_scope<F: FnOnce(&ScopeType) -> ScopeType>(&mut self, scope_id: &str, chain_scope: F) -> Result;
    fn pop_scope(&mut self) -> Result;
    fn scope(&self) -> Option<&ScopeType>;

    fn record_event_key(event_key: &str) -> Result;
    fn record_element_key(element_key: &str) -> Result;    
}

#[derive(Debug,Default)]
pub struct Scopes {
    pub scopes_stack: LinkedHashMap<String, ScopeType>
}

impl<'input: 'scope, 'scope> Scopes {
    pub fn new() -> Self { Default::default() }
}

impl<'input: 'scope, 'scope> ScopeManagement for Scopes {
    #[inline]
    fn push_scope<F: FnOnce(&ScopeType) -> ScopeType>(&mut self, scope_id: &str, chain_scope: F) -> Result {

        let cur_scope = self.scopes_stack.back();
        if let Some((&cur_scope_id, &cur_scope)) = cur_scope {
            if let cur_scope = cur_scope.borrow() {
                let next_scope = chain_scope(cur_scope);
                self.scopes_stack.insert(scope_id.to_owned(), next_scope);
            }
        };
        Ok(())
    }

    #[inline]
    #[allow(dead_code)]
    fn pop_scope(&mut self) -> Result {
        self.scopes_stack.pop_back();
        Ok(())
    }

    #[inline]
    #[allow(dead_code)]
    fn scope(&self) -> Option<&ScopeType> {
        if let Some((ref scope_id, scope)) = self.scopes_stack.back() {
            return Some(scope)
        };
        None
    }

    #[inline]
    #[allow(dead_code)]
    fn record_event_key(event_key: &str) -> Result {
        Ok(())
    }

    #[inline]
    #[allow(dead_code)]
    fn record_element_key(element_key: &str) -> Result {
        Ok(())
    }
}
