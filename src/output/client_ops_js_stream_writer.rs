
use std::io;
use std::clone::Clone;
use std::slice::Iter;
use std::collections::hash_map::HashMap;

use parser::ast::*;
use parser::util::allocate_element_key;
use parser::store::*;
use super::structs::*;
// use super::client_js::*;
// use super::client_html::*;
use super::client_misc::*;
// use super::client_misc_html::*;
use super::client_output::*;
use output::client_js_value_writer::*;
use super::client_ops_writer::*;
use super::client_ops_stream_writer::*;
use super::structs::*;

// pub trait JsWriter {}
// pub struct WriteElementOpsJsStream { }
// impl JsWriter for WriteElementOpsJsStream {}


#[derive(Debug, Default)]
pub struct ElementOpsJsStreamWriter { }

impl<'input: 'scope, 'scope> ElementOpsJsStreamWriter {

    pub fn new() -> Self {
        ElementOpsJsStreamWriter { }
    }

    #[allow(unused_variables)]
    fn write_js_event_bindings(&self,
                                   w: &mut io::Write,
                                   events_iter: Iter<EventsItem>,
                                   scope_prefixes: &ScopePrefixes)
                                   -> Result {
        writeln!(w, "      // Bind actions")?;
        for &(ref element_key, ref event_name, ref params, ref action_ops, ref event_scope) in
            events_iter {
            let event_name = event_name.as_ref().map(String::as_str).map_or("click", |s| s);
            writeln!(w,
                     "  document.querySelector(\"[data-id='{}']\").addEventListener(\"{}\", \
                      function(event) {{",
                     element_key,
                     event_name)
                ?;

            if let &Some(ref action_ops) = action_ops {
                let scope_id = String::new();
                // self.push_scope(scope_id, |prev| prev.with_state_key(event_scope.map(|s| s.as_str())));
                // let action_scope = self.util.scope();

                // let action_scope = event_scope.as_ref()
                //     .map(|event_scope| scope.with_state_key(event_scope.as_str()));
                // let action_scope_prefix: &ScopePrefixType = action_scope.as_ref().map_or(scope, |r| r);

                for ref action_op in action_ops {
                    match *action_op {
                        &ActionOpNode::DispatchAction(ref action_key, ref action_params) => {
                            let action_ty = scope_prefixes.action_prefix(action_key);

                            // let action_ty = scope.action_type(action_key.as_str());
                            /*
                            // TODO: Fix type
                            let action_prefix = scope.as_ref()
                                .map_or("".into(), |s| s.to_uppercase());
                            let action_ty =
                                format!("{}.{}", action_prefix, action_key.to_uppercase());
                            */
                            writeln!(w, " store.dispatch({{\"type\": \"{}\"}}); ", action_ty)?;
                        }
                    }
                }
            }
            writeln!(w, "  }});")?;
        }
        Ok(())
    }

    fn write_store_definition(&mut self, w: &mut io::Write, doc: &DocumentState, scope_prefixes: &ScopePrefixes) -> Result {
        // TODO: Implement default scope?

        // Generate script
        for (ref reducer_key, ref reducer_data) in doc.reducer_key_data.iter() {
            writeln!(w, "  function {}Reducer(state, action) {{", reducer_key)?;

            if let Some(ref actions) = reducer_data.actions {
                for ref action_data in actions {
                    let action_ty = scope_prefixes.action_prefix(reducer_key);

                    match &action_data.state_expr {
                        &Some(ActionStateExprType::SimpleReducerKeyExpr(ref simple_expr)) => {
                            writeln!(w,
                                     "if ('undefined' !== typeof action && '{}' == action.type) \
                                      {{",
                                      action_ty)
                                ?;
                            write!(w, "  return ")?;
                            // write!(w, "Object.assign({{ \"{}\": ", reducer_key)?;
                            // self.write_js_expr_value(w, simple_expr, &self.doc, &action_scope)?;
                            writeln!(w, ";")?;
                            // writeln!(w, "}})")?;
                            writeln!(w, "}}")?;
                        }
                        _ => {}
                    }
             
                }
            }

            // Default expression used to initialize state
            write!(w, "    return state || ")?;
            if let Some(ref default_expr) = reducer_data.default_expr {
                // write!(w, "Object.assign({{ \"{}\": ", reducer_key)?;
                write_js_expr_value(w, default_expr, doc, scope_prefixes)?;
                // write!(w, "}})")?;
            } else {
                write!(w, "null")?;
            }
            writeln!(w, ";")?;

            writeln!(w, "  }}")?;
        }

        writeln!(w, "  var rootReducer = Redux.combineReducers({{")?;
        for (ref reducer_key, _) in doc.reducer_key_data.iter() {
            writeln!(w, "    {}: {}Reducer,", &reducer_key, &reducer_key)?;
        }
        writeln!(w, "  }});")?;

        writeln!(w, "  var store = Redux.createStore(rootReducer, {{}});")?;

        Ok(())
    }

    fn write_component_definition(&mut self, w: &mut io::Write, comp: &Component, doc: &DocumentState, scope_prefixes: &ScopePrefixes) -> Result {
        // TODO: Implement default scope?

        // Generate script
        for (ref reducer_key, ref reducer_data) in doc.reducer_key_data.iter() {
            writeln!(w, "  function {}Reducer(state, action) {{", reducer_key)?;

            // let scope_id = String::new();
            // self.push_scope(scope_id, |prev| Box::new(prev.with_state_key(reducer_key)));

            // let reducer_scope = scope.with_state_key(reducer_key);
            // let reducer_scope_prefix: &ScopePrefixType = &reducer_scope;

            if let Some(ref actions) = reducer_data.actions {
                for ref action_data in actions {
                    // let action_ty = self.util().scope().and_then(|s| s.action_type(&action_data.action_type));

                    let mut action_ty = format!("{}", &action_data.action_type);
                    // if let Some((ref prefix, _)) = self.scopes.

                    match &action_data.state_expr {
                        &Some(ActionStateExprType::SimpleReducerKeyExpr(ref simple_expr)) => {
                            // self.push_scope(scope_id, |prev| prev.action_result(reducer_key));
                            // let action_scope = reducer_scope.action_result(reducer_key);
                            writeln!(w,
                                     "if ('undefined' !== typeof action && '{}' == action.type) \
                                      {{",
                                      action_ty)
                                ?;
                            write!(w, "  return ")?;
                            // write!(w, "Object.assign({{ \"{}\": ", reducer_key)?;
                            write_js_expr_value(w, simple_expr, doc, scope_prefixes)?;
                            writeln!(w, ";")?;
                            // writeln!(w, "}})")?;
                            writeln!(w, "}}")?;
                        }
                        _ => {}
                    }
             
                }
            }

            // Default expression used to initialize state
            write!(w, "    return state || ")?;
            if let Some(ref default_expr) = reducer_data.default_expr {
                // write!(w, "Object.assign({{ \"{}\": ", reducer_key)?;
                write_js_expr_value(w, default_expr, doc, scope_prefixes)?;
                // write!(w, "}})")?;
            } else {
                write!(w, "null")?;
            }
            writeln!(w, ";")?;

            writeln!(w, "  }}")?;
        }

        writeln!(w, "  var rootReducer = Redux.combineReducers({{")?;
        for (ref reducer_key, _) in doc.reducer_key_data.iter() {
            writeln!(w, "    {}: {}Reducer,", &reducer_key, &reducer_key)?;
        }
        writeln!(w, "  }});")?;

        writeln!(w, "  var store = Redux.createStore(rootReducer, {{}});")?;

        Ok(())
    }
}


