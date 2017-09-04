
use std::io;

use parser::*;
use scope::*;
use processing::*;
use output::*;


impl ElementOpsStreamWriter for DefaultOutputWriterHtml {

    fn write_op_element_open<PropIter, EventIter, BindingIter>(&mut self, w: &mut io::Write, ctx: &mut Context, _bindings: &BindingContext, element_tag: &str, element_key: &str, is_void: bool, props: PropIter, _events: EventIter, _binding: BindingIter) -> Result
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

    fn write_op_element_close(&mut self, w: &mut io::Write, __ctx: &mut Context, _bindings: &BindingContext, element_tag: &str) -> Result {
        write!(w, "</{}>", element_tag)?;
        Ok(())
    }

    fn write_op_element_start_block<PropIter: IntoIterator<Item = Prop>>(&mut self, _w: &mut io::Write, __ctx: &mut Context, _bindings: &BindingContext, _block_id: &str, _props: PropIter) -> Result {
        Ok(())
    }

    fn write_op_element_end_block(&mut self, _w: &mut io::Write, __ctx: &mut Context, _bindings: &BindingContext, _block_id: &str) -> Result {
        Ok(())
    }

    fn write_op_element_map_collection_to_block(&mut self, _w: &mut io::Write, __ctx: &mut Context, _bindings: &BindingContext, _coll_expr: &ExprValue, _block_id: &str) -> Result {
        Ok(())
    }

    fn write_op_element_instance_component<PropIter, EventIter, BindingIter>(&mut self, w: &mut io::Write, ctx: &mut Context, _bindings: &BindingContext, _element_tag: &str, element_key: &str, _is_void: bool, _props: PropIter, _events: EventIter, _binding: BindingIter) -> Result
        where PropIter : IntoIterator<Item = Prop>, EventIter: IntoIterator<Item = EventHandler>, BindingIter: IntoIterator<Item = ElementValueBinding>
    {
        let instance_key = ctx.join_path_with(Some("_"), element_key);

        write!(w, "<div key=\"{}\">", instance_key)?;
        write!(w, "</div>")?;

        // write!(w, "component_{}(", element_tag)?;
        // expression_writer.write_expr_to(w, value_writer, ctx, bindings, &instance_key)?;
        // write!(w, ", {{")?;
        // writeln!(w, "}});")?;
        Ok(())
    }

    fn write_op_element_value(&mut self, w: &mut io::Write, ctx: &mut Context, bindings: &BindingContext, expr: &ExprValue, _element_key: &str) -> Result {
        self.write_expr(w, ctx, bindings, expr)?;
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

        let mut writer = DefaultOutputWriterHtml::default();

        let mut s: Vec<u8> = Default::default();
        let key = "key".to_owned();
        assert!(
            writer.write_op_element_open(&mut s, &mut ctx, &bindings, "span", &key, false, empty(), empty(), empty()).is_ok() &&
            writer.write_op_element_close(&mut s, &mut ctx, &bindings, "span").is_ok()
        );
        assert_eq!(str::from_utf8(&s), Ok(indoc![r#"
        <span key="prefix.key"></span>"#
        ]));
            
            // "IncrementalDOM.elementOpen(\"span\", [\"prefix\", \"key\"].join(\".\"));\nIncrementalDOM.elementClose(\"span\");\n".into()));
    }
}