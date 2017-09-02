
use std::io;
use std::iter;
use std::slice::Iter;

use parser::ast::*;
use processing::structs::*;

use output::writers::*;
use scope::scope::*;
use scope::context::*;
use scope::bindings::*;


const STRING_HTML_OPEN_INCDOM_PAGE: &'static str = r#"
<!doctype HTML>
<html>
    <head>
        <meta charset="utf-8" />
        <script src="https://unpkg.com/redux@3.7.1/dist/redux.js"></script>
        <script src="https://ajax.googleapis.com/ajax/libs/incrementaldom/0.5.1/incremental-dom.js" defer="defer"></script>
    </head>
    <body>"#;

const STRING_HTML_OPEN_ROOT_DIV: &'static str  = r#"
        <div id="root">
"#;

const STRING_HTML_CLOSE_ROOT_DIV: &'static str  = r#"
        </div>"#;

const STRING_HTML_OPEN_SCRIPT_IIFE: &'static str  = r#"
        <script>
            (function() {
                function Blank() {}
                Blank.prototype = Object.create(null);
                
                function markExisting(node, key, attrsArr) {
                    IncrementalDOM.importNode(node);
                    var data = node['__incrementalDOMData'];
                    data.staticsApplied = true;
                    data.newAttrs = new Blank();
                }

                function update(root_el, store) {
                    IncrementalDOM.patch(root_el, render.bind(null, store));
                }
"#;

const STRING_JS_OPEN_RENDER: &'static str  = r#"
                function render(store) {
"#;

const STRING_JS_CLOSE_RENDER: &'static str  = r#"                }
"#;

const STRING_HTML_CLOSE_SCRIPT_IIFE: &'static str  = r#"
                document.addEventListener("DOMContentLoaded", function(event) {
                    // Import nodes
                    markExisting([].slice.call(document.querySelectorAll("[key]")));
                    // Define store
                    // Subscribe
                    store.subscribe(update.bind(null, document.querySelector('#root'), store));
                });
            })();
        </script>"#;

const STRING_HTML_CLOSE_INCDOM_PAGE: &'static str  = r#"
    </body>
</html>
"#;

pub struct PageWriter<'input> {
    doc: &'input DocumentState<'input>,
    writers: DefaultOutputWritersBoth
}

impl<'input> PageWriter<'input> {
    pub fn with_doc(doc: &'input DocumentState<'input>) -> Self {
        PageWriter {
            doc: doc,
            writers: Default::default()
        }
    }

    #[allow(unused_variables)]
    pub fn write_js_event_actions(&self,
                                  w: &mut io::Write,
                                  ctx: &mut Context,
                                  bindings: &BindingContext,
                                  action_ops: &Option<Vec<ActionOpNode>>)
                                  -> Result {
        if let &Some(ref action_ops) = action_ops {
            for ref action_op in action_ops {
                match *action_op {
                    &ActionOpNode::DispatchAction(ref action_key, ref action_params) => {
                        // let action_ty = scope.0.make_action_type(action_key);
                        let action_ty = ctx.join_action_path_with(Some("."), action_key).to_uppercase();

                        if let &Some(ref action_params) = action_params {
                            let action_params: PropVec =
                                iter::once(("type".to_owned(),
                                            Some(ExprValue::LiteralString(action_ty.to_owned()))))
                                    .chain(action_params.iter().map(|s| s.clone()))
                                    .collect();

                            write!(w, " store.dispatch(")?;
                            // write_js_props_object(w, Some(action_params.iter()), self.doc, &scope)?;
                            writeln!(w, ");")?;
                        } else {
                            writeln!(w, " store.dispatch({{\"type\": \"{}\"}}); ", action_ty)?;
                        }
                    }
                }
            }
        }
        Ok(())
    }

    #[allow(unused_variables)]
    pub fn write_js_event_bindings<I: IntoIterator<Item = EventsItem>>(&self, w: &mut io::Write, ctx: &mut Context, bindings: &BindingContext, events_iter: I) -> Result {
        writeln!(w, "      // Bind actions")?;
        for (ref element_key,
              ref event_name,
              ref params,
              ref action_ops,
              ref event_scope,
              ref block_id) in events_iter {
            // let complete_key = scope.0.make_complete_element_key_with(element_key);
            let complete_key = ctx.join_path(Some("."));
            let was_enterkey = event_name.as_ref()
                .map_or(false, |event_name| event_name == "enterkey");
            let event_name = match event_name.as_ref().map(|s| s.as_str()) {
                Some("enterkey") => "keydown",
                Some(event_name) => event_name,
                None => "click",
            };

            writeln!(w,
                     "  document.querySelector(\"[key='{}']\").addEventListener(\"{}\", \
                      function(event) {{",
                     complete_key,
                     event_name)
                ?;

            if was_enterkey {
                writeln!(w, "if (event.keyCode == 13) {{")?;
            };

            self.write_js_event_actions(w, ctx, bindings, action_ops)?;

            if was_enterkey {
                writeln!(w, "}}")?;
            };

            writeln!(w, "  }});")?;
        }
        Ok(())
    }

