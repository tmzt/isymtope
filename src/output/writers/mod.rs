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


// pub trait OutputWriter<V: ValueWriter, E: ExpressionWriter<V = V>, S: ElementOpsStreamWriter> {
pub trait OutputWriter {
    // type V: ValueWriter;
    type E: ExpressionWriter;
    // type S: ElementOpsStreamWriter;

    // fn stream_writer(&mut self) -> &mut Self::S;
}

#[derive(Debug, Default)]
pub struct DefaultOutputWriter<E: ExpressionWriter> {
    value_writer: E::V,
    expression_writer: E,
    // stream_writer: S
}

// impl<V: ValueWriter, E: ExpressionWriter<V = V>, S: ElementOpsStreamWriter> OutputWriter for DefaultOutputWriter<E, S> {
impl<V: ValueWriter, E: ExpressionWriter<V = V>> OutputWriter for DefaultOutputWriter<E> {
    // type V = E::V;
    type E = E;
    // type S = S;

    // fn stream_writer(&mut self) -> &mut S { &mut self.stream_writer }
}

// impl<V: ValueWriter, E: ExpressionWriter<V = V>, S: ElementOpsStreamWriter> ExprWriter for DefaultOutputWriter<V, E, S> {
impl<E: ExpressionWriter> ExprWriter for DefaultOutputWriter<E> {
    type E = E;

    fn write_expr(&mut self, w: &mut io::Write, ctx: &mut Context, bindings: &BindingContext, expr: &ExprValue) -> Result {
        self.expression_writer.write_expr_to(w, &mut self.value_writer, ctx, bindings, expr)
    }
}

#[derive(Debug, Default)]
pub struct DefaultOutputWriters {}

pub type DefaultOutputWriterHtml = DefaultOutputWriter<ExpressionWriterHtml>;
pub type DefaultOutputWriterJs = DefaultOutputWriter<ExpressionWriterJs>;

pub trait OutputWritersBoth {
    type Html: OutputWriter;
    type Js: OutputWriter;
    // type ExpressionWriterJs: ExpressionWriter;
    // type ExpressionWriterHtml: ExpressionWriter;

    fn html(&mut self) -> &mut DefaultOutputWriterHtml;
    fn js(&mut self) -> &mut DefaultOutputWriterJs;
}

#[derive(Debug, Default)]
pub struct DefaultOutputWritersBoth {
    pub html: DefaultOutputWriterHtml,
    pub js: DefaultOutputWriterJs
}

impl OutputWritersBoth for DefaultOutputWritersBoth {
    // type ExpressionWriterJs = ExpressionWriterJs;
    // type ExpressionWriterHtml = ExpressionWriterHtml;
    type Html = DefaultOutputWriterHtml;
    type Js = DefaultOutputWriterJs;

    fn html(&mut self) -> &mut DefaultOutputWriterHtml { &mut self.html }
    fn js(&mut self) -> &mut DefaultOutputWriterJs { &mut self.js }
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

        let mut writers = DefaultOutputWritersBoth::default();

        {
            let writer = writers.html();
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
            let writer = writers.html();
            let mut s: Vec<u8> = Default::default();
            let ops = vec![op_1.clone(), op_2.clone()];
            assert!(writer.write_element_ops(&mut s, &mut ctx, &bindings, ops.iter()).is_ok());
            assert_eq!(str::from_utf8(&s), Ok(indoc![r#"
            <span key="Ab.Cd"></span>"#
            ]));
        }

        {
            let writer = writers.js();
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
            let writer = writers.js();
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
            assert!(writers.html().write_element_ops(&mut s_html, &mut ctx, &bindings, ops.iter()).is_ok());
            assert!(writers.js().write_element_ops(&mut s_js, &mut ctx, &bindings, ops.iter()).is_ok());

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