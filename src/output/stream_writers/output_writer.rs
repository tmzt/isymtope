
use std::io;
use std::slice::Iter;

use parser::ast::*;
use processing::structs::*;
// use processing::scope::*;
use scope::context::*;
use scope::bindings::*;


// pub type PropIterator = IntoIterator<Item = Prop>;
// pub type EventHandlerIterator = IntoIterator<Item = EventHandler>;
// pub type BindingIterator = IntoIterator<Item = ElementValueBinding>;

pub trait ExprWriter {
    fn write_expr(&mut self, w: &mut io::Write, value_writer: &mut ValueWriter, ctx: &mut Context, bindings: &BindingContext, expr: &ExprValue) -> Result;
}

pub trait ExpressionWriter { }

pub trait DynamicExpressionWriter : ExpressionWriter {
    fn write_dynamic_expression(&mut self, w: &mut io::Write, value_writer: &mut ValueWriter, ctx: &mut Context, bindings: &BindingContext, op: &ExprOp, left: &ExprValue, right: &ExprValue) -> Result;
}

pub trait ValueWriter {
    fn write_literal_string(&mut self, w: &mut io::Write, s: &str) -> Result;
    fn write_literal_number(&mut self, w: &mut io::Write, n: &i32) -> Result;
    fn write_binding(&mut self, w: &mut io::Write, ctx: &mut Context, bindings: &BindingContext, binding: &BindingType) -> Result;
}

#[derive(Debug, Default)]
pub struct DefaultExpressionWriter {}

fn common_write_expr(w: &mut io::Write, value_writer: &mut ValueWriter, ctx: &mut Context, bindings: &BindingContext, expr: &ExprValue) -> Result {
    match expr {
        &ExprValue::Binding(ref binding) => value_writer.write_binding(w, ctx, bindings, binding),
        _ => Ok(())
    }
}

impl<T:ExpressionWriter> ExprWriter for T {
    default fn write_expr(&mut self, w: &mut io::Write, value_writer: &mut ValueWriter, ctx: &mut Context, bindings: &BindingContext, expr: &ExprValue) -> Result {
        self::common_write_expr(w, value_writer, ctx, bindings, expr)
    }
}

impl ExpressionWriter for DefaultExpressionWriter {}

impl<T: DynamicExpressionWriter> ExprWriter for T {
    default fn write_expr(&mut self, w: &mut io::Write, value_writer: &mut ValueWriter, ctx: &mut Context, bindings: &BindingContext, expr: &ExprValue) -> Result {
        if let &ExprValue::Expr(ref op, ref left, ref right) = expr {
            return self.write_dynamic_expression(w, value_writer, ctx, bindings, op, left, right);
        };

        self::common_write_expr(w, value_writer, ctx, bindings, expr)
    }
}