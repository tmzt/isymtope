
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
use processing::scope::*;
use output::scope::*;
use output::client_misc::*;
use output::client_output::*;
use output::client_ops_writer::*;


pub trait ElementOpsStreamWriter {
    fn write_op_element(&mut self, w: &mut io::Write, op: &ElementOp, doc: &DocumentState, scope: &ElementOpScope, element_key: &str, element_tag: &str, is_void: bool, attrs: Option<Iter<Prop>>, events: Option<Iter<EventHandler>>, value_binding: ElementValueBinding) -> Result;
    fn write_op_element_close(&mut self, w: &mut io::Write, op: &ElementOp, doc: &DocumentState, scope: &ElementOpScope, element_tag: &str) -> Result;
    fn write_op_element_value(&mut self, w: &mut io::Write, op: &ElementOp, doc: &DocumentState, scope: &ElementOpScope, expr: &ExprValue, value_key: &str) -> Result;
    fn write_op_element_start_block(&mut self, w: &mut io::Write, op: &ElementOp, doc: &DocumentState, scope: &ElementOpScope, block_id: &str) -> Result;
    fn write_op_element_end_block(&mut self, w: &mut io::Write, op: &ElementOp, doc: &DocumentState, scope: &ElementOpScope, block_id: &str) -> Result;
    fn write_op_element_map_collection_to_block(&mut self, w: &mut io::Write, op: &ElementOp, doc: &DocumentState, scope: &ElementOpScope, coll_expr: &ExprValue, block_id: &str) -> Result;
    fn write_op_element_instance_component_open(&mut self, w: &mut io::Write, op: &ElementOp, doc: &DocumentState, scope: &ElementOpScope, comp: &Component, attrs: Option<Iter<Prop>>, lens: Option<&LensExprType>) -> Result;
    fn write_op_element_instance_component_close(&mut self, w: &mut io::Write, op: &ElementOp, doc: &DocumentState, scope: &ElementOpScope, comp: &Component) -> Result;
}

// impl<'input: 'scope, 'scope, T, S> WriteOpsContent for S where S: ElementOpsWriter<'input, Output = T> where T: ContentWriter {
