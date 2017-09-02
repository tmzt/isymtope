pub mod output_writer;
pub mod output_writer_js;
pub mod output_writer_html;
pub mod output_stream_writer;
pub mod output_stream_writer_js;
pub mod output_stream_writer_html;

pub use output::stream_writers::output_writer::DefaultOutputWriter;
use output::stream_writers::output_writer_js::{ValueWriterJs, ExpressionWriterJs};
use output::stream_writers::output_writer_html::{ValueWriterHtml, ExpressionWriterHtml};
use output::stream_writers::output_stream_writer::ElementOpsStreamWriter;
// use output::stream_writers::output_stream_writer_Js::ElementOpsStreamWriterJs;
use output::stream_writers::output_stream_writer_html::ElementOpsStreamWriterHtml;
use output::stream_writers::output_stream_writer_js::*;



#[derive(Debug, Default)]
pub struct DefaultOutputWriters {}

impl DefaultOutputWriters {
    pub fn html() -> DefaultOutputWriter<ValueWriterHtml, ExpressionWriterHtml, ElementOpsStreamWriterHtml> { Default::default() }
    pub fn js() -> DefaultOutputWriter<ValueWriterJs, ExpressionWriterJs, ElementOpsStreamWriterJs> { Default::default() }
}


#[cfg(test)]
mod tests {
    use super::*;
    // use output::stream_writers::

    #[test]
    pub fn test_output_default_writers() {
        // let mut ctx = Context::default();
        // ctx.append_path_str("prefix");
        // let bindings = BindingContext::default();

        // let mut writer: DefaultOutputWriter<ValueWriterHtml, ExpressionWriterHtml> = DefaultOutputWriter::default();
        // // let mut value_writer = ValueWriterHtml::default();
        // let mut expr_writer = ExpressionWriterHtml::default();
        // // let mut stream_writer = ElementOpsStreamWriterHtml::default();

        // let mut s: Vec<u8> = Default::default();
        // let key = "key".to_owned();
        // ctx.append_path_str(&key);
        // assert!(
        //     expr_writer.write_op_element_open(&mut s, &mut writer, &mut ctx, &bindings, "span", &key, false, empty(), empty(), empty()).is_ok() &&
        //     expr_writer.write_op_element_close(&mut s, &mut writer, &mut ctx, &bindings, "span", &key).is_ok()
        // );
        // assert_eq!(str::from_utf8(&s), Ok(indoc![r#"
        // <span key="prefix.key"></span>"#
        // ]));
    }

}