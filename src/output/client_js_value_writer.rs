
use std::io;
use std::slice::Iter;

use parser::ast::*;
use processing::structs::*;
use processing::scope::*;

use output::client_ops_writer::*;
use output::client_misc::*;


#[inline]
pub fn write_js_var_reference(w: &mut io::Write,
                                    var_name: Option<&str>,
                                    doc: &DocumentState,
                                    scope: &ElementOpScope)
                                    -> Result {
    // let default_var_scope = scope.0.default_var_scope();
    if let Some(ref var_name) = var_name {
        let var_key = scope.0.make_var_name(var_name);
        write!(w, "{}", var_key)?;
    } else {
        // let default_var_scope = scope.0.default_var_scope();
        let default_var = scope.0.default_var();
        let var_key = default_var
            .unwrap_or_else(|| "default".to_owned());
        write!(w, "{}", var_key)?;
    };

    Ok(())
}

pub fn write_js_sym_value(w: &mut io::Write,
                                sym: &Symbol,
                                doc: &DocumentState,
                                scope: &ElementOpScope)
                                -> Result {
    match sym.sym_ref() {
        &SymbolReferenceType::ResolvedReference(ref key, ref resolved) => {
            match resolved {
                &ResolvedSymbolType::LocalVarReference(ref var_name) => {
                    write_js_var_reference(w, Some(var_name.as_str()), doc, scope)?;
                }

                &ResolvedSymbolType::ParameterReference(ref param_key) => {
                    write_js_var_reference(w, Some(param_key.as_str()), doc, scope)?;
                }

                &ResolvedSymbolType::ReducerKeyReference(ref as_reducer_key) => {
                    let key = format!("store.getState().{}", as_reducer_key);
                    write_js_var_reference(w, Some(&key), doc, &scope)?;
                }

                &ResolvedSymbolType::ActionStateReference(ref ty) => {
                    write_js_var_reference(w, Some("state"), doc, scope)?;
                }

                &ResolvedSymbolType::ActionParamReference(ref key) => {
                    let key = format!("action.{}", key);
                    write_js_var_reference(w, Some(key.as_str()), doc, scope)?;
                }

                &ResolvedSymbolType::LoopIndexReference(ref key, ref block_id) => {
                    let foridx = format!("__{}_{}", key, block_id);
                    write_js_var_reference(w, Some(&foridx), doc, scope)?;
                }

                &ResolvedSymbolType::LoopVarReference(ref var_name) => {
                    write_js_var_reference(w, Some(var_name.as_str()), doc, scope)?;
                }

                &ResolvedSymbolType::BlockParamReference(ref key) => {
                    write_js_var_reference(w, Some(key.as_str()), doc, scope)?;
                }

                &ResolvedSymbolType::PropReference(ref var_name) => {
                    let key = format!("props.{}", var_name);
                    write_js_var_reference(w, Some(&key), doc, &scope)?;
                }

                &ResolvedSymbolType::ElementValueReference(ref ref_element_key) => {
                    let complete_ref_key = scope.0.make_complete_element_key_with(ref_element_key);
                    let key = format!("document.querySelector(\"[key='{}']\").value", complete_ref_key);
                    write_js_var_reference(w, Some(&key), doc, &scope)?;
                }

                _ => {
                    let key = format!("undefined");
                    write_js_var_reference(w, Some(&key), doc, &scope)?;
                }
            };
        }
        _ => {
            let key = format!("undefined");
            write_js_var_reference(w, Some(&key), doc, &scope)?;
        }
    };
    Ok(())
}

#[inline]
pub fn write_js_expr_value(w: &mut io::Write,
                                node: &ExprValue,
                                doc: &DocumentState,
                                scope: &ElementOpScope)
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
            let mut wrote_first = false;
            if let &Some(ref items) = items {
                write!(w, "[")?;
                for ref item in items {
                    if wrote_first {
                        write!(w, ", ")?;
                    };
                    write_js_expr_value(w, item, doc, scope)?;
                    wrote_first = true;
                }
                write!(w, "]")?;
            };
        }

        &ExprValue::DefaultVariableReference => {
            write_js_var_reference(w, None, doc, scope)?;
        }

        &ExprValue::SymbolReference(ref sym) => {
            write_js_sym_value(w, sym, doc, scope)?;
        }

        &ExprValue::SymbolPathReference(ref sym_path) => {
            let mut iter = sym_path.iter();
            if let Some(ref sym) = iter.next() {
                write_js_sym_value(w, sym, doc, scope)?;
            };
            while let Some(ref sym) = iter.next() {
                write!(w, ".")?;
                write_js_sym_value(w, sym, doc, scope)?;
            };
        }

        &ExprValue::Expr(ref op, box ExprValue::SymbolReference(ref l_sym), ref r) => {
            let l_expr = ExprValue::SymbolReference(l_sym.clone());
            write_js_expr_value(w, &l_expr, doc, scope)?;

            let is_array = match l_sym.ty() { Some(&VarType::ArrayVar(..)) => true, _ => false };

            // let is_array = match l_sym {
            //     &Symbol(_, Some(VarType::ArrayVar(_)), _) => true,
            //     _ => false
            // };

            match op {
                &ExprOp::Add if is_array => write!(w, ".concat("),
                &ExprOp::Add => write!(w, " + "),
                &ExprOp::Sub => write!(w, " - "), 
                &ExprOp::Mul => write!(w, " * "),
                &ExprOp::Div => write!(w, " / ")
            }?;

            write_js_expr_value(w, r, doc, scope)?;
            if is_array {
                write!(w, ")")?;
            };
        }

        &ExprValue::Expr(ref sym, ref l, ref r) => {
            write_js_expr_value(w, l, doc, scope)?;
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
            write_js_expr_value(w, r, doc, scope)?;
        }

        &ExprValue::ContentNode(..) => {}
        &ExprValue::DefaultAction(..) => {}
        &ExprValue::Action(..) => {}

        &ExprValue::Undefined => { write!(w, "undefined")?; }
    }
    Ok(())
}

