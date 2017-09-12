
use std::io;

use parser::*;
use scope::*;
use processing::*;
use output::*;


impl ElementOpsStreamWriter for DefaultOutputWriterJs {

    fn write_op_element_open<'a, PropIter, EventIter, BindingIter>(&mut self, w: &mut io::Write, doc: &Document, ctx: &mut Context, bindings: &BindingContext, element_tag: &str, element_key: &str, is_void: bool, props: PropIter, events: EventIter, binding: BindingIter) -> Result
      where PropIter : IntoIterator<Item = ActualPropRef<'a>>, EventIter: IntoIterator<Item = &'a EventHandler>, BindingIter: IntoIterator<Item = &'a ElementValueBinding>
    {
        if !is_void {
            write!(w, "IncrementalDOM.elementOpen(\"{}\", ", element_tag)?;
        } else {
            write!(w, "IncrementalDOM.elementVoid(\"{}\", ", element_tag)?;
        };

        let path_expr = ctx.join_path_as_expr_with(Some("."), element_key);
        self.write_expr(w, doc, ctx, bindings, &path_expr)?;
        write!(w, ", [")?;

        let mut props: PropVec = props.into_iter().map(|p| (p.0.to_owned(), p.1.map(|s| s.to_owned()))).collect();
        props.insert(0, ("key".into(), Some(path_expr.clone())));

        let mut first_item = true;
        for ref prop in props.iter() {
            if let Some(ref expr) = prop.1 {
                if !first_item { write!(w, ", ")?; }
                first_item = false;
                write!(w, "\"{}\", ", &prop.0)?;
                self.write_expr(w, doc, ctx, bindings, &expr)?;
            };
        }

        writeln!(w, "]);")?;

        // Handle special case
        if element_tag == "input" {
            for value_binding in binding {
                if let &Some((ref key, ref sym)) = value_binding {
                    if let &SymbolReferenceType::InitialValue(box ref initial, box ref element) = sym.sym_ref() {
                        let expr = ExprValue::SymbolReference(element.to_owned());
                        self.write_expr(w, doc, ctx, bindings, &expr)?;
                        write!(w, " = ")?;
                        let expr = ExprValue::SymbolReference(initial.to_owned());
                        self.write_expr(w, doc, ctx, bindings, &expr)?;
                        writeln!(w, ";")?;
                    };
                };
            }
        };

        Ok(())
    }

    fn write_op_element_close(&mut self, w: &mut io::Write, _doc: &Document, ctx: &mut Context, _bindings: &BindingContext, element_tag: &str) -> Result {
        writeln!(w, "IncrementalDOM.elementClose(\"{}\");", element_tag)?;
        Ok(())
    }

    fn write_op_element_start_block<PropIter: IntoIterator<Item = Prop>>(&mut self, _w: &mut io::Write, __doc: &Document, ctx: &mut Context, _bindings: &BindingContext, _block_id: &str, _props: PropIter) -> Result {
        Ok(())
    }

    fn write_op_element_end_block(&mut self, _w: &mut io::Write, __doc: &Document, ctx: &mut Context, _bindings: &BindingContext, _block_id: &str) -> Result {
        Ok(())
    }

    fn write_op_element_map_collection_to_block(&mut self, w: &mut io::Write, __doc: &Document, ctx: &mut Context, _bindings: &BindingContext, _coll_expr: &ExprValue, block_id: &str) -> Result {
        write!(w, "(")?;
        // let binding = BindingType::LoopIndexBinding;
        writeln!(w, ").forEach(__{});", block_id)?;
        Ok(())
    }

    fn write_op_element_instance_component<'a, PropIter, EventIter, BindingIter>(&mut self, w: &mut io::Write, doc: &Document, ctx: &mut Context, bindings: &BindingContext, element_tag: &str, element_key: &str, is_void: bool, props: PropIter, events: EventIter, binding: BindingIter) -> Result
      where PropIter : IntoIterator<Item = ActualPropRef<'a>>, EventIter: IntoIterator<Item = &'a EventHandler>, BindingIter: IntoIterator<Item = &'a ElementValueBinding>
    {
        let instance_key = ctx.join_path_as_expr_with(Some("."), element_key);
        self.render_component(w, doc, ctx, bindings, Some("div"), element_tag, InstanceKey::Dynamic(&instance_key), is_void, props, events, binding, None)
    }

