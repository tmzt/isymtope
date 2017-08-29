
use std::io;
use std::slice::Iter;

use parser::ast::*;
use processing::structs::*;
use processing::scope::*;


pub type PropIterator = IntoIterator<Item = Prop>;
pub type EventHandlerIterator = IntoIterator<Item = EventHandler>;
pub type BindingIterator = IntoIterator<Item = ElementValueBinding>;

pub trait ElementOpsStreamWriter {
    fn write_op_element_open(&mut self, w: &mut io::Writer, scope: &Scope, element_tag: &str, element_key: &str, is_void: bool, props: Option<PropIterator>, events: Option<EventHandlerIterator>, binding: Option<ElementValueBinding>) -> Result;
    fn write_op_element_close(&mut self, w: &mut io::Writer, scope: &Scope, element_tag: &str, element_key: &str, props: Option<PropIterator>, events: Option<EventHandlerIterator>, binding: Option<ElementValueBinding>) -> Result;
    fn write_op_element_start_block(&mut self, w: &mut io::Writer, scope: &Scope, block_id: &str, props: Option<PropIterator>) -> Result;
    fn write_op_element_end_block(&mut self, w: &mut io::Writer, scope: &Scope, block_id: &str) -> Result;
    fn write_op_element_map_collection_to_block(&mut self, w: &mut io::Writer, scope: &Scope, coll_expr: &ExprValue, block_id: &str) -> Result;
    fn write_op_element_instance_component(&mut self, w: &mut io::Writer, scope: &Scope, element_tag: &str, element_key: &str, is_void: bool, props: Option<PropIterator>, events: Option<EventHandlerIterator>, binding: Option<ElementValueBinding>) -> Result;
}
