
use std::io;

use parser::ast::*;
use processing::structs::*;
use scope::context::*;
use scope::bindings::*;
use output::writers::*;


#[derive(Debug, Default)]
pub struct ValueWriterHtml {}

impl ValueWriter for ValueWriterHtml {
    fn write_literal_string(&mut self, w: &mut io::Write, s: &str) -> Result {
        write!(w, "{}", s)?;
        Ok(())
    }

    fn write_literal_number(&mut self, w: &mut io::Write, n: &i32) -> Result {
        write!(w, "{}", n)?;
        Ok(())
    }

    fn write_literal_bool(&mut self, w: &mut io::Write, b: bool) -> Result {
        if b { write!(w, "true")?; } else { write!(w, "false")?; }
        Ok(())
    }

    fn write_simple_binding(&mut self, _w: &mut io::Write, _ctx: &mut Context, _bindings: &BindingContext, _binding: &BindingType) -> Result {
        Ok(())
    }

    fn write_op(&mut self, _w: &mut io::Write, _op: &ExprOp) -> Result {
        Ok(())
    }

    fn write_undefined(&mut self, w: &mut io::Write) -> Result {
        write!(w, "[undefined]")?;
        Ok(())
    }
}
impl StaticValueWriter for ValueWriterHtml {}

#[derive(Debug, Default)]
pub struct ExpressionWriterHtml {}

impl ExpressionWriter for ExpressionWriterHtml {
    type V = ValueWriterHtml;

    fn write_expression(&mut self, w: &mut io::Write, doc: &Document, value_writer: &mut Self::V, ctx: &mut Context, bindings: &BindingContext, op: &ExprOp, left: &ExprValue, right: &ExprValue) -> Result {
        self.write_expr_to(w, doc, value_writer, ctx, bindings, left)?;
        value_writer.write_op(w, op)?;
        self.write_expr_to(w, doc, value_writer, ctx, bindings, right)?;

        Ok(())
    }

    fn write_apply_expression<'a, I: IntoIterator<Item = &'a ExprValue>>(&mut self, w: &mut io::Write, doc: &Document, value_writer: &mut Self::V, ctx: &mut Context, bindings: &BindingContext, a_op: &ExprApplyOp, arr: Option<I>) -> Result {
        match a_op {
            &ExprApplyOp::JoinString(ref sep) => {
                write!(w, "[")?;
                let mut first = true;
                if let Some(arr) = arr {
                    for v in arr {
                        if !first { write!(w, ", ")?; }
                        self.write_expr_to(w, doc, value_writer, ctx, bindings, v)?;
                        first = false;
                    }
                };
                write!(w, "].join(\"{}\")", sep.as_ref().map_or("", |s| s.as_str()))?;
            },
            _ => {}
        };
        Ok(())
    }

    fn write_array<'a, I: IntoIterator<Item = &'a ExprValue>>(&mut self, w: &mut io::Write, doc: &Document, value_writer: &mut Self::V, ctx: &mut Context, bindings: &BindingContext, arr: Option<I>, ty: Option<VarType>) -> Result {
        write!(w, "[array]")?;
        Ok(())
    }

    fn write_props<'a, I: IntoIterator<Item = &'a Prop>>(&mut self, w: &mut io::Write, doc: &Document, value_writer: &mut Self::V, ctx: &mut Context, bindings: &BindingContext, props: Option<I>) -> Result {
        // write!(w, "{{")?;
        // if let Some(props) = props {
        //     let mut first = true;
        //     for prop in props {
        //         if !first { write!(w, ", ")?; }
        //         write!(w, "\"{}\": ", &prop.0)?;
        //         if let Some(ref v) = prop.1 {
        //             self.write_expr_to(w, doc, value_writer, ctx, bindings, v)?;
        //         }
        //         first = false;
        //     };
        // };
        // write!(w, "}}")?;
        Ok(())
    }

    fn write_binding(&mut self, w: &mut io::Write, doc: &Document, value_writer: &mut Self::V, ctx: &mut Context, bindings: &BindingContext, binding: &BindingType) -> Result {
        match binding {
            &BindingType::ComponentPropBinding(ref key) => {
                // if let Some(sym) = ctx.resolve_sym(key) {
                if let Some(expr) = ctx.eval_key(doc, key) {
                    self.write_expr_to(w, doc, value_writer, ctx, bindings, &expr)?;
                };
            }

            _ => {}
        };
        Ok(())
    }

    fn write_group(&mut self, w: &mut io::Write, doc: &Document, value_writer: &mut Self::V, ctx: &mut Context, bindings: &BindingContext, inner_expr: Option<&ExprValue>) -> Result {
        Ok(())
    }

    fn write_symbol(&mut self, w: &mut io::Write, doc: &Document, value_writer: &mut Self::V, ctx: &mut Context, bindings: &BindingContext, sym: &Symbol) -> Result {
        match sym.sym_ref() {
            &SymbolReferenceType::InitialValue(_, box ref after) => {
                return self.write_symbol(w, doc, value_writer, ctx, bindings, after);
            }

            &SymbolReferenceType::Binding(ref binding) => {
                return self.write_binding(w, doc, value_writer, ctx, bindings, binding);
            }

            _ => {}
        };

        if let Some(ref expr) = sym.value() {
            return self.write_expr_to(w, doc, value_writer, ctx, bindings, expr);
        };

        if let Some(expr) = ctx.eval_sym(doc, sym) {
            return self.write_expr_to(w, doc, value_writer, ctx, bindings, &expr);
        };

        Ok(())
    }

    fn write_pipeline<'a, I: IntoIterator<Item = &'a ReducedPipelineComponent>>(&mut self, w: &mut io::Write, doc: &Document, value_writer: &mut Self::V, ctx: &mut Context, bindings: &BindingContext, head: Option<&ExprValue>, parts: I) -> Result {
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

//             let res = expr_writer.write_expr(&mut s, doc, &mut value_writer, &mut ctx, &bindings, &expr);
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
//         let res = expr_writer.write_expr(&mut s, doc, &mut value_writer, &mut ctx, &bindings, &expr);
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
//         // let res = writer.write_expr(&mut s, doc, &mut value_writer, &mut ctx, &bindings, &expr);
//         // assert!(res.is_ok());
//         // assert_eq!(str::from_utf8(&s), Ok("store.getState().todo+\"test\"".into()));
        
//     }
// }