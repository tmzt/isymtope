pub mod expr_writer_html;
pub mod expr_writer_js;

pub use self::expr_writer_html::{ValueWriterHtml, ExpressionWriterHtml};
pub use self::expr_writer_js::{ValueWriterJs, ExpressionWriterJs};

use std::io;

use model::*;
use parser::*;
use processing::*;
use scope::*;


pub trait ExprWriter {
    type E: ExpressionWriter;
    fn write_expr(&mut self, w: &mut io::Write, doc: &Document, ctx: &mut Context, bindings: &BindingContext, expr: &ExprValue) -> Result;
}

fn common_write_expr(w: &mut io::Write, value_writer: &mut ValueWriter, _ctx: &mut Context, _bindings: &BindingContext, expr: &ExprValue) -> Result {
    match *expr {
        ExprValue::LiteralString(ref s) => value_writer.write_literal_string(w, s),
        ExprValue::LiteralNumber(ref n) => value_writer.write_literal_number(w, n),
        ExprValue::LiteralBool(b) => value_writer.write_literal_bool(w, b),
        _ => value_writer.write_undefined(w)
    }
}

pub trait ExpressionWriter {
    type V: ValueWriter;

    #[inline]
    fn write_expr_to(&mut self, w: &mut io::Write, doc: &Document, value_writer: &mut Self::V, ctx: &mut Context, bindings: &BindingContext, expr: &ExprValue) -> Result {
        match *expr {
            ExprValue::Expr(ref op, ref left, ref right) => {
                self.write_expression(w, doc, value_writer, ctx, bindings, op, left, right)
            }

            ExprValue::TestValue(ref op, box ref left, ref right) => {
                let right = right.as_ref().map(|&box ref right| right);
                self.write_test(w, doc, value_writer, ctx, bindings, op, left, right)
            }

            ExprValue::Apply(ref a_op, ref arr) => {
                let arr_iter = arr.as_ref().map(|arr| arr.iter().map(|i| i.as_ref()));
                self.write_apply_expression(w, doc, value_writer, ctx, bindings, a_op, arr_iter)
            }

            ExprValue::LiteralArray(ref arr) => {
                let arr_iter = arr.as_ref().map(|arr| arr.iter());
                self.write_array(w, doc, value_writer, ctx, bindings, arr_iter, None)
            }

            ExprValue::LiteralObject(ref props) => {
                let props_iter = props.as_ref().map(|arr| arr.iter());
                self.write_props(w, doc, value_writer, ctx, bindings, props_iter)
            }

            ExprValue::SymbolReference(ref sym) => {
                self.write_symbol(w, doc, value_writer, ctx, bindings, sym)
            }

            ExprValue::Group(ref inner_expr) => {
                let inner_expr = match inner_expr { &Some(box ref e) => Some(e), _ => None };
                self.write_group(w, doc, value_writer, ctx, bindings, inner_expr)
            }

            ExprValue::Binding(ref binding) => {
                self.write_binding(w, doc, value_writer, ctx, bindings, binding)
            }

            ExprValue::ReducedPipeline(ref head, Some(ref parts)) => {
                let head = head.as_ref().map(|&box ref head| head);
                self.write_pipeline(w, doc, value_writer, ctx, bindings, head, parts.iter())
            }

            // Strip off lens wrapper at this point
            ExprValue::Lens(box ref lens) => {
                self.write_lens_inner(w, doc, value_writer, ctx, bindings, lens)
            }

            _ => self::common_write_expr(w, value_writer, ctx, bindings, expr)
        }
    }

