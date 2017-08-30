
use std::io;
use std::clone::Clone;
use std::slice::Iter;

use parser::store::*;
use processing::structs::*;
use processing::scope::*;

use output::client_js_value_writer::*;
use output::client_ops_writer::*;
use output::client_ops_js_stream_writer::output_writer::*;
use output::client_ops_js_stream_writer::output_stream_writer::*;
use output::stream_writers::output_writer_js::*;
use output::stream_writers::output_stream_writer_js::*;

use scope::scope::*;
use scope::context::*;


pub struct WriteJsOps<'input> {
    pub doc: &'input DocumentState<'input>,
    pub stream_writer: ElementOpsJsStreamWriter,
}

impl<'input> WriteJsOps<'input> {
    pub fn with_doc(doc: &'input DocumentState<'input>) -> Self {
        WriteJsOps {
            doc: doc,
            stream_writer: Default::default(),
        }
    }

    pub fn write_js_store(&mut self, w: &mut io::Write, scope: &ElementOpScope) -> Result {
        // TODO: Implement default scope?

        // Generate script
        for (ref reducer_key, ref reducer_data) in self.doc.reducer_key_data.iter() {
            writeln!(w, "  function {}Reducer(state, action) {{", reducer_key)?;

            let reducer_scope_key = scope.0.make_action_type(reducer_key);

            if let Some(ref actions) = reducer_data.actions {
                for ref action_data in actions {
                    let action_ty = format!("{}.{}", reducer_scope_key, &action_data.action_type);

                    match &action_data.state_expr {
                        &Some(ActionStateExprType::SimpleReducerKeyExpr(ref simple_expr)) => {
                            let mut scope = scope.clone();
                            scope.0.set_default_var("state");

                            writeln!(w,
                                     "if ('undefined' !== typeof action && '{}' == action.type) \
                                      {{",
                                     action_ty)
                                ?;
                            write!(w, "  return ")?;
                            write_js_expr_value(w, simple_expr, &self.doc, &scope)?;
                            writeln!(w, ";")?;
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
                write_js_expr_value(w, default_expr, &self.doc, scope)?;
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
                                       ctx: &mut Context,
                                       bindings: &BindingContext,
                                       ops: Iter<'input, ElementOp>,
                                       processing: &DocumentState,
                                       scope: &ElementOpScope)
                                       -> Result {
        let mut value_writer = ValueWriterJs::default();
        let mut expr_writer = ExpressionWriterJs::default();
        let mut stream_writer = ElementOpsStreamWriterJs::default();

        let mut ops_writer =
            ElementOpsWriter::with_doc(&self.doc, &mut self.stream_writer, scope.to_owned());
        ops_writer.write_ops_content(w, ops, &self.doc, false)?;

        Ok(())
    }

    #[inline]
    pub fn write_js_incdom_component(&mut self,
                                     w: &mut io::Write,
                                     ctx: &mut Context,
                                     bindings: &BindingContext,
                                     component_ty: &'input str,
                                     comp: &Component,
                                     ops: Iter<'input, ElementOp>,
                                     processing: &DocumentState,
                                     scope: &ElementOpScope)
                                     -> Result {
        let mut scope = scope.clone();

        // Merge component scope entries
        // TODO: Convert values to props
        for (key, sym) in comp.symbol_map.iter() {
            scope.1.symbol_map.insert(key.to_owned(), sym.to_owned());
        }

        writeln!(w,
                 "  function component_{}(key_prefix, store, props) {{",
                 component_ty)
            ?;
        self.write_js_incdom_ops_content(w, ops, processing, &scope)?;
        writeln!(w, "  }};")?;
        writeln!(w, "")?;

        Ok(())
    }
}
