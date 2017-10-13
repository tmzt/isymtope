use scope::context::*;
use scope::bindings::*;
use output::writers::*;


#[macro_export]
macro_rules! test_writing {
    ($template: ident, $with_processing: expr, $writes: expr) => ({
        let doc: Document = create_document(&$template, $with_processing);

        let bindings = BindingContext::default();
        let mut ctx = Context::default();
        let mut writer = DefaultOutputWriterJs::default();

        let res: Result = $writes(&mut writer, &doc, &mut ctx, &bindings);
        assert!(res.is_ok());
    })
}