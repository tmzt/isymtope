
use std::io;
use std::iter;
use std::slice::Iter;

use parser::ast::*;
use parser::store::*;
use processing::structs::*;
use processing::process::*;
use output::scope::*;
use output::client_misc::*;
use output::client_html::*;
use output::client_js::*;
use output::client_js_value_writer::*;
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

    #[allow(unused_variables)]
    pub fn write_js_event_bindings(&self,
                                   w: &mut io::Write,
                                   events_iter: Iter<EventsItem>,
                                   scope: &ElementOpScope)
                                   -> Result {
        writeln!(w, "      // Bind actions")?;
        for &(ref element_key, ref event_name, ref params, ref action_ops, ref event_scope, ref block_id) in
            events_iter {
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
                            let mut scope = scope.clone();
                            scope.0.append_action_scope(action_key);
                            let action_ty = scope.0.make_action_type(action_key);

                            if let &Some(ref action_params) = action_params {
                                let action_params: PropVec = iter::once(("type".to_owned(), Some(ExprValue::LiteralString(action_ty.to_owned()))))
                                    .chain(action_params.iter().map(|s| s.clone())).collect();

                                write!(w, " store.dispatch(")?;
                                write_js_props_object(w, Some(action_params.iter()), self.doc, &scope)?;
                                writeln!(w, ");")?;
                            } else {
                                writeln!(w, " store.dispatch({{\"type\": \"{}\"}}); ", action_ty)?;
                            }
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


        let mut base_scope: ElementOpScope = Default::default();
        if let Some(ref default_reducer_key) = self.doc.default_reducer_key {
            base_scope.0.append_action_scope(default_reducer_key);
        };

        let base_expr_scope: ExprScopeProcessingState = Default::default();

        let mut events_vec: EventsVec = Default::default();
        self.output_html.write_html_ops_content(w, ops_iter, base_scope, Some(&mut events_vec))?;

        write!(w,
               "{}",
               indoc!(r#"
                </div>
                <script>
                    (function() {
        "#))
            ?;

        let base_scope: ElementOpScope = Default::default();

        // Define components
        for (ref component_ty, ref comp_def) in self.doc.comp_map.iter() {
            if let Some(ref ops) = comp_def.ops {
                self.output_js.write_js_incdom_component(w, component_ty, comp_def, ops.iter(), &mut self.doc, &base_scope)?;
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
                     "    markExisting(document.querySelector(\"[key='{}']\"));",
                     key)
                ?;
        }

        // Event handlers
        if let Some(events_iter) = self.output_html.events_iter() {
            self.write_js_event_bindings(w, events_iter, &base_scope)?;
        }

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
