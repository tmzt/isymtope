
use std::io;

use parser::*;
use scope::*;
use processing::*;
use output::*;


impl ElementOpsStreamWriter for DefaultOutputWriterJs {

    fn write_op_element_open<'a, PropIter, EventIter, BindingIter>(&mut self, w: &mut io::Write, doc: &Document, ctx: &mut Context, bindings: &BindingContext, element_tag: &str, element_key: Option<&str>, is_void: bool, props: PropIter, events: EventIter, binding: BindingIter) -> Result
      where PropIter : IntoIterator<Item = ActualPropRef<'a>> + 'a, EventIter: IntoIterator<Item = &'a EventHandler>, BindingIter: IntoIterator<Item = &'a ElementValueBinding>
    {
        if !is_void {
            write!(w, "IncrementalDOM.elementOpen(\"{}\", ", element_tag)?;
        } else {
            write!(w, "IncrementalDOM.elementVoid(\"{}\", ", element_tag)?;
        };

        let path_expr;
        if let Some(element_key) = element_key {
            path_expr = ctx.join_path_as_expr_with(Some("."), element_key);
        } else {
            path_expr = ctx.join_path_as_expr(Some("."));
        }

        let static_props: Vec<_>;
        let dynamic_props: Vec<_>;
        let class_prop;

        {
            let props: PropVec = props.into_iter().map(|p| (p.0.to_owned(), p.1.map(|s| s.to_owned()))).collect();
            let split_props: Vec<_> = ctx.map_props_to_reduced_values(props.iter()).collect();

            static_props = split_props.iter()
                .filter_map(|&(ref key, ref reduced)| match *reduced { Some(ReducedValue::Static(StaticValue::StaticString(ref s))) => Some((key.to_owned(), s.to_owned())), _ => None })
                .collect();

            // Excluding class if it's dynamic
            dynamic_props = split_props.iter()
                .filter_map(|&(ref key, ref reduced)| match *reduced { Some(ReducedValue::Dynamic(ref expr)) if key != "class" => Some((key.to_owned(), expr.to_owned())), _ => None })
                .collect();

            class_prop = split_props.iter()
                .filter_map(|&(ref key, ref reduced)| match *reduced { Some(ReducedValue::Dynamic(ref expr)) if key == "class" => Some(expr.to_owned()), _ => None })
                .nth(0);
        }

        // Static properties with key
        // let mut first = true;

        self.write_expr(w, doc, ctx, bindings, &path_expr)?;
        write!(w, ", [\"key\", ")?;
        self.write_expr(w, doc, ctx, bindings, &path_expr)?;

        for &(ref a, ref b) in &static_props {
            write!(w, ", \"{}\", \"{}\"", a, b)?;
        }
        write!(w, "]")?;

        // Dynamic properties (varargs)
        for (key, expr) in dynamic_props {
            write!(w, ", \"{}\", ", key)?;
            let initial = expr.initial_value_expr();
            let expr = initial.as_ref().unwrap_or(&expr);
            self.write_expr(w, doc, ctx, bindings, expr)?;

            if element_tag == "input" && key == "checked" {
                write!(w, "?\"{}\":null", key)?;
            };
        }

        // Class list
        if let Some(expr) = class_prop {
            write!(w, ", \"class\", ")?;
            if let Some(expr) = ctx.eval_expr(doc, &expr) {
                if let Some(s) = ctx.reduce_static_expr_to_string(&expr, true) {
                    write!(w, "\"{}\"", s)?;
                };

                if let &ExprValue::LiteralObject(ref props) = &expr {
                    write!(w, "classList(")?;
                    if let Some(ref props) = *props {
                        for &(ref key, ref expr) in props {
                            if let Some(ref expr) = *expr {
                                let initial = expr.initial_value_expr();
                                self.write_expr(w, doc, ctx, bindings, initial.as_ref().unwrap_or(expr))?;
                                write!(w, "&&\"{}\"", key)?;
                            };
                        }
                    };
                    write!(w, ")")?;
                };
            };
        };
        writeln!(w, ");")?;

        let binding = binding.into_iter();

        // Update bound values
        if element_tag == "input" {
            let is_checkbox: bool = static_props.iter().any(|prop| prop.0 == "type" && prop.1 == "checkbox");
            let element = element_key.and_then(|element_key| match element_tag {
                "input" if is_checkbox => {
                    Some(ExprValue::Binding(BindingType::DOMInputCheckboxElementCheckedBinding(ReducedValue::Dynamic(path_expr.clone()).into())))
                    // Some(ExprValue::Binding(BindingType::DOMInputCheckboxElementCheckedBinding(ReducedValue::Static(StaticValue::StaticString(element_key.into())).into())))
                }

                _ => {
                    Some(ExprValue::Binding(BindingType::DOMInputElementValueBinding(element_key.to_owned())))
                }
            });

            let value_binding = binding.filter_map(|binding| binding.as_ref().map(|b| &b.1)).nth(0);
            if let Some(initial) = value_binding.as_ref().and_then(|s| s.initial()) {
                if let Some(element) = element {
                    self.write_expr(w, doc, ctx, bindings, &element)?;
                    write!(w, " = ")?;
                    let expr = ExprValue::SymbolReference(initial.to_owned());
                    self.write_expr(w, doc, ctx, bindings, &expr)?;
                    writeln!(w, ";")?;
                };
            };
        };

        Ok(())
    }

