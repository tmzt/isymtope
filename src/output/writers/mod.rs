pub mod expr_writers;
pub mod ops_writers;
pub mod stream_writers;

pub use self::expr_writers::*;
pub use self::ops_writers::*;
pub use self::stream_writers::*;

use std::io;
use parser::*;
use scope::*;
use processing::*;


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

pub type DefaultOutputWriterHtml = DefaultOutputWriter<ValueWriterHtml, ExpressionWriterHtml, ElementOpsStreamWriterHtml>;
pub type DefaultOutputWriterJs = DefaultOutputWriter<ValueWriterJs, ExpressionWriterJs, ElementOpsStreamWriterJs>;

#[allow(dead_code)]
impl DefaultOutputWriters {
    pub fn html() -> DefaultOutputWriterHtml { Default::default() }
    pub fn js() -> DefaultOutputWriterJs { Default::default() }
}

#[derive(Debug, Default)]
pub struct DefaultOutputWritersBoth {
    pub html: DefaultOutputWriterHtml,
    pub js: DefaultOutputWriterJs
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
        // ctx.append_path_str("Ab");
        let bindings = BindingContext::default();

        let op_1 = ElementOp::ElementOpen("span".into(), "Ab.Cd".into(), None, None, None);
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
                IncrementalDOM.elementOpen("span", "Ab.Cd");
                IncrementalDOM.elementClose("span");
            "#]));
        }

        {
            let mut writer = DefaultOutputWriters::js();
            let mut s: Vec<u8> = Default::default();
            let ops = vec![op_1.clone(), op_2.clone()];
            assert!(writer.write_element_ops(&mut s, &mut ctx, &bindings, ops.iter()).is_ok());
            assert_eq!(str::from_utf8(&s), Ok(indoc![r#"
                IncrementalDOM.elementOpen("span", "Ab.Cd");
                IncrementalDOM.elementClose("span");
            "#]));
        }
    }

    #[test]
    pub fn test_output_default_writers_both() {
        let mut ctx = Context::default();
        // ctx.append_path_str("Ab");
        let bindings = BindingContext::default();

        let op_1 = ElementOp::ElementOpen("span".into(), "Ab.Cd".into(), None, None, None);
        let op_2 = ElementOp::ElementClose("span".into());

        {
            let mut writers = DefaultOutputWritersBoth::default();
            let mut s_html: Vec<u8> = Default::default();
            let mut s_js: Vec<u8> = Default::default();
            let ops = vec![op_1.clone(), op_2.clone()];
            assert!(writers.html.write_element_ops(&mut s_html, &mut ctx, &bindings, ops.iter()).is_ok());
            assert!(writers.js.write_element_ops(&mut s_js, &mut ctx, &bindings, ops.iter()).is_ok());

            assert_eq!(str::from_utf8(&s_html), Ok(indoc![r#"
            <span key="Ab.Cd"></span>"#
            ]));

            assert_eq!(str::from_utf8(&s_js), Ok(indoc![r#"
                IncrementalDOM.elementOpen("span", "Ab.Cd");
                IncrementalDOM.elementClose("span");
            "#]));
        }
    }
}