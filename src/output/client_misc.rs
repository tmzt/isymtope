
use std::io;
use std::fmt;

use std::clone::Clone;
use std::slice::Iter;
use std::collections::hash_map::HashMap;
use parser::ast::*;
use parser::store::*;
use parser::api::*;
use parser::util::allocate_element_key;
use output::structs::*;

use super::client_js::*;
use super::client_html::*;

pub fn write_computed_expr_value(w: &mut io::Write,
                                 node: &ExprValue,
                                 var_prefix: Option<&str>,
                                 default_var: Option<&str>)
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
                    write_computed_expr_value(w, item, var_prefix, default_var)?;
                }
            };
        }

        &ExprValue::DefaultVariableReference => {
            if let Some(ref prefix) = var_prefix {
                write!(w, "{}", prefix)?;
            } else {
                write!(w, "value")?;
            }
        }

        &ExprValue::VariableReference(ref s) => {
            if let Some(ref prefix) = var_prefix {
                write!(w, "{}{}", prefix, s)?;
            } else {
                write!(w, "{}", s)?;
            }
        }

        &ExprValue::Expr(ref sym, ref l, ref r) => {
            // write!(w, "{:?} {:?} {:?}", l, sym, r)?;
            write_computed_expr_value(w, l, var_prefix, default_var)?;
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
            write_computed_expr_value(w, r, var_prefix, default_var)?;
        }

        &ExprValue::ContentNode(..) => {}

        &ExprValue::DefaultAction(..) => {}

        &ExprValue::Action(..) => {}
    }
    Ok(())
}
