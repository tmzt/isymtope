
use std::io;
use std::slice::Iter;

use parser::ast::*;
use processing::structs::*;
use processing::scope::*;
use scope::bindings::*;


// pub type PropIterator = IntoIterator<Item = Prop>;
// pub type EventHandlerIterator = IntoIterator<Item = EventHandler>;
// pub type BindingIterator = IntoIterator<Item = ElementValueBinding>;

pub trait ExprWriter {
    fn write_expr(&mut self, w: &mut ioo::Writer, ctx: &mut Context, expr: &ExprValue) -> Result;
}

pub trait ValueWriter {
    fn write_binding(&mut self, w: &mut io::Writer, ctx: &mut Context, binding: &BindingType) -> Result;
}

#[derive(Debug, Default)]
pub struct DefaultExprWriter {}

impl ExprWriter for DefaultExprWriter {
    fn write_expr(&mut self, w: &mut ioo::Writer, value_writer: &mut ValueWriter, ctx: &mut Context, expr: &ExprValue) -> Result {
        match expr {
            &ExprValue::Binding(ref binding) => {
                value_writer.write_binding(w, ctx, binding)?;
            }
        };

        Ok(())
    }
}