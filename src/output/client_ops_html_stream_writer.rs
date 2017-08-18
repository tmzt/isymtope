
use std::io;
use std::slice::Iter;

use parser::ast::*;
use processing::structs::*;
use processing::scope::*;

use output::client_misc::*;
use output::client_ops_stream_writer::*;


#[derive(Debug, Default)]
pub struct ElementOpsHtmlStreamWriter {
    pub events_vec: EventsVec,
    pub keys_vec: Vec<String>,
}

impl<'input: 'scope, 'scope> ElementOpsHtmlStreamWriter {
    pub fn new() -> Self {
        Default::default()
    }

    #[inline]
    #[allow(unused_variables)]
    fn write_html_js_action(&mut self,
                            w: &mut io::Write,
                            act_iter: Iter<ActionOpNode>,
                            scope: &ElementOpScope)
                            -> Result {
        write!(w, "function(event) {{")?;
        for ref act_op in act_iter {
            match *act_op {
                &ActionOpNode::DispatchAction(ref action, ref params) => {
                    let action_ty = scope.0.make_action_type(action);
                    write!(w, " store.dispatch({{\"type\": \"{}\"}}); ", action_ty)?;
                }
            }
        }
        write!(w, "}}")?;
        Ok(())
    }

    #[inline]
    fn write_element_attribute_expr_value(&mut self,
                                          w: &mut io::Write,
                                          _: &str,
                                          expr: &ExprValue,
                                          doc: &DocumentState,
                                          scope: &ElementOpScope)
                                          -> Result {
        match expr {
            &ExprValue::DefaultAction(_, ref act_ops) => {
                if let &Some(ref act_ops) = act_ops {
                    self.write_html_js_action(w, act_ops.iter(), scope)?;
                };
            }
            &ExprValue::Action(_, _, ref act_ops) => {
                if let &Some(ref act_ops) = act_ops {
                    self.write_html_js_action(w, act_ops.iter(), scope)?;
                };
            }
            _ => {
                write!(w, "\"")?;
                write_computed_expr_value(w, expr, doc, scope)?;
                write!(w, "\"")?;
            }
        };
        Ok(())
    }

    pub fn keys_iter(&self) -> Iter<String> {
        self.keys_vec.iter()
    }
}

impl<'input: 'scope, 'scope> ElementOpsStreamWriter for ElementOpsHtmlStreamWriter {
    fn write_op_element(&mut self,
                        w: &mut io::Write,
                        _: &ElementOp,
                        doc: &DocumentState,
                        scope: &ElementOpScope,
                        complete_key: &str,
                        element_tag: &str,
                        is_void: bool,
                        props: Option<Iter<Prop>>,
                        _: Option<Iter<EventHandler>>,
                        _: ElementValueBinding)
                        -> Result {
        write!(w, "<{} key=\"{}\"", element_tag, complete_key)?;

        if let Some(props) = props {
            for &(ref key, ref expr) in props {
                if let &Some(ref expr) = expr {
                    write!(w, " {}=", key)?;
                    self.write_element_attribute_expr_value(w, key, expr, doc, scope)?;
                }
            }
        }

        if is_void {
            write!(w, " />")?;
        } else {
            write!(w, ">")?;
        };

        self.keys_vec.push(complete_key.to_owned());
        Ok(())
    }

    #[inline]
    #[allow(unused_variables)]
    fn write_op_element_close(&mut self,
                              w: &mut io::Write,
                              op: &ElementOp,
                              doc: &DocumentState,
                              scope: &ElementOpScope,
                              element_tag: &str)
                              -> Result {
        write!(w, "</{}>", element_tag)?;
        Ok(())
    }

    #[inline]
    fn write_op_element_value(&mut self,
                              w: &mut io::Write,
                              _: &ElementOp,
                              doc: &DocumentState,
                              scope: &ElementOpScope,
                              expr: &ExprValue,
                              complete_value_key: &str)
                              -> Result {
        let complete_value_key = format!("{}.v", complete_value_key);

        let is_literal = expr.is_literal();

        if is_literal {
            write!(w, "<span key=\"{}\">", complete_value_key)?;
        };
        write_computed_expr_value(w, expr, doc, scope)?;
        if is_literal {
            write!(w, "</span>")?;
            self.keys_vec.push(complete_value_key.to_owned());
        };

        Ok(())
    }

    #[inline]
    #[allow(unused_variables)]
    fn write_op_element_start_block(&mut self,
                                    w: &mut io::Write,
                                    op: &ElementOp,
                                    doc: &DocumentState,
                                    scope: &ElementOpScope,
                                    block_id: &str)
                                    -> Result {
        // TODO: What should this be in HTML output?
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
        // TODO: What should this be in HTML output?
        Ok(())
    }

    #[inline]
    #[allow(unused_variables)]
    fn write_op_element_map_collection_to_block(&mut self,
                                                w: &mut io::Write,
                                                op: &ElementOp,
                                                doc: &DocumentState,
                                                scope: &ElementOpScope,
                                                coll_expr: &ExprValue,
                                                block_id: &str)
                                                -> Result {
        // TODO: What should this be in HTML output?
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
        let complete_key = scope.0.complete_element_key();
        let element_tag = element_tag.unwrap_or("div");

        write!(w, "<{} key=\"{}\">", element_tag, &complete_key)?;
        self.keys_vec.push(complete_key);
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
        let element_tag = element_tag.unwrap_or("div");
        write!(w, "</{}>", element_tag)?;
        Ok(())
    }
}
