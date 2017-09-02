
use std::io;

use parser::ast::*;
use processing::structs::*;
use scope::context::*;
use scope::bindings::*;
use output::stream_writers::output_writer::*;
use output::stream_writers::output_stream_writer::*;


#[derive(Debug, Clone, Default)]
pub struct ElementOpsStreamWriterHtml {}

impl ElementOpsStreamWriter for ElementOpsStreamWriterHtml {
    fn write_op_element_open<PropIter, EventIter, BindingIter>(&mut self, w: &mut io::Write, ctx: &mut Context, bindings: &BindingContext, element_tag: &str, element_key: &str, is_void: bool, props: PropIter, events: EventIter, binding: BindingIter) -> Result
        where PropIter : IntoIterator<Item = Prop>, EventIter: IntoIterator<Item = EventHandler>, BindingIter: IntoIterator<Item = ElementValueBinding>
    {
        Ok(())
    }

    fn write_op_element_close(&mut self, w: &mut io::Write, ctx: &mut Context, bindings: &BindingContext, element_tag: &str, element_key: &str) -> Result {
        Ok(())
    }

    fn write_op_element_start_block<PropIter: IntoIterator<Item = Prop>>(&mut self, w: &mut io::Write, ctx: &mut Context, bindings: &BindingContext, block_id: &str, props: PropIter) -> Result {
        Ok(())
    }

    fn write_op_element_end_block(&mut self, w: &mut io::Write, ctx: &mut Context, bindings: &BindingContext, block_id: &str) -> Result {
        Ok(())
    }

    fn write_op_element_map_collection_to_block(&mut self, w: &mut io::Write, ctx: &mut Context, bindings: &BindingContext, coll_expr: &ExprValue, block_id: &str) -> Result {
        Ok(())
    }

    fn write_op_element_instance_component<PropIter, EventIter, BindingIter>(&mut self, w: &mut io::Write, ctx: &mut Context, bindings: &BindingContext, element_tag: &str, element_key: &str, is_void: bool, props: PropIter, events: EventIter, binding: BindingIter) -> Result
        where PropIter : IntoIterator<Item = Prop>, EventIter: IntoIterator<Item = EventHandler>, BindingIter: IntoIterator<Item = ElementValueBinding>
    {
        Ok(())
    }
}
