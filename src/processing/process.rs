
use std::clone::Clone;

use parser::ast::*;
use parser::store::*;
use parser::api::*;
use parser::util::allocate_element_key;
use processing::structs::*;
use processing::scope::*;
use output::scope::*;


pub struct ProcessDocument<'input> {
    ast: &'input Template,
    root_block: BlockProcessingState,
    processing: DocumentProcessingState,
    scope: DocumentProcessingScope
}

impl<'inp> Into<DocumentState<'inp>> for ProcessDocument<'inp> {
    fn into(self) -> DocumentState<'inp> {
        DocumentState {
            ast: self.ast,
            root_block: self.root_block,
            comp_map: self.processing.comp_map,
            reducer_key_data: self.processing.reducer_key_data,
            default_state_map: self.processing.default_state_map,
            default_state_symbol: self.processing.default_state_symbol,
            default_reducer_key: self.processing.default_reducer_key
        }
    }
}

impl<'input> ProcessDocument<'input> {
    pub fn from_template<'inp>(ast: &'inp Template) -> ProcessDocument<'inp> {
        let processing = DocumentProcessingState::default();
        let root_block = BlockProcessingState::default();
        let scope = DocumentProcessingScope::default();

        ProcessDocument {
            ast: ast,
            root_block: root_block,
            processing: processing,
            scope: scope
        }
    }

    pub fn collect_js_store_child_scope(&mut self,
                                        reducer_key: &'input str,
                                        nodes: &'input Vec<ScopeNodeType>,
                                        reducer_key_prefix: Option<&str>)
                                        -> DocumentProcessingResult<()> {

        for ref node in nodes {
            match *node {
                &ScopeNodeType::LetNode(ref var_name, ref expr) => {
                    let has_default_sym = self.processing.default_state_symbol.is_some();
                    let has_default_reducer_key = self.processing.default_reducer_key.is_some();
                    let has_expr = expr.is_some();

                    let var_ty = expr.as_ref().and_then(|expr| Self::peek_var_ty(expr));

                    if !has_default_sym {
                        let mut sym = Symbol::reducer_key_with(&var_name, var_ty.as_ref(), expr.as_ref());
                        // TODO: Type
                        // sym.1 = var_ty.as_ref().map(Clone::clone);
                        self.processing.default_state_symbol = Some(sym);
                        // self.processing.default_state_symbol = Some(sym, var_ty.as_ref().map(Clone::clone));
                        // self.processing.default_state_symbol = Some(sym, var_ty.as_ref().map(Clone::clone)));
                    }

                    if !has_default_reducer_key {
                        self.processing.default_reducer_key = Some(var_name.to_owned());
                    }

                    let reducer_entry = self.processing.reducer_key_data.entry(var_name.to_owned())
                        .or_insert_with(|| ReducerKeyData::from_name(&format!("{}", var_name), var_ty.as_ref().map(Clone::clone)));

                    if let &Some(ref expr) = expr {
                        reducer_entry.default_expr = Some(expr.clone());

                        self.processing.default_state_map.entry(var_name.to_owned())
                            .or_insert_with(|| {
                                let var_ty = Self::peek_var_ty(expr);
                                (var_ty, Some(expr.clone()))
                            });
                    };
                }
                &ScopeNodeType::ActionNode(ref action_name, ref simple_expr) => {
                    // let reducer_entry = self.processing.reducer_key_data.entry(reducer_key.to_owned())
                    //     .or_insert_with(|| ReducerKeyData::from_name(&format!("{}", reducer_key)));

                    let action_path = format!("{}{}",
                                              reducer_key_prefix.and_then(|prefix| {
                                                      Some(format!("{}", prefix.to_uppercase()))
                                                  })
                                                  .and_then(|prefix| {
                                                      Some(format!("{}.", prefix.to_uppercase()))
                                                  })
                                                  .unwrap_or_default(),
                                              action_name);

                    let mut action = ReducerActionData::from_name(&action_path, Some(reducer_key));
                    if let &Some(ref simple_expr) = simple_expr {
                        let ActionStateExprType::SimpleReducerKeyExpr(ref expr) = *simple_expr;

                        // let mut processing_scope: ProcessingScope = ProcessingScope::default();

                        // let action_ty = self.default_state_symbol.as_ref()
                        //     .map(|sym| sym.ty().clone())
                        //     .or_else(|| Self::peek_var_ty(expr));

                        // // let ty = Self::peek_var_ty(expr);
                        // action.state_ty = action_ty.clone();
                        // // action.state_ty = Self::peek_var_ty(expr);

                        if let Some(ref sym) = self.processing.default_state_symbol {
                            // processing_scope.2 = Some(sym.to_owned());
                            action.state_ty = sym.ty().map(|s| s.clone());
                        };

                        // Create a new expression and processing scope for the expression
                        // // let mut expr_scope: ExprScopeProcessingState = Default::default();
                        // let processing_scope: ProcessingScope =
                        //     match self.processing.default_state_symbol {
                        //         Some(ref sym) => {
                        //             let sym = Symbol::action_state(sym.ty().or_else(|| ty.as_ref()));
                        //             (None, None, Some(sym.clone()))
                        //         }
                        //         _ => (None, None, None)
                        //     };
         
                        // process_expr(expr, &mut action_block, &self.processing, &processing_scope)?;

                        let resolution_mode = BareSymbolResolutionMode::PropThenReducerKey;
                        let action_expr = map_expr_using_scope(expr, &self.processing, &mut self.scope, &resolution_mode);

                        let typed_expr = map_expr(&action_expr, &|node| match node {
                            &ExprValue::DefaultVariableReference => {
                                let sym = Symbol::action_state(action.state_ty.as_ref());
                                ExprValue::SymbolReference(sym)
                            },
                            _ => node.clone()
                        });

                        action.state_expr = Some(ActionStateExprType::SimpleReducerKeyExpr(typed_expr));
                    };
                    let reducer_entry = self.processing.reducer_key_data.entry(reducer_key.to_owned())
                        .or_insert_with(|| ReducerKeyData::from_name(&format!("{}", reducer_key), None));

                    if let Some(ref mut actions) = reducer_entry.actions {
                        actions.push(action);
                    };
                }
                &ScopeNodeType::ScopeNode(ref scope_name, ref scope_nodes) => {
                    self.collect_js_store_child_scope(scope_name,
                                                      scope_nodes,
                                                      reducer_key_prefix)?;
                }
                _ => {}
            }
        }
        Ok(())
    }

    #[allow(unused_variables)]
    pub fn collect_js_store_api_scope(&mut self,
                                      scope_name: &'input str,
                                      nodes: &'input Vec<ApiNodeType>)
                                      -> DocumentProcessingResult<()> {
        for ref node in nodes {
            match *node {
                &ApiNodeType::ResourceNode(ref resource_data) => {
                    let reducer_name: &'input str = &resource_data.resource_name;

                    self.processing.reducer_key_data.entry(scope_name.to_owned())
                        .or_insert_with(|| ReducerKeyData::from_name(&format!("{}", scope_name), None));
                }
                _ => {}
            }
        }
        Ok(())
    }

    #[inline]
    fn peek_var_ty(expr: &ExprValue) -> Option<VarType> {
        match *expr {
            ExprValue::LiteralNumber(..) => {
                return Some(VarType::Primitive(PrimitiveVarType::Number));
            }
            ExprValue::LiteralString(..) => {
                return Some(VarType::Primitive(PrimitiveVarType::StringVar));
            }
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

    pub fn collect_js_store_default_scope(&mut self,
                                          nodes: &'input Vec<DefaultScopeNodeType>,
                                          resolution_mode: &BareSymbolResolutionMode)
                                          -> DocumentProcessingResult<()> {
        for ref node in nodes {
            match *node {
                &DefaultScopeNodeType::LetNode(ref var_name, ref expr) => {
                    // Within the default scope let defines a new scope and it's default expression

                    let has_default_sym = self.processing.default_state_symbol.is_some();
                    let has_default_reducer_key = self.processing.default_reducer_key.is_some();
                    let has_expr = expr.is_some();

                    let var_ty = expr.as_ref().and_then(|expr| Self::peek_var_ty(expr));

                    if !has_default_sym {
                        let sym = Symbol::reducer_key_with_ty(var_name, var_ty.as_ref());
                        // TODO: Restore type
                        self.processing.default_state_symbol = Some(sym);
                        // self.processing.default_state_symbol = Some(Symbol::reducer_key(var_name));
                    }

                    if !has_default_reducer_key {
                        self.processing.default_reducer_key = Some(var_name.to_owned());
                    }

                    let reducer_entry = self.processing.reducer_key_data.entry(var_name.to_owned())
                        .or_insert_with(|| ReducerKeyData::from_name(&format!("{}", var_name), var_ty.as_ref().map(Clone::clone)));

                    if let &Some(ref expr) = expr {
                        reducer_entry.default_expr = Some(expr.clone());

                        self.processing.default_state_map.entry(var_name.to_owned())
                            .or_insert_with(|| {
                                let var_ty = Self::peek_var_ty(expr);
                                (var_ty, Some(expr.clone()))
                            });
                    };
                }

                &DefaultScopeNodeType::ApiRootNode(ref scope_name, ref api_nodes) => {
                    if let &Some(ref api_nodes) = api_nodes {
                        self.collect_js_store_api_scope(scope_name, api_nodes)?;
                    }
                }
                &DefaultScopeNodeType::ScopeNode(ref scope_name, ref scope_nodes) => {
                    self.collect_js_store_child_scope(scope_name, scope_nodes, None)?;
                }
            }
        }
        Ok(())
    }

    #[allow(dead_code)]
    pub fn process_component_definition(&mut self,
                                        component_data: &'input ComponentDefinitionType)
                                        -> DocumentProcessingResult<()> {
        let name: &'input str = component_data.name.as_str();
        let mut block = BlockProcessingState::default();

        if let Some(ref inputs) = component_data.inputs {
            for input in inputs {
                let prop_ref = Symbol::unresolved(input);

                block.scope.props.insert(input.to_owned(), prop_ref);
            }
        };

        if let Some(ref children) = component_data.children {
            for ref child in children {
                match *child {
                    &NodeType::ContentNode(ref content) => {
                        let mode = BareSymbolResolutionMode::PropThenReducerKey;
                        process_content_node(content, &self.processing, &mut block, &mode)?;
                    }
                    _ => {}
                }
            }
        }

        let comp = Component {
            name: name.to_owned(),
            ops: Some(block.ops_vec),
            uses: None,
            child_map: Default::default(),
            symbol_map: block.scope.symbol_map.clone(),
            props: block.scope.props.clone()
        };

        self.processing.comp_map.insert(name.to_owned(), comp);

        Ok(())
    }

    pub fn process_nodes(&mut self,
                         scope_prefixes: &ScopePrefixes,
                         block: &mut BlockProcessingState)
                         -> Result {
        let mut processed_store = false;

        for ref loc in self.ast.children.iter() {
            match &loc.inner {
                &NodeType::StoreNode(ref scope_nodes) => {
                    // TODO: Allow more than one store?
                    if !processed_store {
                        let mode = BareSymbolResolutionMode::ReducerKeyThenProp;
                        self.collect_js_store_default_scope(scope_nodes, &mode)?;
                        processed_store = true;
                    }
                }
                &NodeType::ComponentDefinitionNode(ref component_data) => {
                    self.process_component_definition(component_data)?;
                }
                &NodeType::ContentNode(ref content) => {
                    let mode = BareSymbolResolutionMode::PropThenReducerKey;
                    process_content_node(content, &self.processing, block, &mode)?;
                }
                _ => {}
            }
        }
        Ok(())
    }

    #[allow(unused_variables)]
    pub fn process_document(&mut self) -> DocumentProcessingResult<()> {
        let mut root_block = BlockProcessingState::default();
        let base_scope: ScopePrefixes = Default::default();

        self.process_nodes(&base_scope, &mut root_block)?;
        self.root_block = root_block;
        Ok(())
    }
}

#[derive(Debug, Clone)]
#[allow(dead_code)]
pub enum BareSymbolResolutionMode {
    ReducerKeyThenProp,
    PropThenReducerKey
}

#[inline]
#[allow(dead_code)]
pub fn resolve_reducer_key(processing: &DocumentProcessingState, scope: &mut DocumentProcessingScope, reducer_key: &str) -> Option<Symbol> {
    // Try to resolve the symbol in the scope, including parameters and loop_vars
    // and previously cached resolutions
    // FIXME: Do we need to split these by type?
    // let cached = expr_scope.symbol_map.get(given).map(|s| s.clone());
    // if cached.is_some() { return cached; }

    // let as_reducer_key = processing_scope.0.as_ref()
    //     .map(|s| format!("{}.{}", s, reducer_key))
    //     .unwrap_or(reducer_key.to_owned());

    // Try to resolve and cache the symbol as a reducer key reference
    if let Some(reducer_data) = processing.reducer_key_data.get(reducer_key) {
        if let Some(ref default_expr) = reducer_data.default_expr {
            scope.add_cached_reducer_key_with_value(reducer_key, default_expr);
            return Some(Symbol::reducer_key_with_value(reducer_key, default_expr));
        };
        
        return Some(Symbol::reducer_key(reducer_key));
    };

    None
}

#[inline]
#[allow(dead_code)]
pub fn resolve_prop(processing: &DocumentProcessingState, scope: &mut DocumentProcessingScope, prop_key: &str) -> Option<Symbol> {
    // FIXME: This is causing us to resolve a reducer key instead.
    // let cached = expr_scope.symbol_map.get(prop_key).map(|s| s.clone());
    // if cached.is_some() { return cached; }

    // Collect unresolved bare symbols as props on the scope
    let prop = scope.props.entry(prop_key.to_owned())
        .or_insert_with(|| Symbol::prop(prop_key));

    // Replace the existing symbol in this map
    // TODO: Remove this hack
    // prop.0 = Some(SymbolReferenceType::PropReference(prop_key.to_owned()));
    // prop.1 = None;

    Some(prop.clone())

    // expr_scope.symbol_map.insert(
    //     prop_key.to_owned(),
    //     (Some(SymbolReferenceType::PropReference(prop_key.to_owned())), None)
    // );

    // None
}

// #[inline]
// #[allow(dead_code)]
// pub fn resolve_existing_symbol(processing: &DocumentProcessingState, scope: &mut DocumentProcessingScope, given: &str, resolution_mode: &BareSymbolResolutionMode) -> Option<Symbol> {
//     // FIXME: Split the cache types and re-enable
//     // let cached = expr_scope.symbol_map.get(given).map(|s| s.clone());
//     // if cached.is_some() { return cached; }

//     match resolution_mode {
//         &BareSymbolResolutionMode::ReducerKeyThenProp => {
//             let sym = resolve_reducer_key(processing, scope, given);
//             if sym.is_some() { return sym; }

//             // FIXME: This overwrites the correct reducer_key entry due to the hack
//             // let sym = resolve_prop(processing, expr_scope, processing_scope, given);
//             // if sym.is_some() { return sym; }
//         }

//         &BareSymbolResolutionMode::PropThenReducerKey => {
//             let sym = resolve_prop(processing, scope, given);
//             if sym.is_some() { return sym; }

//             let sym = resolve_reducer_key(processing, scope, given);
//             if sym.is_some() { return sym; }
//         }
//     };
    
//     None
// }

#[inline]
pub fn map_lens_using_scope<'input>(lens: Option<&LensExprType>,
                processing: &DocumentProcessingState,
                scope: &mut DocumentProcessingScope)
                -> Option<LensExprType> {
    match lens {
        Some(&LensExprType::ForLens(ref ele_key, ref coll_sym)) => {
            let ele_key = ele_key.as_ref().map(|s| s.clone());
            if let Some(resolved) = resolve_sym(coll_sym, processing, scope) {
                return Some(LensExprType::ForLens(ele_key, resolved))
            };
        }
        Some(&LensExprType::GetLens(ref sym)) => {
            if let Some(resolved) = resolve_sym(sym, processing, scope) {
                return Some(LensExprType::GetLens(resolved));
            };

            // None
            // let resolution_mode = BareSymbolResolutionMode::PropThenReducerKey;

            // // Resolve variable as reducer key reference first
            // // let sym = resolve_existing_symbol()

            // let mut lens_scope = scope.clone();
            // if let &ExprValue::VariableReference(ref prop_key) = lens_expr {
            //     lens_scope.with_prop(prop_key, None, None);
            // };

            // let expr = map_expr_using_scope(lens_expr, processing, &mut lens_scope, &resolution_mode);
            // Some(LensExprType::GetLens(expr))
        }
        _ => {}
    };

    None
}

#[inline]
#[allow(dead_code)]
pub fn map_expr<'input, F: Fn(&ExprValue) -> ExprValue>(expr: &'input ExprValue, f: &F) -> ExprValue {
    match expr {
        &ExprValue::Expr(ref op, ref l, ref r) => {
            let l_val = map_expr(l, f);
            let r_val = map_expr(r, f);
            ExprValue::Expr(op.clone(), Box::new(l_val), Box::new(r_val))
        }

        _ => {
            f(expr)
        }
    }
}

#[inline]
#[allow(dead_code)]
pub fn resolve_sym(sym: &Symbol, processing: &DocumentProcessingState, scope: &mut DocumentProcessingScope) -> Option<Symbol> {
    if let &SymbolReferenceType::UnresolvedReference(ref key) = sym.sym_ref() {
        if let Some(_) = scope.params.get(key) {
            return Some(Symbol::param(key));
        };

        if let Some(_) = scope.props.get(key) {
            return Some(Symbol::prop(key));
        };

        if let Some(_) = resolve_reducer_key(processing, scope, key) {
            return Some(Symbol::reducer_key(key));
        };

        if let Some(_) = scope.block_params.get(key) {
            return Some(Symbol::block_param(key));
        };
    }

    return None;
}

#[inline]
#[allow(dead_code)]
pub fn map_expr_using_scope<'input>(expr: &'input ExprValue,
                processing: &DocumentProcessingState,
                scope: &mut DocumentProcessingScope,
                resolution_mode: &BareSymbolResolutionMode)
                -> ExprValue {
    match expr {
        &ExprValue::Expr(ref op, ref l, ref r) => {
            let l_vars = map_expr_using_scope(l, processing, scope, resolution_mode);
            let r_vars = map_expr_using_scope(r, processing, scope, resolution_mode);

            let left_expr = Box::new(l_vars);
            let right_expr = Box::new(r_vars);

            ExprValue::Expr(op.clone(), left_expr, right_expr)
        }

        &ExprValue::SymbolReference(ref sym) => {
            if let Some(sym) = resolve_sym(sym, processing, scope) {
                return ExprValue::SymbolReference(sym);
            };

            expr.clone()
        }

        &ExprValue::DefaultVariableReference => {
            // NOTE: This is currently used primarily for action expressions

            // If we have a valid default var in the scope, expand the DefaultVariableReference into a symbol reference
            // if let Some(ref sym) = (scope.0).2 {
            //     return ExprValue::SymbolReference(sym.clone());
            // };

            ExprValue::DefaultVariableReference
        }

        _ => expr.clone()
    }
}

