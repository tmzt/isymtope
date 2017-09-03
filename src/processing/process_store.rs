
use std::iter;
use parser::ast::*;
use processing::structs::*;
use processing::process_util::*;
use scope::*;


// #[derive(Debug, Default)]
// pub struct StoreOutputProcessing {
//     processing: DocumentProcessingState,
//     actions: Vec<ReducerActionData>
// }

// #[derive(Debug, Default)]
// pub struct StoreOutput {
//     processing: DocumentProcessingState,
//     actions: Vec<ReducerActionData>
// }

// impl Into<StoreOutput> for StoreOutputProcessing {
//     fn into(self) -> StoreOutput {
//         StoreOutput {
//             processing: self.processing,
//             actions: self.actions
//         }
//     }
// }

#[derive(Debug, Default)]
pub struct ProcessStore {}

impl ProcessStore {
    #[inline]
    #[allow(dead_code)]
    pub fn process_let_node(&mut self,
                            // output: &mut StoreOutputProcessing,
                            processing: &mut DocumentProcessingState,
                            _ctx: &mut Context,
                            _bindings: &mut BindingContext,
                            var_name: &str,
                            expr: Option<&ExprValue>)
                            -> DocumentProcessingResult<()> {
        let has_default_sym = processing.default_state_symbol.is_some();
        let has_default_reducer_key = processing.default_reducer_key.is_some();

        let var_ty = expr.as_ref().and_then(|expr| peek_var_ty(expr));

        if !has_default_sym {
            let sym =
                Symbol::reducer_key_with(&var_name, var_ty.as_ref(), expr);
            // TODO: Include type
            processing.default_state_symbol = Some(sym);
        }

        if !has_default_reducer_key {
            processing.default_reducer_key = Some(var_name.to_owned());
        }

        let reducer_entry = processing
            .reducer_key_data
            .entry(var_name.to_owned())
            .or_insert_with(|| {
                ReducerKeyData::from_name(&format!("{}", var_name),
                                            var_ty.as_ref().map(Clone::clone))
            });

        if let Some(expr) = expr {
            reducer_entry.default_expr = Some(expr.to_owned());

            processing
                .default_state_map
                .entry(var_name.to_owned())
                .or_insert_with(|| {
                    let var_ty = peek_var_ty(expr);
                    (var_ty, Some(expr.to_owned()))
                });
        };

        Ok(())
    }

    #[allow(dead_code)]
    pub fn process_action_node<'a, I: IntoIterator<Item = &'a str>>(&mut self,
                            // output: &mut StoreOutputProcessing,
                            processing: &mut DocumentProcessingState,
                            ctx: &mut Context,
                            _bindings: &mut BindingContext,
                            action_name: &str,
                            expr: Option<&ExprValue>,
                            params: I)
                            -> DocumentProcessingResult<()> {
        ctx.push_child_scope();
        // ctx.append_action_path_str(action_name);

        // Add action params
        for param in params {
            // ctx.add_unbound_action_param(param);
            ctx.add_action_param(param);
        }

        let action_path = ctx.join_action_path(Some("."));
        let complete_path = ctx.join_action_path_with(Some("."), &action_name);
        let mut action = ReducerActionData::from_name(&complete_path, Some(&action_path));
        if let Some(ref sym) = processing.default_state_symbol {
            action.state_ty = sym.ty().map(|s| s.clone());
        }

        // TODO: Handle params

        if let Some(ref expr) = expr {
            let action_expr = ctx.reduce_expr_or_return_same(expr);
            // let typed_expr = ExprValue::LiteralString("".into());
            action.state_expr = Some(ActionStateExprType::SimpleReducerKeyExpr(action_expr));
        }

        let reducer_entry = processing
            .reducer_key_data
            .entry(action_path.to_owned())
            .or_insert_with(|| {
                ReducerKeyData::from_name(&format!("{}", action_path), None)
            });

        if let Some(ref mut actions) = reducer_entry.actions {
            actions.push(action);
        } else {
            reducer_entry.actions = Some(vec![action]);
        }
        Ok(())
    }

    #[inline]
    #[allow(dead_code)]
    pub fn process_store_default_scope_node(&mut self,
                                    // output: &mut StoreOutputProcessing,
                                    processing: &mut DocumentProcessingState,
                                    ctx: &mut Context,
                                    bindings: &mut BindingContext,
                                    node: &DefaultScopeNodeType)
                                    -> DocumentProcessingResult<()> {
        match node {
            &DefaultScopeNodeType::LetNode(ref var_name, ref expr) => {
                self.process_let_node(processing, ctx, bindings, var_name.as_ref(), expr.as_ref())?;
            }

            &DefaultScopeNodeType::ApiRootNode(ref _scope_name, ref _api_nodes) => {
                // if let &Some(ref api_nodes) = api_nodes {
                //     self.collect_js_store_api_scope(scope_name, api_nodes)?;
                // }                            // processing_scope.2 = Some(sym.to_owned());
            }

            &DefaultScopeNodeType::ScopeNode(ref scope_name, ref scope_nodes) => {
                for scope_node in scope_nodes {
                    self.process_store_child_scope_node(processing, ctx, bindings, scope_name, scope_node)?;
                }

                ctx.pop_scope();
            }
            // _ => {}
        };
        Ok(())
    }

