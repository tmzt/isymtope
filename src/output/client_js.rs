
use std::io;
use std::slice::Iter;

use parser::ast::*;
use output::structs::*;

#[inline]
pub fn write_js_var_reference(w: &mut io::Write,
                                    var_name: Option<&str>,
                                    processing: &DocumentState,
                                    resolve: &ResolveVars)
                                    -> Result {
    let state_key = resolve.state_lookup_key(var_name);
    let is_scope_key = state_key.map_or(false, |s| processing.default_state_map.contains_key(s.as_str()));
    let var_reference = resolve.var_reference(is_scope_key, var_name);
    write!(w, "{}", var_reference)?;
    Ok(())
}

#[inline]
pub fn write_js_expr_value<'input>(w: &mut io::Write,
                                   node: &ExprValue,
                                   processing: &DocumentState,
                                   resolve: &ResolveVars)
                                   -> Result {
    match node {
        // TODO: Handle the case where quotes appear in the string
        &ExprValue::LiteralString(ref s) => {
            write!(w, "\"{}\"", s)?;
        }
        &ExprValue::LiteralNumber(ref n) => {
            write!(w, "{}", n)?;
        }

        &ExprValue::LiteralArray(ref items) => {
            if let &Some(ref items) = items {
                write!(w, "[")?;
                for ref item in items {
                    write_js_expr_value(w, item, processing, resolve)?;
                }
                write!(w, "]")?;
            };
        }

        &ExprValue::DefaultVariableReference => {
            write_js_var_reference(w, None, processing, resolve)?;
        }

        &ExprValue::VariableReference(ref var_name) => {
            write_js_var_reference(w, Some(var_name.as_str()), processing, resolve)?;
        }

        &ExprValue::Expr(ExprOp::Add, box ExprValue::DefaultVariableReference, ref r) => {
            let state_key = resolve.state_lookup_key(None);
            let state_ty = state_key.as_ref()
                .map_or(None, |s| processing.default_state_map.get(s.as_str()));

            write!(w, "(")?;
            write_js_var_reference(w, None, processing, resolve)?;
            if let Some(&(Some(VarType::ArrayVar(..)), _)) = state_ty {
                write!(w, ").concat(")?;
            } else {
                write!(w, "+ (")?;
            }
            write_js_expr_value(w, r, processing, resolve)?;
            write!(w, ")")?;
        }

        &ExprValue::Expr(ExprOp::Add, box ExprValue::VariableReference(ref var_name), ref r) => {
            let state_ty = resolve.state_lookup_key(Some(var_name.as_str())).as_ref()
                .map_or(None, |s| processing.default_state_map.get(s.as_str()));

            write!(w, "(")?;
            write_js_var_reference(w, None, processing, resolve)?;
            if let Some(&(Some(VarType::ArrayVar(..)), _)) = state_ty {
                write!(w, ").concat(")?;
            } else {
                write!(w, "+ (")?;
            }
            write_js_expr_value(w, r, processing, resolve)?;
            write!(w, ")")?;
        }

        &ExprValue::Expr(ref sym, ref l, ref r) => {
            write_js_expr_value(w, l, processing, resolve)?;
            match sym {
                &ExprOp::Add => {
                    write!(w, " + ")?;
                }
                &ExprOp::Sub => {
                    write!(w, " - ")?;
                }
                &ExprOp::Mul => {
                    write!(w, " * ")?;
                }
                &ExprOp::Div => {
                    write!(w, " / ")?;
                }
            }
            write_js_expr_value(w, r, processing, resolve)?;
        }

        &ExprValue::ContentNode(..) => {}
        &ExprValue::DefaultAction(..) => {}
        &ExprValue::Action(..) => {}
    }
    Ok(())
}