    // fn write_expr(&mut self, w: &mut io::Write, doc: &Document, value_writer: &mut Self::V, ctx: &mut Context, bindings: &BindingContext, expr: &ExprValue) -> Result;
    fn write_expression(&mut self, w: &mut io::Write, doc: &Document, value_writer: &mut Self::V, ctx: &mut Context, bindings: &BindingContext, op: &ExprOp, left: &ExprValue, right: &ExprValue) -> Result;
    fn write_test(&mut self, w: &mut io::Write, doc: &Document, value_writer: &mut Self::V, ctx: &mut Context, bindings: &BindingContext, op: &TestOp, left: &ExprValue, right: Option<&ExprValue>) -> Result;
    fn write_apply_expression<'a, I: IntoIterator<Item = &'a ExprValue>>(&mut self, w: &mut io::Write, doc: &Document, value_writer: &mut Self::V, ctx: &mut Context, bindings: &BindingContext, a_op: &ExprApplyOp, arr: Option<I>) -> Result;
    fn write_array<'a, I: IntoIterator<Item = &'a ExprValue>>(&mut self, w: &mut io::Write, doc: &Document, value_writer: &mut Self::V, ctx: &mut Context, bindings: &BindingContext, arr: Option<I>, ty: Option<VarType>) -> Result;
    fn write_props<'a, I: IntoIterator<Item = &'a Prop>>(&mut self, w: &mut io::Write, doc: &Document, value_writer: &mut Self::V, ctx: &mut Context, bindings: &BindingContext, props: Option<I>) -> Result;
    fn write_formal_params_list<'a, I: IntoIterator<Item = FormalPropRef<'a>>>(&mut self, w: &mut io::Write, doc: &Document, value_writer: &mut Self::V, ctx: &mut Context, bindings: &BindingContext, params: I) -> Result;
    fn write_actual_props<'a, I: IntoIterator<Item = ActualPropRef<'a>>>(&mut self, w: &mut io::Write, doc: &Document, value_writer: &mut Self::V, ctx: &mut Context, bindings: &BindingContext, props: Option<I>) -> Result;
    fn write_binding(&mut self, w: &mut io::Write, doc: &Document, value_writer: &mut Self::V, ctx: &mut Context, bindings: &BindingContext, binding: &BindingType) -> Result;
    fn write_group(&mut self, w: &mut io::Write, doc: &Document, value_writer: &mut Self::V, ctx: &mut Context, bindings: &BindingContext, inner_expr: Option<&ExprValue>) -> Result;

    fn write_symbol(&mut self, w: &mut io::Write, doc: &Document, value_writer: &mut Self::V, ctx: &mut Context, bindings: &BindingContext, sym: &Symbol) -> Result {
        match *sym.sym_ref() {
            SymbolReferenceType::Binding(ref binding) => self.write_binding(w, doc, value_writer, ctx, bindings, binding),
            SymbolReferenceType::InitialValue(_, box ref after) => self.write_symbol(w, doc, value_writer, ctx, bindings, after),
            _ => value_writer.write_undefined(w)
        }
    }

    // Write the value of the lens, stripping off the wrapping
    fn write_lens_inner(&mut self, w: &mut io::Write, doc: &Document, value_writer: &mut Self::V, ctx: &mut Context, bindings: &BindingContext, lens: &LensExprType) -> Result {
        if let Some(expr) = lens.expr() {
            return self.write_expr_to(w, doc, value_writer, ctx, bindings, expr);            
        };
        Ok(())
    }

    fn write_pipeline<'a, I: IntoIterator<Item = &'a ReducedPipelineComponent>>(&mut self, w: &mut io::Write, doc: &Document, value_writer: &mut Self::V, ctx: &mut Context, bindings: &BindingContext, head: Option<&ExprValue>, parts: I) -> Result;
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

        fn write_expression(&mut self, _w: &mut io::Write, _doc: &Document, _value_writer: &mut Self::V, _ctx: &mut Context, _bindings: &BindingContext, op: &ExprOp, left: &ExprValue, right: &ExprValue) -> Result {
            assert!(self.wrote_op.is_none() && self.wrote_left.is_none() && self.wrote_right.is_none(), "Called method more than once.");

            self.wrote_op = Some(op.clone());
            self.wrote_left = Some(left.clone());
            self.wrote_right = Some(right.clone());
            Ok(())
        }

        fn write_test(&mut self, _w: &mut io::Write, _doc: &Document, _value_writer: &mut Self::V, _ctx: &mut Context, _bindings: &BindingContext, _op: &TestOp, _left: &ExprValue, _right: Option<&ExprValue>) -> Result {
            Ok(())
        }

