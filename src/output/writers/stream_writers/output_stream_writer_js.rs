
use std::io;

use parser::*;
use scope::*;
use processing::*;
use output::*;


// #[derive(Debug, Clone, Default)]
// pub struct ElementOpsStreamWriterJs {}

// impl ElementOpsStreamWriter for ElementOpsStreamWriterJs {
// impl ElementOpsStreamWriter for OutputWriter<E = ExpressionWriterJs> {
impl ElementOpsStreamWriter for DefaultOutputWriterJs {
    type O = Self;
    // type S = Self;
    // type O = DefaultOutputWriterJs;
    // type E = ExpressionWriterJs;

    fn write_op_element_open<PropIter, EventIter, BindingIter>(&mut self, w: &mut io::Write, ctx: &mut Context, bindings: &BindingContext, element_tag: &str, element_key: &str, is_void: bool, _props: PropIter, _events: EventIter, _binding: BindingIter) -> Result
        where PropIter : IntoIterator<Item = Prop>, EventIter: IntoIterator<Item = EventHandler>, BindingIter: IntoIterator<Item = ElementValueBinding>
    {
        if !is_void {
            write!(w, "IncrementalDOM.elementOpen(\"{}\", ", element_tag)?;
        } else {
            write!(w, "IncrementalDOM.elementVoid(\"{}\", ", element_tag)?;
        };

        let path_expr = ctx.join_path_as_expr_with(Some("."), element_key);
        self.write_expr(w, ctx, bindings, &path_expr)?;
        // write_js_func_params(scope, w)?;
        writeln!(w, ");")?;

        Ok(())
    }

    fn write_op_element_close(&mut self, w: &mut io::Write, __ctx: &mut Context, _bindings: &BindingContext, element_tag: &str) -> Result {
        writeln!(w, "IncrementalDOM.elementClose(\"{}\");", element_tag)?;
        Ok(())
    }

    fn write_op_element_start_block<PropIter: IntoIterator<Item = Prop>>(&mut self, _w: &mut io::Write, __ctx: &mut Context, _bindings: &BindingContext, _block_id: &str, _props: PropIter) -> Result {
        Ok(())
    }

    fn write_op_element_end_block(&mut self, _w: &mut io::Write, __ctx: &mut Context, _bindings: &BindingContext, _block_id: &str) -> Result {
        Ok(())
    }

    fn write_op_element_map_collection_to_block(&mut self, w: &mut io::Write, __ctx: &mut Context, _bindings: &BindingContext, _coll_expr: &ExprValue, block_id: &str) -> Result {
        write!(w, "(")?;
        // let binding = BindingType::LoopIndexBinding;
        writeln!(w, ").forEach(__{});", block_id)?;
        Ok(())
    }

    fn write_op_element_instance_component<PropIter, EventIter, BindingIter>(&mut self, w: &mut io::Write, ctx: &mut Context, bindings: &BindingContext, element_tag: &str, element_key: &str, _is_void: bool, _props: PropIter, _events: EventIter, _binding: BindingIter) -> Result
        where PropIter : IntoIterator<Item = Prop>, EventIter: IntoIterator<Item = EventHandler>, BindingIter: IntoIterator<Item = ElementValueBinding>
    {
        let instance_key = ctx.join_path_as_expr_with(Some("_"), element_key);

        write!(w, "component_{}(", element_tag)?;
        self.write_expr(w, ctx, bindings, &instance_key)?;
        write!(w, ", {{")?;
        writeln!(w, "}});")?;
        Ok(())
    }

    fn write_op_element_value(&mut self, w: &mut io::Write, ctx: &mut Context, bindings: &BindingContext, expr: &ExprValue, _element_key: &str) -> Result {
        write!(w, "IncrementalDOM.text(")?;
        self.write_expr(w, ctx, bindings, expr)?;
        writeln!(w, ");")?;
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
    use output::writers::*;


    #[test]
    pub fn test_output_stream_writers_js_ops1() {
        let mut ctx = Context::default();
        ctx.append_path_str("prefix");
        let bindings = BindingContext::default();

        let mut value_writer = ValueWriterJs::default();
        let mut expr_writer = ExpressionWriterJs::default();
        let mut stream_writer = ElementOpsStreamWriterJs::default();

        let mut s: Vec<u8> = Default::default();
        let key = "key".to_owned();
        assert!(
            stream_writer.write_op_element_open(&mut s, &mut expr_writer, &mut value_writer, &mut ctx, &bindings, "span", &key, false, empty(), empty(), empty()).is_ok() &&
            stream_writer.write_op_element_close(&mut s, &mut expr_writer, &mut value_writer, &mut ctx, &bindings, "span").is_ok()
        );
        // assert_eq!(str::from_utf8(&s), Ok("IncrementalDOM.elementOpen(\"span\", [\"prefix\", \"key\"].join(\".\"));\nIncrementalDOM.elementClose(\"span\");\n".into()));
        assert_eq!(str::from_utf8(&s), Ok(indoc![r#"
            IncrementalDOM.elementOpen("span", ["prefix", "key"].join("."));
            IncrementalDOM.elementClose("span");
        "#]));
    }
}