
extern crate uuid;
extern crate itertools;

pub mod writers;
pub mod page_writer;
pub mod store_writer;
pub mod events_writer;

pub use processing::structs::Result;
pub use self::writers::*;
pub use self::page_writer::*;
pub use self::store_writer::*;
pub use self::events_writer::*;


#[cfg(test)]
mod tests {
    use std::io;
    use std::fs;
    use std::path::Path;
    use ::broadcast::BroadcastWriter;

    use super::*;
    use model::*;
    use parser::*;
    use processing::*;
    use scope::*;


    fn prepare_document<'a>(template: &'a Template) -> Document {
        let mut ctx = Context::default();
        let mut bindings = BindingContext::default();
        let mut processing = ProcessDocument::from_template(&template);
        assert!(processing.process_document(&mut ctx, &mut bindings).is_ok());
        processing.into()
    }

    pub fn write_html_file<'input>(w: &mut io::Write, template: &'input Template) -> Result {
        let mut ctx = Context::default();
        let bindings = BindingContext::default();
        let doc = prepare_document(template);
        let mut page_writer = PageWriter::with_doc(&doc);
        page_writer.write_page(w, &mut ctx, &bindings)
    }

    fn test_write_html<'input>(src_file: &str, html_file: &str) -> Result {
        let source_path = format!("./res/tests/{}", src_file);
        let output_path = format!("./site/src/assets/demo/{}", html_file);

        let template = ::parser::parse_file(Path::new(&source_path))?;

        let stdout = io::stdout();
        fs::create_dir_all("./site/src/assets/demo").ok().unwrap();
        let file = fs::File::create(Path::new(&output_path))?;
        let stdout = stdout.lock();

        let mut stream = BroadcastWriter::new(file, stdout);
        write_html_file(&mut stream, &template)
    }

    // #[test]
    // Disable as api is not currently supported in default scope
    #[allow(dead_code)]
    pub fn test_output1() {
        assert!(self::test_write_html("test1.ism", "test_output1.html").is_ok());
    }

    #[allow(dead_code)]
    pub fn test_output2() {
        assert!(self::test_write_html("test2.ism", "test_output2.html").is_ok());
    }

    #[test]
    pub fn test_output3() {
        assert!(self::test_write_html("test3.ism", "test_output3.html").is_ok());
    }

    #[test]
    pub fn test_output4() {
        assert!(self::test_write_html("test4.ism", "test_output4.html").is_ok());
    }

    #[test]
    pub fn test_output5() {
        assert!(self::test_write_html("test5_app.ism", "test_output5_app.html").is_ok());
    }

    #[test]
    pub fn test_app_todomvc() {
        assert!(self::test_write_html("app/todomvc/app.ism", "app-mvc.html").is_ok());
    }
}
