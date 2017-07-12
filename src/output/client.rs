
use std::io;
use std::fmt;
use parser::ast::*;
use super::process::ProcessDocument;

use super::client_output::*;
use super::structs::*;

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
        let format = FormatHtml::from_state(doc);

        /*
        let mut doc_str = String::new();
        format.write_html_document(&mut doc_str)?;

        w.write_fmt(format_args!("{}", doc_str))?;
        Ok(())
        */
        format.write_html_document(w)
    }
}
