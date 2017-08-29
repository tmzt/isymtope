
use std::io;

use parser::ast::*;
use processing::structs::*;
use scope::context::*;
use scope::bindings::*;
use output::stream_writers::output_writer::*;


#[derive(Debug, Default)]
pub struct ValueWriterJs {}

impl ValueWriter for ValueWriterJs {
    fn write_literal_string(&mut self, w: &mut io::Write, s: &str) -> Result {
        write!(w, "\"{}\"", s)?;
        Ok(())
    }

    fn write_literal_number(&mut self, w: &mut io::Write, n: &i32) -> Result {
        write!(w, "{}", n)?;
        Ok(())
    }

    fn write_binding(&mut self, w: &mut io::Write, ctx: &mut Context, bindings: &BindingContext, binding: &BindingType) -> Result {
        match binding {
            &BindingType::ReducerPathBinding(ref key, ref paths) => {
                let symbol_path = match paths {
                    &Some(ref paths) => format!("{}.{}", key, paths.join(".")),
                    _ => key.to_owned()
                };
                write!(w, "store.getState().{}", symbol_path)?;
            },
            _ => {}
        };
        Ok(())
    }

    fn write_op(&mut self, w: &mut io::Write, op: &ExprOp) -> Result {
        match op {
            &ExprOp::Add => { write!(w, "+")?; },
            _ => {}
        };
        Ok(())
    }
}

#[derive(Debug, Default)]
pub struct ExpressionWriterJs {}

impl ExpressionWriter for ExpressionWriterJs {}
impl DynamicExpressionWriter for ExpressionWriterJs {
    fn write_dynamic_expression(&mut self, w: &mut io::Write, value_writer: &mut ValueWriter, ctx: &mut Context, bindings: &BindingContext, op: &ExprOp, left: &ExprValue, right: &ExprValue) -> Result {
        self.write_expr(w, value_writer, ctx, bindings, left)?;
        value_writer.write_op(w, op)?;
        self.write_expr(w, value_writer, ctx, bindings, right)?;

        Ok(())
    }
}



#[cfg(test)]
mod tests {
    use super::*;
    use std::str;
    use std::io::Write;
    use scope::context::*;
    use scope::bindings::*;


    #[test]
    fn test_stream_writers_value_writer_js_write_binding1() {
        let mut value_writer = ValueWriterJs::default();
        let mut ctx = Context::default();
        let binding = BindingType::ReducerPathBinding("todo".into(), None);

        {
            let mut s: Vec<u8> = Default::default();
            let bindings = BindingContext::default();
            let res = value_writer.write_binding(&mut s, &mut ctx, &bindings, &binding);
            assert!(res.is_ok());
            assert_eq!(str::from_utf8(&s), Ok("store.getState().todo".into()));
        }

        {
            let mut s: Vec<u8> = Default::default();
            let bindings = BindingContext::default();
            let mut expr_writer = DefaultExpressionWriter::default();
            let expr = ExprValue::Binding(binding.clone());

            let res = expr_writer.write_expr(&mut s, &mut value_writer, &mut ctx, &bindings, &expr);
            assert!(res.is_ok());
            assert_eq!(str::from_utf8(&s), Ok("store.getState().todo".into()));
        }
    }

    #[test]
    fn test_stream_writers_value_writer_js_write_dynamic_expression1() {
        let bindings = BindingContext::default();
        let mut ctx = Context::default();
        let binding = BindingType::ReducerPathBinding("todo".into(), None);
        let literal_string = ExprValue::LiteralString("test".into());

        let expr = ExprValue::Expr(ExprOp::Add,
            Box::new(ExprValue::Binding(binding.clone())),
            Box::new(literal_string.clone())
        );

        let mut value_writer = ValueWriterJs::default();
        let mut expr_writer = ExpressionWriterJs::default();

        let mut s: Vec<u8> = Default::default();
        let res = expr_writer.write_expr(&mut s, &mut value_writer, &mut ctx, &bindings, &expr);
        assert!(res.is_ok());
        assert_eq!(str::from_utf8(&s), Ok("store.getState().todo+\"test\"".into()));
        
    }
}