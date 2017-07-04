

use std::io;
use std::fmt;
use parser::ast::*;

mod format_html {
    use std::clone::Clone;
    use std::slice::Iter;
    use std::fmt;
    use std::collections::hash_map::HashMap;
    use itertools;
    use parser::ast::*;
    use parser::store::*;
    use output::structs::*;

    pub type NodesVec = Vec<(String, Option<String>, Option<Vec<(String, ExprValue)>>)>;
    pub type OpsVec = Vec<ElementOp>;
    pub type ReducerKeyMap<'inp> = HashMap<&'inp str, ReducerKeyData>;

    pub struct FormatHtml<'input> {
        ast: &'input Template
    }

    impl<'input> FormatHtml<'input> {

        pub fn from_template<'inp>(ast: &'inp Template) -> FormatHtml<'inp> {
            FormatHtml { 
                ast: ast
            }
        }

        pub fn write_js_expr_value(&self, w: &mut fmt::Write, node: &ExprValue, var_prefix: Option<&str>, default_var: Option<&str>) -> fmt::Result {
            match node {
                // TODO: Handle the case where quotes appear in the string
                &ExprValue::LiteralString(ref s) => { write!(w, "\"{}\"", s)?; },
                &ExprValue::LiteralNumber(ref n) => { write!(w, "{}", n)?; },

                &ExprValue::DefaultVariableReference => {
                    write!(w, "{}{}",
                        var_prefix.and_then(|prefix| Some(format!("{}.", prefix.to_uppercase()))).unwrap_or_default(),
                        default_var.unwrap_or("value".into())
                    )?;
                },

                &ExprValue::VariableReference(ref s) => {
                    if let Some(ref prefix) = var_prefix {
                        write!(w, "{}{}", prefix, s)?;
                    } else {
                        write!(w, "{}", s)?;
                    }
                },

                &ExprValue::Expr(ref sym, ref l, ref r) => {
                    self.write_js_expr_value(w, l, var_prefix, default_var)?;
                    match sym {
                        &ExprOp::Add => { write!(w, " + ")?; },
                        &ExprOp::Sub => { write!(w, " - ")?; },
                        &ExprOp::Mul => { write!(w, " * ")?; },
                        &ExprOp::Div => { write!(w, " / ")?; },
                    }
                    self.write_js_expr_value(w, r, var_prefix, default_var)?;
                }
            }
            Ok(())
        }

        pub fn write_computed_expr_value(&self, w: &mut fmt::Write, node: &ExprValue, var_prefix: Option<&str>) -> fmt::Result {
            match node {
                &ExprValue::LiteralString(ref s) => { write!(w, "{}", s)?; },
                &ExprValue::LiteralNumber(ref n) => { write!(w, "{}", n)?; },

                &ExprValue::DefaultVariableReference => {
                    if let Some(ref prefix) = var_prefix {
                        write!(w, "{}", prefix)?;
                    } else {
                        write!(w, "value")?;
                    }
                },

                &ExprValue::VariableReference(ref s) => {
                    if let Some(ref prefix) = var_prefix {
                        write!(w, "{}{}", prefix, s)?;
                    } else {
                        write!(w, "{}", s)?;
                    }
                },

                &ExprValue::Expr(ref sym, ref l, ref r) => {
                    write!(w, "{:?} {:?} {:?}", l, sym, r)?;
                }
            }
            Ok(())
        }

        #[allow(dead_code)]
        pub fn write_html_ops_content(&self, w : &mut fmt::Write, ops: Iter<ElementOp>) -> fmt::Result {
            for ref op in ops {
                match *op {
                    &ElementOp::ElementOpen(ref element_tag, ref element_key, ref attrs) => {
                        write!(w, "<{}", element_tag)?;
                        if let &Some(ref element_key) = element_key {
                            write!(w, " key=\"{}\"", element_key)?;
                        }

                        if let &Some(ref attrs) = attrs {
                            for &(ref key, ref expr) in attrs {
                                write!(w, " {}=\"", key)?;
                                self.write_computed_expr_value(w, expr, None)?;
                                write!(w, "\"")?;
                            }
                        }
                        writeln!(w, ">")?;
                    },
                    &ElementOp::ElementClose(ref element_tag) => {
                        writeln!(w, "</{}>", element_tag)?;
                    },
                    &ElementOp::ElementVoid(ref element_tag, ref element_key, ref attrs) => {
                        write!(w, "<{}", element_tag)?;
                        if let &Some(ref element_key) = element_key {
                            write!(w, " key=\"{}\"", element_key)?;
                        }

                        if let &Some(ref attrs) = attrs {
                            for &(ref key, ref expr) in attrs {
                                write!(w, " {}=\"", key)?;
                                self.write_computed_expr_value(w, expr, None)?;
                                write!(w, "\"")?;
                            }
                        }
                        writeln!(w, " />")?;
                    },
                    &ElementOp::Text(ref content) => {
                        write!(w, "{}", content)?;
                    }
                }
            }

            Ok(())
        }

        #[inline]
        fn write_js_incdom_attr_array(&self, w: &mut fmt::Write, attrs: &Vec<(String, ExprValue)>) -> fmt::Result {
            // Collect (static) attrs first
            write!(w, "{}", itertools::join(attrs.iter().map(
                |&(ref key, ref expr)| {
                    let mut expr_str = String::new();
                    self.write_computed_expr_value(&mut expr_str, &expr, None).expect("Could not write attribute value in DOM node.");
                    format!("\"{}\", \"{}\"", key, expr_str)
                }
            ), ", "))?;
            Ok(())
        }

        pub fn write_js_incdom_element(&self, w: &mut fmt::Write, element_data: &ElementType, var_prefix: Option<&str>) -> fmt::Result {
            let element_tag = element_data.element_ty.to_lowercase();
            let mut attrs_str = String::new();

            // Collect (static) attrs first
            if let Some(ref attrs) = element_data.attrs {
                self.write_js_incdom_attr_array(&mut attrs_str, attrs)?;
            }

            let element_key_str = element_data.element_key.as_ref().map(|s| format!("\"{}\"", &s)).unwrap_or("null".into());

            if let Some(ref children) = element_data.children {

                // Open element
                writeln!(w, "IncrementalDOM.elementOpen(\"{}\", {}, [{}]);",
                    element_tag,
                    element_key_str,
                    attrs_str
                )?;

                // Output children
                for child in children {
                    self.write_js_incdom_content(w, child, var_prefix)?;
                }

                // Close element
                writeln!(w, "IncrementalDOM.elementClose(\"{}\");", element_tag)?;
            } else {
                // Void element
                writeln!(w, "IncrementalDOM.elementVoid(\"{}\", {}, [{}]);",
                    element_tag,
                    element_key_str,
                    attrs_str
                )?;
            }
            Ok(())
        }

        pub fn write_js_incdom_content(&self, w: &mut fmt::Write, node: &ContentNodeType, var_prefix: Option<&str>) -> fmt::Result {
            match node {
                &ContentNodeType::ElementNode(ref element_data) => {
                    self.write_js_incdom_element(w, element_data, var_prefix)?;
                },
                &ContentNodeType::ExpressionValueNode(ref expr) => {
                    let mut expr_str = String::new();
                    self.write_js_expr_value(&mut expr_str, &expr, var_prefix, Some("".into()))?;
                    writeln!(w, "IncrementalDOM.text({});", expr_str)?;
                }
            };
            Ok(())
        }

        pub fn write_js_function(&self, w: &mut fmt::Write, component_data: &ComponentDefinitionType) -> fmt::Result {
            writeln!(w, "function {}(props) {{", &component_data.name)?;
            if let Some(ref children) = component_data.children {
                for child in children.iter() {
                    match child {
                        &NodeType::ContentNode(ref content) => {
                            self.write_js_incdom_content(w, content, Some("props.".into()))?;
                        },
                        _ => {}
                    }
                }
            }
            writeln!(w, "}};")?;
            Ok(())
        }

        pub fn collect_js_store_child_scope(&self, reducer_key_data: &mut ReducerKeyMap<'input>, reducer_key: &'input str, nodes: &'input Vec<ScopeNodeType>, reducer_key_prefix: Option<&str>) -> fmt::Result {
            for ref node in nodes {
                match *node {
                    &ScopeNodeType::LetNode(ref var_name, ref expr) => {
                        let reducer_entry = reducer_key_data.entry(var_name).or_insert_with(|| ReducerKeyData::from_name(&format!("{}", var_name)));
                        /*
                        let var_path = format!("{}{}",
                            var_prefix.and_then(|prefix| Some(format!("{}.", prefix.to_uppercase()))).unwrap_or_default(),
                            var_name
                        );
                        */

                        if let &Some(ref expr) = expr {
                            reducer_entry.default_expr = Some(expr.clone());
                        };
                    },
                    &ScopeNodeType::ActionNode(ref action_name, ref simple_expr) => {
                        let reducer_entry = reducer_key_data.entry(reducer_key).or_insert_with(|| ReducerKeyData::from_name(&format!("{}", reducer_key)));

                        let action_path = format!("{}{}",
                            reducer_key_prefix
                                .and_then(|prefix| Some(format!("{}", prefix.to_uppercase())))
                                .and_then(|prefix| Some(format!("{}.", prefix.to_uppercase())))
                                .unwrap_or_default(),
                            action_name
                        );

                        let mut action = ReducerActionData::from_name(&action_path);
                        if let &Some(ref simple_expr) = simple_expr {
                            action.state_expr = Some(simple_expr.clone());
                        };
                        if let Some(ref mut actions) = reducer_entry.actions {
                            actions.push(action);
                        };
                    },
                    &ScopeNodeType::ScopeNode(ref scope_name, ref scope_nodes) => {
                        self.collect_js_store_child_scope(reducer_key_data, scope_name, scope_nodes, reducer_key_prefix)?;
                    },
                    _ => {}
                }
            }
            Ok(())
        }

        pub fn collect_js_store_default_scope(&self, reducer_key_data: &mut HashMap<&'input str, ReducerKeyData>, nodes: &'input Vec<DefaultScopeNodeType>) -> fmt::Result {
            for ref node in nodes {
                match *node {
                    &DefaultScopeNodeType::LetNode(ref var_name, ref expr) => {
                        // Within the default scope let defines a new scope and it's default expression
                        let reducer_entry = reducer_key_data.entry(var_name).or_insert_with(|| ReducerKeyData::from_name(&format!("{}", var_name)));

                        if let &Some(ref expr) = expr {
                            reducer_entry.default_expr = Some(expr.clone());
                        };
                    },
                    &DefaultScopeNodeType::ScopeNode(ref scope_name, ref scope_nodes) => {
                        self.collect_js_store_child_scope(reducer_key_data, scope_name, scope_nodes, None)?;
                    }
                }
            }
            Ok(())
        }

        pub fn write_js_store(&self, w: &mut fmt::Write, reducer_key_data: &HashMap<&'input str, ReducerKeyData>) -> fmt::Result {
            // TODO: Implement default scope?

            // Generate script
            for (ref reducer_key, ref reducer_data) in reducer_key_data.iter() {
                writeln!(w, "  function {}Reducer(state, action) {{", reducer_key)?;
                //writeln!(w, "  /* {:?} */", reducer_data)?;

                if let Some(ref actions) = reducer_data.actions {
                    for ref action_data in actions {
                        let mut state_expr_str = String::new();

                        let action_type = format!("{}.{}",
                            reducer_key.to_uppercase(),
                            action_data.action_type
                        );

                        match &action_data.state_expr {
                            &Some(ActionStateExprType::SimpleReducerKeyExpr(ref simple_expr)) => {
                                self.write_js_expr_value(&mut state_expr_str, simple_expr, None, Some("state".into()))?;
                                writeln!(w, "    if ('undefined' !== typeof action && '{}' == action.type) {{ return {}; }}",
                                    action_type,
                                    state_expr_str
                                )?;
                            },
                            _ => {}
                        }
                    }
                }

                // Default expression used to initialize state
                write!(w, "    return state || ")?;
                if let Some(ref default_expr) = reducer_data.default_expr {
                    self.write_js_expr_value(w, default_expr, None, None)?;
                } else {
                    write!(w, "null")?;
                }
                writeln!(w, ";")?;

                writeln!(w, "  }}")?;
            };

            writeln!(w, "  var rootReducer = Redux.combineReducers({{")?;
            for (ref reducer_key, _) in reducer_key_data.iter() {
                writeln!(w, "    {}: {}Reducer,", &reducer_key, &reducer_key)?;
            }
            writeln!(w, "  }});")?;

            writeln!(w, "  var store = Redux.createStore(rootReducer, {{}});")?;

            Ok(())
        }

        #[inline]
        fn process_content_node(&self, node: &'input ContentNodeType, nodes_vec: &'input mut NodesVec, ops_vec: &'input mut OpsVec) -> fmt::Result {
            match node {
                &ContentNodeType::ElementNode(ref element_data) => {
                    let element_tag = element_data.element_ty.to_lowercase();
                    let ref element_key = element_data.element_key;

                    let attrs = element_data.attrs.as_ref();
                    let node_attrs = attrs.map(|attrs| attrs.iter().map(Clone::clone).collect());
                    let op_attrs = attrs.map(|attrs| attrs.iter().map(Clone::clone).collect());

                    nodes_vec.push((element_tag.clone(), element_key.clone(), node_attrs));

                    if let Some(ref children) = element_data.children {
                        // Push element open
                        ops_vec.push(ElementOp::ElementOpen(element_tag.clone(), element_key.clone(), op_attrs));

                        // Iterate over children
                        for ref child in children {
                            self.process_content_node(child, nodes_vec, ops_vec)?;
                        }

                        // Push element close
                        ops_vec.push(ElementOp::ElementClose(element_tag.clone()));
                    } else {
                        ops_vec.push(ElementOp::ElementVoid(element_tag.clone(), element_key.clone(), op_attrs));
                    }
                },
                &ContentNodeType::ExpressionValueNode(ref expr) => {
                    let mut expr_str = String::new();
                    self.write_computed_expr_value(&mut expr_str, &expr, Some("".into()))?;
                    ops_vec.push(ElementOp::Text(expr_str));
                }
            }
            (Ok(()))
        }

        #[allow(dead_code)]
        pub fn process_nodes(&self, reducer_key_data: &mut self::ReducerKeyMap<'input>, nodes_vec: &mut self::NodesVec, ops_vec: &mut self::OpsVec) -> fmt::Result {
            let mut processed_store = false;

            for ref loc in self.ast.children.iter() {
                match &loc.inner {
                    &NodeType::StoreNode(ref scope_nodes) => {
                        // TODO: Allow more than one store?
                        if !processed_store {
                            self.collect_js_store_default_scope(reducer_key_data, scope_nodes)?;
                            processed_store = true;

                        }
                    },
                    &NodeType::ContentNode(ref content) => {
                        self.process_content_node(content, nodes_vec, ops_vec)?;
                    },
                    _ => {}
                }
            };
            Ok(())
        }

        #[allow(dead_code)]
        pub fn write_html_document(&self, w : &mut fmt::Write) -> fmt::Result {
            writeln!(w, "<!doctype HTML>")?;
            writeln!(w, "<html>")?;
            writeln!(w, "<head>")?;
            writeln!(w, "<script src=\"https://unpkg.com/redux@3.7.1/dist/redux.js\"></script>")?;
            writeln!(w, "<script src=\"https://ajax.googleapis.com/ajax/libs/incrementaldom/0.5.1/incremental-dom.js\" defer=\"defer\"></script>", )?;
            writeln!(w, "<script>", )?;

            // Document processing state
            let mut reducer_key_data: ReducerKeyMap = Default::default();
            let mut nodes_vec: NodesVec = Default::default();
            let mut ops_vec: OpsVec = Default::default();

            // Process document nodes and extract element operations and nodes_vec
            self.process_nodes(&mut reducer_key_data, &mut nodes_vec, &mut ops_vec)?;

            writeln!(w, "(function() {{")?;

            // Javascript output
            writeln!(w, "function render(store) {{")?;

                // Define components
                for ref loc in self.ast.children.iter() {
                    match &loc.inner {
                        &NodeType::ComponentDefinitionNode(ref component_data) => {
                            self.write_js_function(w, component_data)?;
                        },
                        _ => {}
                    }
                }
                writeln!(w, "")?;
                writeln!(w, "")?;

                // Render content nodes as incdom calls
                for ref loc in self.ast.children.iter() {
                    match &loc.inner {
                        &NodeType::ContentNode(ref content) => {
                            self.write_js_incdom_content(w, content, Some("store.getState().".into()))?;
                        },
                        _ => {},
                    }
                }

                // Rebind actions
                // TODO: Implement proper event system
                writeln!(w, "  // Bind action links")?;
                writeln!(w, "  var increment_el = document.querySelector(\"a[href='#increment']\");")?;
                writeln!(w, "  increment_el.onclick = function() {{ store.dispatch({{ type: \"COUNTER.INCREMENT\" }}); }};")?;
                writeln!(w, "  var decrement_el = document.querySelector(\"a[href='#decrement']\");")?;
                writeln!(w, "  decrement_el.onclick = function() {{ store.dispatch({{ type: \"COUNTER.DECREMENT\" }}); }};")?;

            writeln!(w, "}}")?;

            writeln!(w, "function update(root_el, store) {{")?;
            writeln!(w, "  IncrementalDOM.patch(root_el, render.bind(null, store));")?;
            writeln!(w, "}}")?;

            // Callback that will execute after deferred scripts and content is ready
            writeln!(w, "document.addEventListener(\"DOMContentLoaded\", function(event) {{")?;

            writeln!(w, "  // Define store")?;
            self.write_js_store(w, &reducer_key_data)?;

            writeln!(w, "  // Root subscription")?;
            writeln!(w, "  var root_el = document.querySelector(\"#root\");")?;
            writeln!(w, "  store.subscribe(function() {{ update(root_el, store); }});")?;

            writeln!(w, "  // Bind action links")?;
            writeln!(w, "  var increment_el = document.querySelector(\"a[href='#increment']\");")?;
            writeln!(w, "  increment_el.onclick = function() {{ store.dispatch({{ type: \"COUNTER.INCREMENT\" }}); }};")?;
            writeln!(w, "  var decrement_el = document.querySelector(\"a[href='#decrement']\");")?;
            writeln!(w, "  decrement_el.onclick = function() {{ store.dispatch({{ type: \"COUNTER.DECREMENT\" }}); }};")?;

            writeln!(w, "function Blank() {{}}")?;
            writeln!(w, "Blank.prototype = Object.create(null)")?;

            writeln!(w, "function markExisting(node, key, attrsArr) {{")?;
            writeln!(w, "  IncrementalDOM.importNode(node);")?;
            writeln!(w, "  var data = node['__incrementalDOMData'];")?;
            writeln!(w, "  data.staticsApplied = true;")?;
            writeln!(w, "  data.newAttrs = new Blank();")?;
            writeln!(w, "}}")?;

            // Mark the DOM elements we just rendered so incdom will not attempt to replace them on initial render
            for node in nodes_vec.iter() {
                match *node {
                    (_, Some(ref element_key), _) => {
                        writeln!(w, "  markExisting(document.querySelector(\"[key='{}']\"), \"{}\");", element_key, element_key)?;
                    },
                    _ => {
                        writeln!(w, "// Unmatched node: {:?}", node)?;
                    }
                }
            }

            writeln!(w, "}});")?;
            writeln!(w, "}})();")?;

            writeln!(w, "</script>\n</head>")?;

            writeln!(w, "<body>")?;
            writeln!(w, "<div id=\"root\">")?;

            self.write_html_ops_content(w, ops_vec.iter())?;

            writeln!(w, "</div>")?;
            writeln!(w, "</body>")?;
            writeln!(w, "</html>")?;
            Ok(())
        }
    }
}

use self::format_html::FormatHtml;

pub type Result = io::Result<fmt::Result>;

pub struct ClientOutput<'input> {
    ast: &'input Template
}

impl<'input> ClientOutput<'input> {
    pub fn from_template(ast: &'input Template) -> ClientOutput {
        ClientOutput {
            ast: ast
        }
    }

    pub fn write_html(&self, w : &mut io::Write) -> Result {
        let format = FormatHtml::from_template(self.ast);
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