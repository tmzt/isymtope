
use std::io;
use std::clone::Clone;
use std::slice::Iter;

use parser::ast::*;

use processing::structs::*;
use processing::scope::*;

use output::client_ops_stream_writer::*;
use output::client_js_value_writer::*;


#[derive(Debug, Default)]
pub struct ElementOpsJsStreamWriter { }

impl<'input: 'scope, 'scope> ElementOpsJsStreamWriter {
    fn write_element_open(&mut self,
                          w: &mut io::Write,
                          _: &ElementOp,
                          doc: &DocumentState,
                          scope: &ElementOpScope,
                          element_key: &str,
                          element_tag: &str,
                          complete_key: &str,
                          is_void: bool,
                          props: Option<Iter<Prop>>,
                          _: Option<Iter<EventHandler>>,
                          _: ElementValueBinding)
                          -> Result {

        if !is_void {
            write!(w, "IncrementalDOM.elementOpen(\"{}\", ", element_tag)?;
        } else {
            write!(w, "IncrementalDOM.elementVoid(\"{}\", ", element_tag)?;
        };

        let key_expr = ExprValue::LiteralString(format!(".{}", element_key));
        let prefix_expr = scope.0.make_prefix_expr(&key_expr, None);
        if let Some(ref prefix_expr) = prefix_expr {
            write_js_expr_value(w, prefix_expr, doc, &scope)?;
        } else {
            write!(w, "\"{}\"", complete_key)?;
        }
        write!(w, ", [")?;

        // Static attrs
        if props.is_some() {
            write_js_incdom_attr_array(w, props.clone(), doc, &scope, Some(&element_key))?;
        };

        // TODO: Dynamic attributes

        writeln!(w, "]);")?;
        Ok(())
    }

    fn write_element_close(&mut self, w: &mut io::Write, element_tag: &str) -> Result {
        writeln!(w, "IncrementalDOM.elementClose(\"{}\");", element_tag)?;
        Ok(())
    }
}

impl<'input: 'scope, 'scope> ElementOpsStreamWriter for ElementOpsJsStreamWriter {
    fn write_op_element(&mut self,
                        w: &mut io::Write,
                        op: &ElementOp,
                        doc: &DocumentState,
                        scope: &ElementOpScope,
                        element_key: &str,
                        element_tag: &str,
                        is_void: bool,
                        props: Option<Iter<Prop>>,
                        events: Option<Iter<EventHandler>>,
                        value_binding: ElementValueBinding)
                        -> Result {
        let complete_key = scope.0.complete_element_key();

        self.write_element_open(w,
                                op,
                                doc,
                                &scope,
                                element_key,
                                element_tag,
                                &complete_key,
                                is_void,
                                props,
                                events,
                                value_binding)
    }

    #[inline]
    fn write_op_element_close(&mut self,
                              w: &mut io::Write,
                              _: &ElementOp,
                              _: &DocumentState,
                              _: &ElementOpScope,
                              element_tag: &str)
                              -> Result {
        self.write_element_close(w, element_tag)
    }

    #[inline]
    fn write_op_element_value(&mut self,
                              w: &mut io::Write,
                              op: &ElementOp,
                              doc: &DocumentState,
                              scope: &ElementOpScope,
                              expr: &ExprValue,
                              value_key: &str)
                              -> Result {
        let mut scope = scope.clone();
        scope.0.append_key("v");
        let complete_key = scope.0.complete_element_key();

        self.write_element_open(w,
                                op,
                                doc,
                                &scope,
                                value_key,
                                "span",
                                &complete_key,
                                false,
                                None,
                                None,
                                None)?;

        write!(w, "IncrementalDOM.text(")?;
        write_js_expr_value(w, expr, doc, &scope)?;
        writeln!(w, ");")?;

        self.write_element_close(w, "span")?;

        Ok(())
    }

    #[inline]
    fn write_op_element_start_block(&mut self,
                                    w: &mut io::Write,
                                    _: &ElementOp,
                                    _: &DocumentState,
                                    _: &ElementOpScope,
                                    block_id: &str)
                                    -> Result {
        let foridx = &format!("__foridx_{}", block_id);
        writeln!(w,
                 "var __{} = function __{}_(line, {}){{",
                 block_id,
                 block_id,
                 foridx)
            ?; //FIXME

        Ok(())
    }

    #[inline]
    #[allow(unused_variables)]
    fn write_op_element_end_block(&mut self,
                                  w: &mut io::Write,
                                  op: &ElementOp,
                                  doc: &DocumentState,
                                  scope: &ElementOpScope,
                                  block_id: &str)
                                  -> Result {
        writeln!(w, "}};")?;
        Ok(())
    }

    #[inline]
    fn write_op_element_map_collection_to_block(&mut self,
                                                w: &mut io::Write,
                                                _: &ElementOp,
                                                doc: &DocumentState,
                                                scope: &ElementOpScope,
                                                coll_expr: &ExprValue,
                                                block_id: &str)
                                                -> Result {
        write!(w, "(")?;

        let foridx = &format!("__foridx_{}", block_id);
        let mut scope = scope.clone();

        let loopidx_ref = ExprValue::SymbolReference(Symbol::loop_var(foridx));
        scope.0.set_prefix_expr(&loopidx_ref);

        write_js_expr_value(w, coll_expr, doc, &scope)?;
        writeln!(w, ").forEach(__{});", block_id)?;

        Ok(())
    }

    #[inline]
    #[allow(unused_variables)]
    fn write_op_element_instance_component_open(&mut self,
                                                w: &mut io::Write,
                                                op: &ElementOp,
                                                doc: &DocumentState,
                                                scope: &ElementOpScope,
                                                comp: &Component,
                                                props: Option<Iter<Prop>>,
                                                lens: Option<&LensExprType>,
                                                element_tag: Option<&str>)
                                                -> Result {
        Ok(())
    }

    #[inline]
    #[allow(unused_variables)]
    fn write_op_element_instance_component_close(&mut self,
                                                 w: &mut io::Write,
                                                 op: &ElementOp,
                                                 doc: &DocumentState,
                                                 scope: &ElementOpScope,
                                                 comp: &Component,
                                                 element_tag: Option<&str>)
                                                 -> Result {
        Ok(())
    }
}
