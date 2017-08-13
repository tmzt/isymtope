
use std::io;
use std::clone::Clone;
use std::slice::Iter;
use std::collections::hash_map::HashMap;

use parser::ast::*;
use parser::util::allocate_element_key;
use processing::structs::*;
use processing::scope::*;

// use output::client_js::*;
use output::client_misc::*;
use output::client_output::*;
use output::client_js_value_writer::*;
use output::client_ops_writer::*;
use output::client_ops_stream_writer::*;


#[derive(Debug, Default)]
pub struct ElementOpsHtmlStreamWriter {
    pub events_vec: EventsVec,
    pub keys_vec: Vec<String>,
}

impl<'input: 'scope, 'scope> ElementOpsHtmlStreamWriter {

    pub fn new() -> Self { Default::default() }

    #[inline]
    #[allow(unused_variables)]
    fn write_html_js_action(&mut self, w: &mut io::Write, act_iter: Iter<ActionOpNode>, scope: &ElementOpScope) -> Result {
        write!(w, "function(event) {{")?;
        for ref act_op in act_iter {
            match *act_op {
                &ActionOpNode::DispatchAction(ref action, ref params) => {
                    let action_ty = scope.0.make_action_type(action);
                    //let action_type = resolve.action_type(Some(action));
                    write!(w, " store.dispatch({{\"type\": \"{}\"}}); ", action_ty)?;
                }
            }
        }
        write!(w, "}}")?;
        Ok(())
    }

    #[inline]
    fn write_element_attribute_expr_value(&mut self, w: &mut io::Write, key: &str, expr: &ExprValue, doc: &DocumentState, scope: &ElementOpScope) -> Result {
        match expr {
            &ExprValue::Expr(ExprOp::Add, ref l, ref r) => {}

            &ExprValue::DefaultAction(ref params, ref act_ops) => {
                if let &Some(ref act_ops) = act_ops {
                    self.write_html_js_action(w, act_ops.iter(), scope)?;
                };
            }
            &ExprValue::Action(ref event_name, ref params, ref act_ops) => {
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

    pub fn events_iter(&self) -> Iter<EventsItem> {
        self.events_vec.iter()
    }

    pub fn keys_iter(&self) -> Iter<String> {
        self.keys_vec.iter()
    }
}

impl<'input: 'scope, 'scope> ElementOpsStreamWriter for ElementOpsHtmlStreamWriter {
    fn write_op_element(&mut self, w: &mut io::Write, op: &ElementOp, doc: &DocumentState, scope: &ElementOpScope, element_key: &str, element_tag: &str, is_void: bool, attrs: Option<Iter<Prop>>, events: Option<Iter<EventHandler>>, value_binding: ElementValueBinding) -> Result {
        let complete_key = scope.0.complete_element_key();

        write!(w, "<{} key=\"{}\"", element_tag, &complete_key)?;

        if let Some(attrs) = attrs {
            for &(ref key, ref expr) in attrs {
                // Ignore empty attributes// 
                if let &Some(ref expr) = expr {
                    write!(w, " {}=", key)?;
                    self.write_element_attribute_expr_value(w, key, expr, doc, scope)?;
                };
            }
        };

        if is_void {
            write!(w, " />")?;
        } else {
            write!(w, ">")?;
        };

        // Process events
        // if let Some(events) = events {
        //     for &(ref event_name, ref event_params, ref action_ops) in events {
        //         let event_params = event_params.as_ref().map(|event_params| event_params.iter().cloned().collect());
        //         let action_ops = action_ops.as_ref().map(|action_ops| action_ops.iter().cloned().collect());
        //         let event_name = event_name.as_ref().map(Clone::clone);

        //         // let cur_scope = resolve.cur_scope.as_ref().map(|s| format!("{}", s));
        //         let default_action_scope = scope.0.default_action_scope();
        //         self.events_vec.push((complete_key.clone(),
        //                                 event_name,
        //                                 event_params,
        //                                 action_ops,
        //                                 default_action_scope,
        //                                 None));
        //     }
        // };

        self.keys_vec.push(complete_key);
        Ok(())
    }

    #[inline]
    fn write_op_element_close(&mut self, w: &mut io::Write, op: &ElementOp, doc: &DocumentState, scope: &ElementOpScope, element_tag: &str) -> Result {
        write!(w, "</{}>", element_tag)?;
        Ok(())
    }

    #[inline]
    fn write_op_element_value(&mut self, w: &mut io::Write, op: &ElementOp, doc: &DocumentState, scope: &ElementOpScope, expr: &ExprValue, value_key: &str) -> Result {
        let mut scope = scope.clone();
        scope.0.append_key("v");
        let complete_key = scope.0.complete_element_key();

        write!(w, "<span key=\"{}\">", complete_key)?;
        write_computed_expr_value(w, expr, doc, &scope)?;
        write!(w, "</span>")?;

        self.keys_vec.push(complete_key);
        Ok(())
    }

    #[inline]
    fn write_op_element_start_block(&mut self, w: &mut io::Write, op: &ElementOp, doc: &DocumentState, scope: &ElementOpScope, block_id: &str) -> Result {
        // TODO: What should this be in HTML output?
        Ok(())
    }

    #[inline]
    fn write_op_element_end_block(&mut self, w: &mut io::Write, op: &ElementOp, doc: &DocumentState, scope: &ElementOpScope, block_id: &str) -> Result {
        // TODO: What should this be in HTML output?
        Ok(())
    }

    #[inline]
    fn write_op_element_map_collection_to_block(&mut self, w: &mut io::Write, op: &ElementOp, doc: &DocumentState, scope: &ElementOpScope, coll_expr: &ExprValue, block_id: &str) -> Result {
        // TODO: What should this be in HTML output?
        Ok(())
    }

    #[inline]
    fn write_op_element_instance_component_open(&mut self, w: &mut io::Write, op: &ElementOp, doc: &DocumentState, scope: &ElementOpScope, comp: &Component, attrs: Option<Iter<Prop>>, lens: Option<&LensExprType>) -> Result {
        let complete_key = scope.0.complete_element_key();

        write!(w, "<div key=\"{}\">", &complete_key)?;
        self.keys_vec.push(complete_key);
        Ok(())
    }

    #[inline]
    fn write_op_element_instance_component_close(&mut self, w: &mut io::Write, op: &ElementOp, doc: &DocumentState, scope: &ElementOpScope, comp: &Component) -> Result {
        write!(w, "</div>")?;
        Ok(())
    }
}