
use std::io;
use std::iter;

use parser::*;
use scope::*;
use processing::*;
use output::*;


pub trait EventsWriter {
    // fn write_event_binding(&self, w: &mut io::Write, ctx: &mut Context, bindings: &BindingContext, event: &EventsItem) -> Result;
    fn write_event_bindings<'a, I: IntoIterator<Item = &'a EventsItem>>(&mut self, w: &mut io::Write, ctx: &mut Context, bindings: &BindingContext, events_iter: I) -> Result;
}

pub trait EventActionOpsWriter {
    fn write_event_action_ops<'a, I: IntoIterator<Item = &'a ActionOpNode>>(&mut self, w: &mut io::Write, ctx: &mut Context, bindings: &BindingContext, action_ops: I) -> Result;
}

impl<E: OutputWriter + ElementOpsStreamWriter + ExprWriter> EventActionOpsWriter for E {

    #[allow(dead_code)]
    #[allow(unused_variables)]
    fn write_event_action_ops<'a, I: IntoIterator<Item = &'a ActionOpNode>>(&mut self, w: &mut io::Write, ctx: &mut Context, bindings: &BindingContext, action_ops: I) -> Result {
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
                                self.write_expr(w, ctx, bindings, &expr)?;
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
    fn write_event_bindings<'a, I: IntoIterator<Item = &'a EventsItem>>(&mut self, w: &mut io::Write, ctx: &mut Context, bindings: &BindingContext, events_iter: I) -> Result {
        writeln!(w, "      // Bind actions")?;
        for event in events_iter {
        // for &(ref element_key, ref scope_id, ref event_handler) in events_iter {
            let final_event_name: String;
            let mut was_enterkey = false;

            // let complete_key = ctx.join_path_with(Some("."), &event.0);

            match &event.2 {
                &EventHandler::Event(ref event_name, ref params, ref action_ops) => {
                    final_event_name = if event_name == "enterkey" { was_enterkey = true; "keydown".into() } else { event_name.to_owned() };
                }

                &EventHandler::DefaultEvent(ref params, ref action_ops) => {
                    final_event_name = "click".into();
                }
            }

            let path_expr = ctx.join_path_as_expr_with(Some("."), &event.0);

            write!(w, "document.querySelector(\"[key='\" + ")?;
            self.write_expr(w, ctx, bindings, &path_expr)?;
            write!(w, " + \"']\").addEventListener(\"{}\", function(event) {{", final_event_name)?;

            if was_enterkey {
                writeln!(w, "if (event.keyCode == 13) {{")?;
            };

            match &event.2 {
                &EventHandler::Event(_, _, Some(ref action_ops)) | &EventHandler::DefaultEvent(_, Some(ref action_ops)) => {
                    self.write_event_action_ops(w, ctx, bindings, action_ops.iter())?;
                }
                _ => {}
            }

            if was_enterkey {
                writeln!(w, "}}")?;
            };

            writeln!(w, "}});")?;
        }
        Ok(())
    }
}