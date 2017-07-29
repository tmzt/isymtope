
use std::clone::Clone;

use parser::ast::*;
use parser::store::*;
use parser::api::*;
use parser::util::allocate_element_key;
use output::structs::*;
use output::scope::*;
use output::client_ops_writer::*;


pub struct ProcessDocument<'input> {
    ast: &'input Template,
    root_block: BlockProcessingState,
    processing: DocumentProcessingState<'input>,
}

impl<'inp> Into<DocumentState<'inp>> for ProcessDocument<'inp> {
    fn into(self) -> DocumentState<'inp> {
        let default_reducer_key = if self.processing.has_default_state_key { self.processing.default_state_key.get() } else { None };

        DocumentState {
            ast: self.ast,
            root_block: self.root_block,
            comp_map: self.processing.comp_map,
            reducer_key_data: self.processing.reducer_key_data,
            default_state_map: self.processing.default_state_map,
            default_reducer_key: default_reducer_key
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
                    if !self.processing.has_default_state_key {
                        self.processing.default_state_key.replace(Some(var_name));
                        self.processing.has_default_state_key = true;
                    }

                    let reducer_entry = self.processing.reducer_key_data.entry(var_name)
                        .or_insert_with(|| ReducerKeyData::from_name(&format!("{}", var_name)));

                    if let &Some(ref expr) = expr {
                        reducer_entry.default_expr = Some(expr.clone());

                        self.processing.default_state_map.entry(var_name)
                            .or_insert_with(|| {
                                let var_ty = Self::peek_var_ty(expr);
                                (var_ty, Some(expr.clone()))
                            });
                    };
                }
                &ScopeNodeType::ActionNode(ref action_name, ref simple_expr) => {
                    let reducer_entry = self.processing.reducer_key_data.entry(reducer_key)
                        .or_insert_with(|| ReducerKeyData::from_name(&format!("{}", reducer_key)));

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

                        action.state_expr = Some(simple_expr.clone());
                    };
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

                    self.processing.reducer_key_data.entry(scope_name)
                        .or_insert_with(|| ReducerKeyData::from_name(&format!("{}", scope_name)));
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
                    let reducer_entry = self.processing.reducer_key_data.entry(var_name)
                        .or_insert_with(|| ReducerKeyData::from_name(&format!("{}", var_name)));

                    if !self.processing.has_default_state_key {
                        self.processing.default_state_key.replace(Some(var_name));
                        self.processing.has_default_state_key = true;
                    };

                    if let &Some(ref expr) = expr {
                        reducer_entry.default_expr = Some(expr.clone());

                        let var_ty = Self::peek_var_ty(expr);

                        if var_ty.is_some() {
                            self.processing.default_state_map.entry(var_name)
                                .or_insert_with(|| (var_ty.clone(), Some(expr.clone())));
                        }
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

    #[inline]
    fn process_expr(&mut self,
                    expr: &'input ExprValue,
                    block: &mut BlockProcessingState)
                    -> DocumentProcessingResult<()> {
        match expr {
            &ExprValue::Expr(ExprOp::Add,
                             box ExprValue::ContentNode(ref l),
                             box ExprValue::ContentNode(ref r)) => {
                self.process_content_node(l, block)?;
                self.process_content_node(r, block)?;
            }

            &ExprValue::Expr(ExprOp::Add, box ExprValue::ContentNode(ref l), box ref r) => {
                self.process_content_node(l, block)?;
                self.process_expr(r, block)?;
            }

            &ExprValue::Expr(ExprOp::Add, box ref l, box ExprValue::ContentNode(ref r)) => {
                self.process_expr(l, block)?;
                self.process_content_node(r, block)?;
            }

            &ExprValue::Expr(ref op, ref l, ref r) => {
                // Write left expression
                self.process_expr(l, block)?;

                // Write operator
                let expr_str = match op {
                    &ExprOp::Add => "+",
                    &ExprOp::Sub => "-",
                    &ExprOp::Mul => "*",
                    &ExprOp::Div => "/",
                };
                // self.write_computed_expr_value(&mut expr_str, op, var_prefix, default_var)?;
                block.ops_vec.push(ElementOp::WriteValue(ExprValue::LiteralString(String::from(expr_str)),
                                                Some(allocate_element_key())));

                // Write right expression
                self.process_expr(r, block)?;
            }

            &ExprValue::ContentNode(ref node) => {
                self.process_content_node(node, block)?
            }

            _ => {
                block.ops_vec.push(ElementOp::WriteValue(expr.clone(), Some(allocate_element_key())));

            }
        };
        // ops_vec.push(ElementOp::WriteValue(expr.clone(), Some(allocate_element_key())));
        Ok(())
    }

    #[inline]
    fn process_content_node(&mut self,
                            node: &'input ContentNodeType,
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
                if let Some(..) = self.processing.comp_map.get(element_data.element_ty.as_str()) {

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
                            self.process_content_node(child, block)?;
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
                self.process_expr(expr, block)?
            }
            &ContentNodeType::ForNode(ref ele, ref coll_expr, ref nodes) => {
                let block_id = allocate_element_key().replace("-", "_");
                block.ops_vec.push(ElementOp::StartBlock(block_id.clone()));

                if let &Some(ref nodes) = nodes {
                    for ref node in nodes {
                        // FIXME: forvar resolve
                        self.process_content_node(node, block)?;
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
                        self.process_content_node(content, &mut block)?;
                    }
                    _ => {}
                }
            }
        }

        let comp = Component {
            name: name,
            ops: Some(block.ops_vec),
            uses: None,
            child_map: Default::default(),
        };

        self.processing.comp_map.insert(name, comp);

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
                    self.process_content_node(content, block)?;
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
