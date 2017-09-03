
extern crate uuid;
extern crate itertools;

pub mod writers;
pub mod page_writer;
pub mod store_writer;

pub use processing::structs::Result;
pub use self::writers::*;
pub use self::page_writer::*;
pub use self::store_writer::*;

use std::io;
use parser::ast::*;

#[allow(dead_code)]
pub fn write_client_html<'input>(_w: &mut io::Write, _template: &'input Template) -> Result {
    // let output = ClientOutput::from_template(template);
    // output.write_html(w)

    Ok(())
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
