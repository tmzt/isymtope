pub mod expr_writer_html;
pub mod expr_writer_js;

pub use self::expr_writer_html::{ValueWriterHtml, ExpressionWriterHtml};
pub use self::expr_writer_js::{ValueWriterJs, ExpressionWriterJs};

use std::io;

use parser::*;
use processing::*;
use scope::*;


pub trait ExprWriter {
    type E: ExpressionWriter;
    fn write_expr(&mut self, w: &mut io::Write, ctx: &mut Context, bindings: &BindingContext, expr: &ExprValue) -> Result;
}

fn common_write_expr(w: &mut io::Write, value_writer: &mut ValueWriter, ctx: &mut Context, bindings: &BindingContext, expr: &ExprValue) -> Result {
    match expr {
        &ExprValue::LiteralString(ref s) => value_writer.write_literal_string(w, s),
        &ExprValue::LiteralNumber(ref n) => value_writer.write_literal_number(w, n),
        &ExprValue::LiteralBool(b) => value_writer.write_literal_bool(w, b),
        _ => value_writer.write_undefined(w)
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

        if let &ExprValue::LiteralArray(ref arr) = expr {
            let arr_iter = arr.as_ref().map(|arr| arr.iter());
            return self.write_array(w, value_writer, ctx, bindings, arr_iter, None);
        };

        if let &ExprValue::LiteralObject(ref props) = expr {
            let props_iter = props.as_ref().map(|arr| arr.iter());
            return self.write_props(w, value_writer, ctx, bindings, props_iter);
        };

        if let &ExprValue::SymbolReference(ref sym) = expr {
            return self.write_symbol(w, value_writer, ctx, bindings, sym);
        };

        if let &ExprValue::Binding(ref binding) = expr {
            return self.write_binding(w, value_writer, ctx, bindings, binding);
        };

        self::common_write_expr(w, value_writer, ctx, bindings, expr)
    }

    // fn write_expr(&mut self, w: &mut io::Write, value_writer: &mut Self::V, ctx: &mut Context, bindings: &BindingContext, expr: &ExprValue) -> Result;
    fn write_expression(&mut self, w: &mut io::Write, value_writer: &mut Self::V, ctx: &mut Context, bindings: &BindingContext, op: &ExprOp, left: &ExprValue, right: &ExprValue) -> Result;
    fn write_apply_expression<'a, I: IntoIterator<Item = &'a ExprValue>>(&mut self, w: &mut io::Write, value_writer: &mut Self::V, ctx: &mut Context, bindings: &BindingContext, a_op: &ExprApplyOp, arr: Option<I>) -> Result;
    fn write_array<'a, I: IntoIterator<Item = &'a ExprValue>>(&mut self, w: &mut io::Write, value_writer: &mut Self::V, ctx: &mut Context, bindings: &BindingContext, arr: Option<I>, ty: Option<VarType>) -> Result;
    fn write_props<'a, I: IntoIterator<Item = &'a Prop>>(&mut self, w: &mut io::Write, value_writer: &mut Self::V, ctx: &mut Context, bindings: &BindingContext, props: Option<I>) -> Result;
    fn write_binding(&mut self, w: &mut io::Write, value_writer: &mut Self::V, ctx: &mut Context, bindings: &BindingContext, binding: &BindingType) -> Result;

    fn write_symbol(&mut self, w: &mut io::Write, value_writer: &mut Self::V, ctx: &mut Context, bindings: &BindingContext, sym: &Symbol) -> Result {
        match sym.sym_ref() {
            &SymbolReferenceType::Binding(ref binding) => self.write_binding(w, value_writer, ctx, bindings, binding),
            &SymbolReferenceType::InitialValue(_, box ref after) => self.write_symbol(w, value_writer, ctx, bindings, after),
            _ => value_writer.write_undefined(w)
        }
    }
}

pub trait DynamicExpressionWriter : ExpressionWriter { }
pub trait StaticExpressionWriter : ExpressionWriter { }

pub trait ValueWriter {
    fn write_literal_string(&mut self, w: &mut io::Write, s: &str) -> Result;
    fn write_literal_number(&mut self, w: &mut io::Write, n: &i32) -> Result;
    fn write_literal_bool(&mut self, w: &mut io::Write, b: bool) -> Result;
    // fn write_literal_array<'a, I: IntoIterator<Item = &'a ExprValue>> (&mut self, w: &mut io::Write, iter: I, ty: Option<VarType>) -> Result;
    fn write_simple_binding(&mut self, w: &mut io::Write, ctx: &mut Context, bindings: &BindingContext, binding: &BindingType) -> Result;
    fn write_op(&mut self, w: &mut io::Write, op: &ExprOp) -> Result;
    fn write_undefined(&mut self, w: &mut io::Write) -> Result;
}

pub trait DynamicValueWriter : ValueWriter {}
pub trait StaticValueWriter : ValueWriter {}


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

        fn write_literal_bool(&mut self, w: &mut io::Write, b: bool) -> Result {
            Ok(())
        }

        fn write_simple_binding(&mut self, w: &mut io::Write, ctx: &mut Context, bindings: &BindingContext, binding: &BindingType) -> Result {
            if (!self.wrote_binding.is_some()) {
                self.wrote_binding = Some(binding.clone());
            }
            Ok(())
        }

        fn write_op(&mut self, w: &mut io::Write, op: &ExprOp) -> Result {
            Ok(())
        }

        fn write_undefined(&mut self, w: &mut io::Write) -> Result {
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

        fn write_array<'a, I: IntoIterator<Item = &'a ExprValue>>(&mut self, w: &mut io::Write, value_writer: &mut Self::V, ctx: &mut Context, bindings: &BindingContext, arr: Option<I>, ty: Option<VarType>) -> Result {
            Ok(())
        }

        fn write_props<'a, I: IntoIterator<Item = &'a Prop>>(&mut self, w: &mut io::Write, value_writer: &mut Self::V, ctx: &mut Context, bindings: &BindingContext, props: Option<I>) -> Result {
            Ok(())
        }

        fn write_binding(&mut self, w: &mut io::Write, value_writer: &mut Self::V, ctx: &mut Context, bindings: &BindingContext, binding: &BindingType) -> Result {
            Ok(())
        }

        fn write_symbol(&mut self, w: &mut io::Write, value_writer: &mut Self::V, ctx: &mut Context, bindings: &BindingContext, sym: &Symbol) -> Result {
            Ok(())
        }
    }

    #[test]
    fn test_output_writer_default_expression_writer_write_binding_expression() {
        let bindings = BindingContext::default();
        let mut ctx = Context::default();
        let mut value_writer = TestDynamicValueWriter::default();
        let mut s: Vec<u8> = Default::default();

        let mut expression_writer = TestDynamicExpressionWriter::default();
        let binding = BindingType::ReducerPathBinding("todos".into());
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
        let binding = BindingType::ReducerPathBinding("todos".into());

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