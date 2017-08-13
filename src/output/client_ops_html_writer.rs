
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
use output::client_ops_writer::*;


#[derive(Debug)]
pub struct ElementOpsHtmlWriter<'input> {
    pub stream_writer: ElementOpsHtmlStreamWriter<'input>
}

#[derive(Debug)]
pub struct ElementOpsHtmlStreamWriter<'input> {
    pub util: WriteElementOpsUtilImpl<'input>,
    pub events_vec: EventsVec,
    pub keys_vec: Vec<String>,
}

impl<'input> ElementOpsHtmlWriter<'input> {
    pub fn with_doc(doc: &'input DocumentState<'input>) -> Self {
        let stream_writer = ElementOpsHtmlStreamWriter {
            util: WriteElementOpsUtilImpl::with_doc(doc),
            events_vec: Default::default(),
            keys_vec: Default::default()
        };

        ElementOpsHtmlWriter {
            stream_writer: stream_writer
        }
    }
}

impl<'input: 'scope, 'scope> ElementOpsWriterHtml<'input> for ElementOpsHtmlStreamWriter<'input> {
    #[inline]
    fn write_element_attribute_expr_value(&mut self, w: &mut io::Write, key: &str, expr: &'input ExprValue) -> Result {
        match expr {
            &ExprValue::Expr(ExprOp::Add, ref l, ref r) => {}

            &ExprValue::DefaultAction(ref params, ref act_ops) => {
                if let &Some(ref act_ops) = act_ops {
                    self.write_html_js_action(w, act_ops.iter())?;
                };
            }
            &ExprValue::Action(ref event_name, ref params, ref act_ops) => {
                if let &Some(ref act_ops) = act_ops {
                    self.write_html_js_action(w, act_ops.iter())?;
                };
            }
            _ => {
                write!(w, "\"")?;
                write_computed_expr_value(w, expr, None, None)?;
                write!(w, "\"")?;
            }
        };
        Ok(())
    }
}

impl<'input: 'scope, 'scope> ElementOpsStreamWriter<'input> for ElementOpsHtmlStreamWriter<'input> {
    fn write_op_element(&mut self, w: &mut io::Write, op: &'input ElementOp, doc: &DocumentState, scope: &Scope, element_key: &'input str, element_tag: &'input str, is_void: bool, attrs: Option<Iter<Prop>>, events: Option<Iter<EventHandler>>) -> Result {
        let base_key = scope.base_element_key(element_key);

        write!(w, "<{}", element_tag)?;
        write!(w, " data-id=\"{}\"", base_key)?;
        write!(w, " key=\"{}\"", base_key)?;

        if let Some(ref attrs) = attrs {
            for &(ref key, ref expr) in *attrs {
                // Ignore empty attributes
                if let &Some(ref expr) = expr {
                    self.write_element_attribute_expr_value(w, key, expr)?;
                };
            }
        };

        if is_void {
            write!(w, " />")?;
        } else {
            write!(w, ">")?;
        };

        // Process events
        if let Some(ref events) = events {
            for &(ref event_name, ref event_params, ref action_ops) in *events {
                let event_params = event_params.as_ref().map(|event_params| event_params.iter().cloned().collect());
                let action_ops = action_ops.as_ref().map(|action_ops| action_ops.iter().cloned().collect());
                let event_name = event_name.as_ref().map(Clone::clone);

                // let cur_scope = resolve.cur_scope.as_ref().map(|s| format!("{}", s));
                self.events_vec.push((base_key.clone(),
                                        event_name,
                                        event_params,
                                        action_ops,
                                        self.util.scope().key()));
            }
        }

        self.keys_vec.push(base_key);
        Ok(())
    }

    #[inline]
    fn write_op_element_close(&mut self, w: &mut io::Write, op: &'input ElementOp, doc: &DocumentState, scope: &Scope, element_tag: &'input str) -> Result {
        write!(w, "</{}>", element_tag)?;
        Ok(())
    }

    #[inline]
    fn write_op_element_value(&mut self, w: &mut io::Write, op: &'input ElementOp, doc: &DocumentState, scope: &Scope, expr: &ExprValue, value_key: &'input str) -> Result {
        let element_key = scope.base_element_key(value_key);
        write!(w, "<span key=\"{}\" data-id=\"{}\">", element_key, element_key)?;
        write_computed_expr_value(w, expr, None, None)?;
        write!(w, "</span>")?;

        self.keys_vec.push(element_key);
        Ok(())
    }

    #[inline]
    fn write_op_element_start_block(&mut self, w: &mut io::Write, op: &'input ElementOp, doc: &DocumentState, scope: &Scope, block_id: &str) -> Result {
        // TODO: What should this be in HTML output?
        Ok(())
    }

    #[inline]
    fn write_op_element_end_block(&mut self, w: &mut io::Write, op: &'input ElementOp, doc: &DocumentState, scope: &Scope, block_id: &str) -> Result {
        // TODO: What should this be in HTML output?
        Ok(())
    }

    #[inline]
    fn write_op_element_map_collection_to_block(&mut self, w: &mut io::Write, op: &'input ElementOp, doc: &DocumentState, scope: &Scope, coll_expr: &'input ExprValue, block_id: &str) -> Result {
        // TODO: What should this be in HTML output?
        Ok(())
    }

    #[inline]
    fn write_op_element_instance_component_open(&mut self, w: &mut io::Write, op: &'input ElementOp, doc: &DocumentState, scope: &Scope, comp: &'input Component<'input>, component_key: &str, component_id: &str, attrs: Option<Iter<Prop>>, lens: Option<&str>) -> Result {
        let base_key = scope.base_element_key(component_key);
        self.keys_vec.push(base_key);

        write!(w, "<div key=\"{}\" data-id=\"{}\" >", base_key, base_key)?;
        Ok(())
    }

    #[inline]
    fn write_op_element_instance_component_close(&mut self, w: &mut io::Write, op: &'input ElementOp, doc: &DocumentState, scope: &Scope, comp: &'input Component<'input>, component_key: &str, component_id: &str) -> Result {
        write!(w, "</div>")?;
        Ok(())
    }

}

impl<'input> ElementOpsHtmlStreamWriter<'input> {

    #[inline]
    pub fn with_doc(doc: &'input DocumentState<'input>) -> Self {
        ElementOpsHtmlStreamWriter {
            events_vec: Default::default(),
            keys_vec: Default::default()
        }
    }

    #[inline]
    #[allow(unused_variables)]
    fn write_html_js_action(&mut self, w: &mut io::Write, act_iter: Iter<ActionOpNode>) -> Result {
        write!(w, "function(event) {{")?;
        for ref act_op in act_iter {
            match *act_op {
                &ActionOpNode::DispatchAction(ref action, ref params) => {
                    //let action_type = resolve.action_type(Some(action));
                    write!(w, " store.dispatch({{\"type\": \"{}\"}}); ", action)?;
                }
            }
        }
        write!(w, "}}")?;
        Ok(())
    }

}