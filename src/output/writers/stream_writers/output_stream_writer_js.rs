
use std::io;

use parser::ast::*;
use processing::structs::*;
use scope::context::*;
use scope::bindings::*;
use output::writers::*;


// pub trait ElementOpsStreamWriterDynamic : ElementOpsStreamWriter {}

#[derive(Debug, Clone, Default)]
pub struct ElementOpsStreamWriterJs {}

// impl<X: ExprWriter<E = ExpressionWriterJs>> ElementOpsStreamWriter for X {
// impl<E: DynamicExpressionWriter, X> ElementOpsStreamWriter for X where X: ExprWriter<E> {
impl ElementOpsStreamWriter for ElementOpsStreamWriterJs {
    type E = ExpressionWriterJs;

    fn write_op_element_open<PropIter, EventIter, BindingIter>(&mut self, w: &mut io::Write, expression_writer: &mut Self::E, value_writer: &mut <Self::E as ExpressionWriter>::V, ctx: &mut Context, bindings: &BindingContext, element_tag: &str, element_key: &str, is_void: bool, props: PropIter, events: EventIter, binding: BindingIter) -> Result
        where PropIter : IntoIterator<Item = Prop>, EventIter: IntoIterator<Item = EventHandler>, BindingIter: IntoIterator<Item = ElementValueBinding>
    {
        if !is_void {
            write!(w, "IncrementalDOM.elementOpen(\"{}\", ", element_tag)?;
        } else {
            write!(w, "IncrementalDOM.elementVoid(\"{}\", ", element_tag)?;
        };

        let path_expr = ctx.join_path_as_expr_with(Some("."), element_key);
        // let path_expr = ctx.join_path_as_expr(Some("."));
        expression_writer.write_expr_to(w, value_writer, ctx, bindings, &path_expr)?;

        // write_js_expr_value(w, scope, path_expr)?;
        // write_js_func_params(scope, w)?;
        writeln!(w, ");")?;

        Ok(())
    }

    fn write_op_element_close(&mut self, w: &mut io::Write, expression_writer: &mut Self::E, value_writer: &mut <Self::E as ExpressionWriter>::V, ctx: &mut Context, bindings: &BindingContext, element_tag: &str) -> Result {
        writeln!(w, "IncrementalDOM.elementClose(\"{}\");", element_tag)?;
        Ok(())
    }

    fn write_op_element_start_block<PropIter: IntoIterator<Item = Prop>>(&mut self, w: &mut io::Write, expression_writer: &mut Self::E, value_writer: &mut <Self::E as ExpressionWriter>::V, ctx: &mut Context, bindings: &BindingContext, block_id: &str, props: PropIter) -> Result {
        Ok(())
    }

    fn write_op_element_end_block(&mut self, w: &mut io::Write, expression_writer: &mut Self::E, value_writer: &mut <Self::E as ExpressionWriter>::V, ctx: &mut Context, bindings: &BindingContext, block_id: &str) -> Result {
        Ok(())
    }

    fn write_op_element_map_collection_to_block(&mut self, w: &mut io::Write, expression_writer: &mut Self::E, value_writer: &mut <Self::E as ExpressionWriter>::V, ctx: &mut Context, bindings: &BindingContext, coll_expr: &ExprValue, block_id: &str) -> Result {
        write!(w, "(")?;
        let binding = BindingType::LoopIndexBinding;
        writeln!(w, ").forEach(__{});", block_id)?;
        Ok(())
    }

    fn write_op_element_instance_component<PropIter, EventIter, BindingIter>(&mut self, w: &mut io::Write, expression_writer: &mut Self::E, value_writer: &mut <Self::E as ExpressionWriter>::V, ctx: &mut Context, bindings: &BindingContext, element_tag: &str, element_key: &str, is_void: bool, props: PropIter, events: EventIter, binding: BindingIter) -> Result
        where PropIter : IntoIterator<Item = Prop>, EventIter: IntoIterator<Item = EventHandler>, BindingIter: IntoIterator<Item = ElementValueBinding>
    {
        Ok(())
    }

    fn write_op_element_value(&mut self, w: &mut io::Write, expression_writer: &mut Self::E, value_writer: &mut <Self::E as ExpressionWriter>::V, ctx: &mut Context, bindings: &BindingContext, expr: &ExprValue, element_key: &str) -> Result {
        write!(w, "IncrementalDOM.text(")?;
        expression_writer.write_expr_to(w, value_writer, ctx, bindings, expr)?;
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

        // let mut writer: DefaultOutputWriter<ValueWriterJs, ExpressionWriterJs, ElementOpsStreamWriterJs> = DefaultOutputWriter::default();
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