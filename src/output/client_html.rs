
use std::io;
use std::clone::Clone;
use std::slice::Iter;

use parser::ast::*;
use parser::util::allocate_element_key;
use processing::structs::*;
use output::scope::*;
use output::client_js::*;
use output::client_misc::*;
use output::client_output::*;
use output::client_ops_writer::*;
use output::client_ops_stream_writer::*;
use output::client_ops_html_stream_writer::*;


pub struct WriteHtmlOpsContent<'input> {
    doc: &'input DocumentState<'input>,
    stream_writer: ElementOpsHtmlStreamWriter
}

impl<'input> WriteHtmlOpsContent<'input> {
    pub fn with_doc(doc: &'input DocumentState<'input>) -> WriteHtmlOpsContent<'input> {
        WriteHtmlOpsContent {
            doc: doc,
            stream_writer: ElementOpsHtmlStreamWriter::new()
        }
    }

    #[inline]
    #[allow(unused_variables)]
    pub fn write_html_ops_content(&mut self,
                                  w: &mut io::Write,
                                  ops: Iter<ElementOp>,
                                  scope_prefixes: &ScopePrefixes,
                                  expr_scope: &ExprScopeProcessingState)
                                  -> Result {
        let mut ops_writer = ElementOpsWriter::with_doc(&self.doc, &mut self.stream_writer);
        ops_writer.write_ops_content(w, ops, &self.doc, scope_prefixes, expr_scope, true)?;

        Ok(())
    }

    pub fn events_iter(&self) -> Iter<EventsItem> {
        self.stream_writer.events_iter()
    }

    pub fn keys_iter(&self) -> Iter<String> {
        self.stream_writer.keys_iter()
    }
}