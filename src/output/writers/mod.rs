pub mod expr_writers;
pub mod ops_writers;
pub mod stream_writers;

pub use self::expr_writers::*;
pub use self::ops_writers::*;
pub use self::stream_writers::*;

use std::io;
use parser::ast::*;
use scope::context::*;
use scope::bindings::*;
use processing::structs::*;


#[derive(Debug, Default)]
pub struct DefaultOutputWriter<V: ValueWriter, E: ExpressionWriter<V = V>, S: ElementOpsStreamWriter> {
    value_writer: V,
    expression_writer: E,
    stream_writer: S
}

impl<V: ValueWriter, E: ExpressionWriter<V = V>, S: ElementOpsStreamWriter> ExprWriter for DefaultOutputWriter<V, E, S> {
    type E = E;

    fn write_expr(&mut self, w: &mut io::Write, ctx: &mut Context, bindings: &BindingContext, expr: &ExprValue) -> Result {
        self.expression_writer.write_expr_to(w, &mut self.value_writer, ctx, bindings, expr)
    }
}

#[derive(Debug, Default)]
pub struct DefaultOutputWriters {}

impl DefaultOutputWriters {
    pub fn html() -> DefaultOutputWriter<ValueWriterHtml, ExpressionWriterHtml, ElementOpsStreamWriterHtml> { Default::default() }
    pub fn js() -> DefaultOutputWriter<ValueWriterJs, ExpressionWriterJs, ElementOpsStreamWriterJs> { Default::default() }
}


#[cfg(test)]
mod tests {
    use super::*;
    use std::str;
    use scope::context::*;
    use scope::bindings::*;
    use processing::structs::ElementOp;


    #[test]
    pub fn test_output_default_writers() {
        let mut ctx = Context::default();
        ctx.append_path_str("Ab");
        let bindings = BindingContext::default();

        let op_1 = ElementOp::ElementOpen("span".into(), "Cd".into(), None, None, None);
        let op_2 = ElementOp::ElementClose("span".into());

        {
            let mut writer = DefaultOutputWriters::html();
            let mut s: Vec<u8> = Default::default();
            assert!(
                writer.write_element_op(&mut s, &mut ctx, &bindings, &op_1).is_ok() &&
                writer.write_element_op(&mut s, &mut ctx, &bindings, &op_2).is_ok()
            );
            assert_eq!(str::from_utf8(&s), Ok(indoc![r#"
            <span key="Ab.Cd"></span>"#
            ]));
        }

        {
            let mut writer = DefaultOutputWriters::html();
            let mut s: Vec<u8> = Default::default();
            let ops = vec![op_1.clone(), op_2.clone()];
            assert!(writer.write_element_ops(&mut s, &mut ctx, &bindings, ops.iter()).is_ok());
            assert_eq!(str::from_utf8(&s), Ok(indoc![r#"
            <span key="Ab.Cd"></span>"#
            ]));
        }

        {
            let mut writer = DefaultOutputWriters::js();
            let mut s: Vec<u8> = Default::default();
            assert!(
                writer.write_element_op(&mut s, &mut ctx, &bindings, &op_1).is_ok() &&
                writer.write_element_op(&mut s, &mut ctx, &bindings, &op_2).is_ok()
            );
            assert_eq!(str::from_utf8(&s), Ok(indoc![r#"
                IncrementalDOM.elementOpen("span", ["Ab", "Cd"].join("."));
                IncrementalDOM.elementClose("span");
            "#]));
        }

        {
            let mut writer = DefaultOutputWriters::js();
            let mut s: Vec<u8> = Default::default();
            let ops = vec![op_1.clone(), op_2.clone()];
            assert!(writer.write_element_ops(&mut s, &mut ctx, &bindings, ops.iter()).is_ok());
            assert_eq!(str::from_utf8(&s), Ok(indoc![r#"
                IncrementalDOM.elementOpen("span", ["Ab", "Cd"].join("."));
                IncrementalDOM.elementClose("span");
            "#]));
        }
    }

}