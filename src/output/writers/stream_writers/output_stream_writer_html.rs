
use std::io;
use std::collections::HashMap;

use parser::*;
use scope::*;
use processing::*;
use output::*;


impl ElementOpsStreamWriter for DefaultOutputWriterHtml {

    fn write_op_element_open<'a, PropIter, EventIter, BindingIter>(&mut self, w: &mut io::Write, doc: &Document, ctx: &mut Context, bindings: &BindingContext, element_tag: &str, element_key: &str, is_void: bool, props: PropIter, events: EventIter, binding: BindingIter) -> Result
      where PropIter : IntoIterator<Item = ActualPropRef<'a>>, EventIter: IntoIterator<Item = &'a EventHandler>, BindingIter: IntoIterator<Item = &'a ElementValueBinding>
    {
        let complete_key = ctx.join_path_with(Some("."), element_key);
        write!(w, "<{} key=\"{}\"", element_tag, complete_key)?;

        for (key_ref, expr_ref) in props {
            if let Some(expr_ref) = expr_ref {

                if let &ExprValue::SymbolReference(ref sym) = expr_ref {
                    if sym.is_bool() {
                        if let Some(expr) = ctx.eval_sym(doc, sym) {
                            if let ExprValue::LiteralBool(b) = expr {
                                if b { write!(w, " {}=\"{}\"", key_ref, key_ref)?; }
                                continue;
                            };

                        };
                    };
                };

                write!(w, " {}=\"", key_ref)?;
                self.write_expr(w, doc, ctx, bindings, expr_ref)?;
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

    fn write_op_element_close(&mut self, w: &mut io::Write, doc: &Document, _ctx: &mut Context, _bindings: &BindingContext, element_tag: &str) -> Result {
        write!(w, "</{}>", element_tag)?;
        Ok(())
    }

    fn write_op_element_start_block<PropIter: IntoIterator<Item = Prop>>(&mut self, _w: &mut io::Write, doc: &Document, _ctx: &mut Context, _bindings: &BindingContext, _block_id: &str, _props: PropIter) -> Result {
        Ok(())
    }

    fn write_op_element_end_block(&mut self, _w: &mut io::Write, doc: &Document, _ctx: &mut Context, _bindings: &BindingContext, _block_id: &str) -> Result {
        Ok(())
    }

    fn write_op_element_map_collection_to_block(&mut self, _w: &mut io::Write, doc: &Document, _ctx: &mut Context, _bindings: &BindingContext, _coll_expr: &ExprValue, _block_id: &str) -> Result {
        Ok(())
    }

    fn write_op_element_instance_component<'a, PropIter, EventIter, BindingIter>(&mut self, w: &mut io::Write, doc: &Document, ctx: &mut Context, bindings: &BindingContext, element_tag: &str, element_key: &str, is_void: bool, props: PropIter, events: EventIter, binding: BindingIter) -> Result
      where PropIter : IntoIterator<Item = ActualPropRef<'a>>, EventIter: IntoIterator<Item = &'a EventHandler>, BindingIter: IntoIterator<Item = &'a ElementValueBinding>
    {
        let instance_key = ctx.join_path_with(Some("."), element_key);
        self.render_component(w, doc, ctx, bindings, Some("div"), element_tag, InstanceKey::Static(&instance_key), is_void, props, events, binding, None)?;
        Ok(())
    }

    fn write_op_element_value(&mut self, w: &mut io::Write, doc: &Document, ctx: &mut Context, bindings: &BindingContext, expr: &ExprValue, _element_key: &str) -> Result {
        self.write_expr(w, doc, ctx, bindings, expr)?;
        Ok(())
    }
}

impl ElementOpsUtilWriter for DefaultOutputWriterHtml {
    fn render_component<'a, PropIter, EventIter, BindingIter>(&mut self, w: &mut io::Write, doc: &Document, ctx: &mut Context, bindings: &BindingContext, enclosing_tag: Option<&str>, component_ty: &str, instance_key: InstanceKey, is_void: bool, props: PropIter, _events: EventIter, _binding: BindingIter, lens_item: Option<LensItemType<'a>>) -> Result
      where PropIter : IntoIterator<Item = ActualPropRef<'a>>, EventIter: IntoIterator<Item = &'a EventHandler>, BindingIter: IntoIterator<Item = &'a ElementValueBinding>
    {
        let instance_key = instance_key.as_static_string();

        if let Some(enclosing_tag) = enclosing_tag {
            write!(w, "<{} key=\"{}\" data-comp=\"{}\">", enclosing_tag, &instance_key, component_ty)?;
        };

        if let Some(comp) = doc.get_component(component_ty) {
            ctx.push_child_scope();
            ctx.append_path_str(&instance_key);

            let mut props_hash: HashMap<&'a str, Option<&'a ExprValue>> = props.into_iter().collect();

            match lens_item {
                Some(LensItemType::ForLens(ref item_key, ref index, ref item_expr)) => {
                    props_hash.insert(item_key, Some(item_expr));
                    // ctx.add_value(key_ref, item_expr.to_owned());
                }
                _ => {}
            };

            // Bind formal properties
            if let Some(iter) = comp.formal_props_iter() {
                for key_ref in iter {
                    if let Some(expr) = props_hash.get(key_ref) {
                        if let &Some(expr) = expr {
                            ctx.add_value(key_ref, expr.to_owned());
                        };
                    };
                }
            }

            // Bind formal properties
            // for prop in props {
            //     //     let binding = BindingType::ComponentPropBinding(formal.to_owned());
            //     //     ctx.add_sym(formal, Symbol::binding(&binding));
            // }

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
      where PropIter : IntoIterator<Item = ActualPropRef<'a>>, EventIter: IntoIterator<Item = &'a EventHandler>, BindingIter: IntoIterator<Item = &'a ElementValueBinding>
    {
        let mut props: PropVec = props.into_iter().map(|p| (p.0.to_owned(), p.1.map(|s| s.to_owned()))).collect();
        let events: Vec<EventHandler> = events.into_iter().map(|s| s.to_owned()).collect();
        let binding: Vec<ElementValueBinding> = binding.into_iter().map(|s| s.to_owned()).collect();

        // let map_item = ExprValue::Binding(BindingType::MapItemBinding);
        // props.insert(0, (coll_item_key.to_owned(), Some(map_item)));

        // Attempt to resolve coll_expr
        let reduced_expr = ctx.eval_expr(doc, coll_expr);
        let coll_expr = reduced_expr.as_ref().unwrap_or(coll_expr);

        if let &ExprValue::LiteralArray(Some(ref arr)) = coll_expr {
            for (idx, item) in arr.iter().enumerate() {
                ctx.push_child_scope();
                ctx.append_path_str(&format!("{}", idx));

                // let map_item = Symbol::binding(&BindingType::MapItemBinding).with_value(item.to_owned());
                // ctx.add_sym(coll_item_key, map_item);
                // ctx.add_value(coll_item_key, item.to_owned());

                self.render_component(w, doc, ctx, bindings, enclosing_tag, component_ty, instance_key.clone(), false, props.iter().map(|p| (p.0.as_ref(), p.1.as_ref().map(|s| s))), events.iter(), binding.iter(), Some(LensItemType::ForLens(coll_item_key, idx, item)))?;

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


    fn create_document<'a>(template: &'a Template) -> Document {
        let mut ctx = Context::default();
        let mut bindings = BindingContext::default();
        let mut processing = ProcessDocument::from_template(&template);
        assert!(processing.process_document(&mut ctx, &mut bindings).is_ok());
        processing.into()
    }

    #[test]
    pub fn test_output_stream_writers_html_ops1() {
        let template = Template::new(vec![]);
        let doc = create_document(&template);

        let mut ctx = Context::default();
        ctx.append_path_str("prefix");
        let bindings = BindingContext::default();

        let mut writer = DefaultOutputWriterHtml::default();

        let mut s: Vec<u8> = Default::default();
        let key = "key".to_owned();
        assert!(
            writer.write_op_element_open(&mut s, &doc, &mut ctx, &bindings, "span", &key, false, empty(), empty(), empty()).is_ok() &&
            writer.write_op_element_close(&mut s, &doc, &mut ctx, &bindings, "span").is_ok()
        );
        assert_eq!(str::from_utf8(&s), Ok(indoc![r#"
        <span key="prefix.key"></span>"#
        ]));
            
            // "IncrementalDOM.elementOpen(\"span\", [\"prefix\", \"key\"].join(\".\"));\nIncrementalDOM.elementClose(\"span\");\n".into()));
    }
}