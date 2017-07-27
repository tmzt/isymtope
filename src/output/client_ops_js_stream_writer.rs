
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
use super::client_ops_writer::*;
use super::structs::*;

// pub trait JsWriter {}
// pub struct WriteElementOpsJsStream { }
// impl JsWriter for WriteElementOpsJsStream {}


pub struct ElementOpsJsStreamWriter {}

impl<'input> ElementOpsStreamWriter<'input> for ElementOpsJsStreamWriter {
    fn write_op_element(&mut self, w: &mut io::Write, op: &'input ElementOp, doc: &DocumentState, scope_prefix: Option<&ScopePrefixType>, element_key: &'input str, element_tag: &'input str, is_void: bool, attrs: Option<Iter<'input, Prop>>, events: Option<Iter<EventHandler>>) -> Result {

        let idx = 0;
        let base_key = self.scope_prefix(scope_prefix, element_key);
        let element_key = format!("{}_{}", base_key, idx);
        let key_expr = format!("\"{}\" + idx", base_key);

        // let attrs = attrs.as_ref().map(|attrs| attrs.iter().cloned().collect());
        // attrs.push(("data-id", element_key));

        if !is_void {
            write!(w,
                "IncrementalDOM.elementOpen(\"{}\", {}, [",
                element_tag,
                key_expr)
                ?;
        } else {
            write!(w,
                "IncrementalDOM.elementVoid(\"{}\", {}, [",
                element_tag,
                key_expr)
                ?;
        }

        // Static attrs
        if attrs.is_some() {
            self.write_js_incdom_attr_array(w, attrs, doc, scope_prefix, Some(&base_key))?;
        };

        // TODO: Dynamic attributes

        writeln!(w, "]);")?;
        Ok(())
    }

    #[inline]
    fn write_op_element_close(&mut self, w: &mut io::Write, op: &'input ElementOp, doc: &DocumentState, scope_prefix: Option<&ScopePrefixType>, element_tag: &'input str) -> Result {
        write!(w, "</{}>", element_tag)?;
        Ok(())
    }

    #[inline]
    fn write_op_element_value(&mut self, w: &mut io::Write, op: &'input ElementOp, doc: &DocumentState, scope_prefix: Option<&ScopePrefixType>, expr: &ExprValue, value_key: &'input str) -> Result {
        // let base_key = scope.base_element_key(value_key);
        // let key_expr = scope.element_key_expr(value_key);

        writeln!(w,
                "IncrementalDOM.elementOpen(\"span\", \"{}\", [\"key\", \"{}\", \"data-id\", \"{}\"]);",
                value_key,
                value_key,
                value_key)
            ?;
        write!(w, "IncrementalDOM.text(")?;
        self.write_js_expr_value(w, expr, doc, scope_prefix)?;
        writeln!(w, ");")?;
        writeln!(w, "IncrementalDOM.elementClose(\"span\");")?;

        Ok(())
    }

    #[inline]
    fn write_op_element_start_block(&mut self, w: &mut io::Write, op: &'input ElementOp, doc: &DocumentState, scope_prefix: Option<&ScopePrefixType>, block_id: &str) -> Result {
        let foridx = &format!("__foridx_{}", block_id);
        writeln!(w, "var __{} = function __{}_(line, {}){{", block_id, block_id, foridx)?; //FIXME

        Ok(())
    }

    #[inline]
    fn write_op_element_end_block(&mut self, w: &mut io::Write, op: &'input ElementOp, doc: &DocumentState, scope_prefix: Option<&ScopePrefixType>, block_id: &str) -> Result {
        writeln!(w, "}};")?;
        Ok(())
    }

    #[inline]
    fn write_op_element_map_collection_to_block(&mut self, w: &mut io::Write, op: &'input ElementOp, doc: &DocumentState, scope_prefix: Option<&ScopePrefixType>, coll_expr: &'input ExprValue, block_id: &str) -> Result {

        // let forvar_default = &format!("__forvar_{}", block_id);

        write!(w, "(")?;
        self.write_js_expr_value(w, coll_expr, doc, scope_prefix)?;
        writeln!(w, ").map(__{});", block_id)?;

        Ok(())
    }

    #[inline]
    fn write_op_element_instance_component_open(&mut self, w: &mut io::Write, op: &'input ElementOp, doc: &DocumentState, scope_prefix: Option<&ScopePrefixType>, comp: &'input Component<'input>, component_key: &str, component_id: &str, attrs: Option<Iter<'input, Prop>>, lens: Option<&str>) -> Result {
        let base_key = self.scope_prefix(scope_prefix, component_key);
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

            self.write_js_props_object(w, attrs, doc, scope_prefix)?;
        }

