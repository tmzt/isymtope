
use std::io;
use std::fmt;
use parser::ast::*;
use super::process::ProcessDocument;

mod format_html {
    use std::clone::Clone;
    use std::slice::Iter;
    use std::fmt;
    use std::collections::hash_map::HashMap;
    use parser::ast::*;
    use parser::store::*;
    use parser::api::*;
    use parser::util::allocate_element_key;
    use output::structs::*;

    use super::super::client_html::*;
    use super::super::client_js::*;
    use super::super::client_misc::*;
    use super::super::process::*;

    pub struct FormatHtml<'input> {
        doc: DocumentState<'input>
    }

    impl<'input> FormatHtml<'input> {
        pub fn from_state<'inp>(doc: DocumentState<'inp>) -> FormatHtml<'inp> {
            FormatHtml {
                doc: doc
            }
        }

        pub fn write_js_store(&self,
                              w: &mut fmt::Write,
                              reducer_key_data: &HashMap<&'input str, ReducerKeyData>,
                              default_state_map: &DefaultStateMap<'input>)
                              -> fmt::Result {
            // TODO: Implement default scope?

            // Generate script
            for (ref reducer_key, ref reducer_data) in reducer_key_data.iter() {
                writeln!(w, "  function {}Reducer(state, action) {{", reducer_key)?;

                if let Some(ref actions) = reducer_data.actions {
                    for ref action_data in actions {
                        let mut state_expr_str = String::new();

                        let action_type =
                            format!("{}.{}", reducer_key.to_uppercase(), action_data.action_type);

                        match &action_data.state_expr {
                            &Some(ActionStateExprType::SimpleReducerKeyExpr(ref simple_expr)) => {
                                write_js_expr_value(&mut state_expr_str,
                                                         simple_expr,
                                                         default_state_map,
                                                         None,
                                                         Some("state".into()),
                                                         None)?;
                                writeln!(w,
                                         "    if ('undefined' !== typeof action && '{}' == \
                                          action.type) {{ return {}; }}",
                                         action_type,
                                         state_expr_str)
                                    ?;
                            }
                            _ => {}
                        }
                    }
                }

                // Default expression used to initialize state
                write!(w, "    return state || ")?;
                if let Some(ref default_expr) = reducer_data.default_expr {
                    write_js_expr_value(w, default_expr, default_state_map, None, None, None)?;
                } else {
                    write!(w, "null")?;
                }
                writeln!(w, ";")?;

                writeln!(w, "  }}")?;
            }

            writeln!(w, "  var rootReducer = Redux.combineReducers({{")?;
            for (ref reducer_key, _) in reducer_key_data.iter() {
                writeln!(w, "    {}: {}Reducer,", &reducer_key, &reducer_key)?;
            }
            writeln!(w, "  }});")?;

            writeln!(w, "  var store = Redux.createStore(rootReducer, {{}});")?;

            Ok(())
        }

        #[allow(unused_variables)]
        pub fn write_js_event_bindings(&self,
                                       w: &mut fmt::Write,
                                       events_vec: &EventsVec,
                                       action_prefix: Option<&str>)
                                       -> fmt::Result {
            writeln!(w, "      // Bind actions")?;
            for &(ref element_key, ref event_name, ref params, ref action_ops, ref scope) in events_vec {
                let event_name = event_name.as_ref().map(String::as_str).map_or("click", |s| s);
                writeln!(w,
                         "  document.querySelector(\"[key='{}']\").addEventListener(\"{}\", \
                          function(event) {{",
                         element_key,
                         event_name)
                    ?;
                if let &Some(ref action_ops) = action_ops {
                    for ref action_op in action_ops {
                        match *action_op {
                            &ActionOpNode::DispatchAction(ref action_key, ref action_params) => {
                                // TODO: Fix type
                                let action_prefix = scope.as_ref().map_or("".into(), |s| s.to_uppercase());
                                let action_ty = format!("{}.{}",
                                                        action_prefix,
                                                        action_key.to_uppercase());
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
        pub fn write_html_document(&self, w: &mut fmt::Write) -> fmt::Result {
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

            // Fix this
            let default_scope = Some("counter".into());

            write_html_ops_content(w,
                                        self.doc.root_block.ops_vec.iter(),
                                        &mut events_vec,
                                        &self.doc.comp_map,
                                        &mut keys_vec,
                                        None,
                                        default_scope)?;

            write!(w, "{}", indoc!(r#"
                    </div>
                    <script>
                      (function() {
            "#))?;

            // Define components
            for (ref component_ty, ref comp_def) in self.doc.comp_map.iter() {
                if let Some(ref ops) = comp_def.ops {
                    write_js_incdom_component(
                        w,
                        component_ty,
                        ops.iter(),
                        &self.doc.default_state_map,
                        Some("store.getState()."),
                        Some("store.getState()"),
                        None,
                        default_scope,
                        &self.doc.comp_map)?;
                };
            }

            writeln!(w, "/* {:?} */", self.doc.default_state_map)?;
            writeln!(w, "/* {:?} */", self.doc.root_block.ops_vec)?;

            writeln!(w, "function render(store) {{")?;

            // Render content nodes as incdom calls
            write_js_incdom_ops_content(w,
                                             self.doc.root_block.ops_vec.iter(),
                                             &self.doc.default_state_map,
                                             Some("store.getState()."),
                                             Some("store.getState()"),
                                             None,
                                             default_scope,
                                             None,
                                             &self.doc.comp_map)?;

            writeln!(w, "}}")?;

            write!(w, "{}", indoc!(r#"
                        function update(root_el, store) {
                          IncrementalDOM.patch(root_el, render.bind(null, store));
                        }
            "#))?;

            // Callback that will execute after deferred scripts and content is ready
            writeln!(w,
                     "document.addEventListener(\"DOMContentLoaded\", function(event) {{")
                ?;

            writeln!(w, "  // Define store")?;
            self.write_js_store(w, &self.doc.reducer_key_data, &self.doc.default_state_map)?;

            write!(w, "{}", indoc!(r#"
                        function Blank() {}
                        Blank.prototype = Object.create(null);
                        
                        function markExisting(node, key, attrsArr) {
                            IncrementalDOM.importNode(node);
                            var data = node['__incrementalDOMData'];
                            data.staticsApplied = true;
                            data.newAttrs = new Blank();
                        }
            "#))?;

            // Mark the DOM elements we just rendered so incdom will not attempt to replace them on initial render
            for key in keys_vec.iter() {
                writeln!(w,"    markExisting(document.querySelector(\"[key='{}']\"));", key)?;
            }

            // Event handlers
            self.write_js_event_bindings(w, &events_vec, Some("counter"))?;

            write!(w, "{}", indoc!(r#"
                      // Root subscription
                      var root_el = document.querySelector('#root');
                      store.subscribe(function() { update(root_el, store); });


                      });
                    })();
                     </script>
                  </body>
                </html>
            "#))?;

            Ok(())
        }
    }
}

use self::format_html::FormatHtml;
use super::structs::*;

pub type Result = io::Result<fmt::Result>;

pub struct ClientOutput<'input> {
    ast: &'input Template,
}

impl<'input, 'doc: 'input> ClientOutput<'input> {
    pub fn from_template(ast: &'input Template) -> ClientOutput {
        ClientOutput { ast: ast }
    }

    pub fn write_html(&self, w: &mut io::Write) -> Result {
        let doc: DocumentState<'input> = ProcessDocument::from_template(self.ast).into();
        let format = FormatHtml::from_state(doc);

        //let format = FormatHtml::from_template(self.ast, processing);
        let mut doc_str = String::new();

        if let Err(e) = format.write_html_document(&mut doc_str) {
            return Ok(Err(e));
        }

        if let Err(e) = w.write_fmt(format_args!("{}", doc_str)) {
            return Err(e);
        }

        Ok(Ok(()))
    }
}