    fn write_op_element_close(&mut self, w: &mut io::Write, _: &Document, _: &mut Context, _: &BindingContext, element_tag: &str) -> Result {
        writeln!(w, "IncrementalDOM.elementClose(\"{}\");", element_tag)?;
        Ok(())
    }

    fn write_op_element_start_block<PropIter: IntoIterator<Item = Prop>>(&mut self, _: &mut io::Write, _: &Document, _: &mut Context, _: &BindingContext, _: &str, _: PropIter) -> Result {
        Ok(())
    }

    fn write_op_element_end_block(&mut self, _: &mut io::Write, _: &Document, _: &mut Context, _: &BindingContext, _: &str) -> Result {
        Ok(())
    }

    fn write_op_element_map_collection_to_block(&mut self, w: &mut io::Write, _: &Document, _: &mut Context, _: &BindingContext, _: &ExprValue, block_id: &str) -> Result {
        write!(w, "(")?;
        // let binding = BindingType::LoopIndexBinding;
        writeln!(w, ").forEach(__{});", block_id)?;
        Ok(())
    }

    fn write_op_element_instance_component<'a, PropIter, EventIter, BindingIter>(&mut self, w: &mut io::Write, doc: &Document, ctx: &mut Context, bindings: &BindingContext, element_tag: &str, element_key: &str, is_void: bool, props: PropIter, events: EventIter, binding: BindingIter) -> Result
      where PropIter : IntoIterator<Item = ActualPropRef<'a>>, EventIter: IntoIterator<Item = &'a EventHandler>, BindingIter: IntoIterator<Item = &'a ElementValueBinding>
    {
        // let instance_key = ctx.join_path_as_expr_with(Some("."), element_key);
        let instance_key = ctx.join_path_as_expr(Some("."));
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
    fn render_component<'a, PropIter, EventIter, BindingIter>(&mut self, w: &mut io::Write, doc: &Document, ctx: &mut Context, bindings: &BindingContext, _: Option<&str>, component_ty: &str, instance_key: InstanceKey, _: bool, props: PropIter, _: EventIter, _: BindingIter, _: Option<LensItemType<'a>>) -> Result
      where PropIter : IntoIterator<Item = ActualPropRef<'a>>, EventIter: IntoIterator<Item = &'a EventHandler>, BindingIter: IntoIterator<Item = &'a ElementValueBinding>
    {
        let instance_key = instance_key.as_expr();
        write!(w, "component_{}(", component_ty)?;
        self.write_expr(w, doc, ctx, bindings, &instance_key)?;
        write!(w, ", {{")?;
        let mut first_item = true;
        for prop in props {
            if let Some(expr) = prop.1 {
                if !first_item { write!(w, ", ")?; }
                first_item = false;
                write!(w, "\"{}\": ", &prop.0)?;
                self.write_expr(w, doc, ctx, bindings, expr)?;
            };
        }
        writeln!(w, "}}, store);")?;
        Ok(())
    }