        Ok(())
    }

    #[inline]
    fn write_op_element_instance_component_close(&mut self, w: &mut io::Write, op: &'input ElementOp, doc: &DocumentState, scope_prefix: Option<&ScopePrefixType>, comp: &'input Component<'input>, component_key: &'input str, component_id: &str) -> Result {
        writeln!(w, "IncrementalDOM.elementClose(\"div\");")?;
        Ok(())
    }

}


impl<'input: 'scope, 'scope> ElementOpsJsStreamWriter {
    #[allow(unused_variables)]
    fn write_js_event_bindings(&self,
                                   w: &mut io::Write,
                                   events_iter: Iter<EventsItem>,
                                   scope_prefix: Option<&ScopePrefixType>)
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
                            let action_ty = self.scope_action_prefix(scope_prefix, action_key);

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

    fn write_store_definition(&mut self, w: &mut io::Write, doc: &DocumentState, scope_prefix: Option<&ScopePrefixType>) -> Result {
        // TODO: Implement default scope?

        // Generate script
        for (ref reducer_key, ref reducer_data) in doc.reducer_key_data.iter() {
            writeln!(w, "  function {}Reducer(state, action) {{", reducer_key)?;

            if let Some(ref actions) = reducer_data.actions {
                for ref action_data in actions {
                    let action_ty = self.scope_action_prefix(scope_prefix, reducer_key);

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
                self.write_js_expr_value(w, default_expr, doc, scope_prefix)?;
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

    fn write_component_definition(&mut self, w: &mut io::Write, comp: &Component, doc: &DocumentState, scope_prefix: Option<&ScopePrefixType>) -> Result {
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
                            self.write_js_expr_value(w, simple_expr, doc, scope_prefix)?;
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
                self.write_js_expr_value(w, default_expr, doc, scope_prefix)?;
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

    #[inline]
    fn write_js_var_reference(&mut self,
                                        w: &mut io::Write,
                                        var_name: Option<&str>,
                                        doc: &DocumentState,
                                        scope_prefix: Option<&ScopePrefixType>)
                                        -> Result {
        // let state_key = "".to_owned();
        // let state_key = scope.state_lookup_key(var_name);
        // let is_scope_key = state_key.map_or(false, |s| doc.default_state_map.contains_key(s.as_str()));
        // let var_reference = scope.var_reference(is_scope_key, var_name);
        // write!(w, "{}", var_reference)?;
        Ok(())
    }

    #[inline]
    fn write_js_expr_value(&mut self,
                                    w: &mut io::Write,
                                    node: &ExprValue,
                                    doc: &DocumentState,
                                    scope_prefix: Option<&ScopePrefixType>)
                                    -> Result {
        match node {
            // TODO: Handle the case where quotes appear in the string
            &ExprValue::LiteralString(ref s) => {
                write!(w, "\"{}\"", s)?;
            }
            &ExprValue::LiteralNumber(ref n) => {
                write!(w, "{}", n)?;
            }

            &ExprValue::LiteralArray(ref items) => {
                if let &Some(ref items) = items {
                    write!(w, "[")?;
                    for ref item in items {
                        self.write_js_expr_value(w, item, doc, scope_prefix)?;
                    }
                    write!(w, "]")?;
                };
            }

            &ExprValue::DefaultVariableReference => {
                self.write_js_var_reference(w, None, doc, scope_prefix)?;
            }

            &ExprValue::VariableReference(ref var_name) => {
                self.write_js_var_reference(w, Some(var_name.as_str()), doc, scope_prefix)?;
            }

            &ExprValue::Expr(ExprOp::Add, box ExprValue::DefaultVariableReference, ref r) => {
                // let state_ty = self.scope().unwrap().state_lookup_key(None);
                // let state_ty = state_ty.map_or(None, |s| doc.default_state_map.get(s.as_str()));

                // write!(w, "(")?;
                // self.write_js_var_reference(w, None, doc, scope)?;
                // if let Some(&(Some(VarType::ArrayVar(..)), _)) = state_ty {
                //     write!(w, ").concat(")?;
                // } else {
                //     write!(w, "+ (")?;
                // }
                // self.write_js_expr_value(w, r, doc, scope)?;
                // write!(w, ")")?;
            }

            &ExprValue::Expr(ExprOp::Add, box ExprValue::VariableReference(ref var_name), ref r) => {
                // let state_ty = scope.state_lookup_key(Some(var_name.as_str())).as_ref()
                //     .map_or(None, |s| doc.default_state_map.get(s.as_str()));

                // write!(w, "(")?;
                // self.write_js_var_reference(w, None, doc, scope)?;
                // if let Some(&(Some(VarType::ArrayVar(..)), _)) = state_ty {
                //     write!(w, ").concat(")?;
                // } else {
                //     write!(w, "+ (")?;
                // }
                // self.write_js_expr_value(w, r, doc, scope)?;
                // write!(w, ")")?;
            }

            &ExprValue::Expr(ref sym, ref l, ref r) => {
                self.write_js_expr_value(w, l, doc, scope_prefix)?;
                match sym {
                    &ExprOp::Add => {
                        write!(w, " + ")?;
                    }
                    &ExprOp::Sub => {
                        write!(w, " - ")?;
                    }
                    &ExprOp::Mul => {
                        write!(w, " * ")?;
                    }
                    &ExprOp::Div => {
                        write!(w, " / ")?;
                    }
                }
                self.write_js_expr_value(w, r, doc, scope_prefix)?;
            }

            &ExprValue::ContentNode(..) => {}
            &ExprValue::DefaultAction(..) => {}
            &ExprValue::Action(..) => {}
        }
        Ok(())
    }

    #[inline]
    #[allow(unused_variables)]
    fn write_js_props_object(&mut self,
                                    w: &mut io::Write,
                                    props: Option<Iter<'input, Prop>>,
                                    doc: &DocumentState,
                                    scope_prefix: Option<&ScopePrefixType>)
                            -> Result {
        write!(w, "{{")?;
        let mut wrote_first = false;
        if let Some(props) = props {
            for &(ref key, ref expr) in props {
                if wrote_first {
                    write!(w, ", ")?
                } else {
                    wrote_first = true;
                }

                // Write the property name
                write!(w, "\"{}\": ", key)?;

                // Write the property value or undefined for None
                if let &Some(ref expr) = expr {

                    if let &ExprValue::DefaultAction(ref params, ref act_ops) = expr {
                        write!(w, "\"{}\", ", key)?;
                        write!(w, "function(event) {{")?;
                        if let &Some(ref act_ops) = act_ops {
                            for ref act_op in act_ops {
                                match *act_op {
                                    &ActionOpNode::DispatchAction(ref action, ref params) => {
                                        write!(w, " store.dispatch({{\"type\": \"{}\"}}); ", action)?;
                                    }
                                }
                            }
                        }
                        write!(w, "}}")?;
                        continue;
                    };

                    self.write_js_expr_value(w,
                                        &expr,
                                        doc,
                                        scope_prefix)?;

                } else {
                    write!(w, "undefined")?;
                }
            }
        };
        write!(w, "}}")?;
        Ok(())
    }

    #[inline]
    #[allow(dead_code)]
    #[allow(unused_variables)]
    fn write_js_incdom_attr_array(&mut self,
                                        w: &mut io::Write,
                                        attrs: Option<Iter<'scope, Prop>>,
                                        doc: &DocumentState,
                                        scope_prefix: Option<&ScopePrefixType>,
                                        base_key: Option<&str>)
                                        -> Result {
        let mut wrote_first = false;
        if let Some(base_key) = base_key {
            write!(w, "\"data-id\", \"{}\", ", base_key)?;
        };

        if let Some(attrs) = attrs {
            for &(ref key, ref expr) in attrs {
                if let &Some(ref expr) = expr {
                    if wrote_first {
                        write!(w, ", ")?
                    } else {
                        wrote_first = true;
                    }

                    if let &ExprValue::DefaultAction(ref params, ref act_ops) = expr {
                        write!(w, "\"{}\", ", key)?;
                        write!(w, "function(event) {{")?;
                        if let &Some(ref act_ops) = act_ops {
                            for ref act_op in act_ops {
                                match *act_op {
                                    &ActionOpNode::DispatchAction(ref action, ref params) => {
                                        write!(w, " store.dispatch({{\"type\": \"{}\"}}); ", action)?;
                                    }
                                }
                            }
                        }
                        write!(w, "}}")?;
                        continue;
                    };

                    write!(w, "\"{}\", ", key)?;
                    self.write_js_expr_value(w, expr, doc, scope_prefix)?;
                } else {
                    write!(w, "\"{}\", ", key)?;
                    write!(w, "undefined")?;
                }
            }
        };
        Ok(())
    }
}