    #[inline]
    #[allow(dead_code)]
    pub fn process_store_child_scope_node(&mut self,
                                    // output: &mut StoreOutputProcessing,
                                    processing: &mut DocumentProcessingState,
                                    ctx: &mut Context,
                                    bindings: &mut BindingContext,
                                    scope_name: &str,
                                    node: &ScopeNodeType)
                                    -> DocumentProcessingResult<()> {
        ctx.push_child_scope();
        ctx.append_action_path_str(scope_name);

        // Make the current state available using the reducer key (scope_name)
        ctx.add_action_state_binding(scope_name);

        // Make the current state available as `state`
        ctx.add_action_state_binding("state");

        match node {
            &ScopeNodeType::LetNode(ref var_name, ref expr) => {
                self.process_let_node(processing, ctx, bindings, var_name.as_ref(), expr.as_ref())?;
            }

            &ScopeNodeType::ActionNode(ref action_name, ref simple_expr, ref params) => {
                if let &Some(ActionStateExprType::SimpleReducerKeyExpr(ref expr)) = simple_expr {
                    let reduced_expr = ctx.reduce_expr_or_return_same(expr);

                    if let &Some(ref params) = params {
                        self.process_action_node(processing, ctx, bindings, action_name, Some(&reduced_expr), params.iter().map(|s| s.as_str()))?;
                    } else {
                        self.process_action_node(processing, ctx, bindings, action_name, Some(&reduced_expr), iter::empty())?;
                    };
                }
            }

            &ScopeNodeType::ScopeNode(ref scope_name, ref scope_nodes) => {
                for scope_node in scope_nodes {
                    self.process_store_child_scope_node(processing, ctx, bindings, scope_name, scope_node)?;
                }
            }
            _ => {}
        };

        ctx.pop_scope();
        Ok(())
    }
}


#[cfg(test)]
mod tests {
    use super::*;
    use scope::context::*;
    use scope::bindings::*;

    #[test]
    pub fn test_processing_process_store_1() {
        let mut ctx = Context::default();
        let mut bindings = BindingContext::default();
        // let mut output = StoreOutputProcessing::default();
        let mut processing = DocumentProcessingState::default();
        let mut process_store = ProcessStore::default();

        let store_nodes = vec![
            DefaultScopeNodeType::LetNode("todos".into(), Some(ExprValue::LiteralNumber(0))),
            DefaultScopeNodeType::ScopeNode("todos".into(), vec![
                ScopeNodeType::ActionNode("add".into(), Some(ActionStateExprType::SimpleReducerKeyExpr(ExprValue::Expr(
                    ExprOp::Add,
                    Box::new(ExprValue::SymbolReference(Symbol::unresolved("todos"))),
                    Box::new(ExprValue::SymbolReference(Symbol::unresolved("value")))
                ))), None)
            ])
        ];

        let res = process_store.process_store_default_scope_node(
            &mut processing,
            &mut ctx,
            &mut bindings,
            &store_nodes[0]
        );
        assert!(res.is_ok());

        let res = process_store.process_store_default_scope_node(
            &mut processing,
            &mut ctx,
            &mut bindings,
            &store_nodes[1]
        );
        assert!(res.is_ok());

        // let output: StoreOutput = output.into();
        assert!(processing.reducer_key_data.contains_key("todos"));
        assert!(!processing.reducer_key_data.contains_key("add"));
        assert_eq!(processing.reducer_key_data.get("todos"), Some(
            &ReducerKeyData { reducer_key: "todos".into(), default_expr: Some(ExprValue::LiteralNumber(0)), ty: Some(VarType::Primitive(PrimitiveVarType::Number)), actions: Some(vec![
                ReducerActionData {
                    action_type: "TODOS.ADD".into(),
                    state_expr: Some(ActionStateExprType::SimpleReducerKeyExpr(ExprValue::Expr(
                        ExprOp::Add,
                        Box::new(ExprValue::SymbolReference(Symbol::binding(&BindingType::ActionStateBinding))),
                        // Box::new(ExprValue::SymbolReference(Symbol::unresolved("todos"))),
                        Box::new(ExprValue::SymbolReference(Symbol::unresolved("value")))
                    ))),
                    state_ty: Some(VarType::Primitive(PrimitiveVarType::Number)),
                    default_scope_key: Some("todos".into())
                }
            ])}
        ));
    }
}