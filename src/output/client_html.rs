
use std::io;
use std::clone::Clone;
use std::slice::Iter;

use parser::ast::*;
use parser::util::allocate_element_key;
use processing::structs::*;
use processing::scope::*;

use output::client_js::*;
use output::client_misc::*;
use output::client_output::*;
use output::client_ops_writer::*;
use output::client_ops_stream_writer::*;
use output::client_ops_html_stream_writer::*;


pub struct WriteHtmlOpsContent<'input> {
    doc: &'input DocumentState<'input>,
    stream_writer: ElementOpsHtmlStreamWriter,
    events_vec: Option<EventsVec>,
    component_instances: Option<Vec<(String, String)>>,
}

impl<'input> WriteHtmlOpsContent<'input> {
    pub fn with_doc(doc: &'input DocumentState<'input>) -> WriteHtmlOpsContent<'input> {
        WriteHtmlOpsContent {
            doc: doc,
            stream_writer: ElementOpsHtmlStreamWriter::new(),
            events_vec: None,
            component_instances: None,
        }
    }

    #[inline]
    #[allow(unused_variables)]
    pub fn write_html_ops_content(&mut self,
                                  w: &mut io::Write,
                                  ops: Iter<ElementOp>,
                                  scope: ElementOpScope,
                                  events_vec: Option<&mut EventsVec>)
                                  -> Result {
        let mut ops_writer = ElementOpsWriter::with_doc(&self.doc, &mut self.stream_writer, scope);
        ops_writer.write_ops_content(w, ops, &self.doc, true)?;
        let events_vec: EventsVec = ops_writer.events_iter().map(|s| s.clone()).collect();
        let comp_instances: Vec<(String, String)> =
            ops_writer.component_instances_iter().map(|s| s.clone()).collect();
        self.events_vec = Some(events_vec);
        self.component_instances = Some(comp_instances);

        Ok(())
    }

    pub fn events_iter(&self) -> Option<Iter<EventsItem>> {
        self.events_vec.as_ref().map(|s| s.iter())
    }

    pub fn component_instances_iter(&self) -> Option<Iter<(String, String)>> {
        self.component_instances.as_ref().map(|s| s.iter())
    }

    pub fn keys_iter(&self) -> Iter<String> {
        self.stream_writer.keys_iter()
    }
}