        fn write_apply_expression<'a, I: IntoIterator<Item = &'a ExprValue>>(&mut self, _w: &mut io::Write, _doc: &Document, _value_writer: &mut Self::V, _ctx: &mut Context, _bindings: &BindingContext, _a_op: &ExprApplyOp, _arr: Option<I>) -> Result {
            Ok(())
        }

        fn write_array<'a, I: IntoIterator<Item = &'a ExprValue>>(&mut self, _w: &mut io::Write, _doc: &Document, _value_writer: &mut Self::V, _ctx: &mut Context, _bindings: &BindingContext, _arr: Option<I>, _ty: Option<VarType>) -> Result {
            Ok(())
        }

        fn write_props<'a, I: IntoIterator<Item = &'a Prop>>(&mut self, _w: &mut io::Write, _doc: &Document, _value_writer: &mut Self::V, _ctx: &mut Context, _bindings: &BindingContext, _props: Option<I>) -> Result {
            Ok(())
        }

        fn write_formal_params_list<'a, I: IntoIterator<Item = FormalPropRef<'a>>>(&mut self, w: &mut io::Write, doc: &Document, value_writer: &mut Self::V, ctx: &mut Context, bindings: &BindingContext, params: I) -> Result {
            Ok(())
        }

        fn write_actual_props<'a, I: IntoIterator<Item = ActualPropRef<'a>>>(&mut self, w: &mut io::Write, doc: &Document, value_writer: &mut Self::V, ctx: &mut Context, bindings: &BindingContext, props: Option<I>) -> Result {
            Ok(())
        }

        fn write_binding(&mut self, _w: &mut io::Write, _doc: &Document, _value_writer: &mut Self::V, _ctx: &mut Context, _bindings: &BindingContext, _binding: &BindingType) -> Result {
            Ok(())
        }

        fn write_group(&mut self, _w: &mut io::Write, _doc: &Document, _value_writer: &mut Self::V, _ctx: &mut Context, _bindings: &BindingContext, _inner_expr: Option<&ExprValue>) -> Result {
            Ok(())
        }

        fn write_symbol(&mut self, _w: &mut io::Write, _doc: &Document, _value_writer: &mut Self::V, _ctx: &mut Context, _bindings: &BindingContext, sym: &Symbol) -> Result {
            Ok(())
        }

        fn write_pipeline<'a, I: IntoIterator<Item = &'a ReducedPipelineComponent>>(&mut self, _w: &mut io::Write, _doc: &Document, _value_writer: &mut Self::V, _ctx: &mut Context, _bindings: &BindingContext, _head: Option<&ExprValue>, _parts: I) -> Result {
            Ok(())
        }
    }

    fn create_document<'a>(template: &'a Template) -> Document {
        let mut ctx = Context::default();
        let mut bindings = BindingContext::default();
        let mut processing = ProcessDocument::from_template(&template);
        assert!(processing.process_document(&mut ctx, &mut bindings).is_ok());
        processing.into()
    }

    #[test]
    fn test_output_writer_default_expression_writer_write_binding_expression() {
        let template = Template::new(vec![]);
        let doc = create_document(&template);

        let bindings = BindingContext::default();
        let mut ctx = Context::default();
        let mut value_writer = TestDynamicValueWriter::default();
        let mut s: Vec<u8> = Default::default();

        let mut expression_writer = TestDynamicExpressionWriter::default();
        let binding = BindingType::ReducerPathBinding("todos".into());
        let expr = ExprValue::Binding(binding.clone());
        assert!(expression_writer.write_expr_to(&mut s, &doc, &mut value_writer, &mut ctx, &bindings, &expr).is_ok());
        assert_eq!(value_writer.wrote_binding, Some(binding));
    }

    #[test]
    fn test_output_writer_dynamic_expression_writer() {
        let template = Template::new(vec![]);
        let doc = create_document(&template);

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
        
        assert!(expression_writer.write_expr_to(&mut s, &doc, &mut value_writer, &mut ctx, &bindings, &expr).is_ok());
        assert_eq!(expression_writer.wrote_op, Some(ExprOp::Add));
        assert_eq!(expression_writer.wrote_left, Some(left));
        assert_eq!(expression_writer.wrote_right, Some(right));
    }

}