    #[inline]
    #[allow(unused_variables)]
    pub fn write_component_definitions(&mut self, w: &mut io::Write, ctx: &mut Context, bindings: &BindingContext) -> Result {
        for (ref component_ty, ref comp_def) in self.doc.comp_map.iter() {
            if let Some(ref ops) = comp_def.ops_iter() {
                // let mut scope = base_scope.clone();
                // let param_expr = ExprValue::SymbolReference(Symbol::param("key_prefix"));
                // scope.0.set_prefix_expr(&param_expr);

                // self.output_js
                //     .write_js_incdom_component(w,
                //                                component_ty,
                //                                comp_def,
                //                                ops.iter(),
                //                                &mut self.doc,
                //                                &scope)?;
            };
        }
        Ok(())
    }

    #[inline]
    #[allow(unused_variables)]
    pub fn write_render_definition(&mut self, w: &mut io::Write, ctx: &mut Context, bindings: &BindingContext) -> Result {
        write!(w, "{}", self::STRING_JS_OPEN_RENDER)?;

        // Render content nodes as incdom calls
        // self.output_js
        //     .write_js_incdom_ops_content(w,
        //                                  ctx,
        //                                  bindings,
        //                                  self.doc.root_block.ops_vec.iter(),
        //                                  &mut self.doc,
        //                                  &base_scope)?;

        let ops_iter = self.doc.root_block.ops_vec.iter();
        self.writers.js.write_element_ops(w, ctx, bindings, ops_iter)?;

        write!(w, "{}", self::STRING_JS_CLOSE_RENDER)?;
        Ok(())
    }

    #[inline]
    #[allow(unused_variables)]
    pub fn write_event_bindings(&mut self, w: &mut io::Write, ctx: &mut Context, bindings: &BindingContext) -> Result {
        // Event handlers
        // if let Some(events_iter) = self.output_html.events_iter() {
        //     self.write_js_event_bindings(w, events_iter)?;
        // }

        // Component event handlers
        // if let Some(comp_instances) = self.output_html.component_instances_iter() {
        //     for &(ref complete_key, ref comp_ty) in comp_instances {
        //         if let Some(ref comp) = self.doc.comp_map.get(comp_ty) {
        //             // let mut scope = base_scope.clone();
        //             // scope.0.append_key(complete_key);
        //             // if let Some(ref events) = comp.events {
        //             //     self.write_js_event_bindings(w, events.iter())?;
        //             // };
        //         }
        //     }
        // }
        Ok(())
    }

    #[inline]
    #[allow(unused_variables)]
    pub fn write_root_html(&mut self, w: &mut io::Write, ctx: &mut Context, bindings: &BindingContext) -> Result {
        write!(w, "{}", self::STRING_HTML_OPEN_ROOT_DIV)?;
        let ops_iter = self.doc.root_block.ops_vec.iter();
        self.writers.html.write_element_ops(w, ctx, bindings, ops_iter)?;
        write!(w, "{}", self::STRING_HTML_CLOSE_ROOT_DIV)?;
        Ok(())
    }

    #[inline]
    #[allow(unused_variables)]
    pub fn write_script_html(&mut self, w: &mut io::Write, ctx: &mut Context, bindings: &BindingContext) -> Result {
        write!(w, "{}", self::STRING_HTML_OPEN_SCRIPT_IIFE)?;

        writeln!(w, "/* {:?} */", self.doc.default_state_map)?;
        writeln!(w, "/* {:?} */", self.doc.root_block.ops_vec)?;

        // Define components
        self.write_component_definitions(w, ctx, bindings)?;
        self.write_render_definition(w, ctx, bindings)?;
        self.write_event_bindings(w, ctx, bindings)?;

        write!(w, "{}", self::STRING_HTML_CLOSE_SCRIPT_IIFE)?;
        Ok(())
    }

