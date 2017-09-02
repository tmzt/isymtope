
use std::io;

use parser::ast::*;
use processing::structs::*;
use scope::context::*;
use scope::bindings::*;
use output::stream_writers::output_writer::*;
use output::stream_writers::output_stream_writer::*;
use output::stream_writers::output_writer_js::*;


#[derive(Debug, Clone, Default)]
pub struct ElementOpsStreamWriterJs {}

impl<X: ExprWriter<E = ExpressionWriterJs>> ElementOpsStreamWriter for X {
    fn write_op_element_open<PropIter, EventIter, BindingIter>(&mut self, w: &mut io::Write, ctx: &mut Context, bindings: &BindingContext, element_tag: &str, element_key: &str, is_void: bool, props: PropIter, events: EventIter, binding: BindingIter) -> Result
        where PropIter : IntoIterator<Item = Prop>, EventIter: IntoIterator<Item = EventHandler>, BindingIter: IntoIterator<Item = ElementValueBinding>
    {
        if !is_void {
            write!(w, "IncrementalDOM.elementOpen(\"{}\", ", element_tag)?;
        } else {
            write!(w, "IncrementalDOM.elementVoid(\"{}\", ", element_tag)?;
        };

        let path_expr = ctx.join_path_as_expr(Some("."));
        self.write_expr(w, ctx, bindings, &path_expr)?;

        // write_js_expr_value(w, scope, path_expr)?;
        // write_js_func_params(scope, w)?;
        writeln!(w, ");")?;

        Ok(())
    }

    fn write_op_element_close(&mut self, w: &mut io::Write, ctx: &mut Context, bindings: &BindingContext, element_tag: &str, element_key: &str) -> Result {
        writeln!(w, "IncrementalDOM.elementClose(\"{}\");", element_tag)?;
        Ok(())
    }

    fn write_op_element_start_block<PropIter: IntoIterator<Item = Prop>>(&mut self, w: &mut io::Write, ctx: &mut Context, bindings: &BindingContext, block_id: &str, props: PropIter) -> Result {
        Ok(())
    }

    fn write_op_element_end_block(&mut self, w: &mut io::Write, ctx: &mut Context, bindings: &BindingContext, block_id: &str) -> Result {
        Ok(())
    }

    fn write_op_element_map_collection_to_block(&mut self, w: &mut io::Write, ctx: &mut Context, bindings: &BindingContext, coll_expr: &ExprValue, block_id: &str) -> Result {
        write!(w, "(")?;
        let binding = BindingType::LoopIndexBinding;
        writeln!(w, ").forEach(__{});", block_id)?;
        Ok(())
    }

    fn write_op_element_instance_component<PropIter, EventIter, BindingIter>(&mut self, w: &mut io::Write, ctx: &mut Context, bindings: &BindingContext, element_tag: &str, element_key: &str, is_void: bool, props: PropIter, events: EventIter, binding: BindingIter) -> Result
        where PropIter : IntoIterator<Item = Prop>, EventIter: IntoIterator<Item = EventHandler>, BindingIter: IntoIterator<Item = ElementValueBinding>
    {
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::str;
    use std::iter::empty;
    use scope::context::*;
    use scope::bindings::*;
    use output::stream_writers::output_writer_js::*;


    #[test]
    pub fn test_output_stream_writers_js_ops1() {
        let mut ctx = Context::default();
        ctx.append_path_str("prefix");
        let bindings = BindingContext::default();

        let mut writer: DefaultOutputWriter<ValueWriterJs, ExpressionWriterJs> = DefaultOutputWriter::default();

        let mut s: Vec<u8> = Default::default();
        let key = "key".to_owned();
        ctx.append_path_str(&key);
        assert!(
            writer.write_op_element_open(&mut s, &mut ctx, &bindings, "span", &key, false, empty(), empty(), empty()).is_ok() &&
            writer.write_op_element_close(&mut s,&mut ctx, &bindings, "span", &key).is_ok()
        );
        assert_eq!(str::from_utf8(&s), Ok("IncrementalDOM.elementOpen(\"span\", [\"prefix\", \"key\"].join(\".\"));\nIncrementalDOM.elementClose(\"span\");\n".into()));
    }
}