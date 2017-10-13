
use std::io;
use std::iter;

use model::*;
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
            match *action_op {
                ActionOpNode::DispatchAction(ref action_key, ref action_params) |
                ActionOpNode::DispatchActionTo(ref action_key, ref action_params, _)
                  => {
                    let path = match *action_op { ActionOpNode::DispatchActionTo(_, _, ref path) => Some(path), _ => None };
                    let action_ty = path.map(|s| format!("{}.{}", s, action_key))
                        .unwrap_or_else(|| ctx.action_path_str_with(action_key))
                        .to_uppercase();

                    if let Some(ref action_params) = *action_params {
                        let action_params: PropVec =
                            iter::once(("type".to_owned(),
                                        Some(ExprValue::LiteralString(action_ty.to_owned()))))
                                .chain(action_params.iter().cloned())
                                .collect();

                        write!(w, " store.dispatch({{")?;

                        let mut first_item = true;
                        for prop in action_params {
                            if let Some(ref expr) = prop.1 {
                                let expr = ctx.reduce_expr_or_return_same(expr);

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

            let key_expr = complete_key.as_expr();

            match event.2 {
                EventHandler::Event(ref event_name, _, _) if event_name == "enterkey" => {
                    was_enterkey = true; final_event_name = "keydown".into();
                }

                EventHandler::Event(ref event_name, _, _) => {
                    final_event_name = event_name.to_owned();
                }

                EventHandler::DefaultEvent(_, _) => {
                    final_event_name = "click".into();
                }
            }

            write!(w, "setEventListener(")?;
            self.write_expr(w, doc, ctx, bindings, &key_expr)?;
            write!(w, ", ")?;

            let dom_binding = ExprValue::Binding(BindingType::DOMElementBinding(key_expr.clone().into()));

            self.write_expr(w, doc, ctx, bindings, &dom_binding)?;
            write!(w, ", \"{}\", function(event) {{", final_event_name)?;

            if was_enterkey {
                writeln!(w, "if (event.keyCode == 13) {{")?;
            };

            ctx.push_child_scope();


            match event.2 {
                EventHandler::Event(_, _, Some(ref action_ops)) | EventHandler::DefaultEvent(_, Some(ref action_ops)) => {
                    ctx.push_child_scope();
                    if let EventHandler::Event(ref event_name, _, _) = event.2 {
                        if event_name == "change" {
                            let checked_binding = BindingType::DOMInputCheckboxElementCheckedBinding(Box::new(ReducedValue::Dynamic(key_expr.to_owned())));
                            ctx.add_binding_value(&BindingType::EventElementValueBinding, ExprValue::Binding(checked_binding));
                        }
                    };

                    self.write_event_action_ops(w, doc, ctx, bindings, action_ops.iter())?;
                    ctx.pop_scope();
                }
                _ => {}
            }

            // match event.2 {
            //     EventHandler::Event(_, _, Some(ref action_ops)) | EventHandler::DefaultEvent(_, Some(ref action_ops)) => {
            //         self.write_event_action_ops(w, doc, ctx, bindings, action_ops.iter())?;
            //     }
            //     _ => {}
            // }

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
            let path_expr = ctx.path_expr_with(&event.0);
            self.write_event(w, doc, ctx, bindings, InstanceKey::Dynamic(&path_expr), event)?;
        }
        Ok(())
    }

    fn write_bound_events<'a, I: IntoIterator<Item = &'a BoundEvent>>(&mut self, w: &mut io::Write, doc: &Document, ctx: &mut Context, bindings: &BindingContext, events_iter: I) -> Result {
        for bound_event in events_iter {
            let instance_key = bound_event.instance_key();
            let complete_key = bound_event.complete_key();
            let event_item = bound_event.event_item();

            ctx.push_child_scope();
            ctx.add_binding_value(&BindingType::ComponentKeyBinding, ExprValue::LiteralString(instance_key.to_owned()));
            ctx.append_path_expr(&ExprValue::Binding(BindingType::ComponentKeyBinding));

            if let Some(props) = bound_event.props() {
                for &(ref k, ref v) in props {
                    if let Some(ref v) = *v {
                        ctx.add_binding_value(&BindingType::ComponentPropBinding(k.to_owned()), v.to_owned());
                    }
                }
            };

            // ctx.append_path_str(&instance_key);
            self.write_event(w, doc, ctx, bindings, InstanceKey::Static(&complete_key), &event_item)?;
            ctx.pop_scope();
        }
        Ok(())
    }
}
