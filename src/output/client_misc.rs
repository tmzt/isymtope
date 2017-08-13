
use std::io;

use parser::ast::*;
use processing::structs::*;
use processing::scope::*;


pub fn reduce_expr_to_string(expr: &ExprValue, doc: &DocumentState, scope: &ElementOpScope) -> String {
    match expr {
        &ExprValue::LiteralString(ref s) => format!("{}", s),
        &ExprValue::LiteralNumber(ref n) => format!("{}", n),
        &ExprValue::LiteralArray(..) => format!("[array]"),
        _ => format!("[invalid]")
    }
}

pub fn eval_sym(sym: &Symbol, doc: &DocumentState, scope: &ElementOpScope) -> Option<ExprValue> {
    let sym_ref = sym.sym_ref();
    let sym_ty = sym.ty();
    if let &SymbolReferenceType::ResolvedReference(_, ref resolved) = sym_ref {
        match resolved {
            &ResolvedSymbolType::PropReference(ref key) => {
                if let Some(sym_val) = scope.1.props.get(key) {
                    return sym_val.value().map(|s| s.to_owned());
                }
            }

            &ResolvedSymbolType::BlockParamReference(ref key) => {
                if let Some(sym_val) = scope.1.block_params.get(key) {
                    return sym_val.value().map(|s| s.to_owned());
                }
            }

            &ResolvedSymbolType::ReducerKeyReference(ref key) => {
                if let Some(expr) = doc.resolve_symbol_value(sym) {
                    return Some(expr.clone());
                };
            }

            _ => {}
        };
    };

    None
}

pub fn reduce_expr(expr: &ExprValue, doc: &DocumentState, scope: &ElementOpScope) -> Option<ExprValue> {
    match expr {
        &ExprValue::LiteralString(..) => Some(expr.clone()),
        &ExprValue::LiteralNumber(..) => Some(expr.clone()),
        &ExprValue::LiteralArray(..) => Some(expr.clone()),

        &ExprValue::Expr(ref op, ref l, ref r) => {
            let l_expr = reduce_expr(l, doc, scope);
            let r_expr = reduce_expr(r, doc, scope);

            match op {
                &ExprOp::Add => {
                    match (l_expr, r_expr) {
                        // (&Some(ExprValue::LiteralString(ref l_str)), _) => {
                        //     let r_str = reduce_expr_to_string(r_expr, doc, scope);
                        //     return Some(ExprValue::LiteralString(format!("{}{}", l_str, r_str)));
                        // },
                        // (_, &Some(ExprValue::LiteralString(ref r_str))) => {
                        //     let l_str = reduce_expr_to_string(r_expr, doc, scope);
                        //     return Some(ExprValue::LiteralString(format!("{}{}", l_str, r_str)));
                        // },
                        (Some(ref l_val), Some(ref r_val)) => {
                            let l_str = reduce_expr_to_string(l_val, doc, scope);
                            let r_str = reduce_expr_to_string(r_val, doc, scope);
                            return Some(ExprValue::LiteralString(format!("{}{}", l_str, r_str)));
                        }
                        _ => {}
                    };
                }
                _ => {}
            };
            Some(ExprValue::LiteralString("[invalid expression]".to_owned()))
        }

        &ExprValue::SymbolReference(ref sym) => {
            if let Some(ref expr) = eval_sym(sym, doc, &scope) {
                return Some(expr.clone());
            };
            Some(expr.clone())
        }

        _ => Some(expr.clone())
    }
}

#[inline]
#[allow(dead_code)]
pub fn resolve_document_symbol(sym: &Symbol, doc: &DocumentState, scope: &mut ElementOpScope) -> Option<Symbol> {
    if let &SymbolReferenceType::UnresolvedReference(ref key) = sym.sym_ref() {
        if let Some(_) = scope.1.params.get(key) {
            return Some(Symbol::param(key));
        };

        if let Some(_) = scope.1.props.get(key) {
            return Some(Symbol::prop(key));
        };

        if let Some(reducer_data) = doc.reducer_key_data.get(key) {
            if let Some(ref default_expr) = reducer_data.default_expr {
                scope.add_cached_reducer_key_with_value(key, default_expr);
                return Some(Symbol::reducer_key_with_value(key, default_expr));
            };
            
            return Some(Symbol::reducer_key(key));
        };

        if let Some(_) = scope.1.block_params.get(key) {
            return Some(Symbol::block_param(key));
        };

        if let Some(value_binding) = scope.1.element_value_bindings.get(key) {
            return Some(value_binding.to_owned());
        };
    }

    return None;
}

pub fn write_computed_expr_value(w: &mut io::Write,
                                 node: &ExprValue,
                                 doc: &DocumentState,
                                 scope: &ElementOpScope)
                                 -> Result {
    match node {
        &ExprValue::LiteralString(ref s) => {
            write!(w, "{}", s)?;
        }
        &ExprValue::LiteralNumber(ref n) => {
            write!(w, "{}", n)?;
        }

        &ExprValue::LiteralArray(ref items) => {
            if let &Some(ref items) = items {
                for ref item in items {
                    write_computed_expr_value(w, item, doc, scope)?;
                }
            };
        }

        &ExprValue::DefaultVariableReference => {
            let default_var = scope.0.default_var().unwrap_or("value".to_owned());
            write!(w, "{}", default_var)?;
        }

        &ExprValue::ContentNode(..) => {}
        &ExprValue::DefaultAction(..) => {}
        &ExprValue::Action(..) => {}

        _ => {
            if let Some(ref expr) = reduce_expr(node, doc, scope) {
                write_computed_expr_value(w, &expr, doc, scope)?;
            };
        }
    }
    Ok(())
}
