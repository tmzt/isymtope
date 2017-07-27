
use std::io;
use std::clone::Clone;
use std::slice::Iter;

use parser::ast::*;
use parser::util::allocate_element_key;
use output::structs::*;
use output::client_misc::*;
use output::client_output::*;
use output::client_ops_writer::*;
use output::client_ops_stream_writer::*;
use output::client_ops_js_stream_writer::*;

pub struct WriteJsOps<'input> {
    pub doc: &'input DocumentState<'input>,
}

impl<'input> WriteJsOps<'input> {
    pub fn with_doc(doc: &'input DocumentState<'input>) -> WriteJsOps<'input> {
        WriteJsOps {
            doc: doc,
        }
    }

    #[inline]
    #[allow(unused_variables)]
    pub fn write_js_incdom_ops_content(&mut self,
                                            w: &mut io::Write,
                                            ops: Iter<ElementOp>,
                                            processing: &DocumentState,
                                            resolve: &ResolveVars)
                                            -> Result {
        let mut stream_writer = ElementOpsJsStreamWriter {};
        let mut ops_writer = ElementOpsWriter::with_doc(&self.doc, &mut stream_writer);

        ops_writer.write_ops_content(w, ops, &self.doc, None)?;

        Ok(())
    }

    #[inline]
    pub fn write_js_incdom_component(&mut self,
                                            w: &mut io::Write,
                                            component_ty: &'input str,
                                            ops: Iter<ElementOp>,
                                            processing: &DocumentState,
                                            resolve: &ResolveVars,
                                            key_prefix: Option<&str>)
                                            -> Result {

        writeln!(w,
                "  function component_{}(key_prefix, store, props) {{",
                component_ty)
            ?;
        self.write_js_incdom_ops_content(w, ops, processing, resolve)?;
        writeln!(w, "  }};")?;
        writeln!(w, "")?;
        Ok(())
    }
}