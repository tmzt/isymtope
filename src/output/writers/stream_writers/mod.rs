pub mod output_stream_writer_html;
pub mod output_stream_writer_js;

use std::io;

use parser::*;
use scope::*;
use processing::*;
use output::writers::*;


pub trait ElementOpsStreamWriter {
    fn write_op_element_open<'a, PropIter, EventIter, BindingIter>(&mut self, w: &mut io::Write, ctx: &mut Context, bindings: &BindingContext, element_tag: &str, element_key: &str, is_void: bool, props: PropIter, events: EventIter, binding: BindingIter) -> Result
      where PropIter : IntoIterator<Item = &'a Prop>, EventIter: IntoIterator<Item = &'a EventHandler>, BindingIter: IntoIterator<Item = &'a ElementValueBinding>;
    fn write_op_element_close(&mut self, w: &mut io::Write, ctx: &mut Context, bindings: &BindingContext, element_tag: &str) -> Result;
    fn write_op_element_start_block<PropIter: IntoIterator<Item = Prop>>(&mut self, w: &mut io::Write, ctx: &mut Context, bindings: &BindingContext, block_id: &str, props: PropIter) -> Result;
    fn write_op_element_end_block(&mut self, w: &mut io::Write, ctx: &mut Context, bindings: &BindingContext, block_id: &str) -> Result;
    fn write_op_element_map_collection_to_block(&mut self, w: &mut io::Write, ctx: &mut Context, bindings: &BindingContext, coll_expr: &ExprValue, block_id: &str) -> Result;
    fn write_op_element_instance_component<'a, PropIter, EventIter, BindingIter>(&mut self, w: &mut io::Write, doc: &Document, ctx: &mut Context, bindings: &BindingContext, element_tag: &str, element_key: &str, _is_void: bool, _props: PropIter, _events: EventIter, _binding: BindingIter) -> Result
      where PropIter : IntoIterator<Item = &'a Prop>, EventIter: IntoIterator<Item = &'a EventHandler>, BindingIter: IntoIterator<Item = &'a ElementValueBinding>;
   fn write_op_element_value(&mut self, w: &mut io::Write, ctx: &mut Context, bindings: &BindingContext, expr: &ExprValue, element_key: &str) -> Result;
}

pub trait ElementOpsUtilWriter {
    fn render_component<'a, PropIter, EventIter, BindingIter>(&mut self, w: &mut io::Write, doc: &Document, ctx: &mut Context, bindings: &BindingContext, enclosing_tag: Option<&str>, component_ty: &str, instance_key: &str, is_void: bool, _props: PropIter, _events: EventIter, _binding: BindingIter) -> Result
      where PropIter : IntoIterator<Item = &'a Prop>, EventIter: IntoIterator<Item = &'a EventHandler>, BindingIter: IntoIterator<Item = &'a ElementValueBinding>;

    fn write_map_collection_to_component<'a, PropIter, EventIter, BindingIter>(&mut self, w: &mut io::Write, doc: &Document, ctx: &mut Context, bindings: &BindingContext, coll_item_key: &str, coll_expr: &ExprValue, enclosing_tag: Option<&str>, component_ty: &str, instance_key: &str, props: PropIter, events: EventIter, binding: BindingIter) -> Result
      where PropIter : IntoIterator<Item = &'a Prop>, EventIter: IntoIterator<Item = &'a EventHandler>, BindingIter: IntoIterator<Item = &'a ElementValueBinding>;
}