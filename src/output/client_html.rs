
use std::io;
use std::clone::Clone;
use std::slice::Iter;

use parser::ast::*;
use parser::util::allocate_element_key;
use output::structs::*;
use output::client_js::*;
use output::client_misc::*;
use output::client_output::*;


pub struct WriteHtmlOpsContent<'input> {
    doc: &'input DocumentState<'input>,
    //format_html: &'input FormatHtml<'input>,
    pub events_vec: EventsVec,
    pub keys_vec: Vec<String>,
}

impl<'input> WriteHtmlOpsContent<'input> {
    pub fn with_doc(doc: &'input DocumentState<'input>) -> WriteHtmlOpsContent<'input> {
        WriteHtmlOpsContent {
            doc: doc,
            //format_html: format_html,
            events_vec: Default::default(),
            keys_vec: Default::default(),
        }
    }

    #[inline]
    #[allow(unused_variables)]
    pub fn write_html_js_action(&mut self, w: &mut io::Write, act_iter: Iter<ActionOpNode>) -> Result {
        write!(w, "function(event) {{")?;
        for ref act_op in act_iter {
            match *act_op {
                &ActionOpNode::DispatchAction(ref action, ref params) => {
                    //let action_type = resolve.action_type(Some(action));
                    write!(w, " store.dispatch({{\"type\": \"{}\"}}); ", action)?;
                }
            }
        }
        write!(w, "}}")?;
        Ok(())
    }

    #[inline]
    #[allow(unused_variables)]
    pub fn write_html_ops_content(&mut self,
                                  w: &mut io::Write,
                                  ops: Iter<ElementOp>,
                                  resolve: &ResolveVars)
                                  -> Result {
        for ref op in ops {
            let mut is_void = false;

            if let &ElementOp::ElementVoid(..) = *op {
                is_void = true;
            }

            match *op {
                &ElementOp::ElementOpen(ref element_tag,
                                        ref element_key,
                                        ref attrs,
                                        ref events) |
                &ElementOp::ElementVoid(ref element_tag,
                                        ref element_key,
                                        ref attrs,
                                        ref events) => {

                    let element_key = element_key.as_ref().map_or("null", |s| s);
                    let base_key = resolve.base_element_key(element_key);

                    write!(w, "<{}", element_tag)?;
                    write!(w, " data-id=\"{}\"", base_key)?;
                    write!(w, " key=\"{}\"", base_key)?;

                    if let &Some(ref attrs) = attrs {
                        for &(ref key, ref expr) in attrs {
                            // Ignore empty attributes
                            if let &Some(ref expr) = expr {
                                match expr {
                                    &ExprValue::Expr(ExprOp::Add, ref l, ref r) => {}

                                    &ExprValue::DefaultAction(ref params, ref act_ops) => {
                                        if let &Some(ref act_ops) = act_ops {
                                            self.write_html_js_action(w, act_ops.iter())?;
                                            continue;
                                        };
                                    }
                                    &ExprValue::Action(ref event_name, ref params, ref act_ops) => {
                                        if let &Some(ref act_ops) = act_ops {
                                            self.write_html_js_action(w, act_ops.iter())?;
                                            continue;
                                        };
                                    }
                                    _ => {
                                        write!(w, " {}=\"", key)?;
                                        write_computed_expr_value(w, expr, None, None)?;
                                        write!(w, "\"")?;
                                    }
                                };
                            };
                        }
                    };

                    if is_void {
                        write!(w, " />")?;
                    } else {
                        write!(w, ">")?;
                    };

                    // Process events
                    if let &Some(ref events) = events {
                        for &(ref event_name, ref event_params, ref action_ops) in events {
                            let event_params = event_params.as_ref().map(Clone::clone);
                            let action_ops = action_ops.as_ref().map(Clone::clone);
                            let event_name = event_name.as_ref().map(Clone::clone);
                            // let cur_scope = resolve.cur_scope.as_ref().map(|s| format!("{}", s));
                            self.events_vec.push((base_key.clone(),
                                                  event_name,
                                                  event_params,
                                                  action_ops,
                                                  resolve.cur_state_key.clone()));
                        }
                    }

                    self.keys_vec.push(base_key);
                }
                &ElementOp::ElementClose(ref element_tag) => {
                    write!(w, "</{}>", element_tag)?;
                }
                &ElementOp::WriteValue(ref expr, ref element_key) => {
                    let element_key = element_key.as_ref()
                        .map_or_else(allocate_element_key, |s| s.clone());
                    write!(w, "<span key=\"{}\" data-id=\"{}\">", element_key, element_key)?;
                    write_computed_expr_value(w, expr, None, None)?;
                    write!(w, "</span>")?;

                    self.keys_vec.push(element_key);
                }
                &ElementOp::InstanceComponent(ref component_ty,
                                              ref element_key,
                                              ref props,
                                              ref lens) => {
                    // Try to locate a matching component
                    if let Some(ref comp) = self.doc.comp_map.get(component_ty.as_str()) {
                        // Render a component
                        let component_scope = lens.as_ref()
                            .map(|lens| resolve.with_state_key(lens));
                        let resolve = component_scope.as_ref().map_or(resolve, |r| r);

                        let element_key = element_key.as_ref().map_or("null", |s| s);
                        let base_key = resolve.base_element_key(element_key);

                        write!(w, "<div key=\"{}\" data-id=\"{}\" >", base_key, base_key)?;

                        let default_scope = lens;

                        if let Some(ref component_ops) = comp.ops {
                            self.write_html_ops_content(w,
                                                        component_ops.iter(),
                                                        resolve)?;
                        };

                        write!(w, "</div>")?;

                        self.keys_vec.push(base_key);
                    };
                }

                _ => {}
            }
        }

        Ok(())
    }
}