impl<'input: 'scope, 'scope> ElementOpsStreamWriter<'input> for ElementOpsJsStreamWriter {
    fn write_op_element(&mut self, w: &mut io::Write, op: &'input ElementOp, doc: &DocumentState, scope_prefixes: &ScopePrefixes, element_key: &str, element_tag: &'input str, is_void: bool, attrs: Option<Iter<'input, Prop>>, events: Option<Iter<EventHandler>>) -> Result {

        let idx = 0;
        let base_key = scope_prefixes.key_prefix(element_key);
        let base_expr = scope_prefixes.key_expr_prefix(&base_key);
        let element_key = format!("{}_{}", base_key, idx);
        // let key_expr = format!("\"{}\" + idx", base_key);
        let key_expr = format!("\"{}\"", base_key);

        // let attrs = attrs.as_ref().map(|attrs| attrs.iter().cloned().collect());
        // attrs.push(("data-id", element_key));

        if !is_void {
            write!(w, "IncrementalDOM.elementOpen(\"{}\", ", element_tag)?;
        } else {
            write!(w, "IncrementalDOM.elementVoid(\"{}\", ", element_tag)?;
        };

        write!(w, "\"{}\"", base_key)?;
        if let Some(ref expr_prefix) = base_expr {
            write!(w, " + ")?;
            write_js_expr_value(w, expr_prefix, doc, scope_prefixes)?;
        };
        write!(w, ", [")?;

        // Static attrs
        if attrs.is_some() {
            write_js_incdom_attr_array(w, attrs, doc, scope_prefixes, Some(&base_key))?;
        };

        // TODO: Dynamic attributes

        writeln!(w, "]);")?;
        Ok(())
    }

