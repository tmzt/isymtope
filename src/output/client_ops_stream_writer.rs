
use std::io;
use std::slice::Iter;

use parser::ast::*;
use processing::structs::*;
use processing::scope::*;


pub trait ElementOpsStreamWriter {
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
                        -> Result;
    fn write_op_element_close(&mut self,
                              w: &mut io::Write,
                              op: &ElementOp,
                              doc: &DocumentState,
                              scope: &ElementOpScope,
                              element_tag: &str)
                              -> Result;
    fn write_op_element_value(&mut self,
                              w: &mut io::Write,
                              op: &ElementOp,
                              doc: &DocumentState,
                              scope: &ElementOpScope,
                              expr: &ExprValue,
                              value_key: &str)
                              -> Result;
    fn write_op_element_start_block(&mut self,
                                    w: &mut io::Write,
                                    op: &ElementOp,
                                    doc: &DocumentState,
                                    scope: &ElementOpScope,
                                    block_id: &str)
                                    -> Result;
    fn write_op_element_end_block(&mut self,
                                  w: &mut io::Write,
                                  op: &ElementOp,
                                  doc: &DocumentState,
                                  scope: &ElementOpScope,
                                  block_id: &str)
                                  -> Result;
    fn write_op_element_map_collection_to_block(&mut self,
                                                w: &mut io::Write,
                                                op: &ElementOp,
                                                doc: &DocumentState,
                                                scope: &ElementOpScope,
                                                coll_expr: &ExprValue,
                                                block_id: &str)
                                                -> Result;
    fn write_op_element_instance_component_open(&mut self,
                                                w: &mut io::Write,
                                                op: &ElementOp,
                                                doc: &DocumentState,
                                                scope: &ElementOpScope,
                                                comp: &Component,
                                                props: Option<Iter<Prop>>,
                                                lens: Option<&LensExprType>,
                                                element_tag: Option<&str>)
                                                -> Result;
    fn write_op_element_instance_component_close(&mut self,
                                                 w: &mut io::Write,
                                                 op: &ElementOp,
                                                 doc: &DocumentState,
                                                 scope: &ElementOpScope,
                                                 comp: &Component,
                                                 element_tag: Option<&str>)
                                                 -> Result;
}
