
use std::io;
use std::clone::Clone;
use std::borrow::Borrow;
use std::slice::Iter;
use std::marker::PhantomData;
use std::collections::hash_map::HashMap;

use linked_hash_map::LinkedHashMap;

use parser::ast::*;
use parser::util::allocate_element_key;
use parser::store::*;
use processing::structs::*;
use output::scope::*;
use output::client_misc::*;
use output::client_output::*;
use output::client_ops_writer::*;


pub trait ElementOpsStreamWriter<'input> {
    fn write_op_element(&mut self, w: &mut io::Write, op: &'input ElementOp, doc: &DocumentState, scope_prefixes: &ScopePrefixes, element_key: &str, element_tag: &'input str, is_void: bool, attrs: Option<Iter<'input, Prop>>, events: Option<Iter<EventHandler>>) -> Result;
    fn write_op_element_close(&mut self, w: &mut io::Write, op: &'input ElementOp, doc: &DocumentState, scope_prefixes: &ScopePrefixes, element_tag: &'input str) -> Result;
    fn write_op_element_value(&mut self, w: &mut io::Write, op: &'input ElementOp, doc: &DocumentState, scope_prefixes: &ScopePrefixes, expr: &ExprValue, value_key: &str) -> Result;
    fn write_op_element_start_block(&mut self, w: &mut io::Write, op: &'input ElementOp, doc: &DocumentState, scope_prefixes: &ScopePrefixes, block_id: &str) -> Result;
    fn write_op_element_end_block(&mut self, w: &mut io::Write, op: &'input ElementOp, doc: &DocumentState, scope_prefixes: &ScopePrefixes, block_id: &str) -> Result;
    fn write_op_element_map_collection_to_block(&mut self, w: &mut io::Write, op: &'input ElementOp, doc: &DocumentState, scope_prefixes: &ScopePrefixes, coll_expr: &'input ExprValue, block_id: &str) -> Result;
    fn write_op_element_instance_component_open(&mut self, w: &mut io::Write, op: &'input ElementOp, doc: &DocumentState, scope_prefixes: &ScopePrefixes, comp: &'input Component<'input>, component_key: &str, component_id: &str, attrs: Option<Iter<'input, Prop>>, lens: Option<&str>) -> Result;
    fn write_op_element_instance_component_close(&mut self, w: &mut io::Write, op: &'input ElementOp, doc: &DocumentState, scope_prefixes: &ScopePrefixes, comp: &'input Component<'input>, component_key: &str, component_id: &str) -> Result;
}

// impl<'input: 'scope, 'scope, T, S> WriteOpsContent for S where S: ElementOpsWriter<'input, Output = T> where T: ContentWriter {
