
use std::io;

use parser::ast::*;
use processing::structs::*;
use scope::context::*;
use scope::bindings::*;


pub trait ExprWriter {
    type E: ExpressionWriter;
    fn write_expr(&mut self, w: &mut io::Write, ctx: &mut Context, bindings: &BindingContext, expr: &ExprValue) -> Result;
}

fn common_write_expr(w: &mut io::Write, value_writer: &mut ValueWriter, ctx: &mut Context, bindings: &BindingContext, expr: &ExprValue) -> Result {
    match expr {
        &ExprValue::Binding(ref binding) => value_writer.write_binding(w, ctx, bindings, binding),
        &ExprValue::LiteralString(ref s) => value_writer.write_literal_string(w, s),
        &ExprValue::LiteralNumber(ref n) => value_writer.write_literal_number(w, n),
        _ => Ok(())
    }
}

pub trait ExpressionWriter {
    type V: ValueWriter;

    #[inline]
    fn write_expr_to(&mut self, w: &mut io::Write, value_writer: &mut Self::V, ctx: &mut Context, bindings: &BindingContext, expr: &ExprValue) -> Result {
        if let &ExprValue::Expr(ref op, ref left, ref right) = expr {
            return self.write_expression(w, value_writer, ctx, bindings, op, left, right);
        };

        if let &ExprValue::Apply(ref a_op, ref arr) = expr {
            let arr_iter = arr.as_ref().map(|arr| arr.iter().map(|i| i.as_ref()));
            return self.write_apply_expression(w, value_writer, ctx, bindings, a_op, arr_iter);
        };

        self::common_write_expr(w, value_writer, ctx, bindings, expr)
    }

    // fn write_expr(&mut self, w: &mut io::Write, value_writer: &mut Self::V, ctx: &mut Context, bindings: &BindingContext, expr: &ExprValue) -> Result;
    fn write_expression(&mut self, w: &mut io::Write, value_writer: &mut Self::V, ctx: &mut Context, bindings: &BindingContext, op: &ExprOp, left: &ExprValue, right: &ExprValue) -> Result;
    fn write_apply_expression<'a, I: IntoIterator<Item = &'a ExprValue>>(&mut self, w: &mut io::Write, value_writer: &mut Self::V, ctx: &mut Context, bindings: &BindingContext, a_op: &ExprApplyOp, arr: Option<I>) -> Result;
}

pub trait DynamicExpressionWriter : ExpressionWriter { }
pub trait StaticExpressionWriter : ExpressionWriter { }

pub trait ValueWriter {
    fn write_literal_string(&mut self, w: &mut io::Write, s: &str) -> Result;
    fn write_literal_number(&mut self, w: &mut io::Write, n: &i32) -> Result;
    fn write_binding(&mut self, w: &mut io::Write, ctx: &mut Context, bindings: &BindingContext, binding: &BindingType) -> Result;
    fn write_op(&mut self, w: &mut io::Write, op: &ExprOp) -> Result;
}

pub trait DynamicValueWriter : ValueWriter {}
pub trait StaticValueWriter : ValueWriter {}

#[derive(Debug, Default)]
pub struct DefaultOutputWriter<V: ValueWriter, E: ExpressionWriter<V = V>> {
    value_writer: V,
    expression_writer: E
}

impl<V: ValueWriter, E: ExpressionWriter<V = V>> ExprWriter for DefaultOutputWriter<V, E> {
    // type E = ExpressionWriterJs;
    type E = E;

    fn write_expr(&mut self, w: &mut io::Write, ctx: &mut Context, bindings: &BindingContext, expr: &ExprValue) -> Result {
        self.expression_writer.write_expr_to(w, &mut self.value_writer, ctx, bindings, expr)
    }
}


#[cfg(test)]
mod tests {
    use super::*;
    use scope::bindings::*;
    use scope::context::*;

    #[derive(Debug, Default)]
    struct TestDynamicValueWriter { pub wrote_binding: Option<BindingType> }
    impl ValueWriter for TestDynamicValueWriter {
        fn write_literal_string(&mut self, w: &mut io::Write, s: &str) -> Result {
            Ok(())
        }

        fn write_literal_number(&mut self, w: &mut io::Write, n: &i32) -> Result {
            Ok(())
        }

        fn write_binding(&mut self, w: &mut io::Write, ctx: &mut Context, bindings: &BindingContext, binding: &BindingType) -> Result {
            if (!self.wrote_binding.is_some()) {
                self.wrote_binding = Some(binding.clone());
            }
            Ok(())
        }

        fn write_op(&mut self, w: &mut io::Write, op: &ExprOp) -> Result {
            Ok(())
        }
    }
    impl DynamicValueWriter for TestDynamicValueWriter {}

    #[derive(Debug, Default)]
    struct TestDynamicExpressionWriter { wrote_op: Option<ExprOp>, wrote_left: Option<ExprValue>, wrote_right: Option<ExprValue> }
    impl ExpressionWriter for TestDynamicExpressionWriter {
        type V = TestDynamicValueWriter;

        fn write_expression(&mut self, w: &mut io::Write, value_writer: &mut Self::V, ctx: &mut Context, bindings: &BindingContext, op: &ExprOp, left: &ExprValue, right: &ExprValue) -> Result {
            assert!(self.wrote_op.is_none() && self.wrote_left.is_none() && self.wrote_right.is_none(), "Called method more than once.");

            self.wrote_op = Some(op.clone());
            self.wrote_left = Some(left.clone());
            self.wrote_right = Some(right.clone());
            Ok(())
        }

        fn write_apply_expression<'a, I: IntoIterator<Item = &'a ExprValue>>(&mut self, w: &mut io::Write, value_writer: &mut Self::V, ctx: &mut Context, bindings: &BindingContext, a_op: &ExprApplyOp, arr: Option<I>) -> Result {
            Ok(())
        }
    }
    // impl DynamicExpressionWriter for TestDynamicExpressionWriter {}

    #[test]
    fn test_output_writer_default_expression_writer_write_binding_expression() {
        let bindings = BindingContext::default();
        let mut ctx = Context::default();
        let mut value_writer = TestDynamicValueWriter::default();
        let mut s: Vec<u8> = Default::default();

        let mut expression_writer = TestDynamicExpressionWriter::default();
        let binding = BindingType::ReducerPathBinding("todos".into(), None);
        let expr = ExprValue::Binding(binding.clone());
        assert!(expression_writer.write_expr_to(&mut s, &mut value_writer, &mut ctx, &bindings, &expr).is_ok());
        assert_eq!(value_writer.wrote_binding, Some(binding));
    }

    #[test]
    fn test_output_writer_dynamic_expression_writer() {
        let bindings = BindingContext::default();
        let mut ctx = Context::default();
        let mut value_writer = TestDynamicValueWriter::default();
        let mut s: Vec<u8> = Default::default();

        let mut expression_writer = TestDynamicExpressionWriter::default();
        let binding = BindingType::ReducerPathBinding("todos".into(), None);

        let left = ExprValue::Binding(binding.clone());
        let right = ExprValue::LiteralString("test".into());
        let expr = ExprValue::Expr(ExprOp::Add,
            Box::new(left.clone()),
            Box::new(right.clone())
        );
        
        assert!(expression_writer.write_expr_to(&mut s, &mut value_writer, &mut ctx, &bindings, &expr).is_ok());
        assert_eq!(expression_writer.wrote_op, Some(ExprOp::Add));
        assert_eq!(expression_writer.wrote_left, Some(left));
        assert_eq!(expression_writer.wrote_right, Some(right));
    }

}