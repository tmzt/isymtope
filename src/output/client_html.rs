
use std::io;
use std::clone::Clone;
use std::slice::Iter;

use parser::ast::*;
use parser::util::allocate_element_key;
use output::structs::*;
use output::client_js::*;
use output::client_misc::*;


#[inline]
#[allow(unused_variables)]
pub fn write_html_ops_content<'input>(
                                w: &mut io::Write,
                                ops: Iter<ElementOp>,
                                events_vec: &mut EventsVec,
                                keys_vec: &mut Vec<String>,
                                processing: &DocumentState,
                                resolve: &ResolveVars,
                                key_prefix: Option<&str>)
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
                                    ref events)
                | &ElementOp::ElementVoid(ref element_tag,
                                    ref element_key,
                                    ref attrs,
                                    ref events) => {

                let element_key = format!("{}{}",
                    key_prefix.map_or("".into(), |s| format!("{}_", s)),
                    element_key.as_ref().map_or_else(allocate_element_key, Clone::clone)
                );
                
                write!(w, "<{}", element_tag)?;
                write!(w, " key=\"{}\"", element_key)?;

                if let &Some(ref attrs) = attrs {
                    for &(ref key, ref expr) in attrs {
                        // Ignore empty attributes
                        if let &Some(ref expr) = expr {
                            match expr {
                                &ExprValue::Expr(ExprOp::Add, ref l, ref r) => {


                                }

                                &ExprValue::DefaultAction(ref params, ref act_ops) => {
                                    if let &Some(ref act_ops) = act_ops {
                                        write_js_action(w, act_ops.iter())?;
                                        continue;
                                    };
                                },
                                &ExprValue::Action(ref event_name, ref params, ref act_ops) => {
                                    if let &Some(ref act_ops) = act_ops {
                                        write_js_action(w, act_ops.iter())?;
                                        continue;
                                    };
                                },
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
                    write!(w, ">")?;
                } else {
                    write!(w, ">")?;
                };

                // Process events
                if let &Some(ref events) = events {
                    for &(ref event_name, ref event_params, ref action_ops) in events {
                        let event_params = event_params.as_ref().map(Clone::clone);
                        let action_ops = action_ops.as_ref().map(Clone::clone);
                        let event_name = event_name.as_ref().map(Clone::clone);
                        //let cur_scope = resolve.cur_scope.as_ref().map(|s| format!("{}", s));
                        events_vec.push((element_key.clone(),
                                            event_name,
                                            event_params,
                                            action_ops,
                                            resolve.cur_state_key.clone()));
                    }
                }

                keys_vec.push(element_key);
            }
            &ElementOp::ElementClose(ref element_tag) => {
                write!(w, "</{}>", element_tag)?;
            }
            &ElementOp::WriteValue(ref expr, ref element_key) => {
                let element_key = element_key.as_ref().map_or_else(allocate_element_key, |s| s.clone());
                write!(w, "<span key=\"{}\">", element_key)?;
                write_computed_expr_value(w, expr, None, None)?;
                write!(w, "</span>")?;

                keys_vec.push(element_key);
            }
            &ElementOp::InstanceComponent(ref component_ty,
                                            ref element_key,
                                            ref props,
                                            ref lens) => {
                // Try to locate a matching component
                if let Some(ref comp) = processing.comp_map.get(component_ty.as_str()) {
                    // Render a component
                    let component_scope = lens.as_ref().map(|lens| resolve.with_state_key(lens));
                    let resolve = component_scope.as_ref().map_or(resolve, |r| r);

                    let element_key = format!("{}{}",
                        key_prefix.as_ref().map_or("".into(), |s| format!("{}_", s)),
                        element_key.as_ref().map_or_else(allocate_element_key, |s| s.clone())
                    );
                
                    write!(w, "<div key=\"{}\" >", element_key)?;

                    let default_scope = lens;

                    if let Some(ref component_ops) = comp.ops {
                        write_html_ops_content(w,
                                                    component_ops.iter(),
                                                    events_vec,
                                                    keys_vec,
                                                    processing,
                                                    resolve,
                                                    Some(format!("{}_", element_key).as_str()))?;
                    };

                    write!(w, "</div>")?;

                    keys_vec.push(element_key);
                };
            },

            _ => {}
        }
    }

    Ok(())
}