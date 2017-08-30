
use std::io;
use parser::ast::*;
use processing::process::*;

use super::client_output::*;
use processing::structs::*;
use scope::scope::*;
use scope::context::*;
use scope::bindings::*;


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

        let mut ctx = Context::default();
        let bindings = BindingContext::default();

        format.write_html_document(w, &mut ctx, &bindings)
    }
}


#[cfg(test)]
mod tests {
    use super::*;
    use processing::structs::*;


    pub fn test_output_client_write_html() {
        let doc = DocumentState::default();
    }
}