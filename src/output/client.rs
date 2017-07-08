
use std::io;
use std::fmt;
use parser::ast::*;

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

    pub type NodesVec = Vec<(String, Option<String>, Option<Vec<(String, ExprValue)>>)>;
    pub type ReducerKeyMap<'inp> = HashMap<&'inp str, ReducerKeyData>;

    pub struct FormatHtml<'input> {
        ast: &'input Template,
    }

    impl<'input> FormatHtml<'input> {
        pub fn from_template<'inp>(ast: &'inp Template) -> FormatHtml<'inp> {
            FormatHtml { ast: ast }
        }

        #[inline]
        pub fn write_js_expr_value(&self,
                                   w: &mut fmt::Write,
                                   node: &ExprValue,
                                   var_prefix: Option<&str>,
                                   default_var: Option<&str>)
                                   -> fmt::Result {
            match node {
                // TODO: Handle the case where quotes appear in the string
                &ExprValue::LiteralString(ref s) => {
                    write!(w, "\"{}\"", s)?;
                }
                &ExprValue::LiteralNumber(ref n) => {
                    write!(w, "{}", n)?;
                }

                &ExprValue::DefaultVariableReference => {
                    write!(w,
                           "{}{}",
                           var_prefix.and_then(|prefix| Some(format!("{}.", prefix.to_uppercase())))
                               .unwrap_or_default(),
                           default_var.unwrap_or("value".into()))
                        ?;
                }

                &ExprValue::VariableReference(ref s) => {
                    if let Some(ref prefix) = var_prefix {
                        write!(w, "{}{}", prefix, s)?;
                    } else {
                        write!(w, "{}", s)?;
                    }
                }

                &ExprValue::Expr(ref sym, ref l, ref r) => {
                    self.write_js_expr_value(w, l, var_prefix, default_var)?;
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
                    self.write_js_expr_value(w, r, var_prefix, default_var)?;
                }

                &ExprValue::DefaultAction(..) => {}
                &ExprValue::Action(..) => {}
            }
            Ok(())
        }

        #[inline]
        #[allow(unused_variables)]
        pub fn write_computed_expr_value(&self,
                                         w: &mut fmt::Write,
                                         node: &ExprValue,
                                         var_prefix: Option<&str>)
                                         -> fmt::Result {
            match node {
                &ExprValue::LiteralString(ref s) => {
                    write!(w, "{}", s)?;
                }
                &ExprValue::LiteralNumber(ref n) => {
                    write!(w, "{}", n)?;
                }

                &ExprValue::DefaultVariableReference => {
                    if let Some(ref prefix) = var_prefix {
                        write!(w, "{}", prefix)?;
                    } else {
                        write!(w, "value")?;
                    }
                }

                &ExprValue::VariableReference(ref s) => {
                    if let Some(ref prefix) = var_prefix {
                        write!(w, "{}{}", prefix, s)?;
                    } else {
                        write!(w, "{}", s)?;
                    }
                }

                &ExprValue::Expr(ref sym, ref l, ref r) => {
                    write!(w, "{:?} {:?} {:?}", l, sym, r)?;
                }

                &ExprValue::DefaultAction(..) => {}

                &ExprValue::Action(..) => {}
            }
            Ok(())
        }

        #[inline]
        #[allow(unused_variables)]
        pub fn write_js_action(&self,
                               w: &mut fmt::Write,
                               act_iter: Iter<ActionOpNode>)
                               -> fmt::Result {
            write!(w, "function(event) {{")?;
            for ref act_op in act_iter {
                match *act_op {
                    &ActionOpNode::DispatchAction(ref action, ref params) => {
                        write!(w, " store.dispatch({{\"type\": \"{}\"}}); ", action)?;
                    }
                }
            }
            write!(w, "}}")?;
            Ok(())
        }

        #[inline]
        #[allow(unused_variables)]
        pub fn write_html_ops_content(&self,
                                      w: &mut fmt::Write,
                                      ops: Iter<ElementOp>,
                                      nodes_vec: &mut NodesVec,
                                      events_vec: &mut EventsVec,
                                      comp_map: &'input ComponentMap<'input>)
                                      -> fmt::Result {
            for ref op in ops {
                match *op {
                    &ElementOp::ElementOpen(ref element_tag,
                                            ref element_key,
                                            ref attrs,
                                            ref events) => {
                        let element_key = element_key.as_ref()
                            .map_or_else(allocate_element_key, Clone::clone);

                        write!(w, "<{}", element_tag)?;
                        write!(w, " key=\"{}\"", element_key)?;

                        if let &Some(ref attrs) = attrs {
                            for &(ref key, ref expr) in attrs {
                                match expr {
                                    &ExprValue::DefaultAction(ref params, ref act_ops) => {
                                        if let &Some(ref act_ops) = act_ops {
                                            self.write_js_action(w, act_ops.iter())?;
                                            continue;
                                        };
                                    }
                                    &ExprValue::Action(ref event_name, ref params, ref act_ops) => {
                                        if let &Some(ref act_ops) = act_ops {
                                            self.write_js_action(w, act_ops.iter())?;
                                            continue;
                                        };
                                    }
                                    _ => {
                                        write!(w, " {}=\"", key)?;
                                        self.write_computed_expr_value(w, expr, None)?;
                                        write!(w, "\"")?;
                                    }
                                };
                            }
                        };
                        write!(w, ">")?;

                        // Process events
                        if let &Some(ref events) = events {
                            for &(ref event_name, ref event_params, ref action_ops) in events {
                                let event_params = event_params.as_ref().map(Clone::clone);
                                let action_ops = action_ops.as_ref().map(Clone::clone);
                                let event_name = event_name.as_ref().map(Clone::clone);
                                events_vec.push((element_key.clone(),
                                                 event_name,
                                                 event_params,
                                                 action_ops));
                            }
                        }

                        nodes_vec.push((element_tag.clone(),
                                        Some(element_key.clone()),
                                        attrs.as_ref()
                            .map(|attrs| attrs.iter().map(Clone::clone).collect())));
                    }
                    &ElementOp::ElementClose(ref element_tag) => {
                        write!(w, "</{}>", element_tag)?;
                    }
                    &ElementOp::ElementVoid(ref element_tag,
                                            ref element_key,
                                            ref attrs,
                                            ref events) => {
                        let element_key = element_key.as_ref()
                            .map_or_else(allocate_element_key, Clone::clone);

                        write!(w, "<{}", element_tag)?;
                        write!(w, " key=\"{}\"", element_key)?;

                        if let &Some(ref attrs) = attrs {
                            for &(ref key, ref expr) in attrs {
                                write!(w, " {}=\"", key)?;
                                self.write_computed_expr_value(w, expr, None)?;
                                write!(w, "\"")?;
                            }
                        }
                        write!(w, " />")?;

                        nodes_vec.push((element_tag.clone(),
                                        Some(element_key),
                                        attrs.as_ref()
                            .map(|attrs| attrs.iter().map(Clone::clone).collect())));
                    }
                    &ElementOp::WriteValue(ref expr, ref element_key) => {
                        let key_str = element_key.as_ref().map_or("null", |s| s);
                        write!(w, "<span key=\"{}\">", key_str)?;
                        self.write_computed_expr_value(w, expr, None)?;
                        write!(w, "</span>")?;

                        nodes_vec.push((String::from("span"),
                                        element_key.as_ref().map(Clone::clone),
                                        None));
                    }
                    &ElementOp::InstanceComponent(ref component_ty,
                                                  ref component_key,
                                                  ref attrs) => {
                        // Try to locate a matching component
                        if let Some(ref comp) = comp_map.get(component_ty.as_str()) {
                            // Render a component

                            write!(w, "<div")?;
                            if let &Some(ref component_key) = component_key {
                                write!(w, " key=\"{}\"", component_key)?;
                            }
                            write!(w, ">")?;

                            if let Some(ref component_ops) = comp.ops {
                                self.write_html_ops_content(w,
                                                            component_ops.iter(),
                                                            nodes_vec,
                                                            events_vec,
                                                            comp_map)?;
                            };

                            write!(w, "</div>")?;

                            nodes_vec.push((String::from("div"),
                                            component_key.as_ref().map(Clone::clone),
                                            None));
                        };
                    }
                }
            }

            Ok(())
        }

        #[inline]
        #[allow(unused_variables)]
        fn write_js_incdom_attr_array(&self,
                                      w: &mut fmt::Write,
                                      attrs: &Vec<(String, ExprValue)>)
                                      -> fmt::Result {
            let mut wrote_first = false;
            for &(ref key, ref expr) in attrs {
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

                write!(w, "\"{}\", \"", key)?;
                self.write_computed_expr_value(w, &expr, None)?;
                write!(w, "\"")?;
            }
            Ok(())
        }

        #[inline]
        #[allow(unused_variables)]
        pub fn write_js_incdom_ops_content(&self,
                                           w: &mut fmt::Write,
                                           ops: Iter<ElementOp>,
                                           var_prefix: Option<&str>,
                                           default_var: Option<&str>,
                                           key_prefix: Option<&str>,
                                           comp_map: &ComponentMap<'input>)
                                           -> fmt::Result {
            for ref op in ops {
                match *op {
                    &ElementOp::ElementOpen(ref element_tag,
                                            ref element_key,
                                            ref attrs,
                                            ref events) => {
                        let element_key = element_key.as_ref().map_or("null", |s| s);

                        write!(w,
                               "IncrementalDOM.elementOpen(\"{}\", \"{}\", [",
                               element_tag,
                               element_key)
                            ?;

                        // Static attrs
                        if let &Some(ref attrs) = attrs {
                            self.write_js_incdom_attr_array(w, attrs)?;
                        };

                        // TODO: Dynamic attributes

                        writeln!(w, "]);")?;
                    }
                    &ElementOp::ElementClose(ref element_tag) => {
                        writeln!(w, "IncrementalDOM.elementClose(\"{}\");", element_tag)?;
                    }
                    &ElementOp::ElementVoid(ref element_tag,
                                            ref element_key,
                                            ref attrs,
                                            ref events) => {
                        let element_key = element_key.as_ref().map_or("null", |s| s);

                        write!(w,
                               "IncrementalDOM.elementVoid(\"{}\", \"{}\", [);",
                               element_tag,
                               element_key)
                            ?;

                        // Static attrs
                        if let &Some(ref attrs) = attrs {
                            self.write_js_incdom_attr_array(w, attrs)?;
                        };

                        // TODO: Dynamic attributes

                        writeln!(w, "]);")?;
                    }
                    &ElementOp::WriteValue(ref expr, ref element_key) => {
                        let element_key = element_key.as_ref().map_or("null", |s| s);
                        writeln!(w,
                                 "IncrementalDOM.elementOpen(\"span\", \"{}\", [\"key\", \
                                  \"{}\"]);",
                                 element_key,
                                 element_key)
                            ?;
                        write!(w, "IncrementalDOM.text(")?;
                        self.write_js_expr_value(w, expr, var_prefix, default_var)?;
                        writeln!(w, ");")?;
                        writeln!(w, "IncrementalDOM.elementClose(\"span\");")?;
                    }
                    &ElementOp::InstanceComponent(ref component_ty,
                                                  ref component_key,
                                                  ref attrs) => {
                        let comp = comp_map.get(component_ty.as_str());
                        if comp.is_some() {
                            let component_key = component_key.as_ref().map_or("null", |s| s);
                            writeln!(w,
                                     "IncrementalDOM.elementOpen(\"div\", \"{}\", []);",
                                     component_key)
                                ?;
                            write!(w, "component_{}(store, [", component_ty)?;
                            if let &Some(ref attrs) = attrs {
                                self.write_js_incdom_attr_array(w, attrs)?;
                            }
                            writeln!(w, "]);")?;
                            writeln!(w, "IncrementalDOM.elementClose(\"div\");")?;
                        }
                    }
                }
            }

            Ok(())
        }

        #[allow(dead_code)]
        pub fn write_js_incdom_component(&self,
                                         w: &mut fmt::Write,
                                         component_name: &str,
                                         component_key: &str,
                                         ops: Iter<ElementOp>,
                                         comp_map: &ComponentMap<'input>)
                                         -> fmt::Result {
            writeln!(w, "function {}(props) {{", component_name)?;
            self.write_js_incdom_ops_content(w,
                                             ops,
                                             Some("props."),
                                             Some("props"),
                                             Some(component_key),
                                             comp_map)?;
            writeln!(w, "}};")?;
            Ok(())
        }

        pub fn collect_js_store_child_scope(&self,
                                            reducer_key_data: &mut ReducerKeyMap<'input>,
                                            reducer_key: &'input str,
                                            nodes: &'input Vec<ScopeNodeType>,
                                            reducer_key_prefix: Option<&str>)
                                            -> fmt::Result {
            for ref node in nodes {
                match *node {
                    &ScopeNodeType::LetNode(ref var_name, ref expr) => {
                        let reducer_entry = reducer_key_data.entry(var_name)
                            .or_insert_with(|| ReducerKeyData::from_name(&format!("{}", var_name)));

                        if let &Some(ref expr) = expr {
                            reducer_entry.default_expr = Some(expr.clone());
                        };
                    }
                    &ScopeNodeType::ActionNode(ref action_name, ref simple_expr) => {
                        let reducer_entry = reducer_key_data.entry(reducer_key)
                            .or_insert_with(|| {
                                ReducerKeyData::from_name(&format!("{}", reducer_key))
                            });

                        let action_path = format!("{}{}",
                                                  reducer_key_prefix.and_then(|prefix| {
                                                          Some(format!("{}", prefix.to_uppercase()))
                                                      })
                                                      .and_then(|prefix| {
                                                          Some(format!("{}.",
                                                                       prefix.to_uppercase()))
                                                      })
                                                      .unwrap_or_default(),
                                                  action_name);

                        let mut action = ReducerActionData::from_name(&action_path);
                        if let &Some(ref simple_expr) = simple_expr {
                            action.state_expr = Some(simple_expr.clone());
                        };
                        if let Some(ref mut actions) = reducer_entry.actions {
                            actions.push(action);
                        };
                    }
                    &ScopeNodeType::ScopeNode(ref scope_name, ref scope_nodes) => {
                        self.collect_js_store_child_scope(reducer_key_data,
                                                          scope_name,
                                                          scope_nodes,
                                                          reducer_key_prefix)?;
                    }
                    _ => {}
                }
            }
            Ok(())
        }

        #[allow(unused_variables)]
        pub fn collect_js_store_api_scope(&self,
                                          reducer_key_data: &mut ReducerKeyMap<'input>,
                                          scope_name: &'input str,
                                          nodes: &'input Vec<ApiNodeType>)
                                          -> fmt::Result {
            for ref node in nodes {
                match *node {
                    &ApiNodeType::ResourceNode(ref resource_data) => {
                        let reducer_name: &'input str = &resource_data.resource_name;

                        let reducer_entry = reducer_key_data.entry(scope_name)
                            .or_insert_with(|| {
                                ReducerKeyData::from_name(&format!("{}", scope_name))
                            });
                    }
                    _ => {}
                }
            }
            Ok(())
        }

        pub fn collect_js_store_default_scope(&self,
                                              reducer_key_data: &mut HashMap<&'input str,
                                                                             ReducerKeyData>,
                                              nodes: &'input Vec<DefaultScopeNodeType>)
                                              -> fmt::Result {
            for ref node in nodes {
                match *node {
                    &DefaultScopeNodeType::LetNode(ref var_name, ref expr) => {
                        // Within the default scope let defines a new scope and it's default expression
                        let reducer_entry = reducer_key_data.entry(var_name)
                            .or_insert_with(|| ReducerKeyData::from_name(&format!("{}", var_name)));

                        if let &Some(ref expr) = expr {
                            reducer_entry.default_expr = Some(expr.clone());
                        };
                    }
                    &DefaultScopeNodeType::ApiRootNode(ref scope_name, ref api_nodes) => {
                        if let &Some(ref api_nodes) = api_nodes {
                            self.collect_js_store_api_scope(reducer_key_data,
                                                            scope_name,
                                                            api_nodes)?;
                        }
                    }
                    &DefaultScopeNodeType::ScopeNode(ref scope_name, ref scope_nodes) => {
                        self.collect_js_store_child_scope(reducer_key_data,
                                                          scope_name,
                                                          scope_nodes,
                                                          None)?;
                    }
                }
            }
            Ok(())
        }

        pub fn write_js_store(&self,
                              w: &mut fmt::Write,
                              reducer_key_data: &HashMap<&'input str, ReducerKeyData>)
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
                                self.write_js_expr_value(&mut state_expr_str,
                                                         simple_expr,
                                                         None,
                                                         Some("state".into()))?;
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
                    self.write_js_expr_value(w, default_expr, None, None)?;
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

        #[inline]
        fn process_content_node(&self,
                                node: &'input ContentNodeType,
                                ops_vec: &'input mut OpsVec,
                                events_vec: &'input mut EventsVec,
                                comp_map: &'input ComponentMap<'input>)
                                -> fmt::Result {
            match node {
                &ContentNodeType::ElementNode(ref element_data) => {
                    let element_tag = element_data.element_ty.to_lowercase();
                    let element_key =
                        element_data.element_key.as_ref().map_or(String::from(""), Clone::clone);
                    let op_attrs = element_data.attrs
                        .as_ref()
                        .map(|attrs| attrs.iter().map(Clone::clone).collect());
                    let events = element_data.events
                        .as_ref()
                        .map(|attrs| attrs.iter().map(Clone::clone).collect());

                    // Try to locate a matching component
                    let comp = comp_map.get(element_data.element_ty.as_str());
                    if let Some(..) = comp {
                        // Render a component during render
                        ops_vec.push(ElementOp::InstanceComponent(element_tag,
                                                                  Some(element_key),
                                                                  op_attrs));
                    } else {
                        // Treat this as an HTML element
                        // TODO: Support imported elements

                        // Process events
                        if let Some(ref events) = element_data.events {
                            for &(ref event_name, ref event_params, ref action_ops) in events {
                                let event_name = event_name.as_ref().map(Clone::clone);
                                let event_params = event_params.as_ref().map(Clone::clone);
                                let action_ops = action_ops.as_ref().map(Clone::clone);
                                events_vec.push((element_key.clone(),
                                                 event_name,
                                                 event_params,
                                                 action_ops));
                            }
                        }

                        if let Some(ref children) = element_data.children {
                            // Push element open
                            ops_vec.push(ElementOp::ElementOpen(element_tag.clone(),
                                                                Some(element_key),
                                                                op_attrs,
                                                                events));

                            // Iterate over children
                            for ref child in children {
                                self.process_content_node(child, ops_vec, events_vec, comp_map)?;
                            }

                            // Push element close
                            ops_vec.push(ElementOp::ElementClose(element_tag.clone()));
                        } else {
                            ops_vec.push(ElementOp::ElementVoid(element_tag.clone(),
                                                                Some(element_key),
                                                                op_attrs,
                                                                events));
                        }
                    }
                }
                &ContentNodeType::ExpressionValueNode(ref expr) => {
                    ops_vec.push(ElementOp::WriteValue(expr.clone(), Some(allocate_element_key())));
                }
            }
            (Ok(()))
        }

        #[allow(dead_code)]
        pub fn process_component_definition(&self,
                                            component_data: &'input ComponentDefinitionType,
                                            _: &mut self::ReducerKeyMap<'input>,
                                            comp_map: &mut ComponentMap<'input>)
                                            -> fmt::Result {
            let name: &'input str = component_data.name.as_str();
            let mut ops: OpsVec = Vec::new();
            let mut events: EventsVec = Vec::new();

            if let Some(ref children) = component_data.children {
                for ref child in children {
                    match *child {
                        &NodeType::ContentNode(ref content) => {
                            self.process_content_node(content, &mut ops, &mut events, comp_map)?;
                        }
                        _ => {}
                    }
                }
            }

            let comp = Component {
                name: name,
                ops: Some(ops),
                uses: None,
                child_map: Default::default(),
            };

            comp_map.insert(name, comp);

            Ok(())
        }

        pub fn process_nodes(&self,
                             reducer_key_data: &mut self::ReducerKeyMap<'input>,
                             ops_vec: &mut OpsVec,
                             events_vec: &mut EventsVec,
                             comp_map: &mut ComponentMap<'input>)
                             -> fmt::Result {
            let mut processed_store = false;

            for ref loc in self.ast.children.iter() {
                match &loc.inner {
                    &NodeType::StoreNode(ref scope_nodes) => {
                        // TODO: Allow more than one store?
                        if !processed_store {
                            self.collect_js_store_default_scope(reducer_key_data, scope_nodes)?;
                            processed_store = true;

                        }
                    }
                    &NodeType::ComponentDefinitionNode(ref component_data) => {
                        self.process_component_definition(component_data,
                                                          reducer_key_data,
                                                          comp_map)?;
                    }
                    &NodeType::ContentNode(ref content) => {
                        self.process_content_node(content, ops_vec, events_vec, comp_map)?;
                    }
                    _ => {}
                }
            }
            Ok(())
        }

        #[allow(unused_variables)]
        pub fn write_js_event_bindings(&self,
                                       w: &mut fmt::Write,
                                       events_vec: &EventsVec,
                                       action_prefix: Option<&str>)
                                       -> fmt::Result {
            writeln!(w, "  // Bind actions")?;
            for &(ref element_key, ref event_name, ref params, ref action_ops) in events_vec {
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
                                let action_prefix = action_prefix.map_or("", |s| s);
                                let action_ty = format!("{}.{}",
                                                        action_prefix.to_uppercase(),
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
            // Document processing state
            let mut reducer_key_data: ReducerKeyMap = Default::default();
            let mut comp_map: ComponentMap = Default::default();
            let mut nodes_vec: NodesVec = Default::default();
            let mut ops_vec: OpsVec = Default::default();
            let mut events_vec: EventsVec = Default::default();

            // Process document nodes and populate processing state
            self.process_nodes(&mut reducer_key_data,
                               &mut ops_vec,
                               &mut events_vec,
                               &mut comp_map)?;

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

            self.write_html_ops_content(w,
                                        ops_vec.iter(),
                                        &mut nodes_vec,
                                        &mut events_vec,
                                        &comp_map)?;

            write!(w, "{}", indoc!(r#"
                    </div>
                    <script>
                      (function() {
            "#))?;

            // Define components
            for (ref component_ty, ref comp_def) in comp_map.iter() {
                writeln!(w, "  function component_{}(store, props) {{", component_ty)?;
                if let Some(ref ops) = comp_def.ops {
                    self.write_js_incdom_ops_content(w,
                                                     ops.iter(),
                                                     Some("store.getState()."),
                                                     Some("store.getState()"),
                                                     None,
                                                     &comp_map)?;
                }
                writeln!(w, "  }};")?;
                writeln!(w, "")?;
            }

            writeln!(w, "function render(store) {{")?;

            // Render content nodes as incdom calls
            self.write_js_incdom_ops_content(w,
                                             ops_vec.iter(),
                                             Some("store.getState()."),
                                             Some("store.getState()"),
                                             None,
                                             &comp_map)?;

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
            self.write_js_store(w, &reducer_key_data)?;

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
            for node in nodes_vec.iter() {
                match *node {
                    (ref element_tag, Some(ref element_key), _) => {
                        writeln!(w,
                                 "  markExisting(document.querySelector(\"[key='{}']\"), \
                                  \"{}\"); // {}",
                                 element_key,
                                 element_key,
                                 element_tag)
                            ?;
                    }
                    _ => {
                        writeln!(w, "// Unmatched node: {:?}", node)?;
                    }
                }
            }

            writeln!(w,
                     "  store.subscribe(function() {{ update(root_el, store); }});")
                ?;

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

pub type Result = io::Result<fmt::Result>;

pub struct ClientOutput<'input> {
    ast: &'input Template,
}

impl<'input> ClientOutput<'input> {
    pub fn from_template(ast: &'input Template) -> ClientOutput {
        ClientOutput { ast: ast }
    }

    pub fn write_html(&self, w: &mut io::Write) -> Result {
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