#[inline]
#[allow(unused_variables)]
pub fn write_js_action(w: &mut io::Write, act_iter: Iter<ActionOpNode>) -> Result {
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
fn write_js_incdom_attr_array<'input>(w: &mut io::Write,
                                      attrs: &Vec<Prop>,
                                      processing: &DocumentState,
                                      resolve: &ResolveVars)
                                      -> Result {
    let mut wrote_first = false;
    for &(ref key, ref expr) in attrs {
        if let &Some(ref expr) = expr {
            if wrote_first {
                write!(w, ", ")?
            } else {
                wrote_first = true;
            }

            if let &ExprValue::DefaultAction(ref params, ref act_ops) = expr {
                write!(w, "\"{}\", ", key)?;
                write!(w, "function(event) {{")?;
                if let &Some(ref act_ops) = act_ops {
                    for ref act_op in act_ops {
                        match *act_op {
                            &ActionOpNode::DispatchAction(ref action, ref params) => {
                                write!(w, " store.dispatch({{\"type\": \"{}\"}}); ", action)?;
                            }
                        }
                    }
                }
                write!(w, "}}")?;
                continue;
            };

            write!(w, "\"{}\", ", key)?;
            write_js_expr_value(w, expr, processing, resolve)?;
        } else {
            write!(w, "\"{}\", ", key)?;
            write!(w, "undefined")?;
        }
    }
    Ok(())
}

#[inline]
#[allow(unused_variables)]
fn write_js_props_object<'input>(w: &mut io::Write,
                                 props: &Vec<Prop>,
                                 processing: &DocumentState,
                                 resolve: &ResolveVars)
                         -> Result {
    write!(w, "{{")?;
    let mut wrote_first = false;
    for &(ref key, ref expr) in props {
        if wrote_first {
            write!(w, ", ")?
        } else {
            wrote_first = true;
        }

        // Write the property name
        write!(w, "\"{}\": ", key)?;

        // Write the property value or undefined for None
        if let &Some(ref expr) = expr {

            if let &ExprValue::DefaultAction(ref params, ref act_ops) = expr {
                write!(w, "\"{}\", ", key)?;
                write!(w, "function(event) {{")?;
                if let &Some(ref act_ops) = act_ops {
                    for ref act_op in act_ops {
                        match *act_op {
                            &ActionOpNode::DispatchAction(ref action, ref params) => {
                                write!(w, " store.dispatch({{\"type\": \"{}\"}}); ", action)?;
                            }
                        }
                    }
                }
                write!(w, "}}")?;
                continue;
            };

            write_js_expr_value(w,
                                &expr,
                                processing,
                                resolve)?;

        } else {
            write!(w, "undefined")?;
        }
    }
    write!(w, "}}")?;
    Ok(())
}

