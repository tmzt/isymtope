#[cfg(test)]
#[macro_use]
pub mod tests;

pub mod output_stream_writer_html;
pub mod output_stream_writer_js;


use std::io;

use model::*;
use parser::*;
use scope::*;
use processing::*;


pub trait ElementOpsStreamWriter {
    fn write_op_element_open<'a, PropIter, EventIter, BindingIter>(&mut self, w: &mut io::Write, doc: &Document, ctx: &mut Context, bindings: &BindingContext, element_tag: &str, element_key: Option<&str>, is_void: bool, props: PropIter, events: EventIter, binding: BindingIter) -> Result
      where PropIter : IntoIterator<Item = ActualPropRef<'a>> + 'a, EventIter: IntoIterator<Item = &'a EventHandler>, BindingIter: IntoIterator<Item = &'a ElementValueBinding>;
    fn write_op_element_close(&mut self, w: &mut io::Write, doc: &Document, ctx: &mut Context, bindings: &BindingContext, element_tag: &str) -> Result;
    fn write_op_element_start_block<PropIter: IntoIterator<Item = Prop>>(&mut self, w: &mut io::Write, doc: &Document, ctx: &mut Context, bindings: &BindingContext, block_id: &str, props: PropIter) -> Result;
    fn write_op_element_end_block(&mut self, w: &mut io::Write, doc: &Document, ctx: &mut Context, bindings: &BindingContext, block_id: &str) -> Result;
    fn write_op_element_map_collection_to_block(&mut self, w: &mut io::Write, doc: &Document, ctx: &mut Context, bindings: &BindingContext, coll_expr: &ExprValue, block_id: &str) -> Result;
    fn write_op_element_instance_component<'a, PropIter, EventIter, BindingIter>(&mut self, w: &mut io::Write, doc: &Document, ctx: &mut Context, bindings: &BindingContext, element_tag: &str, element_key: &str, _is_void: bool, _props: PropIter, _events: EventIter, _binding: BindingIter) -> Result
      where PropIter : IntoIterator<Item = ActualPropRef<'a>>, EventIter: IntoIterator<Item = &'a EventHandler>, BindingIter: IntoIterator<Item = &'a ElementValueBinding>;
   fn write_op_element_value(&mut self, w: &mut io::Write, doc: &Document, ctx: &mut Context, bindings: &BindingContext, expr: &ExprValue, element_key: &str) -> Result;
}

#[derive(Debug, PartialEq)]
pub enum LensItemType<'a> {
  ForLens(&'a str, usize, &'a ExprValue),
  GetLens(&'a str, &'a ExprValue)
}

pub trait ElementOpsUtilWriter {
    fn render_component<'a, 'b, 'c, PropIter, EventIter, BindingIter>(&mut self, w: &mut io::Write, doc: &Document, ctx: &mut Context, bindings: &BindingContext, enclosing_tag: Option<&str>, component_ty: &str, instance_key: InstanceKey, is_void: bool, props: PropIter, _events: EventIter, _binding: BindingIter) -> Result
      where PropIter : IntoIterator<Item = ActualPropRef<'a>>, EventIter: IntoIterator<Item = &'b EventHandler>, BindingIter: IntoIterator<Item = &'c ElementValueBinding>;

    fn write_map_collection_to_component<'a, 'b, 'c, PropIter, EventIter, BindingIter>(&mut self, w: &mut io::Write, doc: &Document, ctx: &mut Context, bindings: &BindingContext, coll_item_key: &str, coll_expr: &ExprValue, enclosing_tag: Option<&str>, component_ty: &str, instance_key: InstanceKey, props: PropIter, events: EventIter, binding: BindingIter) -> Result
      where PropIter : IntoIterator<Item = ActualPropRef<'a>>, EventIter: IntoIterator<Item = &'b EventHandler>, BindingIter: IntoIterator<Item = &'c ElementValueBinding>;

    fn render_component_with_query_results<'a, 'b, 'c, PropIter, EventIter, BindingIter>(&mut self, w: &mut io::Write, doc: &Document, ctx: &mut Context, bindings: &BindingContext, coll_item_key: &str, coll_expr: &ExprValue, enclosing_tag: Option<&str>, component_ty: &str, instance_key: InstanceKey, props: PropIter, events: EventIter, binding: BindingIter) -> Result
      where PropIter : IntoIterator<Item = ActualPropRef<'a>>, EventIter: IntoIterator<Item = &'b EventHandler>, BindingIter: IntoIterator<Item = &'c ElementValueBinding>
    {
        // Alias query results to single prop
        let events = events.into_iter();
        let binding = binding.into_iter();

        // let expr = ctx.eval_expr(doc, coll_expr);

        let props = vec![(coll_item_key, Some(coll_expr))].into_iter();
        self.render_component(w, doc, ctx, bindings, enclosing_tag, component_ty, instance_key, false, props, events, binding)?;

        Ok(())
    }
}

#[allow(dead_code)]
pub fn create_document<'a, F>(template: &'a Template, mut f: F) -> Document
  where F: FnMut(&mut ProcessDocument, &mut Context, &mut BindingContext) -> Result
{
    let mut ctx = Context::default();
    let mut bindings = BindingContext::default();
    let mut processing = ProcessDocument::from_template(&template);
    assert!(processing.process_document(&mut ctx, &mut bindings).is_ok());

    let res = f(&mut processing, &mut ctx, &mut bindings);
    assert!(res.is_ok());

    processing.into()
}