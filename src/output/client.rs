
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

    pub type ReducerKeyMap<'inp> = HashMap<&'inp str, ReducerKeyData>;

    pub struct FormatHtml<'input> {
        ast: &'input Template,
    }

    impl<'input> FormatHtml<'input> {
        pub fn from_template<'inp>(ast: &'inp Template) -> FormatHtml<'inp> {
            FormatHtml { ast: ast }
        }

        #[inline]
        pub fn write_js_expr_var_reference(&self,
                                   w: &mut fmt::Write,
                                   var_name: &str,
                                   var_prefix: Option<&str>)
                                   -> fmt::Result {
            if let Some(ref prefix) = var_prefix {
                write!(w, "{}{}", prefix, var_name)?;
            } else {
                write!(w, "{}", var_name)?;
            };
            Ok(())
        }

        #[inline]
        pub fn write_js_expr_value(&self,
                                   w: &mut fmt::Write,
                                   node: &ExprValue,
                                   default_state_map: &DefaultStateMap<'input>,
                                   var_prefix: Option<&str>,
                                   default_var: Option<&str>,
                                   action_prefix: Option<&str>)
                                   -> fmt::Result {
            match node {
                // TODO: Handle the case where quotes appear in the string
                &ExprValue::LiteralString(ref s) => {
                    write!(w, "\"{}\"", s)?;
                }
                &ExprValue::LiteralNumber(ref n) => {
                    write!(w, "{}", n)?;
                }

                &ExprValue::LiteralArray(ref items) => {
                    if let &Some(ref items) = items {
                        write!(w, "[")?;
                        for ref item in items {
                            self.write_js_expr_value(w, item, default_state_map, var_prefix, default_var, action_prefix)?;                    
                        };
                        write!(w, "]")?;
                    };
                }

                &ExprValue::DefaultVariableReference => {
                    write!(w, "{}", default_var.unwrap_or("value".into()))?;
                }

                &ExprValue::VariableReference(ref var_name) => {
                    if let Some(_) = default_state_map.get(var_name.as_str()) {
                        //let store_prefix = format!("store.getState(){}", default_scope.as_ref().map_or("", |s| format!("{}.", s)));
                        self.write_js_expr_var_reference(w, var_name, Some("store.getState()."))?; // FIXME
                    } else {
                        self.write_js_expr_var_reference(w, var_name, None)?;
                    }
                }

                &ExprValue::Expr(ExprOp::Add, box ExprValue::DefaultVariableReference, ref r) => {
                    let state_var_name = "lines".into(); // FIXME
                    let var_name = "store.getState().lines".into();
                    if let Some(entry) = default_state_map.get(state_var_name) {
                        if let Some(VarType::ArrayVar(..)) = entry.0 {
                            write!(w, "((")?;
                            self.write_js_expr_var_reference(w, var_name, var_prefix)?;
                            write!(w, ").concat(")?;
                            self.write_js_expr_value(w, r, default_state_map, var_prefix, default_var, action_prefix)?;
                            write!(w, "))")?;
                        } else {
                            self.write_js_expr_var_reference(w, var_name, var_prefix)?;
                            write!(w, " + ")?;
                            self.write_js_expr_value(w, r, default_state_map, var_prefix, default_var, action_prefix)?;
                        }
                    }
                }

                &ExprValue::Expr(ExprOp::Add, box ExprValue::VariableReference(ref var_name), ref r) => {
                    if let Some(entry) = default_state_map.get(var_name.as_str()) {
                        if let Some(VarType::ArrayVar(..)) = entry.0 {
                            write!(w, "((")?;
                            self.write_js_expr_var_reference(w, var_name, var_prefix)?;
                            write!(w, ").concat(")?;
                            self.write_js_expr_value(w, r, default_state_map, var_prefix, default_var, action_prefix)?;
                            write!(w, "))")?;
                        } else {
                            self.write_js_expr_var_reference(w, var_name, var_prefix)?;
                            write!(w, " + ")?;
                            self.write_js_expr_value(w, r, default_state_map, var_prefix, default_var, action_prefix)?;
                        }
                    }
                }

                &ExprValue::Expr(ref sym, ref l, ref r) => {
                    self.write_js_expr_value(w, l, default_state_map, var_prefix, default_var, action_prefix)?;
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
                    self.write_js_expr_value(w, r, default_state_map, var_prefix, default_var, action_prefix)?;
                }

                &ExprValue::ContentNode(..) => {}
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
                                         var_prefix: Option<&str>,
                                         default_var: Option<&str>)
                                         -> fmt::Result {
            match node {
                &ExprValue::LiteralString(ref s) => {
                    write!(w, "{}", s)?;
                }
                &ExprValue::LiteralNumber(ref n) => {
                    write!(w, "{}", n)?;
                }

                &ExprValue::LiteralArray(ref items) => {
                    if let &Some(ref items) = items {
                        for ref item in items {
                            self.write_computed_expr_value(w, item, var_prefix, default_var)?;
                        };
                    };
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
                    // write!(w, "{:?} {:?} {:?}", l, sym, r)?;
                    self.write_computed_expr_value(w, l, var_prefix, default_var)?;
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
                    self.write_computed_expr_value(w, r, var_prefix, default_var)?;
                }

                &ExprValue::ContentNode(..) => {}

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
                                      events_vec: &mut EventsVec,
                                      comp_map: &'input ComponentMap<'input>,
                                      keys_vec: &mut Vec<String>,
                                      key_prefix: Option<&str>,
                                      default_scope: Option<&str>)
                                      -> fmt::Result {
            for ref op in ops {
                let mut is_void = false;

                if let &ElementOp::ElementVoid(..) = *op {
                    is_void = true;
                }

                match *op {
                    &ElementOp::ElementOpen(ref element_tag,
                                            ref element_key,
                                            ref attrs,
                                            ref events)
                        | &ElementOp::ElementVoid(ref element_tag,
                                            ref element_key,
                                            ref attrs,
                                            ref events) => {

                        let element_key = format!("{}{}",
                            key_prefix.map_or("".into(), |s| format!("{}_", s)),
                            element_key.as_ref().map_or_else(allocate_element_key, Clone::clone)
                        );
                        
                        write!(w, "<{}", element_tag)?;
                        write!(w, " key=\"{}\"", element_key)?;

                        if let &Some(ref attrs) = attrs {
                            for &(ref key, ref expr) in attrs {
                                // Ignore empty attributes
                                if let &Some(ref expr) = expr {
                                    match expr {
                                        &ExprValue::Expr(ExprOp::Add, ref l, ref r) => {


                                        }

                                        &ExprValue::DefaultAction(ref params, ref act_ops) => {
                                            if let &Some(ref act_ops) = act_ops {
                                                self.write_js_action(w, act_ops.iter())?;
                                                continue;
                                            };
                                        },
                                        &ExprValue::Action(ref event_name, ref params, ref act_ops) => {
                                            if let &Some(ref act_ops) = act_ops {
                                                self.write_js_action(w, act_ops.iter())?;
                                                continue;
                                            };
                                        },
                                        _ => {
                                            write!(w, " {}=\"", key)?;
                                            self.write_computed_expr_value(w, expr, None, None)?;
                                            write!(w, "\"")?;
                                        }
                                    };
                                };
                            }
                        };

                        if is_void {
                            write!(w, ">")?;
                        } else {
                            write!(w, ">")?;
                        };

                        // Process events
                        if let &Some(ref events) = events {
                            for &(ref event_name, ref event_params, ref action_ops) in events {
                                let event_params = event_params.as_ref().map(Clone::clone);
                                let action_ops = action_ops.as_ref().map(Clone::clone);
                                let event_name = event_name.as_ref().map(Clone::clone);
                                let default_scope = default_scope.as_ref().map(|s| format!("{}", s));
                                events_vec.push((element_key.clone(),
                                                 event_name,
                                                 event_params,
                                                 action_ops,
                                                 default_scope));
                            }
                        }

                        keys_vec.push(element_key);
                    }
                    &ElementOp::ElementClose(ref element_tag) => {
                        write!(w, "</{}>", element_tag)?;
                    }
                    &ElementOp::WriteValue(ref expr, ref element_key) => {
                        let element_key = element_key.as_ref().map_or_else(allocate_element_key, |s| s.clone());
                        write!(w, "<span key=\"{}\">", element_key)?;
                        self.write_computed_expr_value(w, expr, None, None)?;
                        write!(w, "</span>")?;

                        keys_vec.push(element_key);
                    }
                    &ElementOp::InstanceComponent(ref component_ty,
                                                  ref element_key,
                                                  ref props,
                                                  ref lens) => {
                        // Try to locate a matching component
                        if let Some(ref comp) = comp_map.get(component_ty.as_str()) {
                            // Render a component

                            let element_key = format!("{}{}",
                                key_prefix.as_ref().map_or("".into(), |s| format!("{}_", s)),
                                element_key.as_ref().map_or_else(allocate_element_key, |s| s.clone())
                            );
                        
                            write!(w, "<div key=\"{}\" >", element_key)?;

                            let default_scope = lens;

                            if let Some(ref component_ops) = comp.ops {
                                self.write_html_ops_content(w,
                                                            component_ops.iter(),
                                                            events_vec,
                                                            comp_map,
                                                            keys_vec,
                                                            Some(&element_key),
                                                            default_scope.as_ref().map(String::as_str))?;
                            };

                            write!(w, "</div>")?;

                            keys_vec.push(element_key);
                        };
                    },

                    _ => {}
                }
            }

            Ok(())
        }

        #[inline]
        #[allow(unused_variables)]
        fn write_js_incdom_attr_array(&self,
                                      w: &mut fmt::Write,
                                      attrs: &Vec<Prop>,
                                      default_state_map: &DefaultStateMap<'input>,
                                      var_prefix: Option<&str>,
                                      default_var: Option<&str>,
                                      action_prefix: Option<&str>)
                                      -> fmt::Result {
            let mut wrote_first = false;
            for &(ref key, ref expr) in attrs {
                if let &Some(ref expr) = expr {
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

                    write!(w, "\"{}\", ", key)?;
                    self.write_js_expr_value(w, &expr, default_state_map, var_prefix, default_var, action_prefix)?;
                } else {
                    write!(w, "\"{}\", ", key)?;
                    write!(w, "undefined")?;
                }
            }
            Ok(())
        }

        #[inline]
        #[allow(unused_variables)]
        fn write_js_props_object(&self,
                                      w: &mut fmt::Write,
                                      props: &Vec<Prop>,
                                      default_state_map: &DefaultStateMap<'input>,
                                      var_prefix: Option<&str>,
                                      default_var: Option<&str>,
                                      default_scope: Option<&str>)
                                      -> fmt::Result {
            write!(w, "{{")?;
            let mut wrote_first = false;
            for &(ref key, ref expr) in props {
                if wrote_first {
                    write!(w, ", ")?
                } else {
                    wrote_first = true;
                }

                // Write the property name
                write!(w, "\"{}\": ", key)?;

                // Write the property value or undefined for None
                if let &Some(ref expr) = expr {

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

                    self.write_js_expr_value(w, &expr, default_state_map, var_prefix, default_var, default_scope)?;
                } else {
                    write!(w, "undefined")?;
                }
            };
            write!(w, "}}")?;
            Ok(())
        }

        #[inline]
        #[allow(unused_variables)]
        pub fn write_js_incdom_ops_content(&self,
                                           w: &mut fmt::Write,
                                           ops: Iter<ElementOp>,
                                           default_state_map: &DefaultStateMap,
                                           var_prefix: Option<&str>,
                                           default_var: Option<&str>,
                                           key_prefix: Option<&str>,
                                           default_scope: Option<&str>,
                                           key_var_prefix: Option<&str>,
                                           comp_map: &ComponentMap<'input>)
                                           -> fmt::Result {
            for ref op in ops {
                let mut is_void = false;
                if let &ElementOp::ElementVoid(..) = *op {
                    is_void = true;
                }

                match *op {
                    &ElementOp::ElementOpen(ref element_tag,
                                            ref element_key,
                                            ref attrs,
                                            ref events)
                    |   &ElementOp::ElementVoid(ref element_tag,
                                                ref element_key,
                                                ref attrs,
                                                ref events) => {

                        let element_key = format!("{}{}",
                            key_prefix.as_ref().map_or("".into(), |s| format!("{}_", s)),
                            element_key.as_ref().map_or("null", |s| s)
                        );
                        let key_var_prefix = key_var_prefix.as_ref().map_or("".into(), |s| format!("{} + ", s));

                        if !is_void {
                            write!(w,
                                "IncrementalDOM.elementOpen(\"{}\", {}\"{}\", [",
                                element_tag,
                                key_var_prefix,
                                element_key)
                                ?;
                        } else {
                            write!(w,
                                "IncrementalDOM.elementVoid(\"{}\", {}\"{}\", [",
                                element_tag,
                                key_var_prefix,
                                element_key)
                                ?;
                        }

                        // Static attrs
                        if let &Some(ref attrs) = attrs {
                            self.write_js_incdom_attr_array(w, attrs, default_state_map, var_prefix, default_var, default_scope)?;
                        };

                        // TODO: Dynamic attributes

                        writeln!(w, "]);")?;
                    }
                    &ElementOp::ElementClose(ref element_tag) => {
                        writeln!(w, "IncrementalDOM.elementClose(\"{}\");", element_tag)?;
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
                        self.write_js_expr_value(w, expr, default_state_map, var_prefix, default_var, default_scope)?;
                        writeln!(w, ");")?;
                        writeln!(w, "IncrementalDOM.elementClose(\"span\");")?;
                    }
                    &ElementOp::InstanceComponent(ref component_ty,
                                                  ref component_key,
                                                  ref props,
                                                  ref lens) => {
                        let comp = comp_map.get(component_ty.as_str());
                        if comp.is_some() {
                            let component_key = component_key.as_ref().map_or("null", |s| s);
                            writeln!(w,
                                     "IncrementalDOM.elementOpen(\"div\", \"{}\", []);",
                                     component_key)
                                ?;
                            write!(w, "component_{}(\"{}_\", store, ", component_ty, component_key)?;
                            if let &Some(ref props) = props {
                                let var_prefix = lens.as_ref().map(|s| format!("store.getState().{}.", s));
                                let default_var = lens.as_ref().map(|s| format!("store.getState.{}", s));
                                let default_scope = lens.as_ref().map(|s| s.as_str());
                                self.write_js_props_object(
                                    w,
                                    props,
                                    default_state_map,
                                    var_prefix.as_ref().map(String::as_str),
                                    default_var.as_ref().map(String::as_str),
                                    default_scope)?;
                            }
                            writeln!(w, ");")?;
                            writeln!(w, "IncrementalDOM.elementClose(\"div\");")?;
                        }
                    },

                    &ElementOp::StartBlock(ref block_id) => {
                        //writeln!(w, "var __{} = function __{}_(__forvar_{}){{", block_id, block_id, block_id)?;
                        writeln!(w, "var __{} = function __{}_(line){{", block_id, block_id)?; //FIXME
                    }

                    &ElementOp::EndBlock(..) => {
                        writeln!(w, "}};")?;
                    }

                    &ElementOp::MapCollection(ref block_id, ref ele, ref coll_expr) => {
                        write!(w, "(")?;

                        let forvar_default = &format!("__forvar_{}", block_id);
                        let forvar_prefix = &format!("__forvar_{}{}", block_id,
                            ele.as_ref().map_or("", |s| s)
                        );

                        self.write_js_expr_value(w, coll_expr, default_state_map, Some(forvar_prefix), Some(forvar_default), default_scope)?;
                        writeln!(w, ").map(__{});", block_id)?;
                    }
                }
            }

            Ok(())
        }

        #[inline]
        #[allow(dead_code)]
        pub fn write_js_incdom_component(&self,
                                           w: &mut fmt::Write,
                                           component_ty: &'input str,
                                           ops: Iter<ElementOp>,
                                           default_state_map: &DefaultStateMap,
                                           var_prefix: Option<&str>,
                                           default_var: Option<&str>,
                                           key_prefix: Option<&str>,
                                           default_scope: Option<&str>,
                                           comp_map: &ComponentMap<'input>)
                                         -> fmt::Result {

            writeln!(w, "  function component_{}(key_prefix, store, props) {{", component_ty)?;
            self.write_js_incdom_ops_content(w,
                                                ops,
                                                default_state_map,
                                                var_prefix,
                                                default_var,
                                                key_prefix,
                                                default_scope,
                                                Some("key_prefix"),
                                                comp_map)?;
            writeln!(w, "  }};")?;
            writeln!(w, "")?;
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

        #[inline]
        fn peek_var_ty(expr: &ExprValue) -> Option<VarType> {
            match *expr {
                ExprValue::LiteralNumber(..) => { return Some(VarType::Primitive(PrimitiveVarType::Number)); },
                ExprValue::LiteralString(..) => { return Some(VarType::Primitive(PrimitiveVarType::StringVar)); },
                ExprValue::LiteralArray(Some(ref items)) => {
                    if !items.is_empty() {
                        if let Some(ref first_item) = items.get(0) {
                            if let Some(var_ty) = Self::peek_var_ty(first_item) {
                                return Some(VarType::ArrayVar(Some(Box::new(var_ty))));
                            }
                            return Some(VarType::ArrayVar(None));
                        };
                    };
                    return Some(VarType::ArrayVar(None));
                }
                _ => {}
            };
            None
        }

        pub fn collect_js_store_default_scope(&self,
                                              reducer_key_data: &mut HashMap<&'input str,
                                                                             ReducerKeyData>,
                                              default_state_map: &mut DefaultStateMap<'input>,
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

                            let var_ty = Self::peek_var_ty(expr);

                            if var_ty.is_some() {
                                default_state_map.entry(var_name)
                                    .or_insert_with(|| (var_ty.clone(), Some(expr.clone())));
                            }
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
                                self.write_js_expr_value(&mut state_expr_str,
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
                    self.write_js_expr_value(w, default_expr, default_state_map, None, None, None)?;
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
        fn process_expr(&self,
                                expr: &'input ExprValue,
                                ops_vec: &'input mut OpsVec,
                                events_vec: &'input mut EventsVec,
                                comp_map: &'input ComponentMap<'input>,
                                var_prefix: Option<&str>,
                                default_var: Option<&str>,
                                default_scope: Option<&str>)
                                -> fmt::Result {
                match expr {
                    &ExprValue::Expr(ExprOp::Add, box ExprValue::ContentNode(ref l), box ExprValue::ContentNode(ref r)) => {
                        self.process_content_node(
                            l,
                            ops_vec,
                            events_vec,
                            comp_map,
                            var_prefix,
                            default_var,
                            default_scope)?;

                        self.process_content_node(
                            r,
                            ops_vec,
                            events_vec,
                            comp_map,
                            var_prefix,
                            default_var,
                            default_scope)?;
                    }

                    &ExprValue::Expr(ExprOp::Add, box ExprValue::ContentNode(ref l), box ref r) => {
                        self.process_content_node(
                            l,
                            ops_vec,
                            events_vec,
                            comp_map,
                            var_prefix,
                            default_var,
                            default_scope)?;

                        self.process_expr(
                            r,
                            ops_vec,
                            events_vec,
                            comp_map,
                            var_prefix,
                            default_var,
                            default_scope)?;
                    }

                    &ExprValue::Expr(ExprOp::Add, box ref l, box ExprValue::ContentNode(ref r)) => {
                        self.process_expr(
                            l,
                            ops_vec,
                            events_vec,
                            comp_map,
                            var_prefix,
                            default_var,
                            default_scope)?;

                        self.process_content_node(
                            r,
                            ops_vec,
                            events_vec,
                            comp_map,
                            var_prefix,
                            default_var,
                            default_scope)?;
                    }

                    &ExprValue::Expr(ref op, ref l, ref r) => {
                        // Write left expression
                        self.process_expr(l, ops_vec, events_vec, comp_map, default_var, var_prefix, default_scope)?;

                        // Write operator
                        let expr_str = match op {
                            &ExprOp::Add => "+",
                            &ExprOp::Sub => "-",
                            &ExprOp::Mul => "*",
                            &ExprOp::Div => "/"
                        };
                        //self.write_computed_expr_value(&mut expr_str, op, var_prefix, default_var)?;
                        ops_vec.push(ElementOp::WriteValue(ExprValue::LiteralString(String::from(expr_str)), Some(allocate_element_key())));

                        // Write right expression
                        self.process_expr(r, ops_vec, events_vec, comp_map, default_var, var_prefix, default_scope)?;
                    }

                    &ExprValue::ContentNode(ref node) => {
                        self.process_content_node(
                            node,
                            ops_vec,
                            events_vec,
                            comp_map,
                            var_prefix,
                            default_var,
                            default_scope)?;
                    }

                    _ => {
                        ops_vec.push(ElementOp::WriteValue(expr.clone(), Some(allocate_element_key())));
                        
                    }
                };
                //ops_vec.push(ElementOp::WriteValue(expr.clone(), Some(allocate_element_key())));
                Ok(())
        }

        #[inline]
        fn process_content_node(&self,
                                node: &'input ContentNodeType,
                                ops_vec: &'input mut OpsVec,
                                events_vec: &'input mut EventsVec,
                                comp_map: &'input ComponentMap<'input>,
                                var_prefix: Option<&str>,
                                default_var: Option<&str>,
                                default_scope: Option<&str>)
                                -> fmt::Result {
            match node {
                &ContentNodeType::ElementNode(ref element_data) => {
                    let element_tag = element_data.element_ty.to_lowercase();
                    let element_key =
                        element_data.element_key.as_ref().map_or(String::from(""), Clone::clone);

                    let attrs = element_data.attrs.as_ref().map(Clone::clone);
                    let lens = element_data.lens.as_ref().map(Clone::clone);

                    let events = element_data.events
                        .as_ref()
                        .map(|attrs| attrs.iter().map(Clone::clone).collect());

                    // Try to locate a matching component
                    let comp = comp_map.get(element_data.element_ty.as_str());
                    if let Some(..) = comp {

                        // Render a component during render
                        ops_vec.push(ElementOp::InstanceComponent(element_tag,
                                                                  Some(element_key),
                                                                  attrs,
                                                                  lens));

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
                                                 action_ops,
                                                 None));
                            }
                        }

                        if let Some(ref children) = element_data.children {
                            // Push element open
                            ops_vec.push(ElementOp::ElementOpen(element_tag.clone(),
                                                                Some(element_key),
                                                                attrs,
                                                                events));

                            // Iterate over children
                            for ref child in children {
                                self.process_content_node(
                                    child,
                                    ops_vec,
                                    events_vec,
                                    comp_map,
                                    var_prefix,
                                    default_var,
                                    default_scope)?;
                            }

                            // Push element close
                            ops_vec.push(ElementOp::ElementClose(element_tag.clone()));
                        } else {
                            ops_vec.push(ElementOp::ElementVoid(element_tag.clone(),
                                                                Some(element_key),
                                                                attrs,
                                                                events));
                        }
                    }
                }
                &ContentNodeType::ExpressionValueNode(ref expr) => {
                    self.process_expr(expr, ops_vec, events_vec, comp_map, var_prefix, default_var, default_scope)?;
                }
                &ContentNodeType::ForNode(ref ele, ref coll_expr, ref nodes) => {
                    let block_id = allocate_element_key().replace("-", "_");
                    ops_vec.push(ElementOp::StartBlock(block_id.clone()));

                    let forvar_default = &format!("__forvar_{}", block_id);
                    let forvar_prefix = &format!("__forvar_{}{}", block_id,
                        ele.as_ref().map_or("", |s| s)
                    );

                    if let &Some(ref nodes) = nodes {
                        for ref node in nodes {
                            self.process_content_node(
                                node,
                                ops_vec,
                                events_vec,
                                comp_map,
                                Some(forvar_prefix), // FIXME
                                Some(forvar_default), // FIXME
                                default_scope)?;
                        }
                    };

                    ops_vec.push(ElementOp::EndBlock(block_id.clone()));
                    ops_vec.push(ElementOp::MapCollection(block_id.clone(), ele.as_ref().map(Clone::clone), coll_expr.clone()));
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
                            self.process_content_node(content, &mut ops, &mut events, comp_map, None, None, None)?;
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
                             default_state_map: &mut DefaultStateMap<'input>,
                             ops_vec: &mut OpsVec,
                             events_vec: &mut EventsVec,
                             comp_map: &mut ComponentMap<'input>,
                             var_prefix: Option<&str>,
                             default_var: Option<&str>,
                             default_scope: Option<&str>)
                             -> fmt::Result {
            let mut processed_store = false;

            for ref loc in self.ast.children.iter() {
                match &loc.inner {
                    &NodeType::StoreNode(ref scope_nodes) => {
                        // TODO: Allow more than one store?
                        if !processed_store {
                            self.collect_js_store_default_scope(reducer_key_data, default_state_map, scope_nodes)?;
                            processed_store = true;

                        }
                    }
                    &NodeType::ComponentDefinitionNode(ref component_data) => {
                        self.process_component_definition(component_data,
                                                          reducer_key_data,
                                                          comp_map)?;
                    }
                    &NodeType::ContentNode(ref content) => {
                        self.process_content_node(content, ops_vec, events_vec, comp_map, var_prefix, default_var, default_scope)?;
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
            // Document processing state
            let mut reducer_key_data: ReducerKeyMap = Default::default();
            let mut default_state_map: DefaultStateMap = Default::default();
            let mut comp_map: ComponentMap = Default::default();
            let mut ops_vec: OpsVec = Default::default();
            let mut events_vec: EventsVec = Default::default();
            let mut keys_vec: Vec<String> = Default::default();

            // Process document nodes and populate processing state
            self.process_nodes(&mut reducer_key_data,
                               &mut default_state_map,
                               &mut ops_vec,
                               &mut events_vec,
                               &mut comp_map,
                               None,
                               None,
                               None)?;

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

            self.write_html_ops_content(w,
                                        ops_vec.iter(),
                                        &mut events_vec,
                                        &comp_map,
                                        &mut keys_vec,
                                        None,
                                        default_scope)?;

            write!(w, "{}", indoc!(r#"
                    </div>
                    <script>
                      (function() {
            "#))?;

            // Define components
            for (ref component_ty, ref comp_def) in comp_map.iter() {
                if let Some(ref ops) = comp_def.ops {
                    self.write_js_incdom_component(
                        w,
                        component_ty,
                        ops.iter(),
                        &default_state_map,
                        Some("store.getState()."),
                        Some("store.getState()"),
                        None,
                        default_scope,
                        &comp_map)?;
                };
            }

            writeln!(w, "/* {:?} */", default_state_map)?;
            writeln!(w, "/* {:?} */", ops_vec)?;

            writeln!(w, "function render(store) {{")?;

            // Render content nodes as incdom calls
            self.write_js_incdom_ops_content(w,
                                             ops_vec.iter(),
                                             &default_state_map,
                                             Some("store.getState()."),
                                             Some("store.getState()"),
                                             None,
                                             default_scope,
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
            self.write_js_store(w, &reducer_key_data, &default_state_map)?;

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