    fn write_map_collection_to_component<'a, PropIter, EventIter, BindingIter>(&mut self, w: &mut io::Write, doc: &Document, ctx: &mut Context, bindings: &BindingContext, _: &str, coll_expr: &ExprValue, enclosing_tag: Option<&str>, component_ty: &str, instance_key: InstanceKey, props: PropIter, events: EventIter, binding: BindingIter) -> Result
      where PropIter : IntoIterator<Item = ActualPropRef<'a>>, EventIter: IntoIterator<Item = &'a EventHandler>, BindingIter: IntoIterator<Item = &'a ElementValueBinding>
    {
        let instance_key = ExprValue::Apply(ExprApplyOp::JoinString(Some(".".into())), Some(vec![instance_key.as_expr().into(), ExprValue::Binding(BindingType::MapIndexBinding).into()]));

        write!(w, "(")?;
        self.write_expr(w, doc, ctx, bindings, coll_expr)?;
        write!(w, ").forEach(function(item, idx) {{")?;
        self.render_component(w, doc, ctx, bindings, enclosing_tag, component_ty, InstanceKey::Dynamic(&instance_key), false, props, events, binding, None)?;
        writeln!(w, "}});")?;
        Ok(())
    }
}


#[cfg(test)]
#[macro_use]
mod tests {


    use super::*;
    use std::io;
    use std::str;
    use std::iter::empty;
    use scope::context::*;
    use scope::bindings::*;
    use output::writers::*;

    #[macro_use]
    use output::writers::stream_writers::tests;


    #[test]
    pub fn test_output_stream_writers_js_ops1() {
        let template = Template::new(vec![]);
        let mut s: Vec<u8> = Default::default();

        test_writing!(
            template,
            |_, _, _| Ok(()),
            |writer: &mut DefaultOutputWriterJs, doc: &Document, ctx: & mut Context, bindings: &BindingContext| -> Result {
                ctx.append_path_str("prefix");

                writer.write_op_element_open(&mut s, doc, ctx, bindings, "span", Some("key".into()), false, empty(), empty(), empty())?;
                writer.write_op_element_close(&mut s, doc, ctx, bindings, "span")?;
                Ok(())
            }
        );

        assert_eq!(str::from_utf8(&s), Ok(r#"IncrementalDOM.elementOpen("span", "prefix.key", ["key", "prefix.key"]);
IncrementalDOM.elementClose("span");
"#));
    }

    #[test]
    pub fn test_output_stream_writers_js_ops2() {
        let template = Template::new(vec![]);

        let comp = ComponentDefinitionType {
            name: "todo_item".into(),
            inputs: None,
            children: None
        };

        let mut s: Vec<u8> = Default::default();

        test_writing!(
            template,
            move |processing: &mut ProcessDocument, ctx: &mut Context, bindings: &mut BindingContext| {
                processing.process_component_definition(ctx, bindings, &comp)
            },
            // |writer: &mut DefaultOutputWriterJs, doc: &Document, ctx: & mut Context, bindings: &BindingContext| -> Result {
            |writer: &mut DefaultOutputWriterJs, doc, ctx: &mut Context, bindings| -> Result {
                ctx.append_path_str("prefix");

                writer.write_op_element_open(&mut s, doc, ctx, bindings, "span", Some("key".into()), false, empty(), empty(), empty())?;
                writer.write_op_element_close(&mut s, doc, ctx, bindings, "span")?;
                Ok(())
            }
        );

        assert_eq!(str::from_utf8(&s), Ok(r#"IncrementalDOM.elementOpen("span", "prefix.key", ["key", "prefix.key"]);
IncrementalDOM.elementClose("span");
"#));
        // }
    }

}