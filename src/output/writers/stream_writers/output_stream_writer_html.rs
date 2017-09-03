
use std::io;

use parser::*;
use scope::*;
use processing::*;
use output::*;


pub trait ElementOpsStreamWriterStatic : ElementOpsStreamWriter {}

#[derive(Debug, Clone, Default)]
pub struct ElementOpsStreamWriterHtml {}

impl ElementOpsStreamWriter for ElementOpsStreamWriterHtml {
    type E = ExpressionWriterHtml;

    fn write_op_element_open<PropIter, EventIter, BindingIter>(&mut self, w: &mut io::Write, _expression_writer: &mut Self::E, _value_writer: &mut <Self::E as ExpressionWriter>::V, ctx: &mut Context, _bindings: &BindingContext, element_tag: &str, element_key: &str, is_void: bool, props: PropIter, _events: EventIter, _binding: BindingIter) -> Result
        where PropIter : IntoIterator<Item = Prop>, EventIter: IntoIterator<Item = EventHandler>, BindingIter: IntoIterator<Item = ElementValueBinding>
    {
        let complete_key = ctx.join_path_with(Some("."), element_key);
        write!(w, "<{} key=\"{}\"", element_tag, complete_key)?;

        for (key, expr) in props {
            if let Some(ref _expr) = expr {
                write!(w, " {}=", key)?;
                // self.write_element_attribute_expr_value(w, key, expr, doc, scope)?;
            }
        }

        if is_void {
            write!(w, " />")?;
        } else {
            write!(w, ">")?;
        };

        // self.keys_vec.push(complete_key.to_owned());
        Ok(())
    }

    fn write_op_element_close(&mut self, w: &mut io::Write, _expression_writer: &mut Self::E, _value_writer: &mut <Self::E as ExpressionWriter>::V, _ctx: &mut Context, _bindings: &BindingContext, element_tag: &str) -> Result {
        write!(w, "</{}>", element_tag)?;
        Ok(())
    }

    fn write_op_element_start_block<PropIter: IntoIterator<Item = Prop>>(&mut self, _w: &mut io::Write, _expression_writer: &mut Self::E, _value_writer: &mut <Self::E as ExpressionWriter>::V, _ctx: &mut Context, _bindings: &BindingContext, _block_id: &str, _props: PropIter) -> Result {
        Ok(())
    }

    fn write_op_element_end_block(&mut self, _w: &mut io::Write, _expression_writer: &mut Self::E, _value_writer: &mut <Self::E as ExpressionWriter>::V, _ctx: &mut Context, _bindings: &BindingContext, _block_id: &str) -> Result {
        Ok(())
    }

    fn write_op_element_map_collection_to_block(&mut self, _w: &mut io::Write, _expression_writer: &mut Self::E, _value_writer: &mut <Self::E as ExpressionWriter>::V, _ctx: &mut Context, _bindings: &BindingContext, _coll_expr: &ExprValue, _block_id: &str) -> Result {
        Ok(())
    }

    fn write_op_element_instance_component<PropIter, EventIter, BindingIter>(&mut self, _w: &mut io::Write, _expression_writer: &mut Self::E, _value_writer: &mut <Self::E as ExpressionWriter>::V, _ctx: &mut Context, _bindings: &BindingContext, _element_tag: &str, _element_key: &str, _is_void: bool, _props: PropIter, _events: EventIter, _binding: BindingIter) -> Result
        where PropIter : IntoIterator<Item = Prop>, EventIter: IntoIterator<Item = EventHandler>, BindingIter: IntoIterator<Item = ElementValueBinding>
    {
        Ok(())
    }

    fn write_op_element_value(&mut self, w: &mut io::Write, expression_writer: &mut Self::E, value_writer: &mut <Self::E as ExpressionWriter>::V, ctx: &mut Context, bindings: &BindingContext, expr: &ExprValue, _element_key: &str) -> Result {
        expression_writer.write_expr_to(w, value_writer, ctx, bindings, expr)?;
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
    pub fn test_output_stream_writers_html_ops1() {
        let mut ctx = Context::default();
        ctx.append_path_str("prefix");
        let bindings = BindingContext::default();

        // let mut writer: DefaultOutputWriter<ValueWriterHtml, ExpressionWriterHtml, ElementOpsStreamWriterHtml> = DefaultOutputWriter::default();
        let mut value_writer = ValueWriterHtml::default();
        let mut expr_writer = ExpressionWriterHtml::default();
        let mut stream_writer = ElementOpsStreamWriterHtml::default();

        let mut s: Vec<u8> = Default::default();
        let key = "key".to_owned();
        assert!(
            stream_writer.write_op_element_open(&mut s, &mut expr_writer, &mut value_writer, &mut ctx, &bindings, "span", &key, false, empty(), empty(), empty()).is_ok() &&
            stream_writer.write_op_element_close(&mut s, &mut expr_writer, &mut value_writer, &mut ctx, &bindings, "span").is_ok()
        );
        assert_eq!(str::from_utf8(&s), Ok(indoc![r#"
        <span key="prefix.key"></span>"#
        ]));
            
            // "IncrementalDOM.elementOpen(\"span\", [\"prefix\", \"key\"].join(\".\"));\nIncrementalDOM.elementClose(\"span\");\n".into()));
    }
}