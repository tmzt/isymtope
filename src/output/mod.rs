
extern crate uuid;
extern crate itertools;

pub mod client;

use std::io;
use parser::ast::*;

use self::client::{ClientOutput, Result};

#[allow(dead_code)]
pub fn write_client_html(w: &mut io::Write, template: &Template) -> Result {
    let output = ClientOutput::new();
    output.write_html(w, template)
}

#[cfg(test)]
mod tests {
    use std::io;
    use std::fs;
    use std::path::Path;

    #[test]
    pub fn test_output1() {
        let template = ::parser::parse_file(Path::new("./res/tests/test1.ism")).ok().unwrap();
        let stdout = io::stdout();
        let mut stream = stdout.lock();

        //assert!(write!(stream, "test").is_ok());
        assert!(super::write_client_html(&mut stream, &template).is_ok());

        let mut file = fs::File::create(Path::new("./output/test_output1.html")).ok().unwrap();
        assert!(super::write_client_html(&mut file, &template).is_ok());
    }
}
