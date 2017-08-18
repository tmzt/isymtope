
use std::clone::Clone;

use parser::ast::*;
use parser::store::*;
use parser::api::*;
use parser::util::allocate_element_key;
use processing::structs::*;
use processing::scope::*;
use processing::process_util::*;
use processing::process_content::*;


pub struct ProcessDocument<'input> {
    ast: &'input Template,
    root_block: BlockProcessingState,
    processing: DocumentProcessingState,
    scope: ElementOpScope
}

impl<'inp> Into<DocumentState<'inp>> for ProcessDocument<'inp> {
    fn into(self) -> DocumentState<'inp> {
        DocumentState {
            ast: self.ast,
            root_block: self.root_block,
            comp_map: self.processing.comp_map,
            block_map: self.processing.block_map,
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
        let scope = ElementOpScope::default();

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
                &ScopeNodeType::ActionNode(ref action_name, ref simple_expr, ref params) => {
                    let mut scope = ElementOpScope::default();
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

                        if let &Some(ref params) = params {
                            for param in params {
                                scope.1.add_action_param(param);
                            }
                        };

                        let resolution_mode = BareSymbolResolutionMode::PropThenReducerKey;
                        let action_expr = map_expr_using_scope(expr, &self.processing, &mut scope, &resolution_mode);

                        let typed_expr = map_expr(&action_expr, &|node| match node {
                            &ExprValue::DefaultVariableReference => {
                                let sym = Symbol::action_state(action.state_ty.as_ref());
                                ExprValue::SymbolReference(sym)
                            },
                            &ExprValue::SymbolReference(ref sym) => {
                                match sym.sym_ref() {
                                    &SymbolReferenceType::ResolvedReference(..) => node.clone(),
                                    &SymbolReferenceType::UnresolvedReference(ref key) => {
                                        let action_param = Symbol::action_param(key);
                                        ExprValue::SymbolReference(action_param)
                                    }
                                }
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

                block.scope.1.props.insert(input.to_owned(), prop_ref);
            }
        };

        if let Some(ref children) = component_data.children {
            for ref child in children {
                match *child {
                    &NodeType::ContentNode(ref content) => {
                        let mut scope = block.scope.clone();
                        if let Some(ref default_reducer_key) = self.processing.default_reducer_key {
                            scope.0.append_action_scope(default_reducer_key);
                        };

                        let mut content_processor = ProcessContent::new(scope);
                        let mode = BareSymbolResolutionMode::PropThenReducerKey;
                        content_processor.process_content_node(content, &self.processing, &mut block, None, &mode)?;
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
            symbol_map: block.scope.1.symbol_map.clone(),
            props: block.scope.1.props.clone(),
            events: Some(block.events_vec.clone())
        };

        self.processing.comp_map.insert(name.to_owned(), comp);

        Ok(())
    }

    pub fn process_nodes(&mut self,
                         scope_prefixes: &ScopePrefixes,
                         block: &mut BlockProcessingState)
                         -> Result {
        let mut processed_store = false;

        let mut base_scope: ElementOpScope = Default::default();

        // Process store related nodes first
        for ref loc in self.ast.children.iter() {
            match &loc.inner {
                &NodeType::StoreNode(ref scope_nodes) => {
                    // TODO: Allow more than one store?
                    if !processed_store {
                        let mode = BareSymbolResolutionMode::ReducerKeyThenProp;
                        self.collect_js_store_default_scope(scope_nodes, &mode)?;

                        // Update scope with default reducer key
                        if let Some(ref default_reducer_key) = self.processing.default_reducer_key {
                            base_scope.0.append_action_scope(default_reducer_key);
                        };

                        processed_store = true;
                    }
                }
                _ => {}
            }
        }

        for ref loc in self.ast.children.iter() {
            match &loc.inner {
                &NodeType::ComponentDefinitionNode(ref component_data) => {
                    self.process_component_definition(component_data)?;
                }
                &NodeType::ContentNode(ref content) => {
                    let mode = BareSymbolResolutionMode::PropThenReducerKey;
                    // let mut content_processor = ProcessContent::with_root_node(content);
                    let mut scope = base_scope.clone();
                    let mut content_processor = ProcessContent::new(scope);
                    // content_processor.process(&self.processing)?;
                    content_processor.process_content_node(content, &self.processing, block, None, &mode)?;
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

#[inline]
#[allow(dead_code)]
pub fn resolve_prop(processing: &DocumentProcessingState, scope: &mut ElementOpScope, prop_key: &str) -> Option<Symbol> {
    // FIXME: This is causing us to resolve a reducer key instead.
    // let cached = expr_scope.symbol_map.get(prop_key).map(|s| s.clone());
    // if cached.is_some() { return cached; }

    // Collect unresolved bare symbols as props on the scope
    let prop = scope.1.props.entry(prop_key.to_owned())
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
// pub fn resolve_existing_symbol(processing: &DocumentProcessingState, scope: &mut ElementOpScope, given: &str, resolution_mode: &BareSymbolResolutionMode) -> Option<Symbol> {
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