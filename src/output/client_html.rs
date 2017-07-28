
use std::io;
use std::clone::Clone;
use std::slice::Iter;

use parser::ast::*;
use parser::util::allocate_element_key;
use output::structs::*;
use output::client_js::*;
use output::client_misc::*;
use output::client_output::*;
use output::client_ops_writer::*;
use output::client_ops_stream_writer::*;
use output::client_ops_html_stream_writer::*;


pub struct WriteHtmlOpsContent<'input> {
    doc: &'input DocumentState<'input>,
    pub events_vec: EventsVec,
    pub keys_vec: Vec<String>,
}

impl<'input> WriteHtmlOpsContent<'input> {
    pub fn with_doc(doc: &'input DocumentState<'input>) -> WriteHtmlOpsContent<'input> {
        WriteHtmlOpsContent {
            doc: doc,
            events_vec: Default::default(),
            keys_vec: Default::default(),
        }
    }

    #[inline]
    #[allow(unused_variables)]
    pub fn write_html_ops_content(&mut self,
                                  w: &mut io::Write,
                                  ops: Iter<ElementOp>,
                                  scope_prefix: Option<ScopePrefixType>)
                                  -> Result {
        let mut stream_writer = ElementOpsHtmlStreamWriter::new();
        let mut ops_writer = ElementOpsWriter::with_doc(&self.doc, &mut stream_writer);

        ops_writer.write_ops_content(w, ops, &self.doc, None)?;

        Ok(())
    }
}