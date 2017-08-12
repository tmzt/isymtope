
use std::io;
use std::clone::Clone;
use std::slice::Iter;
use std::collections::hash_map::HashMap;

use parser::ast::*;
use parser::util::allocate_element_key;
use parser::store::*;
use processing::structs::*;
use processing::scope::*;
use output::scope::*;
use output::client_misc::*;
use output::client_output::*;
use output::client_js_value_writer::*;
use output::client_ops_writer::*;
use output::client_ops_stream_writer::*;


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
                                   scope: &ElementOpScope)
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
                            let action_ty = scope.0.make_action_type(action_key);

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

    fn write_store_definition(&mut self, w: &mut io::Write, doc: &DocumentState, scope: &ElementOpScope) -> Result {
        // TODO: Implement default scope?

        // Generate script
        for (ref reducer_key, ref reducer_data) in doc.reducer_key_data.iter() {
            writeln!(w, "  function {}Reducer(state, action) {{", reducer_key)?;

            if let Some(ref actions) = reducer_data.actions {
                for ref action_data in actions {
                    let action_ty = scope.0.make_action_type(reducer_key);

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
                write_js_expr_value(w, default_expr, doc, scope)?;
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

    fn write_component_definition(&mut self, w: &mut io::Write, comp: &Component, doc: &DocumentState, scope: &ElementOpScope) -> Result {
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
                            write_js_expr_value(w, simple_expr, doc, scope)?;
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
                write_js_expr_value(w, default_expr, doc, scope)?;
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

    fn write_element_open(&mut self, w: &mut io::Write, op: &ElementOp, doc: &DocumentState, scope: &ElementOpScope, element_key: &str, element_tag: &str, complete_key: &str, is_void: bool, attrs: Option<Iter<Prop>>, events: Option<Iter<EventHandler>>, value_binding: ElementValueBinding) -> Result {
        let mut scope = scope.clone();
        let param_expr = ExprValue::SymbolReference(Symbol::param("key_prefix"));
        let key_expr = ExprValue::LiteralString(format!(".{}", element_key));
        scope.0.set_prefix_expr(&param_expr);

        if !is_void {
            write!(w, "IncrementalDOM.elementOpen(\"{}\", ", element_tag)?;
        } else {
            write!(w, "IncrementalDOM.elementVoid(\"{}\", ", element_tag)?;
        };

        let prefix_expr = scope.0.make_prefix_expr(&key_expr, None);
        if let Some(ref prefix_expr) = prefix_expr {
            write_js_expr_value(w, prefix_expr, doc, &scope)?;
        } else {
            write!(w, "\"{}\"", complete_key)?;
        }
        write!(w, ", [")?;

        // let default_attrs = vec![
        //     ("key".to_owned(), Some(ExprValue::LiteralString(complete_key.to_owned()))),
        //     ("data-id".to_owned(), Some(ExprValue::LiteralString(complete_key.to_owned())))
        // ];
        // let attrs = attrs.or_else(|| Some(default_attrs.iter()));

        // Static attrs
        if attrs.is_some() {
            write_js_incdom_attr_array(w, attrs, doc, &scope, Some(&element_key))?;
        };

        // TODO: Dynamic attributes

        writeln!(w, "]);")?;
        Ok(())
    }

    fn write_element_close(&mut self, w: &mut io::Write, element_tag: &str) -> Result {
        writeln!(w,
            "IncrementalDOM.elementClose(\"{}\");",
            element_tag)?;
        Ok(())
    }

}

impl<'input: 'scope, 'scope> ElementOpsStreamWriter for ElementOpsJsStreamWriter {
    fn write_op_element(&mut self, w: &mut io::Write, op: &ElementOp, doc: &DocumentState, scope: &ElementOpScope, element_key: &str, element_tag: &str, is_void: bool, attrs: Option<Iter<Prop>>, events: Option<Iter<EventHandler>>, value_binding: ElementValueBinding) -> Result {
        let mut scope = scope.clone();
        let complete_key = scope.0.complete_element_key();
        let param_expr = ExprValue::SymbolReference(Symbol::param("key_prefix"));
        let key_expr = ExprValue::LiteralString(format!(".{}", element_key));
        scope.0.set_prefix_expr(&param_expr);

        self.write_element_open(w, op, doc, &scope, element_key, element_tag, &complete_key, is_void, attrs, events, value_binding)
    }

    #[inline]
    fn write_op_element_close(&mut self, w: &mut io::Write, op: &ElementOp, doc: &DocumentState, scope: &ElementOpScope, element_tag: &str) -> Result {
        self.write_element_close(w, element_tag)
    }

    #[inline]
    fn write_op_element_value(&mut self, w: &mut io::Write, op: &ElementOp, doc: &DocumentState, scope: &ElementOpScope, expr: &ExprValue, value_key: &str) -> Result {
        let mut scope = scope.clone();
        scope.0.append_key("v");
        let complete_key = scope.0.complete_element_key();
        let param_expr = ExprValue::SymbolReference(Symbol::param("key_prefix"));
        let key_expr = ExprValue::LiteralString(format!(".{}.v", value_key));
        scope.0.set_prefix_expr(&param_expr);

        self.write_element_open(w, op, doc, &scope, value_key, "span", &complete_key, false, None, None, None)?;

        write!(w, "IncrementalDOM.text(")?;
        write_js_expr_value(w, expr, doc, &scope)?;
        writeln!(w, ");")?;

        self.write_element_close(w, "span")?;

        Ok(())
    }

    #[inline]
    fn write_op_element_start_block(&mut self, w: &mut io::Write, op: &ElementOp, doc: &DocumentState, scope: &ElementOpScope, block_id: &str) -> Result {
        let foridx = &format!("__foridx_{}", block_id);
        writeln!(w, "var __{} = function __{}_(line, {}){{", block_id, block_id, foridx)?; //FIXME

        Ok(())
    }

    #[inline]
    fn write_op_element_end_block(&mut self, w: &mut io::Write, op: &ElementOp, doc: &DocumentState, scope: &ElementOpScope, block_id: &str) -> Result {
        writeln!(w, "}};")?;
        Ok(())
    }

    #[inline]
    fn write_op_element_map_collection_to_block(&mut self, w: &mut io::Write, op: &ElementOp, doc: &DocumentState, scope: &ElementOpScope, coll_expr: &ExprValue, block_id: &str) -> Result {
        // let forvar_default = &format!("__forvar_{}", block_id);

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
    fn write_op_element_instance_component_open(&mut self, w: &mut io::Write, op: &ElementOp, doc: &DocumentState, scope: &ElementOpScope, comp: &Component, attrs: Option<Iter<Prop>>, lens: Option<&LensExprType>) -> Result {
        let mut scope = scope.clone();
        scope.0.clear_index();

        let complete_key = scope.0.complete_element_key();
        let component_ty = &comp.name;

        // let key_expr = ExprValue::LiteralString(format!(".{}", complete_key));
        // let prefix_expr = scope.0.make_prefix_expr(&key_expr, None);

        if let Some(&LensExprType::ForLens(Some(ref ele_key), ref coll_sym)) = lens {
            let ele_expr = ExprValue::SymbolReference(Symbol::prop(ele_key));
            let coll_expr = ExprValue::SymbolReference(coll_sym.to_owned());
            // write_js_expr_value(w, &coll_expr, doc, &scope)?;

            let attrs = vec![
                (ele_key.to_owned(), Some(ele_expr))
            ];
            write!(w, "((")?;
            // write_js_props_object(w, Some(attrs.iter()), doc, scope)?;
            write_js_expr_value(w, &coll_expr, doc, &scope)?;
            // write!(w, ").map(function(v) {{ return {{  {} }}))")?;

            write!(w, ").map(function(v) {{ return ")?;
            // write_js_props_object(w, Some(attrs.iter()), doc, scope)?;
            write!(w, "{{ {}: v }}", ele_key)?;
            writeln!(w, "; }}))")?;

            // let loop_prefix_expr = prefix_expr
            //     .unwrap_or_else(|| ExprValue::LiteralString(format!("{}", complete_key)));

            // let foridx_expr = ExprValue::Expr(ExprOp::Add,
            //     Box::new(ExprValue::Expr(ExprOp::Add,
            //         Box::new(loop_prefix_expr.clone()),
            //         Box::new(ExprValue::LiteralString(".".to_owned()))
            //     )),
            //     Box::new(ExprValue::SymbolReference(Symbol::param("foridx")))
            // );

            // let foridx_expr = ExprValue::Expr(ExprOp::Add,
            //     Box::new(ExprValue::Expr(ExprOp::Add,
            //         Box::new(ExprValue::LiteralString())
            //         Box::new(loop_prefix_expr.clone()),
            //         Box::new(ExprValue::LiteralString(".".to_owned()))
            //     )),
            //     Box::new(ExprValue::SymbolReference(Symbol::param("key_prefix"))),
            // );

            let foridx_expr = ExprValue::Expr(ExprOp::Add,
                Box::new(ExprValue::LiteralString(format!("{}.", complete_key))),
                Box::new(ExprValue::SymbolReference(Symbol::param("foridx")))
            );

            // let foridx_expr = ExprValue::Expr(ExprOp::Add,
            //     Box::new(ExprValue::Expr(ExprOp::Add,
            //         Box::new(loop_prefix_expr.clone()),
            //         Box::new(ExprValue::LiteralString(".".to_owned()))
            //     )),
            //     Box::new(ExprValue::SymbolReference(Symbol::param("key_prefix"))),
            // );

            // let foridx_expr = ExprValue::Expr(ExprOp::Add,
            //     Box::new(ExprValue::Expr(ExprOp::Add,
            //         Box::new(ExprValue::SymbolReference(Symbol::param("key_prefix"))),
            //         Box::new(ExprValue::LiteralString(".".to_owned()))
            //     )),
            //     Box::new(ExprValue::SymbolReference(Symbol::param("foridx")))
            // );
            scope.0.set_prefix_expr(&foridx_expr);

            writeln!(w, ".map(function(props, foridx) {{")?;

        };

        // let key_expr = ExprValue::LiteralString(format!(".{}", complete_key));
        // let prefix_expr = scope.0.make_prefix_expr(&key_expr, None);

        write!(w, "component_{}(", component_ty)?;
        if let Some(ref prefix_expr) = scope.0.prefix_expr() {
            write_js_expr_value(w, prefix_expr, doc, &scope)?;
        } else {
            write!(w, "\"{}\"", complete_key)?;
        };
        write!(w, ", store")?;

        if attrs.is_some() {
            write!(w, ", ")?;
            // TODO: Fix lens support
            write_js_props_object(w, attrs, doc, &scope)?;
        }
        writeln!(w, ");")?;

        if lens.is_some() {
            writeln!(w, "}});")?;
        };

        Ok(())
    }

    #[inline]
    fn write_op_element_instance_component_close(&mut self, w: &mut io::Write, op: &ElementOp, doc: &DocumentState, scope: &ElementOpScope, comp: &Component) -> Result {
        // writeln!(w, "IncrementalDOM.elementClose(\"div\");")?;
        Ok(())
    }

}