#[inline]
pub fn process_content_node<'input>(
                        node: &'input ContentNodeType,
                        processing: &DocumentProcessingState,
                        block: &mut BlockProcessingState,
                        resolution_mode: &BareSymbolResolutionMode)
                        -> DocumentProcessingResult<()> {

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
            if let Some(..) = processing.comp_map.get(element_data.element_ty.as_str()) {

                // Attempt to map lens values
                // FIXME
                let mut scope = DocumentProcessingScope::default();
                let lens = map_lens_using_scope(lens.as_ref(), processing, &mut block.scope);

                let attrs = match lens {
                    Some(LensExprType::GetLens(ref sym)) => {
                        let mut attrs = attrs.as_ref().map_or_else(|| Default::default(), |s| s.clone());

                        if let &SymbolReferenceType::UnresolvedReference(ref key) = sym.sym_ref() {
                            if let Some(ref sym) = resolve_reducer_key(processing, &mut scope, key) {
                                let value = Some(ExprValue::SymbolReference(sym.clone()));
                                attrs.push((key.clone(), value));
                            };
                        };

                        Some(attrs)
                    }

                    Some(LensExprType::ForLens(ref ele_key, ref coll_sym)) => {
                        let mut attrs = attrs.as_ref().map_or_else(|| Default::default(), |s| s.clone());

                        // let resolved = resolve_sym(coll_sym, processing, &mut scope);
                        // let coll_expr = ExprValue::SymbolReference(coll_sym.clone());
                        // let coll_expr = map_expr_using_scope(&coll_expr, processing, &mut block.scope, resolution_mode);
                        // let ele_sym = Symbol::unresolved(ele_key.to_owned());

                        // if let &SymbolReferenceType::UnresolvedReference(ref key) = coll_sym.sym_ref() {
                        //     if let Some(ref sym) = resolve_reducer_key(processing, &mut scope, key) {
                        //         let value = Some(ExprValue::SymbolReference(sym.clone()));
                        //         attrs.push((key.clone(), value));
                        //     };
                        // };

                        if let &Some(ref ele_key) = ele_key {
                            let sym = Symbol::prop(ele_key);
                            let value = Some(ExprValue::SymbolReference(sym.clone()));
                            attrs.push((ele_key.clone(), value));
                        };

                        Some(attrs)
                    }

                    _ => attrs
                };

                // Render a component during render
                block.ops_vec.push(ElementOp::InstanceComponent(element_tag,
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
                        block.events_vec.push((element_key.clone(),
                                            event_name,
                                            event_params,
                                            action_ops,
                                            None));
                    }
                }

                // This should only be Some if there are actually children
                if let Some(ref children) = element_data.children {
                    // Push element open
                    block.ops_vec.push(ElementOp::ElementOpen(element_tag.clone(),
                                                        Some(element_key),
                                                        attrs,
                                                        events));

                    // Iterate over children
                    for ref child in children {
                        process_content_node(child, processing, block, resolution_mode)?;
                    }

                    // Push element close
                    block.ops_vec.push(ElementOp::ElementClose(element_tag.clone()));
                } else {
                    block.ops_vec.push(ElementOp::ElementVoid(element_tag.clone(),
                                                        Some(element_key),
                                                        attrs,
                                                        events));
                }
            }
        }
        &ContentNodeType::ExpressionValueNode(ref expr) => {
            let expr = map_expr_using_scope(expr, processing, &mut block.scope, resolution_mode);

            block.ops_vec.push(ElementOp::WriteValue(expr, Some(allocate_element_key())));
        }
        &ContentNodeType::ForNode(ref ele, ref coll_expr, ref nodes) => {
            let block_id = allocate_element_key().replace("-", "_");
            block.ops_vec.push(ElementOp::StartBlock(block_id.clone()));

            let coll_expr = map_expr_using_scope(coll_expr, processing, &mut block.scope, resolution_mode);

            // Add forvar as a parameter in the symbol map
            if let &Some(ref ele_key) = ele {
                block.scope.add_loop_var(ele_key);
            }

            if let &Some(ref nodes) = nodes {
                for ref node in nodes {
                    // FIXME: forvar resolve
                    process_content_node(node, processing, block, resolution_mode)?;
                }
            };

            block.ops_vec.push(ElementOp::EndBlock(block_id.clone()));
            block.ops_vec.push(ElementOp::MapCollection(block_id.clone(),
                                                    ele.as_ref().map(Clone::clone),
                                                    coll_expr.clone()));
        }
    }
    (Ok(()))
}