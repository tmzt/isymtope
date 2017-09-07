
use std::io;

use parser::*;
use scope::*;
use processing::*;
use output::*;


impl ElementOpsStreamWriter for DefaultOutputWriterHtml {

    fn write_op_element_open<'a, PropIter, EventIter, BindingIter>(&mut self, w: &mut io::Write, ctx: &mut Context, bindings: &BindingContext, element_tag: &str, element_key: &str, is_void: bool, props: PropIter, events: EventIter, binding: BindingIter) -> Result
      where PropIter : IntoIterator<Item = &'a Prop>, EventIter: IntoIterator<Item = &'a EventHandler>, BindingIter: IntoIterator<Item = &'a ElementValueBinding>
    {
        let complete_key = ctx.join_path_with(Some("."), element_key);
        write!(w, "<{} key=\"{}\"", element_tag, complete_key)?;

        for &(ref key, ref expr) in props {
            if let &Some(ref expr) = expr {
                write!(w, " {}=\"", key)?;
                self.write_expr(w, ctx, bindings, expr)?;
                write!(w, "\"")?;
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

    fn write_op_element_instance_component<'a, PropIter, EventIter, BindingIter>(&mut self, w: &mut io::Write, doc: &Document, ctx: &mut Context, bindings: &BindingContext, element_tag: &str, element_key: &str, is_void: bool, props: PropIter, events: EventIter, binding: BindingIter) -> Result
      where PropIter : IntoIterator<Item = &'a Prop>, EventIter: IntoIterator<Item = &'a EventHandler>, BindingIter: IntoIterator<Item = &'a ElementValueBinding>
    {
        let instance_key = ctx.join_path_with(Some("."), element_key);
        self.render_component(w, doc, ctx, bindings, Some("div"), element_tag, InstanceKey::Static(&instance_key), is_void, props, events, binding)?;
        Ok(())
    }

    fn write_op_element_value(&mut self, w: &mut io::Write, ctx: &mut Context, bindings: &BindingContext, expr: &ExprValue, _element_key: &str) -> Result {
        self.write_expr(w, ctx, bindings, expr)?;
        Ok(())
    }
}

impl ElementOpsUtilWriter for DefaultOutputWriterHtml {
    fn render_component<'a, PropIter, EventIter, BindingIter>(&mut self, w: &mut io::Write, doc: &Document, ctx: &mut Context, bindings: &BindingContext, enclosing_tag: Option<&str>, component_ty: &str, instance_key: InstanceKey, is_void: bool, _props: PropIter, _events: EventIter, _binding: BindingIter) -> Result
      where PropIter : IntoIterator<Item = &'a Prop>, EventIter: IntoIterator<Item = &'a EventHandler>, BindingIter: IntoIterator<Item = &'a ElementValueBinding>
    {
        let instance_key = instance_key.as_static_string();

        if let Some(enclosing_tag) = enclosing_tag {
            write!(w, "<{} key=\"{}\" data-comp=\"{}\">", enclosing_tag, &instance_key, component_ty)?;
        };

        if let Some(comp) = doc.get_component(component_ty) {
            ctx.push_child_scope();
            ctx.append_path_str(&instance_key);
            if let Some(ops_iter) = comp.root_block().ops_iter() {
                self.write_element_ops(w, doc, ctx, bindings, ops_iter)?;
            }
            ctx.pop_scope();
        };

        if let Some(enclosing_tag) = enclosing_tag {
            write!(w, "</{}>", enclosing_tag)?;
        };
        Ok(())
    }

    fn write_map_collection_to_component<'a, PropIter, EventIter, BindingIter>(&mut self, w: &mut io::Write, doc: &Document, ctx: &mut Context, bindings: &BindingContext, coll_item_key: &str, coll_expr: &ExprValue, enclosing_tag: Option<&str>, component_ty: &str, instance_key: InstanceKey, props: PropIter, events: EventIter, binding: BindingIter) -> Result
      where PropIter : IntoIterator<Item = &'a Prop>, EventIter: IntoIterator<Item = &'a EventHandler>, BindingIter: IntoIterator<Item = &'a ElementValueBinding>
    {
        let mut props: PropVec = props.into_iter().map(|s| s.to_owned()).collect();
        let events: Vec<EventHandler> = events.into_iter().map(|s| s.to_owned()).collect();
        let binding: Vec<ElementValueBinding> = binding.into_iter().map(|s| s.to_owned()).collect();

        let map_item = ExprValue::Binding(BindingType::MapItemBinding);
        props.insert(0, (coll_item_key.to_owned(), Some(map_item)));

        // Attempt to resolve coll_expr
        let reduced_expr = ctx.reduce_expr_and_resolve(doc, coll_expr);
        let coll_expr = reduced_expr.as_ref().unwrap_or(coll_expr);

        if let &ExprValue::LiteralArray(Some(ref arr)) = coll_expr {
            for (idx, item) in arr.iter().enumerate() {
                ctx.push_child_scope();
                ctx.append_path_str(&format!("{}", idx));

                let map_item = Symbol::binding(&BindingType::MapItemBinding).with_value(item.to_owned());
                ctx.add_sym(coll_item_key, map_item);

                self.render_component(w, doc, ctx, bindings, enclosing_tag, component_ty, instance_key.clone(), false, props.iter(), events.iter(), binding.iter())?;

                ctx.pop_scope();
            }
        };
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