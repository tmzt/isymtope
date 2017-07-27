
use std::io;
use std::clone::Clone;
use std::slice::Iter;
use std::collections::hash_map::HashMap;

use parser::ast::*;
use parser::util::allocate_element_key;
use output::structs::*;
// use output::client_js::*;
use output::client_misc::*;
use output::client_output::*;
use output::client_ops_writer::*;


#[derive(Debug, Default)]
pub struct ElementOpsHtmlStreamWriter {
    pub events_vec: EventsVec,
    pub keys_vec: Vec<String>,
}

impl<'input: 'scope, 'scope> ElementOpsHtmlStreamWriter {

    pub fn new() -> Self { Default::default() }

    fn scope_prefix(&self, scope_prefix: Option<&ScopePrefixType>, key: &str) -> String {
        match scope_prefix {
            Some(&ScopePrefixType::ScopePrefix(ref prefix)) => {
                format!("{}_{}", prefix, key)
            },
            _ => format!("{}", key)
        }
    }

    fn scope_action_prefix(&self, scope_prefix: Option<&ScopePrefixType>, key: &str) -> String {
        match scope_prefix {
            Some(&ScopePrefixType::ScopePrefix(ref prefix)) => {
                format!("{}.{}", prefix.to_uppercase(), key.to_uppercase())
            },
            _ => format!("{}", key.to_uppercase())
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

impl<'input: 'scope, 'scope> ElementOpsStreamWriter<'input> for ElementOpsHtmlStreamWriter {
    fn write_op_element(&mut self, w: &mut io::Write, op: &'input ElementOp, doc: &DocumentState, scope_prefix: Option<&ScopePrefixType>, element_key: &'input str, element_tag: &'input str, is_void: bool, attrs: Option<Iter<'input, Prop>>, events: Option<Iter<EventHandler>>) -> Result {
        let base_key = self.scope_prefix(scope_prefix, element_key);

        write!(w, "<{}", element_tag)?;
        write!(w, " data-id=\"{}\"", base_key)?;
        write!(w, " key=\"{}\"", base_key)?;

        if let Some(attrs) = attrs {
            for &(ref key, ref expr) in attrs {
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
        if let Some(events) = events {
            for &(ref event_name, ref event_params, ref action_ops) in events {
                let event_params = event_params.as_ref().map(|event_params| event_params.iter().cloned().collect());
                let action_ops = action_ops.as_ref().map(|action_ops| action_ops.iter().cloned().collect());
                let event_name = event_name.as_ref().map(Clone::clone);

                // let cur_scope = resolve.cur_scope.as_ref().map(|s| format!("{}", s));
                self.events_vec.push((base_key.clone(),
                                        event_name,
                                        event_params,
                                        action_ops,
                                        Some(base_key.clone())));
            }
        }

        self.keys_vec.push(base_key);
        Ok(())
    }

    #[inline]
    fn write_op_element_close(&mut self, w: &mut io::Write, op: &'input ElementOp, doc: &DocumentState, scope_prefix: Option<&ScopePrefixType>, element_tag: &'input str) -> Result {
        write!(w, "</{}>", element_tag)?;
        Ok(())
    }

    #[inline]
    fn write_op_element_value(&mut self, w: &mut io::Write, op: &'input ElementOp, doc: &DocumentState, scope_prefix: Option<&ScopePrefixType>, expr: &ExprValue, value_key: &'input str) -> Result {
        let element_key = self.scope_prefix(scope_prefix, value_key);

        write!(w, "<span key=\"{}\" data-id=\"{}\">", element_key, element_key)?;
        write_computed_expr_value(w, expr, None, None)?;
        write!(w, "</span>")?;

        self.keys_vec.push(element_key);
        Ok(())
    }

    #[inline]
    fn write_op_element_start_block(&mut self, w: &mut io::Write, op: &'input ElementOp, doc: &DocumentState, scope_prefix: Option<&ScopePrefixType>, block_id: &str) -> Result {
        // TODO: What should this be in HTML output?
        Ok(())
    }

    #[inline]
    fn write_op_element_end_block(&mut self, w: &mut io::Write, op: &'input ElementOp, doc: &DocumentState, scope_prefix: Option<&ScopePrefixType>, block_id: &str) -> Result {
        // TODO: What should this be in HTML output?
        Ok(())
    }

    #[inline]
    fn write_op_element_map_collection_to_block(&mut self, w: &mut io::Write, op: &'input ElementOp, doc: &DocumentState, scope_prefix: Option<&ScopePrefixType>, coll_expr: &'input ExprValue, block_id: &str) -> Result {
        // TODO: What should this be in HTML output?
        Ok(())
    }

    #[inline]
    fn write_op_element_instance_component_open(&mut self, w: &mut io::Write, op: &'input ElementOp, doc: &DocumentState, scope_prefix: Option<&ScopePrefixType>, comp: &'input Component<'input>, component_key: &str, component_id: &str, attrs: Option<Iter<Prop>>, lens: Option<&str>) -> Result {
        let base_key = self.scope_prefix(scope_prefix, component_key);

        write!(w, "<div key=\"{}\" data-id=\"{}\" >", &base_key, &base_key)?;
        self.keys_vec.push(base_key);
        Ok(())
    }

    #[inline]
    fn write_op_element_instance_component_close(&mut self, w: &mut io::Write, op: &'input ElementOp, doc: &DocumentState, scope_prefix: Option<&ScopePrefixType>, comp: &'input Component<'input>, component_key: &str, component_id: &str) -> Result {
        write!(w, "</div>")?;
        Ok(())
    }
}