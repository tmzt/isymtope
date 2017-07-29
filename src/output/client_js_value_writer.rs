
use std::io;
use std::slice::Iter;

use parser::ast::*;
use processing::structs::*;
use output::scope::*;
use output::client_ops_writer::*;


#[inline]
pub fn write_js_var_reference(w: &mut io::Write,
                                    var_name: Option<&str>,
                                    doc: &DocumentState,
                                    scope_prefixes: &ScopePrefixes)
                                    -> Result {
    let default_var_scope = scope_prefixes.default_var_scope();
    if let Some(ref var_name) = var_name {
        let var_key = scope_prefixes.var_prefix(var_name);
        write!(w, "{}", var_key)?;
    } else {
        let default_var_scope = scope_prefixes.default_var_scope();
        let default_var = scope_prefixes.default_var();
        let var_key = default_var_scope
            .or_else(|| default_var)    
            .unwrap_or("default".to_owned());
        write!(w, "{}", var_key)?;
    };

    // let state_key = "".to_owned();
    // let state_key = scope.state_lookup_key(var_name);
    // let is_scope_key = state_key.map_or(false, |s| doc.default_state_map.contains_key(s.as_str()));
    // let var_reference = scope.var_reference(is_scope_key, var_name);
    // write!(w, "{}", var_reference)?;
    // write!(w, "{}", var_key)?;
    Ok(())
}

