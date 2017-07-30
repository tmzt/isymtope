
use std::io;

use parser::ast::*;
use processing::structs::*;
use output::scope::*;

pub fn write_computed_expr_value(w: &mut io::Write,
                                 node: &ExprValue,
                                 doc: &DocumentState,
                                 scope_prefixes: &ScopePrefixes)
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
                    write_computed_expr_value(w, item, doc, scope_prefixes)?;
                }
            };
        }

        &ExprValue::DefaultVariableReference => {
            let default_var = scope_prefixes.default_var().unwrap_or("value".to_owned());
            write!(w, "{}", default_var)?;
        }

        &ExprValue::VariableReference(ref var_name) => {
            let var_key = scope_prefixes.var_prefix(var_name);
            write!(w, "{}", var_key)?;
        }

        &ExprValue::SymbolReference(ref sym) => {
            match sym {
                &(Some(ref sym), _) => {
                    match sym {
                        &SymbolReferenceType::LocalVarReference(ref var_name) => {
                            write!(w, "{}", var_name)?;
                        }

                        &SymbolReferenceType::ParameterReference(ref param_key) => {
                            write!(w, "{}", param_key)?;
                        }

                        &SymbolReferenceType::ReducerKeyReference(ref as_reducer_key) => {
                            if let Some(ref reducer_data) = doc.reducer_key_data.get(as_reducer_key) {
                                if let Some(ref default_expr) = reducer_data.default_expr {
                                    write_computed_expr_value(w, default_expr, doc, scope_prefixes)?;
                                    return Ok(());
                                };
                            };

                            write!(w, "{}", as_reducer_key)?;
                        }

                        _ => {}
                    };
                }
                _ => {}
            };
        }

        &ExprValue::Expr(ref sym, ref l, ref r) => {
            // write!(w, "{:?} {:?} {:?}", l, sym, r)?;
            write_computed_expr_value(w, l, doc, scope_prefixes)?;
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
            write_computed_expr_value(w, r, doc, scope_prefixes)?;
        }

        &ExprValue::ContentNode(..) => {}

        &ExprValue::DefaultAction(..) => {}

        &ExprValue::Action(..) => {}
    }
    Ok(())
}
