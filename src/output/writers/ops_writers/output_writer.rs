
use std::io;
use std::iter;

use parser::ast::*;
use processing::structs::*;
use scope::context::*;
use scope::bindings::*;
use output::stream_writers::output_stream_writer::*;


pub trait ExprWriter {
    type E: ExpressionWriter;
    fn write_expr(&mut self, w: &mut io::Write, ctx: &mut Context, bindings: &BindingContext, expr: &ExprValue) -> Result;
}

pub trait ElementOpsWriter {
    type E: ExpressionWriter;
    type S: ElementOpsStreamWriter<E = Self::E>;

    fn write_element_op(&mut self, w: &mut io::Write, ctx: &mut Context, bindings: &BindingContext, op: &ElementOp) -> Result;
    fn write_element_ops<'a, I: IntoIterator<Item = &'a ElementOp>>(&mut self, w: &mut io::Write, ctx: &mut Context, bindings: &BindingContext, ops: I) -> Result;
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
pub struct DefaultOutputWriter<V: ValueWriter, E: ExpressionWriter<V = V>, S: ElementOpsStreamWriter> {
    value_writer: V,
    expression_writer: E,
    stream_writer: S
}

impl<V: ValueWriter, E: ExpressionWriter<V = V>, S: ElementOpsStreamWriter> ExprWriter for DefaultOutputWriter<V, E, S> {
    // type E = ExpressionWriterJs;
    type E = E;

    fn write_expr(&mut self, w: &mut io::Write, ctx: &mut Context, bindings: &BindingContext, expr: &ExprValue) -> Result {
        self.expression_writer.write_expr_to(w, &mut self.value_writer, ctx, bindings, expr)
    }
}

fn write_element_op<S: ElementOpsStreamWriter>(w: &mut io::Write, stream_writer: &mut S, expression_writer: &mut S::E, value_writer: &mut <S::E as ExpressionWriter>::V, ctx: &mut Context, bindings: &BindingContext, op: &ElementOp) -> Result {

    let is_void = if let &ElementOp::ElementVoid(..) = op { true } else { false };

    match op {
            &ElementOp::ElementOpen(ref element_tag,
                                    ref element_key,
                                    ref props,
                                    ref events,
                                    ref value_binding) |
            &ElementOp::ElementVoid(ref element_tag,
                                    ref element_key,
                                    ref props,
                                    ref events,
                                    ref value_binding) => {
                ctx.push_child_scope();
                ctx.append_path_str(element_key);

                // let props = if output_component_contents {
                //     props.as_ref().map(|props| props.iter().map(|p| ctx.reduce_expr()))
                //     // props.as_ref().map(|p| map_props_using_scope(p.iter(), &scope))
                // } else {
                //     props.as_ref().map(|props| props.iter().map(|p| ctx.reduce_expr()))
                //     // props.as_ref().map(|p| map_prop_references(p.iter(), &scope))
                // };

                // let props = props.as_ref().map(|props| props.iter().filter_map(|p| p.1.as_ref().map(|expr| ctx.reduce_expr(expr))));


                // let prop_list = prop_list.as_ref().map(|s| s.iter().map(|s| &s));

                // let complete_key = scope.0.make_complete_element_key_with(element_key);
                // self.push_scope_as(scope.clone(), &complete_key);

                // let events = events.as_ref().map(|events| events.iter());
                // let value_binding = value_binding.as_ref().map(|s| s.clone());

                // let events = events.as_ref().map_or_else(|| iter::empty(), |v| v.iter());
                // let value_bindings = value_bindings.as_ref().map_or_else(|| iter::empty(), |v| v.iter());

                stream_writer.write_op_element_open(
                    w,
                    expression_writer,
                    value_writer,
                    ctx,
                    bindings,
                    element_tag,
                    element_key,
                    is_void,
                    iter::empty(),
                    iter::empty(),
                    iter::empty(),
                )?;

                // self.stream_writer.write_op_element_open(
                //     w,
                //     self,
                //     ctx,
                //     bindings,
                //     element_key,
                //     element_tag,
                //     is_void,
                //     props,
                //     events,
                //     value_bindings
                // )?;



                // self.stream_writer
                //     .write_op_element(w,
                //                       op,
                //                       doc,
                //                       &scope,
                //                       &complete_key,
                //                       element_tag,
                //                       is_void,
                //                       props.as_ref().map(|s| s.iter()),
                //                       events,
                //                       value_bindings)?;

                if is_void {
                    // Pop scope for self closing, this fixes issue with ElementVoid which
                    // was not being emitted previously by the parser/processor code.
                    ctx.pop_scope();
                };
            }

            &ElementOp::ElementClose(ref element_tag) => {
                stream_writer.write_op_element_close(
                    w,
                    expression_writer,
                    value_writer,
                    ctx,
                    bindings,
                    element_tag,
                )?;

                ctx.pop_scope();
            }

            _ => {}
    };

    Ok(())
}

impl<V: ValueWriter, E: ExpressionWriter<V = V>, S: ElementOpsStreamWriter<E = E>> ElementOpsWriter for DefaultOutputWriter<V, E, S> {
    type E = E;
    type S = S;

    fn write_element_op(&mut self, w: &mut io::Write, ctx: &mut Context, bindings: &BindingContext, op: &ElementOp) -> Result {
        write_element_op(w, &mut self.stream_writer, &mut self.expression_writer, &mut self.value_writer, ctx, bindings, op)
    }

    fn write_element_ops<'a, I: IntoIterator<Item = &'a ElementOp>>(&mut self, w: &mut io::Write, ctx: &mut Context, bindings: &BindingContext, ops: I) -> Result {
        for op in ops {
            write_element_op(w, &mut self.stream_writer, &mut self.expression_writer, &mut self.value_writer, ctx, bindings, op)?;
        }
        Ok(())
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