    #[inline]
    fn write_op_element_close(&mut self, w: &mut io::Write, op: &'input ElementOp, doc: &DocumentState, scope_prefixes: &ScopePrefixes, element_tag: &'input str) -> Result {
        writeln!(w,
            "IncrementalDOM.elementClose(\"{}\");",
            element_tag)?;
        Ok(())
    }

    #[inline]
    fn write_op_element_value(&mut self, w: &mut io::Write, op: &'input ElementOp, doc: &DocumentState, scope_prefixes: &ScopePrefixes, expr: &ExprValue, value_key: &str) -> Result {
        // let base_key = scope.base_element_key(value_key);
        // let key_expr = scope.element_key_expr(value_key);

        write!(w, "IncrementalDOM.elementOpen(\"span\", ")?;
        write!(w, "\"{}\"", value_key)?;
        if let Some(ref expr_prefix) = scope_prefixes.key_expr_prefix(value_key) {
            write!(w, " + ")?;
            write_js_expr_value(w, expr_prefix, doc, scope_prefixes)?;
        };
        writeln!(w, ", [\"key\", \"{}\", \"data-id\", \"{}\"]);", value_key, value_key)?;

        write!(w, "IncrementalDOM.text(")?;
        write_js_expr_value(w, expr, doc, scope_prefixes)?;
        writeln!(w, ");")?;

        writeln!(w, "IncrementalDOM.elementClose(\"span\");")?;

        Ok(())
    }

    #[inline]
    fn write_op_element_start_block(&mut self, w: &mut io::Write, op: &'input ElementOp, doc: &DocumentState, scope_prefixes: &ScopePrefixes, block_id: &str) -> Result {
        let foridx = &format!("__foridx_{}", block_id);
        writeln!(w, "var __{} = function __{}_(line, {}){{", block_id, block_id, foridx)?; //FIXME

        Ok(())
    }

    #[inline]
    fn write_op_element_end_block(&mut self, w: &mut io::Write, op: &'input ElementOp, doc: &DocumentState, scope_prefixes: &ScopePrefixes, block_id: &str) -> Result {
        writeln!(w, "}};")?;
        Ok(())
    }

    #[inline]
    fn write_op_element_map_collection_to_block(&mut self, w: &mut io::Write, op: &'input ElementOp, doc: &DocumentState, scope_prefixes: &ScopePrefixes, coll_expr: &'input ExprValue, block_id: &str) -> Result {

        // let forvar_default = &format!("__forvar_{}", block_id);

        write!(w, "(")?;

        let foridx = &format!("__foridx_{}", block_id);
        let scope_prefixes = with_key_expr_prefix(scope_prefixes, ExprValue::VariableReference(foridx.clone()));
        let scope_prefixes = prepend_var_prefix(&scope_prefixes, "store.getState()");
        write_js_expr_value(w, coll_expr, doc, &scope_prefixes)?;
        writeln!(w, ").forEach(__{});", block_id)?;

        Ok(())
    }

    #[inline]
    fn write_op_element_instance_component_open(&mut self, w: &mut io::Write, op: &'input ElementOp, doc: &DocumentState, scope_prefixes: &ScopePrefixes, comp: &'input Component<'input>, component_key: &str, component_id: &str, attrs: Option<Iter<'input, Prop>>, lens: Option<&str>) -> Result {
        let base_key = scope_prefixes.key_prefix(component_key);
        let component_ty = comp.name;

        writeln!(w,
                "IncrementalDOM.elementOpen(\"div\", \"{}\", []);",
                component_key)
            ?;
        write!(w,
            "component_{}(\"{}_\", store, ",
            component_ty,
            component_key)
            ?;
        if attrs.is_some() {
            let var_prefix = lens.as_ref().map(|s| format!("store.getState().{}.", s));
            let default_var = lens.as_ref().map(|s| format!("store.getState.{}", s));
            let default_scope = lens.as_ref().map(|s| s);

            write_js_props_object(w, attrs, doc, scope_prefixes)?;
        }
        writeln!(w, ");")?;

        Ok(())
    }

    #[inline]
    fn write_op_element_instance_component_close(&mut self, w: &mut io::Write, op: &'input ElementOp, doc: &DocumentState, scope_prefixes: &ScopePrefixes, comp: &'input Component<'input>, component_key: &str, component_id: &str) -> Result {
        writeln!(w, "IncrementalDOM.elementClose(\"div\");")?;
        Ok(())
    }

}