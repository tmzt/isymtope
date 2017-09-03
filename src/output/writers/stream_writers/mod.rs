pub mod output_stream_writer_html;
pub mod output_stream_writer_js;

pub use self::output_stream_writer_html::ElementOpsStreamWriterHtml;
pub use self::output_stream_writer_js::ElementOpsStreamWriterJs;

use std::io;

use parser::*;
use scope::*;
use processing::*;
use output::writers::*;


pub trait ElementOpsStreamWriter {
    type E: ExpressionWriter;

    fn write_op_element_open<PropIter, EventIter, BindingIter>(&mut self, w: &mut io::Write, expression_writer: &mut Self::E, value_writer: &mut <Self::E as ExpressionWriter>::V, ctx: &mut Context, bindings: &BindingContext, element_tag: &str, element_key: &str, is_void: bool, props: PropIter, events: EventIter, binding: BindingIter) -> Result
        where PropIter : IntoIterator<Item = Prop>, EventIter: IntoIterator<Item = EventHandler>, BindingIter: IntoIterator<Item = ElementValueBinding>;
    fn write_op_element_close(&mut self, w: &mut io::Write, expression_writer: &mut Self::E, value_writer: &mut <Self::E as ExpressionWriter>::V, ctx: &mut Context, bindings: &BindingContext, element_tag: &str) -> Result;
    fn write_op_element_start_block<PropIter: IntoIterator<Item = Prop>>(&mut self, w: &mut io::Write, expression_writer: &mut Self::E, value_writer: &mut <Self::E as ExpressionWriter>::V, ctx: &mut Context, bindings: &BindingContext, block_id: &str, props: PropIter) -> Result;
    fn write_op_element_end_block(&mut self, w: &mut io::Write, expression_writer: &mut Self::E, value_writer: &mut <Self::E as ExpressionWriter>::V, ctx: &mut Context, bindings: &BindingContext, block_id: &str) -> Result;
    fn write_op_element_map_collection_to_block(&mut self, w: &mut io::Write, expression_writer: &mut Self::E, value_writer: &mut <Self::E as ExpressionWriter>::V, ctx: &mut Context, bindings: &BindingContext, coll_expr: &ExprValue, block_id: &str) -> Result;
    fn write_op_element_instance_component<PropIter, EventIter, BindingIter>(&mut self, w: &mut io::Write, expression_writer: &mut Self::E, value_writer: &mut <Self::E as ExpressionWriter>::V, ctx: &mut Context, bindings: &BindingContext, element_tag: &str, element_key: &str, is_void: bool, props: PropIter, events: EventIter, binding: BindingIter) -> Result
        where PropIter : IntoIterator<Item = Prop>, EventIter: IntoIterator<Item = EventHandler>, BindingIter: IntoIterator<Item = ElementValueBinding>;
    fn write_op_element_value(&mut self, w: &mut io::Write, expression_writer: &mut Self::E, value_writer: &mut <Self::E as ExpressionWriter>::V, ctx: &mut Context, bindings: &BindingContext, expr: &ExprValue, element_key: &str) -> Result;
}
