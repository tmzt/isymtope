
use std::clone::Clone;

use parser::ast::*;
use parser::store::*;
use parser::api::*;
use parser::util::allocate_element_key;
use processing::structs::*;
use output::scope::*;
use output::client_ops_writer::*;


pub struct ProcessDocument<'input> {
    ast: &'input Template,
    root_block: BlockProcessingState,
    processing: DocumentProcessingState,
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

        ProcessDocument {
            ast: ast,
            root_block: root_block,
            processing: processing
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
                        let sym_ref: SymbolRefType = Some(SymbolReferenceType::ReducerKeyReference(var_name.to_owned()));
                        self.processing.default_state_symbol = Some((sym_ref, var_ty.as_ref().map(Clone::clone)));
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
                        action.state_ty = Self::peek_var_ty(expr);

                        // Create a new expression and processing scope for the expression
                        let mut expr_scope: ExprScopeProcessingState = Default::default();
                        let processing_scope: ProcessingScope =
                            match self.processing.default_state_symbol {
                                Some(ref sym) => {
                                    let ty = &sym.1;
                                    let sym_ref = SymbolReferenceType::ActionStateReference(ty.as_ref().map(Clone::clone));
                                    (None, None, Some((Some(sym_ref), ty.as_ref().map(Clone::clone))))
                                }
                                _ => (None, None, None)
                            };
         
                        // process_expr(expr, &mut action_block, &self.processing, &processing_scope)?;

                        let action_expr = map_expr_using_scope(expr, &self.processing, &mut expr_scope, &processing_scope);
                        action.state_expr = Some(ActionStateExprType::SimpleReducerKeyExpr(action_expr));
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
                                          nodes: &'input Vec<DefaultScopeNodeType>)
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
                        let sym_ref: SymbolRefType = Some(SymbolReferenceType::ReducerKeyReference(var_name.to_owned()));
                        self.processing.default_state_symbol = Some((sym_ref, var_ty.as_ref().map(Clone::clone)));
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

        if let Some(ref children) = component_data.children {
            for ref child in children {
                match *child {
                    &NodeType::ContentNode(ref content) => {
                        process_content_node(content, &self.processing, &mut block)?;
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
                        self.collect_js_store_default_scope(scope_nodes)?;
                        processed_store = true;
                    }
                }
                &NodeType::ComponentDefinitionNode(ref component_data) => {
                    self.process_component_definition(component_data)?;
                }
                &NodeType::ContentNode(ref content) => {
                    process_content_node(content, &self.processing, block)?;
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
pub fn resolve_symbol(expr_scope: &ExprScopeProcessingState, given: &str) -> Option<Symbol> {
    if let Some(sym) = expr_scope.symbol_map.get(given) {
        match sym {
            &(Some(_), _) => { return Some(sym.clone()) },
            _ => {}
        };
    };
    None
}

#[inline]
pub fn map_expr_using_scope<'input>(expr: &'input ExprValue,
                processing: &DocumentProcessingState,
                expr_scope: &mut ExprScopeProcessingState,
                processing_scope: &ProcessingScope)
                -> ExprValue {
    match expr {
        &ExprValue::ContentNode(ref content) => {
            // Pass content through
            ExprValue::ContentNode(content.clone())
        }

        &ExprValue::Expr(ref op, ref l, ref r) => {

            let left_expr = Box::new(map_expr_using_scope(l, processing, expr_scope, processing_scope));
            let right_expr = Box::new(map_expr_using_scope(r, processing, expr_scope, processing_scope));

            ExprValue::Expr(op.clone(), left_expr, right_expr)
        }

        &ExprValue::VariableReference(ref given) => {
            let as_reducer_key = processing_scope.0.as_ref()
                .map(|s| format!("{}.{}", s, given))
                .unwrap_or("".to_owned());

            // Try to resolve the symbol in the scope, including parameters and loop_vars
            if let Some(ref sym) = resolve_symbol(expr_scope, given) {
                return ExprValue::SymbolReference(sym.clone());
            };

            // Try to resolve and cache the symbol as a reducer key reference
            if let Some(ref reducer_data) = processing.reducer_key_data.get(&as_reducer_key) {
                let ty = &reducer_data.ty;
                expr_scope.symbol_map.insert(given.to_owned(), (
                    Some(SymbolReferenceType::ReducerKeyReference(as_reducer_key.to_owned())),
                    ty.as_ref().map(Clone::clone)
                ));
                return ExprValue::SymbolReference((Some(SymbolReferenceType::ReducerKeyReference(as_reducer_key)), ty.as_ref().map(Clone::clone)));
            };

            // Default to local variable
            ExprValue::SymbolReference((Some(SymbolReferenceType::LocalVarReference(given.clone())), None))
        }

        &ExprValue::DefaultVariableReference => {

            // If we have a valid default var in the scope, expand the DefaultVariableReference into a symbol reference
            if let Some(ref sym) = processing_scope.2 {
                return ExprValue::SymbolReference(sym.clone())
            };
            ExprValue::DefaultVariableReference
        }

        _ => expr.clone()
    }
}

#[inline]
pub fn process_content_node<'input>(
                        node: &'input ContentNodeType,
                        processing: &DocumentProcessingState,
                        block: &mut BlockProcessingState)
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

                if let Some(ref children) = element_data.children {
                    // Push element open
                    block.ops_vec.push(ElementOp::ElementOpen(element_tag.clone(),
                                                        Some(element_key),
                                                        attrs,
                                                        events));

                    // Iterate over children
                    for ref child in children {
                        process_content_node(child, processing, block)?;
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
            // FIXME
            let processing_scope: ProcessingScope = Default::default();
            let expr = map_expr_using_scope(expr, processing, &mut block.expr_scope, &processing_scope);
            block.ops_vec.push(ElementOp::WriteValue(expr, Some(allocate_element_key())));
        }
        &ContentNodeType::ForNode(ref ele, ref coll_expr, ref nodes) => {
            let block_id = allocate_element_key().replace("-", "_");
            block.ops_vec.push(ElementOp::StartBlock(block_id.clone()));

            // Add forvar as a parameter in the symbol map
            if let &Some(ref ele_key) = ele {
                block.expr_scope.symbol_map.insert(ele_key.to_owned(), (
                    Some(SymbolReferenceType::LoopVarReference(ele_key.to_owned())),
                    None
                ));
            }

            if let &Some(ref nodes) = nodes {
                for ref node in nodes {
                    // FIXME: forvar resolve
                    process_content_node(node, processing, block)?;
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
