
use std::io;
use parser::ast::*;
use processing::process::*;

use super::client_output::*;
use processing::structs::*;

pub struct ClientOutput<'input> {
    ast: &'input Template,
}

impl<'input, 'doc: 'input> ClientOutput<'input> {
    pub fn from_template(ast: &'input Template) -> ClientOutput {
        ClientOutput { ast: ast }
    }

    pub fn write_html(&self, w: &mut io::Write) -> Result {
        let mut processing = ProcessDocument::from_template(self.ast);
        processing.process_document()?;

        let doc: DocumentState<'input> = processing.into();
        let mut format = FormatHtml::with_doc(&doc);

        format.write_html_document(w)
    }
}