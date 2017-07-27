
use std::io;
use std::clone::Clone;
use std::slice::Iter;

use parser::ast::*;
use parser::store::*;
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

    fn scope_prefix(&self, scope_prefix: Option<&ScopePrefixType>, key: &str) -> String {
        match scope_prefix {
            Some(&ScopePrefixType::ScopePrefix(ref prefix)) => {
                format!("{}.{}", prefix, key)
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

    pub fn write_js_store(&mut self,
                          w: &mut io::Write,
                          scope_prefix: Option<&ScopePrefixType>)
                          -> Result {
        // TODO: Implement default scope?

        // Generate script
        for (ref reducer_key, ref reducer_data) in self.doc.reducer_key_data.iter() {
            writeln!(w, "  function {}Reducer(state, action) {{", reducer_key)?;

            let reducer_scope_key = self.scope_prefix(scope_prefix, reducer_key);

            if let Some(ref actions) = reducer_data.actions {
                for ref action_data in actions {
                    let action_ty = format!("{}.{}", reducer_scope_key, &action_data.action_type);

                    match &action_data.state_expr {
                        &Some(ActionStateExprType::SimpleReducerKeyExpr(ref simple_expr)) => {

                            writeln!(w,
                                     "if ('undefined' !== typeof action && '{}' == action.type) \
                                      {{",
                                     action_ty)
                                ?;
                            write!(w, "  return ")?;
                            // write!(w, "Object.assign({{ \"{}\": ", reducer_key)?;
                            // self.output_js.write_js_expr_value(w, simple_expr, &self.doc, &action_scope)?;
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
                // self.output_js.write_js_expr_value(w, default_expr, &mut self.doc, &resolve)?;
                // write!(w, "}})")?;
            } else {
                write!(w, "null")?;
            }
            writeln!(w, ";")?;

            writeln!(w, "  }}")?;
        }

        writeln!(w, "  var rootReducer = Redux.combineReducers({{")?;
        for (ref reducer_key, _) in self.doc.reducer_key_data.iter() {
            writeln!(w, "    {}: {}Reducer,", &reducer_key, &reducer_key)?;
        }
        writeln!(w, "  }});")?;

        writeln!(w, "  var store = Redux.createStore(rootReducer, {{}});")?;

        Ok(())
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