#[inline]
#[allow(unused_variables)]
pub fn write_js_incdom_ops_content<'input>(w: &mut io::Write,
                                           ops: Iter<ElementOp>,
                                           processing: &DocumentState,
                                           resolve: &ResolveVars,
                                           key_prefix: Option<&str>,
                                           key_var_prefix: Option<&str>)
                                           -> Result {
    for ref op in ops {
        let mut is_void = false;
        if let &ElementOp::ElementVoid(..) = *op {
            is_void = true;
        }

        match *op {
            &ElementOp::ElementOpen(ref element_tag, ref element_key, ref attrs, ref events) |
            &ElementOp::ElementVoid(ref element_tag, ref element_key, ref attrs, ref events) => {

                let element_key = format!("{}{}",
                                          key_prefix.as_ref()
                                              .map_or("".into(), |s| format!("{}_", s)),
                                          element_key.as_ref().map_or("null", |s| s));
                let key_var_prefix = key_var_prefix.as_ref()
                    .map_or("".into(), |s| format!("{} + ", s));

                if !is_void {
                    write!(w,
                           "IncrementalDOM.elementOpen(\"{}\", {}\"{}\", [",
                           element_tag,
                           key_var_prefix,
                           element_key)
                        ?;
                } else {
                    write!(w,
                           "IncrementalDOM.elementVoid(\"{}\", {}\"{}\", [",
                           element_tag,
                           key_var_prefix,
                           element_key)
                        ?;
                }

                // Static attrs
                if let &Some(ref attrs) = attrs {
                    write_js_incdom_attr_array(w, attrs, processing, resolve)?;
                };

                // TODO: Dynamic attributes

                writeln!(w, "]);")?;
            }
            &ElementOp::ElementClose(ref element_tag) => {
                writeln!(w, "IncrementalDOM.elementClose(\"{}\");", element_tag)?;
            }
            &ElementOp::WriteValue(ref expr, ref element_key) => {
                let element_key = element_key.as_ref().map_or("null", |s| s);
                writeln!(w,
                         "IncrementalDOM.elementOpen(\"span\", \"{}\", [\"key\", \"{}\"]);",
                         element_key,
                         element_key)
                    ?;
                write!(w, "IncrementalDOM.text(")?;
                write_js_expr_value(w, expr, processing, resolve)?;
                writeln!(w, ");")?;
                writeln!(w, "IncrementalDOM.elementClose(\"span\");")?;
            }
            &ElementOp::InstanceComponent(ref component_ty,
                                          ref component_key,
                                          ref props,
                                          ref lens) => {
                let component_scope = lens.as_ref().map(|lens| resolve.with_state_key(lens));
                let resolve = component_scope.as_ref().map_or(resolve, |r| r);

                let comp = processing.comp_map.get(component_ty.as_str());
                if comp.is_some() {
                    let component_key = component_key.as_ref().map_or("null", |s| s);
                    writeln!(w,
                             "IncrementalDOM.elementOpen(\"div\", \"{}\", []);",
                             component_key)
                        ?;
                    write!(w,
                           "component_{}(\"{}_\", store, ",
                           component_ty,
                           component_key)
                        ?;
                    if let &Some(ref props) = props {
                        let var_prefix = lens.as_ref().map(|s| format!("store.getState().{}.", s));
                        let default_var = lens.as_ref().map(|s| format!("store.getState.{}", s));
                        let default_scope = lens.as_ref().map(|s| s.as_str());
                        write_js_props_object(w, props, processing, resolve)?;
                    }
                    writeln!(w, ");")?;
                    writeln!(w, "IncrementalDOM.elementClose(\"div\");")?;
                }
            }

            &ElementOp::StartBlock(ref block_id) => {
                // writeln!(w, "var __{} = function __{}_(__forvar_{}){{", block_id, block_id, block_id)?;
                writeln!(w, "var __{} = function __{}_(line){{", block_id, block_id)?; //FIXME
            }

            &ElementOp::EndBlock(..) => {
                writeln!(w, "}};")?;
            }

            &ElementOp::MapCollection(ref block_id, ref ele, ref coll_expr) => {
                write!(w, "(")?;

                let forvar_default = &format!("__forvar_{}", block_id);
                let forvar_prefix =
                    &format!("__forvar_{}{}", block_id, ele.as_ref().map_or("", |s| s));

                /*
                write_js_expr_value(w,
                                    coll_expr,
                                    default_state_map,
                                    Some(forvar_prefix),
                                    Some(forvar_default),
                                    default_scope)
                    ?;
                */

                //let block_scope = resolve.block_scope(block_id, ele.as_ref().map(String::as_str));
                let block_scope = resolve.block_scope(block_id, None);
                write_js_expr_value(w, coll_expr, processing, &block_scope)?;

                writeln!(w, ").map(__{});", block_id)?;
            }
        }
    }

    Ok(())
}

#[inline]
#[allow(dead_code)]
pub fn write_js_incdom_component<'input>(w: &mut io::Write,
                                         component_ty: &'input str,
                                         ops: Iter<ElementOp>,
                                         processing: &DocumentState,
                                         resolve: &ResolveVars,
                                         key_prefix: Option<&str>)
                                         -> Result {

    writeln!(w,
             "  function component_{}(key_prefix, store, props) {{",
             component_ty)
        ?;
    write_js_incdom_ops_content(w, ops, processing, resolve, key_prefix, Some("key_prefix"))?;
    writeln!(w, "  }};")?;
    writeln!(w, "")?;
    Ok(())
}
