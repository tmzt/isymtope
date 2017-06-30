
pub mod client;

use std::io;
use parser::parse_file;
use parser::ast::*;

use self::client::{ClientOutput, Result};

pub fn write_client_html(w: &mut io::Write, template: &Template) -> Result {
    let output = ClientOutput::new();
    output.write_html(w, template)
}

#[cfg(test)]
mod tests {
    use std::io;

    #[test]
    pub fn test_output1() {
        let template = super::parse_file(::std::path::Path::new("./res/tests/test1.ism")).ok().unwrap();
        let stdout = io::stdout();
        let mut stream = stdout.lock();

        //assert!(write!(stream, "test").is_ok());
        assert!(super::write_client_html(&mut stream, &template).is_ok());
    }
}
