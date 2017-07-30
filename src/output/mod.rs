
extern crate uuid;
extern crate itertools;

pub mod client;
pub mod client_output;
pub mod client_html;
pub mod client_js;
pub mod client_js_value_writer;
pub mod client_misc;
pub mod client_ops_writer;
pub mod client_ops_stream_writer;
pub mod client_ops_js_stream_writer;
pub mod client_ops_html_stream_writer;
pub mod scope;

pub use processing::structs::Result;

use std::io;
use parser::ast::*;

use self::client::ClientOutput;

#[allow(dead_code)]
pub fn write_client_html<'input>(w: &mut io::Write, template: &'input Template) -> Result {
    let output = ClientOutput::from_template(template);
    output.write_html(w)
}

#[cfg(test)]
mod tests {
    use std::io;
    use std::fs;
    use std::path::Path;
    use super::Result;
    use ::broadcast::BroadcastWriter;

    fn test_write_html<'input>(src_file: &str, html_file: &str) -> Result {
        let source_path = format!("./res/tests/{}", src_file);
        let output_path = format!("./output/{}", html_file);

        let template = ::parser::parse_file(Path::new(&source_path))?;

        let stdout = io::stdout();
        fs::create_dir_all("./output").ok().unwrap();
        let file = fs::File::create(Path::new(&output_path))?;
        let stdout = stdout.lock();

        let mut stream = BroadcastWriter::new(file, stdout);
        super::write_client_html(&mut stream, &template)
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

    #[allow(dead_code)]
    pub fn test_output3() {
        assert!(self::test_write_html("test3.ism", "test_output3.html").is_ok());
    }

    #[test]
    pub fn test_output4() {
        assert!(self::test_write_html("test4.ism", "test_output4.html").is_ok());
    }
}