#[inline]
pub fn write_js_expr_value(w: &mut io::Write,
                                node: &ExprValue,
                                doc: &DocumentState,
                                scope_prefixes: &ScopePrefixes)
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
                    write_js_expr_value(w, item, doc, scope_prefixes)?;
                }
                write!(w, "]")?;
            };
        }

        &ExprValue::DefaultVariableReference => {
            write_js_var_reference(w, None, doc, scope_prefixes)?;
        }

        &ExprValue::VariableReference(ref var_name) => {
            write_js_var_reference(w, Some(var_name.as_str()), doc, scope_prefixes)?;
        }

        &ExprValue::SymbolReference(ref sym) => {
            match sym {
                &(Some(ref sym), _) => {
                    match sym {
                        &SymbolReferenceType::LocalVarReference(ref var_name) => {
                            write_js_var_reference(w, Some(var_name.as_str()), doc, scope_prefixes)?;
                        }

                        &SymbolReferenceType::ParameterReference(ref param_key) => {
                            write_js_var_reference(w, Some(param_key.as_str()), doc, scope_prefixes)?;
                        }

                        &SymbolReferenceType::ReducerKeyReference(ref as_reducer_key) => {
                            write_js_var_reference(w, Some(as_reducer_key.as_str()), doc, scope_prefixes)?;
                        }

                        &SymbolReferenceType::ActionStateReference(ref ty) => {
                            write_js_var_reference(w, Some("state"), doc, scope_prefixes)?;
                        }

                        _ => {}
                    };
                }
                _ => {}
            };
        }

        // &ExprValue::Expr(ExprOp::Add, box ExprValue::DefaultVariableReference, ref r) => {
        //     // let state_ty = scope().unwrap().state_lookup_key(None);
        //     // let state_ty = state_ty.map_or(None, |s| doc.default_state_map.get(s.as_str()));

        //     // write!(w, "(")?;
        //     // write_js_var_reference(w, None, doc, scope)?;
        //     // if let Some(&(Some(VarType::ArrayVar(..)), _)) = state_ty {
        //     //     write!(w, ").concat(")?;
        //     // } else {
        //     //     write!(w, "+ (")?;
        //     // }
        //     // write_js_expr_value(w, r, doc, scope)?;
        //     // write!(w, ")")?;

        //     // let state_ty = scope().unwrap().state_lookup_key(None);
        //     // let state_ty = state_ty.map_or(None, |s| doc.default_state_map.get(s.as_str()));
        //     write!(w, "(")?;
        //     let 
        //     write_js_expr_value(w, ExprValue::DefaultVariableReference, doc, scope_prefixes)?;
        //     // write_js_var_reference(w, None, doc, scope_prefixes)?;
        //     // if let Some(&(Some(VarType::ArrayVar(..)), _)) = state_ty {
        //         write!(w, ").concat(")?;
        //     // } else {
        //         // write!(w, "+ (")?;
        //     // }
        //     write_js_expr_value(w, r, doc, scope_prefixes)?;
        //     write!(w, ")")?;
        // }

        // &ExprValue::Expr(ExprOp::Add, box ExprValue::VariableReference(ref var_name), ref r) => {
        //     // let state_ty = scope.state_lookup_key(Some(var_name.as_str())).as_ref()
        //     //     .map_or(None, |s| doc.default_state_map.get(s.as_str()));

        //     // write!(w, "(")?;
        //     // write_js_var_reference(w, None, doc, scope)?;
        //     // if let Some(&(Some(VarType::ArrayVar(..)), _)) = state_ty {
        //     //     write!(w, ").concat(")?;
        //     // } else {
        //     //     write!(w, "+ (")?;
        //     // }
        //     // write_js_expr_value(w, r, doc, scope)?;
        //     // write!(w, ")")?;
        // }

        &ExprValue::Expr(ref op, box ExprValue::SymbolReference(ref l_sym), ref r) => {
            let l_expr = ExprValue::SymbolReference(l_sym.clone());
            write_js_expr_value(w, &l_expr, doc, scope_prefixes)?;

            let is_array = match l_sym {
                &(Some(_), Some(VarType::ArrayVar(_))) => true,
                _ => false
            };

            match op {
                &ExprOp::Add if is_array => write!(w, ".concat("),
                &ExprOp::Add => write!(w, " + "),
                &ExprOp::Sub => write!(w, " - "), 
                &ExprOp::Mul => write!(w, " * "),
                &ExprOp::Div => write!(w, " / ")
            }?;

            write_js_expr_value(w, r, doc, scope_prefixes)?;
            if is_array {
                write!(w, ")")?;
            };
        }

        &ExprValue::Expr(ref sym, ref l, ref r) => {
            write_js_expr_value(w, l, doc, scope_prefixes)?;
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
            write_js_expr_value(w, r, doc, scope_prefixes)?;
        }

        &ExprValue::ContentNode(..) => {}
        &ExprValue::DefaultAction(..) => {}
        &ExprValue::Action(..) => {}
    }
    Ok(())
}

#[inline]
#[allow(unused_variables)]
pub fn write_js_props_object<'input>(w: &mut io::Write,
                                props: Option<Iter<'input, Prop>>,
                                doc: &DocumentState,
                                scope_prefixes: &ScopePrefixes)
                        -> Result {
    write!(w, "{{")?;
    let mut wrote_first = false;
    if let Some(props) = props {
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
                                    doc,
                                    scope_prefixes)?;

            } else {
                write!(w, "undefined")?;
            }
        }
    };
    write!(w, "}}")?;
    Ok(())
}

#[inline]
#[allow(dead_code)]
#[allow(unused_variables)]
pub fn write_js_incdom_attr_array<'input>(w: &mut io::Write,
                                    attrs: Option<Iter<'input, Prop>>,
                                    doc: &DocumentState,
                                    scope_prefixes: &ScopePrefixes,
                                    base_key: Option<&str>)
                                    -> Result {
    let mut wrote_first = false;
    if let Some(base_key) = base_key {
        write!(w, "\"data-id\", \"{}\", ", base_key)?;
    };

    if let Some(attrs) = attrs {
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
                write_js_expr_value(w, expr, doc, scope_prefixes)?;
            } else {
                write!(w, "\"{}\", ", key)?;
                write!(w, "undefined")?;
            }
        }
    };
    Ok(())
}