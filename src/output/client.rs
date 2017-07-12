
use std::io;
use std::fmt;
use parser::ast::*;
use super::process::ProcessDocument;

use super::client_output::*;
use super::structs::*;

pub type Result = io::Result<fmt::Result>;

pub struct ClientOutput<'input> {
    ast: &'input Template,
}

impl<'input, 'doc: 'input> ClientOutput<'input> {
    pub fn from_template(ast: &'input Template) -> ClientOutput {
        ClientOutput { ast: ast }
    }

    pub fn write_html(&self, w: &mut io::Write) -> Result {
        let doc: DocumentState<'input> = ProcessDocument::from_template(self.ast).into();
        let format = FormatHtml::from_state(doc);

        //let format = FormatHtml::from_template(self.ast, processing);
        let mut doc_str = String::new();

        if let Err(e) = format.write_html_document(&mut doc_str) {
            return Ok(Err(e));
        }

        if let Err(e) = w.write_fmt(format_args!("{}", doc_str)) {
            return Err(e);
        }

        Ok(Ok(()))
    }
}