    fn write_op_element_value(&mut self, w: &mut io::Write, doc: &Document, ctx: &mut Context, bindings: &BindingContext, expr: &ExprValue, _element_key: &str) -> Result {
        write!(w, "IncrementalDOM.text(")?;
        self.write_expr(w, doc, ctx, bindings, expr)?;
        writeln!(w, ");")?;
        Ok(())
    }
}

impl ElementOpsUtilWriter for DefaultOutputWriterJs {
    fn render_component<'a, PropIter, EventIter, BindingIter>(&mut self, w: &mut io::Write, doc: &Document, ctx: &mut Context, bindings: &BindingContext, enclosing_tag: Option<&str>, component_ty: &str, instance_key: InstanceKey, _is_void: bool, props: PropIter, _events: EventIter, _binding: BindingIter, _lens_item: Option<LensItemType<'a>>) -> Result
      where PropIter : IntoIterator<Item = ActualPropRef<'a>>, EventIter: IntoIterator<Item = &'a EventHandler>, BindingIter: IntoIterator<Item = &'a ElementValueBinding>
    {
        let instance_key = instance_key.as_expr();
        write!(w, "component_{}(", component_ty)?;
        self.write_expr(w, doc, ctx, bindings, &instance_key)?;
        write!(w, ", {{")?;
        let mut first_item = true;
        for ref prop in props.into_iter() {
            if let Some(ref expr) = prop.1 {
                if !first_item { write!(w, ", ")?; }
                first_item = false;
                write!(w, "\"{}\": ", &prop.0)?;
                self.write_expr(w, doc, ctx, bindings, &expr)?;
            };
        }
        writeln!(w, "}}, store);")?;
        Ok(())
    }

    fn write_map_collection_to_component<'a, PropIter, EventIter, BindingIter>(&mut self, w: &mut io::Write, doc: &Document, ctx: &mut Context, bindings: &BindingContext, coll_item_key: &str, coll_expr: &ExprValue, enclosing_tag: Option<&str>, component_ty: &str, instance_key: InstanceKey, props: PropIter, events: EventIter, binding: BindingIter) -> Result
      where PropIter : IntoIterator<Item = ActualPropRef<'a>>, EventIter: IntoIterator<Item = &'a EventHandler>, BindingIter: IntoIterator<Item = &'a ElementValueBinding>
    {
        let instance_key = instance_key.as_static_string();
        let path_expr = ctx.join_path_as_expr_with(Some("."), &instance_key);

        write!(w, "(")?;
        self.write_expr(w, doc, ctx, bindings, coll_expr)?;
        write!(w, ").forEach(function(item, idx) {{ component_{}(", component_ty)?;
        self.write_expr(w, doc, ctx, bindings, &path_expr)?;
        write!(w, " + \".\" + idx, {{")?;

        let mut first_item = true;
        for ref prop in props.into_iter() {
            if let Some(ref expr) = prop.1 {
                if !first_item { write!(w, ", ")?; }
                first_item = false;
                write!(w, "\"{}\": ", &prop.0)?;
                self.write_expr(w, doc, ctx, bindings, &expr)?;
            };
        }
        writeln!(w, "}}); }});")?;
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


    fn create_document<'a>(template: &'a Template) -> Document {
        let mut ctx = Context::default();
        let mut bindings = BindingContext::default();
        let mut processing = ProcessDocument::from_template(&template);
        assert!(processing.process_document(&mut ctx, &mut bindings).is_ok());
        processing.into()
    }

    #[test]
    pub fn test_output_stream_writers_js_ops1() {
        let template = Template::new(vec![]);
        let doc = create_document(&template);

        let mut ctx = Context::default();
        ctx.append_path_str("prefix");
        let bindings = BindingContext::default();

        let mut writer = DefaultOutputWriterJs::default();

        let mut s: Vec<u8> = Default::default();
        let key = "key".to_owned();
        assert!(
            writer.write_op_element_open(&mut s, &doc, &mut ctx, &bindings, "span", &key, false, empty(), empty(), empty()).is_ok() &&
            writer.write_op_element_close(&mut s, &doc, &mut ctx, &bindings, "span").is_ok()
        );
        // assert_eq!(str::from_utf8(&s), Ok("IncrementalDOM.elementOpen(\"span\", [\"prefix\", \"key\"].join(\".\"));\nIncrementalDOM.elementClose(\"span\");\n".into()));
        assert_eq!(str::from_utf8(&s), Ok(indoc![r#"
            IncrementalDOM.elementOpen("span", ["prefix", "key"].join("."));
            IncrementalDOM.elementClose("span");
        "#]));
    }
}