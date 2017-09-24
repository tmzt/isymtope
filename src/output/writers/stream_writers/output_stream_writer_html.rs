
use std::io;
use std::iter;
use std::collections::HashMap;

use parser::*;
use scope::*;
use processing::*;
use output::*;


impl ElementOpsStreamWriter for DefaultOutputWriterHtml {

    fn write_op_element_open<'a, PropIter, EventIter, BindingIter>(&mut self, w: &mut io::Write, doc: &Document, ctx: &mut Context, bindings: &BindingContext, element_tag: &str, element_key: Option<&str>, is_void: bool, props: PropIter, events: EventIter, binding: BindingIter) -> Result
      where PropIter : IntoIterator<Item = ActualPropRef<'a>> + 'a, EventIter: IntoIterator<Item = &'a EventHandler>, BindingIter: IntoIterator<Item = &'a ElementValueBinding>
    {
        let complete_key;
        if let Some(element_key) = element_key {
            complete_key = ctx.join_path_with(Some("."), element_key);
        } else {
            complete_key = ctx.join_path(Some("."));
        }

        write!(w, "<{} key=\"{}\"", element_tag, complete_key)?;

        for (key_ref, expr_ref) in props {
            if let Some(expr_ref) = expr_ref {

                if key_ref == "class" {
                    if let &ExprValue::LiteralObject(ref props) = expr_ref {
                        write!(w, " class=\"")?;
                        let props_iter = props.as_ref().map(|arr| arr.iter());
                        if let Some(props_iter) = props_iter {
                            let mut first = true;
                            for prop in props_iter {
                                if let Some(ref expr) = prop.1 {
                                    let expr = ctx.eval_expr(doc, expr);
                                    let expr = expr.as_ref().or(prop.1.as_ref());
                                    if let Some(&ExprValue::LiteralBool(b)) = expr {
                                        if !first { write!(w, " ")?; }
                                        if b { write!(w, "{}", prop.0)?; }
                                        first = false;
                                    };
                                };
                            }
                        };
                        write!(w, "\"")?;
                        continue;
                    };
                };

                if let &ExprValue::SymbolReference(ref sym) = expr_ref {
                    if sym.is_bool() {
                        if let Some(expr) = ctx.eval_sym_initial(doc, sym, true) {
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

        // if let Some(enclosing_tag) = enclosing_tag {
        //     write!(w, "<{} key=\"{}\" data-comp=\"{}\">", enclosing_tag, &instance_key, component_ty)?;
        // };

        if let Some(comp) = doc.get_component(component_ty) {
            ctx.push_child_scope();
            ctx.append_path_str(&instance_key);

            ctx.add_binding_value(&BindingType::ComponentKeyBinding, ExprValue::LiteralString(instance_key.to_owned()));

            let mut props_hash: HashMap<&'a str, Option<&'a ExprValue>> = props.into_iter().collect();

            match lens_item {
                Some(LensItemType::ForLens(ref item_key, ref index, ref item_expr)) => {
                    props_hash.insert(item_key, Some(item_expr));
                    // ctx.add_value(key_ref, item_expr.to_owned());
                }
                _ => {}
            };

            // Bind formal properties
            // if let Some(iter) = comp.formal_props_iter() {
            //     for key_ref in iter {
            //         if let Some(expr) = props_hash.get(key_ref) {
            //             if let &Some(expr) = expr {
            //                 ctx.add_value(key_ref, expr.to_owned());
            //             };
            //         };
            //     }
            // }

            if let Some(formals) = comp.formal_props_iter() {
                let bound_props: Vec<_> = formals.into_iter()
                    .zip(props_hash.into_iter())
                    .filter_map(|(formal, prop)| {
                        if formal != prop.0 {
                            return None;
                        };

                        Some(prop)
                    }).collect();

                // let event_item = comp.root_block().events_iter()
                //     .and_then(|iter| iter.into_iter()
                //         // .filter(|e| e.0 == instance_key)
                //         .take(1).nth(0));

                let events_iter = comp.root_block().events_iter();

                if let Some(events_iter) = events_iter {
                    for event_item in events_iter {
                        // let complete_key = format!("{}.{}", &instance_key, event_item.0);
                        self.event(&instance_key, event_item, bound_props.iter())?;
                    };
                };

                for prop in bound_props {
                    if let Some(expr) = prop.1 {
                        ctx.add_value(prop.0, expr.to_owned());
                    }
                } 
            };

            // Bind formal properties
            // for prop in props {
            //     //     let binding = BindingType::ComponentPropBinding(formal.to_owned());
            //     //     ctx.add_sym(formal, Symbol::binding(&binding));
            // }

            self.write_block(w, doc, ctx, bindings, comp.root_block(), Some("div"), true)?;

            // if let Some(ops_iter) = comp.root_block().ops_iter() {
            //     self.write_element_ops(w, doc, ctx, bindings, ops_iter)?;
            // }
            ctx.pop_scope();
        };

        // if let Some(enclosing_tag) = enclosing_tag {
        //     write!(w, "</{}>", enclosing_tag)?;
        // };
        Ok(())
    }

    fn write_map_collection_to_component<'a, PropIter, EventIter, BindingIter>(&mut self, w: &mut io::Write, doc: &Document, ctx: &mut Context, bindings: &BindingContext, coll_item_key: &str, coll_expr: &ExprValue, enclosing_tag: Option<&str>, component_ty: &str, instance_key: InstanceKey, props: PropIter, events: EventIter, binding: BindingIter) -> Result
      where PropIter : IntoIterator<Item = ActualPropRef<'a>>, EventIter: IntoIterator<Item = &'a EventHandler>, BindingIter: IntoIterator<Item = &'a ElementValueBinding>
    {
        let props: PropVec = props.into_iter().map(|p| (p.0.to_owned(), p.1.map(|s| s.to_owned()))).collect();
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
                // ctx.append_path_str(&format!("{}", idx));

                let instance_key = format!("{}.{}", instance_key.as_static_string(), idx);
                // let instance_key = InstanceKey::Static(&instance_key);

                // let map_item = Symbol::binding(&BindingType::MapItemBinding).with_value(item.to_owned());
                // ctx.add_sym(coll_item_key, map_item);
                // ctx.add_value(coll_item_key, item.to_owned());

                self.render_component(w, doc, ctx, bindings, enclosing_tag, component_ty, InstanceKey::Static(&instance_key), false, props.iter().map(|p| (p.0.as_ref(), p.1.as_ref().map(|s| s))), events.iter(), binding.iter(), Some(LensItemType::ForLens(coll_item_key, idx, item)))?;

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
            writer.write_op_element_open(&mut s, &doc, &mut ctx, &bindings, "span", Some(&key), false, empty(), empty(), empty()).is_ok() &&
            writer.write_op_element_close(&mut s, &doc, &mut ctx, &bindings, "span").is_ok()
        );
        assert_eq!(str::from_utf8(&s), Ok(indoc![r#"
        <span key="prefix.key"></span>"#
        ]));
            
            // "IncrementalDOM.elementOpen(\"span\", [\"prefix\", \"key\"].join(\".\"));\nIncrementalDOM.elementClose(\"span\");\n".into()));
    }
}