    #[allow(dead_code)]
    #[allow(unused_variables)]
    pub fn write_page(&mut self, w: &mut io::Write, ctx: &mut Context, bindings: &BindingContext) -> Result {
        // if let Some(ref default_reducer_key) = self.doc.default_reducer_key {
        //     ctx.append_action_path_str(default_reducer_key);
        // };

        write!(w, "{}", self::STRING_HTML_OPEN_INCDOM_PAGE)?;

        // let mut events_vec: EventsVec = Default::default();
        // self.output_html.write_html_ops_content(w, ops_iter, base_scope, Some(&mut events_vec))?;

        self.write_root_html(w, ctx, bindings)?;
        writeln!(w, "")?;
        self.write_script_html(w, ctx, bindings)?;

        // Mark the DOM elements we just rendered so incdom will not attempt to replace them on initial render
        // let keys_iter = self.output_html.keys_iter();
        // for key in keys_iter {
        //     writeln!(w,
        //              "    markExisting(document.querySelector(\"[key='{}']\"));",
        //              key)
        //         ?;
        // }

        write!(w, "{}", self::STRING_HTML_CLOSE_INCDOM_PAGE)?;

        Ok(())
    }
}


#[cfg(test)]
mod tests {
    use super::*;
    use std::str;
    use processing::*;

    
    fn create_template() -> Template {
        let node_1 = ContentNodeType::ExpressionValueNode(ExprValue::LiteralString("hi".into()), "45daee35".into());
        let node_2 = ContentNodeType::ElementNode(ElementType { element_ty: "span".into(), element_key: "Cd".into(), attrs: None, lens: None, bindings: None, children: Some(vec![node_1])});
        let node_3 = ContentNodeType::ElementNode(ElementType { element_ty: "div".into(), element_key: "Ab".into(), attrs: None, lens: None, bindings: None, children: Some(vec![node_2])});
        
        let nodes: Vec<Loc<NodeType, (usize, usize)>> = vec![
            Loc {inner: NodeType::ContentNode(node_3), pos: (0,0)}
        ];
        Template { children: nodes }
    }

    fn prepare_document<'a>(template: &'a Template) -> DocumentState<'a> {
        let mut ctx = Context::default();
        let mut bindings = BindingContext::default();
        let mut processing = ProcessDocument::from_template(&template);
        assert!(processing.process_document(&mut ctx, &mut bindings).is_ok());
        processing.into()
    }

    #[test]
    pub fn test_output_page_writer_html() {
        let template = create_template();
        let doc = prepare_document(&template);
        let mut page_writer = PageWriter::with_doc(&doc);
 
        let mut ctx = Context::default();
        let bindings = BindingContext::default();
        let mut s: Vec<u8> = Default::default();
        let res = page_writer.write_page(&mut s, &mut ctx, &bindings);
        assert!(res.is_ok());
        assert_diff!(str::from_utf8(&s).unwrap(), r#"
<!doctype HTML>
<html>
    <head>
        <meta charset="utf-8" />
        <script src="https://unpkg.com/redux@3.7.1/dist/redux.js"></script>
        <script src="https://ajax.googleapis.com/ajax/libs/incrementaldom/0.5.1/incremental-dom.js" defer="defer"></script>
    </head>
    <body>
        <div id="root">
<div key="Ab"><span key="Ab.Cd">hi</span></div>
        </div>

        <script>
            (function() {
                function Blank() {}
                Blank.prototype = Object.create(null);
                
                function markExisting(node, key, attrsArr) {
                    IncrementalDOM.importNode(node);
                    var data = node['__incrementalDOMData'];
                    data.staticsApplied = true;
                    data.newAttrs = new Blank();
                }

                function update(root_el, store) {
                    IncrementalDOM.patch(root_el, render.bind(null, store));
                }
/* {} */
/* [ElementOpen("div", "Ab", None, None, None), ElementOpen("span", "Ab.Cd", None, None, None), WriteValue(LiteralString("hi"), "Ab.Cd.45daee35"), ElementClose("span"), ElementClose("div")] */

                function render(store) {
IncrementalDOM.elementOpen("div", "Ab");
IncrementalDOM.elementOpen("span", "Ab.Cd");
IncrementalDOM.text("hi");
IncrementalDOM.elementClose("span");
IncrementalDOM.elementClose("div");
                }

                document.addEventListener("DOMContentLoaded", function(event) {
                    // Import nodes
                    markExisting([].slice.call(document.querySelectorAll("[key]")));
                    // Define store
                    // Subscribe
                    store.subscribe(update.bind(null, document.querySelector('#root'), store));
                });
            })();
        </script>
    </body>
</html>
"#, "\n", 0);
    }
}