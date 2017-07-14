
use std::io;

use parser::ast::*;
use parser::store::*;
use output::structs::*;
use output::client_html::*;
use output::client_js::*;

pub struct FormatHtml<'input> {
    doc: DocumentState<'input>,
}

impl<'input> FormatHtml<'input> {
    pub fn from_state<'inp>(doc: DocumentState<'inp>) -> FormatHtml<'inp> {
        FormatHtml { doc: doc }
    }

    pub fn write_js_store(&self,
                          w: &mut io::Write,
                          processing: &DocumentState,
                          resolve: &ResolveVars)
                          -> Result {
        // TODO: Implement default scope?

        // Generate script
        for (ref reducer_key, ref reducer_data) in processing.reducer_key_data.iter() {
            writeln!(w, "  function {}Reducer(state, action) {{", reducer_key)?;

            let reducer_scope = resolve.with_state_key(reducer_key);

            if let Some(ref actions) = reducer_data.actions {
                for ref action_data in actions {
                    let action_ty = reducer_scope.action_type(&action_data.action_type);

                    match &action_data.state_expr {
                        &Some(ActionStateExprType::SimpleReducerKeyExpr(ref simple_expr)) => {
                            let action_scope = reducer_scope.action_result(reducer_key);
                            writeln!(w,
                                     "if ('undefined' !== typeof action && '{}' == action.type) \
                                      {{",
                                     action_ty)
                                ?;
                            write!(w, "  return ")?;
                            // write!(w, "Object.assign({{ \"{}\": ", reducer_key)?;
                            write_js_expr_value(w, simple_expr, processing, &action_scope)?;
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
                write_js_expr_value(w, default_expr, &self.doc, &resolve)?;
                // write!(w, "}})")?;
            } else {
                write!(w, "null")?;
            }
            writeln!(w, ";")?;

            writeln!(w, "  }}")?;
        }

        writeln!(w, "  var rootReducer = Redux.combineReducers({{")?;
        for (ref reducer_key, _) in processing.reducer_key_data.iter() {
            writeln!(w, "    {}: {}Reducer,", &reducer_key, &reducer_key)?;
        }
        writeln!(w, "  }});")?;

        writeln!(w, "  var store = Redux.createStore(rootReducer, {{}});")?;

        Ok(())
    }

    #[allow(unused_variables)]
    pub fn write_js_event_bindings(&self,
                                   w: &mut io::Write,
                                   events_vec: &EventsVec,
                                   resolve: &ResolveVars)
                                   -> Result {
        writeln!(w, "      // Bind actions")?;
        for &(ref element_key, ref event_name, ref params, ref action_ops, ref event_scope) in
            events_vec {
            let event_name = event_name.as_ref().map(String::as_str).map_or("click", |s| s);
            writeln!(w,
                     "  document.querySelector(\"[key='{}']\").addEventListener(\"{}\", \
                      function(event) {{",
                     element_key,
                     event_name)
                ?;

            let resolve = resolve;

            if let &Some(ref action_ops) = action_ops {
                let action_scope = event_scope.as_ref()
                    .map(|event_scope| resolve.with_state_key(event_scope));
                let resolve = action_scope.as_ref().map_or(resolve, |r| r);

                for ref action_op in action_ops {
                    match *action_op {
                        &ActionOpNode::DispatchAction(ref action_key, ref action_params) => {
                            let action_ty = resolve.action_type(action_key.as_str());
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

    #[allow(dead_code)]
    #[allow(unused_variables)]
    pub fn write_html_document(&self, w: &mut io::Write) -> Result {
        // Output state
        let mut events_vec: EventsVec = Default::default();
        let mut keys_vec: Vec<String> = Default::default();

        // Output
        write!(w, "{}", indoc!(r#"
            <!doctype HTML>
            <html>
                <head>
                <script src="https://unpkg.com/redux@3.7.1/dist/redux.js"></script>
                <script src="https://ajax.googleapis.com/ajax/libs/incrementaldom/0.5.1/incremental-dom.js" defer="defer"></script>
                </head>
                <body>
                <div id="root">
        "#))?;

        // FIXME
        // let resolve = ResolveVars::default_resolver("counter");
        let resolve = ResolveVars::default_resolver();

        write_html_ops_content(w,
                               self.doc.root_block.ops_vec.iter(),
                               &mut events_vec,
                               &mut keys_vec,
                               &self.doc,
                               &resolve,
                               None)
            ?;

        write!(w,
               "{}",
               indoc!(r#"
                </div>
                <script>
                    (function() {
        "#))
            ?;

        // Define components
        for (ref component_ty, ref comp_def) in self.doc.comp_map.iter() {
            if let Some(ref ops) = comp_def.ops {
                write_js_incdom_component(w, component_ty, ops.iter(), &self.doc, &resolve, None)?;
            };
        }

        writeln!(w, "/* {:?} */", self.doc.default_state_map)?;
        writeln!(w, "/* {:?} */", self.doc.root_block.ops_vec)?;

        writeln!(w, "function render(store) {{")?;

        // Render content nodes as incdom calls
        write_js_incdom_ops_content(w,
                                    self.doc.root_block.ops_vec.iter(),
                                    &self.doc,
                                    &resolve,
                                    None,
                                    None)
            ?;

        writeln!(w, "}}")?;

        write!(w,
               "{}",
               indoc!(r#"
                    function update(root_el, store) {
                        IncrementalDOM.patch(root_el, render.bind(null, store));
                    }
        "#))
            ?;

        // Callback that will execute after deferred scripts and content is ready
        writeln!(w,
                 "document.addEventListener(\"DOMContentLoaded\", function(event) {{")
            ?;

        writeln!(w, "  // Define store")?;
        self.write_js_store(w, &self.doc, &resolve)?;

        write!(w,
               "{}",
               indoc!(r#"
                    function Blank() {}
                    Blank.prototype = Object.create(null);
                    
                    function markExisting(node, key, attrsArr) {
                        IncrementalDOM.importNode(node);
                        var data = node['__incrementalDOMData'];
                        data.staticsApplied = true;
                        data.newAttrs = new Blank();
                    }
        "#))
            ?;

        // Mark the DOM elements we just rendered so incdom will not attempt to replace them on initial render
        for key in keys_vec.iter() {
            writeln!(w,
                     "    markExisting(document.querySelector(\"[key='{}']\"));",
                     key)
                ?;
        }

        // Event handlers
        self.write_js_event_bindings(w, &events_vec, &resolve)?;

        write!(w,
               "{}",
               indoc!(r#"
                    // Root subscription
                    var root_el = document.querySelector('#root');
                    store.subscribe(function() { update(root_el, store); });


                    });
                })();
                    </script>
                </body>
            </html>
        "#))
            ?;

        Ok(())
    }
}
