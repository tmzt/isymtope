
use std::io;
use std::slice::Iter;

use parser::ast::*;
use parser::store::*;
use output::structs::*;
use output::client_html::*;
use output::client_js::*;
use output::client_ops_writer::*;
use output::client_ops_js_stream_writer::*;


pub struct FormatHtml<'input> {
    doc: &'input DocumentState<'input>,
    output_html: WriteHtmlOpsContent<'input>,
    output_js: WriteJsOps<'input>,
}

impl<'input: 'scope, 'scope> FormatHtml<'input> {
    pub fn with_doc(doc: &'input DocumentState<'input>) -> Self {
        FormatHtml {
            doc: doc,
            output_html: WriteHtmlOpsContent::with_doc(doc),
            output_js: WriteJsOps::with_doc(doc),
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

    #[allow(unused_variables)]
    pub fn write_js_event_bindings(&self,
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
                // let action_scope = event_scope.as_ref()
                //     .map(|event_scope| resolve.with_state_key(event_scope));
                // let resolve = action_scope.as_ref().map_or(resolve, |r| r);

                for ref action_op in action_ops {
                    match *action_op {
                        &ActionOpNode::DispatchAction(ref action_key, ref action_params) => {
                            // let action_ty = resolve.action_type(action_key.as_str());
                            let action_ty = self.scope_prefix(scope_prefix, action_key);
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
    pub fn write_html_document(&mut self, w: &mut io::Write) -> Result {
        // Output state
        // let mut events_vec: EventsVec = Default::default();
        // let mut keys_vec: Vec<String> = Default::default();

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

        let ops_iter = self.doc.root_block.ops_vec.iter();

        let base_scope: ScopePrefixes = Default::default();
        self.output_html.write_html_ops_content(w, ops_iter, &base_scope)?;

        write!(w,
               "{}",
               indoc!(r#"
                </div>
                <script>
                    (function() {
        "#))
            ?;

        let base_scope: ScopePrefixes = Default::default();

        // Define components
        for (ref component_ty, ref comp_def) in self.doc.comp_map.iter() {
            if let Some(ref ops) = comp_def.ops {
                self.output_js.write_js_incdom_component(w, component_ty, ops.iter(), &mut self.doc, &base_scope, None)?;
            };
        }

        writeln!(w, "/* {:?} */", self.doc.default_state_map)?;
        writeln!(w, "/* {:?} */", self.doc.root_block.ops_vec)?;

        writeln!(w, "function render(store) {{")?;

        // Render content nodes as incdom calls
        self.output_js.write_js_incdom_ops_content(w,
                                    self.doc.root_block.ops_vec.iter(),
                                    &mut self.doc,
                                    &base_scope)
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
        self.output_js.write_js_store(w, &base_scope)?;

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
        let keys_iter = self.output_html.keys_iter();
        for key in keys_iter {
            writeln!(w,
                     "    markExisting(document.querySelector(\"[data-id='{}']\"));",
                     key)
                ?;
        }

        // Event handlers
        let events_iter = self.output_html.events_iter();
        self.write_js_event_bindings(w, events_iter, None)?;

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