#[inline]
#[allow(unused_variables)]
pub fn write_js_props_object<'input>(w: &mut io::Write,
                                props: Option<Iter<'input, Prop>>,
                                doc: &DocumentState,
                                scope: &ElementOpScope)
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
                                    scope)?;

            } else {
                write!(w, "undefined")?;
            }
        }
    };
    write!(w, "}}")?;
    Ok(())
}

#[inline]
pub fn write_js_lens_mapping_open(w: &mut io::Write, op: &ElementOp, doc: &DocumentState, scope: &ElementOpScope, lens: &LensExprType) -> Result {
    let mut scope = scope.clone();
    let complete_key = scope.0.complete_element_key();

    if let &LensExprType::ForLens(Some(ref ele_key), ref coll_sym) = lens {
        let ele_expr = ExprValue::SymbolReference(Symbol::prop(ele_key));
        let coll_expr = ExprValue::SymbolReference(coll_sym.to_owned());
        // write_js_expr_value(w, &coll_expr, doc, &scope)?;

        let attrs = vec![
            (ele_key.to_owned(), Some(ele_expr))
        ];
        write!(w, "((")?;
        // write_js_props_object(w, Some(attrs.iter()), doc, scope)?;
        write_js_expr_value(w, &coll_expr, doc, &scope)?;
        // write!(w, ").map(function(v) {{ return {{  {} }}))")?;

        write!(w, ").map(function(v) {{ return ")?;
        // write_js_props_object(w, Some(attrs.iter()), doc, scope)?;
        write!(w, "{{ {}: v }}", ele_key)?;
        writeln!(w, "; }}))")?;

        let foridx_expr = ExprValue::Expr(ExprOp::Add,
            Box::new(ExprValue::LiteralString(format!("{}.", complete_key))),
            Box::new(ExprValue::SymbolReference(Symbol::param("foridx")))
        );
        scope.0.set_prefix_expr(&foridx_expr);

        writeln!(w, ".map(function(props, foridx) {{")?;
    };
    Ok(())
}

#[inline]
pub fn write_js_lens_mapping_close(w: &mut io::Write) -> Result {
    writeln!(w, "}});")?;
    Ok(())
}

#[inline]
pub fn map_invoke_component_props(props: Iter<&Prop>, scope: &ElementOpScope) -> Result {
    Ok(())
}

#[inline]
pub fn write_js_invoke_component(w: &mut io::Write, op: &ElementOp, doc: &DocumentState, scope: &ElementOpScope, comp: &Component, props: Option<Iter<Prop>>, component_key: &str) -> Result {
    let mut scope = scope.clone();
    scope.0.append_base_key(&format!("c_{}", component_key));
    scope.0.clear_index();

    let complete_key = scope.0.complete_element_key();
    let component_ty = &comp.name;
    // let element_tag = element_tag.unwrap_or("div");

        // .map(|p| reduce_expr(p, doc, &scope).or(p));

    // let key_expr = ExprValue::LiteralString(format!(".{}", complete_key));
    // let prefix_expr = scope.0.make_prefix_expr(&key_expr, None);

    let key_expr = ExprValue::LiteralString(format!(".{}", complete_key));
    let prefix_expr = scope.0.make_prefix_expr(&key_expr, None);

    write!(w, "component_{}(", component_ty)?;
    if let Some(ref prefix_expr) = scope.0.prefix_expr() {
        write_js_expr_value(w, prefix_expr, doc, &scope)?;
    } else {
        write!(w, "\"{}\"", complete_key)?;
    };
    write!(w, ", store")?;

    if let Some(props) = props {
        let props = map_props_using_scope(props, &scope);
        let props: PropVec = props.iter()
            .map(|p| {
                if let Some(sym) = scope.1.props.get(&p.0) {
                    // let expr = p.1.as_ref().and_then(|expr| reduce_expr(expr, doc,  &scope))
                    //     .or_else(|| p.1.to_owned());
                    let expr = ExprValue::SymbolReference(sym.to_owned());

                    return (p.0.to_owned(), Some(expr));
                };

                // let expr = p.1.as_ref().and_then(|expr| reduce_expr(expr, doc,  &scope))
                //     .or_else(|| p.1.to_owned());

                p.to_owned()
            }).collect();

        write!(w, ", ")?;
        // TODO: Fix lens support
        write_js_props_object(w, Some(props.iter()), doc, &scope)?;
    };

    writeln!(w, ");")?;

    Ok(())
}

#[inline]
#[allow(dead_code)]
#[allow(unused_variables)]
pub fn write_js_incdom_attr_array<'input>(w: &mut io::Write,
                                    attrs: Option<Iter<'input, Prop>>,
                                    doc: &DocumentState,
                                    scope: &ElementOpScope,
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
                write_js_expr_value(w, expr, doc, scope)?;
            } else {
                write!(w, "\"{}\", ", key)?;
                write!(w, "undefined")?;
            }
        }
    };
    Ok(())
}