
use std::io;
use std::iter;

use parser::*;
use scope::*;
use processing::*;
use output::*;


pub trait EventsWriter {
    fn write_event<'a>(&mut self, w: &mut io::Write, doc: &Document, ctx: &mut Context, bindings: &BindingContext, complete_key: InstanceKey<'a>, event: &EventsItem) -> Result;
    fn write_event_bindings<'a, I: IntoIterator<Item = &'a EventsItem>>(&mut self, w: &mut io::Write, doc: &Document, ctx: &mut Context, bindings: &BindingContext, events_iter: I) -> Result;
    fn write_bound_events<'a, I: IntoIterator<Item = &'a BoundEvent>>(&mut self, w: &mut io::Write, doc: &Document, ctx: &mut Context, bindings: &BindingContext, events_iter: I) -> Result;
}

pub trait EventActionOpsWriter {
    fn write_event_action_ops<'a, I: IntoIterator<Item = &'a ActionOpNode>>(&mut self, w: &mut io::Write, doc: &Document, ctx: &mut Context, bindings: &BindingContext, action_ops: I) -> Result;
}

impl<E: OutputWriter + ElementOpsStreamWriter + ExprWriter> EventActionOpsWriter for E {

    #[allow(dead_code)]
    #[allow(unused_variables)]
    fn write_event_action_ops<'a, I: IntoIterator<Item = &'a ActionOpNode>>(&mut self, w: &mut io::Write, doc: &Document, ctx: &mut Context, bindings: &BindingContext, action_ops: I) -> Result {
        for action_op in action_ops {
            match action_op {
                &ActionOpNode::DispatchAction(ref action_key, ref action_params) => {
                    // let action_ty = scope.0.make_action_type(action_key);
                    let action_ty = ctx.join_action_path_with(Some("."), action_key).to_uppercase();

                    if let &Some(ref action_params) = action_params {
                        let action_params: PropVec =
                            iter::once(("type".to_owned(),
                                        Some(ExprValue::LiteralString(action_ty.to_owned()))))
                                .chain(action_params.iter().map(|s| s.clone()))
                                .collect();

                        write!(w, " store.dispatch({{")?;

                        let mut first_item = true;
                        for ref prop in action_params {
                            if let Some(ref expr) = prop.1 {
                                if !first_item { write!(w, ", ")?; }
                                first_item = false;
                                write!(w, "\"{}\": ", &prop.0)?;
                                self.write_expr(w, doc, ctx, bindings, &expr)?;
                            };
                        }
                        // write_js_props_object(w, Some(action_params.iter()), self.doc, &scope)?;
                        writeln!(w, "}});")?;
                    } else {
                        writeln!(w, " store.dispatch({{\"type\": \"{}\"}}); ", action_ty)?;
                    }
                }
            }
        }
        Ok(())
    }
}

impl<E: OutputWriter + ElementOpsStreamWriter + ExprWriter + EventActionOpsWriter> EventsWriter for E {

    #[allow(dead_code)]
    fn write_event<'a>(&mut self, w: &mut io::Write, doc: &Document, ctx: &mut Context, bindings: &BindingContext, complete_key: InstanceKey<'a>, event: &EventsItem) -> Result {
            let final_event_name: String;
            let mut was_enterkey = false;

            match &event.2 {
                &EventHandler::Event(ref event_name, _, _) => {
                    final_event_name = if event_name == "enterkey" { was_enterkey = true; "keydown".into() } else { event_name.to_owned() };
                }

                &EventHandler::DefaultEvent(_, _) => {
                    final_event_name = "click".into();
                }
            }

            // let dom_binding = ExprValue::Binding(&BindingType::DOMElementBinding())
            // self.write_expr(w, doc, ctx, bindings, &expr)?;


            let dom_binding = match complete_key {
                InstanceKey::Static(s) => {
                    ExprValue::Binding(BindingType::DOMElementBinding(Box::new(ExprValue::LiteralString(s.to_owned()))))
                },
                InstanceKey::Dynamic(e) => {
                    ExprValue::Binding(BindingType::DOMElementBinding(Box::new(e.to_owned())))
                }
            };
            self.write_expr(w, doc, ctx, bindings, &dom_binding)?;

            write!(w, ".addEventListener(\"{}\", function(event) {{", final_event_name)?;

            if was_enterkey {
                writeln!(w, "if (event.keyCode == 13) {{")?;
            };

            ctx.push_child_scope();
            // match complete_key {
            //     InstanceKey::Static(s) => {
            //         ctx.append_path_str(s);
            //     },
            //     InstanceKey::Dynamic(e) => {
            //         ctx.append_path_expr(e);
            //     }
            // };

            match &event.2 {
                &EventHandler::Event(_, _, Some(ref action_ops)) | &EventHandler::DefaultEvent(_, Some(ref action_ops)) => {
                    self.write_event_action_ops(w, doc, ctx, bindings, action_ops.iter())?;
                }
                _ => {}
            }

            ctx.pop_scope();

            if was_enterkey {
                writeln!(w, "}}")?;
            };

            writeln!(w, "}});")?;
        Ok(())
    }


    #[allow(dead_code)]
    fn write_event_bindings<'a, I: IntoIterator<Item = &'a EventsItem>>(&mut self, w: &mut io::Write, doc: &Document, ctx: &mut Context, bindings: &BindingContext, events_iter: I) -> Result {
        writeln!(w, "      // Bind actions")?;
        for event in events_iter {
            let path_expr = ctx.join_path_as_expr_with(Some("."), &event.0);
            self.write_event(w, doc, ctx, bindings, InstanceKey::Dynamic(&path_expr), event)?;
        }
        Ok(())
    }

    fn write_bound_events<'a, I: IntoIterator<Item = &'a BoundEvent>>(&mut self, w: &mut io::Write, doc: &Document, ctx: &mut Context, bindings: &BindingContext, events_iter: I) -> Result {
        for bound_event in events_iter.into_iter() {
            let instance_key = bound_event.instance_key();
            let event_item = bound_event.event_item();

            ctx.push_child_scope();
            ctx.add_binding_value(&BindingType::ComponentKeyBinding, ExprValue::LiteralString(instance_key.to_owned()));
            ctx.append_path_expr(&ExprValue::Binding(BindingType::ComponentKeyBinding));

            // ctx.append_path_str(&instance_key);
            self.write_event(w, doc, ctx, &bindings, InstanceKey::Static(&instance_key), &event_item)?;
            ctx.pop_scope();
        }
        Ok(())
    }
}