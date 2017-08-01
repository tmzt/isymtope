
use std::io;

use parser::ast::*;
use processing::structs::*;
use output::scope::*;


pub fn reduce_expr_to_string(expr: &ExprValue, doc: &DocumentState, scope: &ElementOpScope) -> String {
    match expr {
        &ExprValue::LiteralString(ref s) => format!("{}", s),
        &ExprValue::LiteralNumber(ref n) => format!("{}", n),
        &ExprValue::LiteralArray(..) => format!("[array]"),
        _ => format!("[invalid]")
    }
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
            match sym {
                &Symbol(ref sym, _, _) => {
                    match sym {
                        &SymbolReferenceType::ReducerKeyReference(ref as_reducer_key) => {
                            if let Some(ref reducer_data) = doc.reducer_key_data.get(as_reducer_key) {
                                if let Some(ref default_expr) = reducer_data.default_expr {
                                    return reduce_expr(default_expr, doc, scope);
                                };
                            };
                        }

                        &SymbolReferenceType::LoopVarReference(ref var_name) => {
                            if let Some(ref reducer_data) = doc.reducer_key_data.get(var_name) {
                                if let Some(ref default_expr) = reducer_data.default_expr {
                                    return reduce_expr(default_expr, doc, scope);
                                };
                            };
                        }

                        &SymbolReferenceType::PropReference(ref prop_name) => {
                            if let Some(ref reducer_data) = doc.reducer_key_data.get(prop_name) {
                                if let Some(ref default_expr) = reducer_data.default_expr {
                                    return reduce_expr(default_expr, doc, scope);
                                };
                            };
                        }

                        _ => {}
                    };
                }
                _ => {}
            };
            None
        }

        _ => None
    }
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

        // &ExprValue::VariableReference(ref var_name) => {
        //     let var_key = scope.0.var_prefix(var_name);
        //     write!(w, "{}", var_key)?;
        // }

        // &ExprValue::SymbolReference(ref sym) => {
        //     let expr = reduce_expr(expr, )
        //     match sym {
        //         &(Some(ref sym), _) => {
        //             match sym {
        //                 &SymbolReferenceType::LocalVarReference(ref var_name) => {
        //                     write!(w, "{}", var_name)?;
        //                 }

        //                 &SymbolReferenceType::ParameterReference(ref param_key) => {
        //                     write!(w, "{}", param_key)?;
        //                 }

        //                 &SymbolReferenceType::ReducerKeyReference(ref as_reducer_key) => {
        //                     if let Some(ref reducer_data) = doc.reducer_key_data.get(as_reducer_key) {
        //                         if let Some(ref default_expr) = reducer_data.default_expr {
        //                             write_computed_expr_value(w, default_expr, doc, scope)?;
        //                             return Ok(());
        //                         };
        //                     };

        //                     write!(w, "{}", as_reducer_key)?;
        //                 }

        //                 &SymbolReferenceType::LoopVarReference(ref var_name) => {
        //                     if let Some(ref eval_scope) = scope.2 {
        //                         if let Some(ref value) = eval_scope.symbol_values.get(var_name) {
        //                             if let SymbolValueType::ConstantValue(ref expr) = value.0 {
        //                                 write_computed_expr_value(w, expr, doc, scope)?;
        //                                 return Ok(());
        //                             }
        //                         }

        //                     }
        //                     write!(w, "{}", var_name)?;
        //                 }

        //                 _ => {}
        //             };
        //         }
        //         _ => {}
        //     };
        // }

        // &ExprValue::Expr(..) => {
        //     let expr = reduce_expr(node, doc, scope);
        //     write_computed_expr_value(w, &expr, doc, scope)?;
        // }

        // &ExprValue::ContentNode(..) => {}

        // &ExprValue::DefaultAction(..) => {}

        // &ExprValue::Action(..) => {}
    }
    Ok(())
}
