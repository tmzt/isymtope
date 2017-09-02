
use std::io;

use parser::ast::*;
use processing::structs::*;
use scope::context::*;
use scope::bindings::*;
use output::stream_writers::output_writer::*;


#[derive(Debug, Default)]
pub struct ValueWriterHtml {}

impl ValueWriter for ValueWriterHtml {
    fn write_literal_string(&mut self, w: &mut io::Write, s: &str) -> Result {
        Ok(())
    }

    fn write_literal_number(&mut self, w: &mut io::Write, n: &i32) -> Result {
        Ok(())
    }

    fn write_binding(&mut self, w: &mut io::Write, ctx: &mut Context, bindings: &BindingContext, binding: &BindingType) -> Result {
        Ok(())
    }

    fn write_op(&mut self, w: &mut io::Write, op: &ExprOp) -> Result {
        Ok(())
    }
}
impl StaticValueWriter for ValueWriterHtml {}

#[derive(Debug, Default)]
pub struct ExpressionWriterHtml {}

impl ExpressionWriter for ExpressionWriterHtml {
    type V = ValueWriterHtml;

    fn write_expression(&mut self, w: &mut io::Write, value_writer: &mut Self::V, ctx: &mut Context, bindings: &BindingContext, op: &ExprOp, left: &ExprValue, right: &ExprValue) -> Result {
        self.write_expr(w, ctx, bindings, left)?;
        value_writer.write_op(w, op)?;
        self.write_expr(w, ctx, bindings, right)?;

        Ok(())
    }

    fn write_apply_expression<'a, I: IntoIterator<Item = &'a ExprValue>>(&mut self, w: &mut io::Write, value_writer: &mut Self::V, ctx: &mut Context, bindings: &BindingContext, a_op: &ExprApplyOp, arr: Option<I>) -> Result {
        match a_op {
            &ExprApplyOp::JoinString(ref sep) => {
                write!(w, "[")?;
                let mut first = true;
                if let Some(arr) = arr {
                    for v in arr {
                        if !first { write!(w, ", ")?; }
                        self.write_expr(w, ctx, bindings, v)?;
                        first = false;
                    }
                };
                write!(w, "].join(\"{}\")", sep.as_ref().map_or("", |s| s.as_str()))?;
            },
            _ => {}
        };
        Ok(())
    }
}


// #[cfg(test)]
// mod tests {
//     use super::*;
//     use std::str;
//     use std::io::Write;
//     use scope::context::*;
//     use scope::bindings::*;


//     #[test]
//     fn test_stream_writers_value_writer_js_write_binding1() {
//         let mut value_writer = ValueWriterJs::default();
//         let mut ctx = Context::default();
//         let binding = BindingType::ReducerPathBinding("todo".into(), None);

//         {
//             let mut s: Vec<u8> = Default::default();
//             let bindings = BindingContext::default();
//             let res = value_writer.write_binding(&mut s, &mut ctx, &bindings, &binding);
//             assert!(res.is_ok());
//             assert_eq!(str::from_utf8(&s), Ok("store.getState().todo".into()));
//         }

//         {
//             let mut s: Vec<u8> = Default::default();
//             let bindings = BindingContext::default();
//             // let mut expr_writer = DefaultExpressionWriter::default();
//             let mut expr_writer = ExpressionWriterJs::default();
//             let expr = ExprValue::Binding(binding.clone());

//             let res = expr_writer.write_expr(&mut s, &mut value_writer, &mut ctx, &bindings, &expr);
//             assert!(res.is_ok());
//             assert_eq!(str::from_utf8(&s), Ok("store.getState().todo".into()));
//         }
//     }

//     #[test]
//     fn test_stream_writers_value_writer_js_write_dynamic_expression1() {
//         let bindings = BindingContext::default();
//         let mut ctx = Context::default();
//         let binding = BindingType::ReducerPathBinding("todo".into(), None);
//         let literal_string = ExprValue::LiteralString("test".into());

//         let expr = ExprValue::Expr(ExprOp::Add,
//             Box::new(ExprValue::Binding(binding.clone())),
//             Box::new(literal_string.clone())
//         );

//         let mut value_writer = ValueWriterJs::default();
//         let mut expr_writer = ExpressionWriterJs::default();

//         let mut s: Vec<u8> = Default::default();
//         let res = expr_writer.write_expr(&mut s, &mut value_writer, &mut ctx, &bindings, &expr);
//         assert!(res.is_ok());
//         assert_eq!(str::from_utf8(&s), Ok("store.getState().todo+\"test\"".into()));
        
//     }

//     #[test]
//     fn test_stream_writers_writerjs_write_dynamic_expression1() {
//         let bindings = BindingContext::default();
//         let mut ctx = Context::default();

//         let binding = BindingType::ReducerPathBinding("todo".into(), None);
//         let literal_string = ExprValue::LiteralString("test".into());

//         let expr = ExprValue::Expr(ExprOp::Add,
//             Box::new(ExprValue::Binding(binding.clone())),
//             Box::new(literal_string.clone())
//         );

//         let mut s: Vec<u8> = Default::default();
//         let mut writer = WriterJs::default();
//         // let res = writer.write_expr(&mut s, &mut value_writer, &mut ctx, &bindings, &expr);
//         // assert!(res.is_ok());
//         // assert_eq!(str::from_utf8(&s), Ok("store.getState().todo+\"test\"".into()));
        
//     }
// }