
use std::iter;
use parser::ast::*;
use parser::store::*;
use processing::structs::*;
use processing::process_util::*;
use scope::scope::*;
use scope::context::*;
use scope::bindings::*;


#[derive(Debug, Default)]
pub struct StoreOutputProcessing {
    processing: DocumentProcessingState,
    actions: Vec<ReducerActionData>
}

#[derive(Debug, Default)]
pub struct StoreOutput {
    processing: DocumentProcessingState,
    actions: Vec<ReducerActionData>
}

impl Into<StoreOutput> for StoreOutputProcessing {
    fn into(self) -> StoreOutput {
        StoreOutput {
            processing: self.processing,
            actions: self.actions
        }
    }
}

#[derive(Debug, Default)]
pub struct ProcessStore {}

impl ProcessStore {
    #[inline]
    pub fn process_let_node(&mut self,
                            output: &mut StoreOutputProcessing,
                            ctx: &mut Context,
                            bindings: &mut BindingContext,
                            var_name: &str,
                            expr: Option<&ExprValue>)
                            -> DocumentProcessingResult<()> {
        let has_default_sym = output.processing.default_state_symbol.is_some();
        let has_default_reducer_key = output.processing.default_reducer_key.is_some();
        let has_expr = expr.is_some();

        let var_ty = expr.as_ref().and_then(|expr| peek_var_ty(expr));

        if !has_default_sym {
            let sym =
                Symbol::reducer_key_with(&var_name, var_ty.as_ref(), expr);
            // TODO: Include type
            output.processing.default_state_symbol = Some(sym);
        }

        if !has_default_reducer_key {
            output.processing.default_reducer_key = Some(var_name.to_owned());
        }

        let reducer_entry = output.processing
            .reducer_key_data
            .entry(var_name.to_owned())
            .or_insert_with(|| {
                ReducerKeyData::from_name(&format!("{}", var_name),
                                            var_ty.as_ref().map(Clone::clone))
            });

        if let Some(expr) = expr {
            reducer_entry.default_expr = Some(expr.to_owned());

            output.processing
                .default_state_map
                .entry(var_name.to_owned())
                .or_insert_with(|| {
                    let var_ty = peek_var_ty(expr);
                    (var_ty, Some(expr.to_owned()))
                });
        };

        Ok(())
    }

    pub fn process_action_node<'a, I: IntoIterator<Item = &'a str>>(&mut self,
                            output: &mut StoreOutputProcessing,
                            ctx: &mut Context,
                            bindings: &mut BindingContext,
                            action_name: &str,
                            expr: Option<&ExprValue>,
                            params: I)
                            -> DocumentProcessingResult<()> {
        ctx.push_child_scope();
        ctx.append_action_path_str(action_name);

        let action_path = ctx.join_action_path(Some("."));
        let mut action = ReducerActionData::from_name(&action_path, Some(&action_path));
        if let Some(ref sym) = output.processing.default_state_symbol {
            action.state_ty = sym.ty().map(|s| s.clone());
        }

        // TODO: Handle params

        // let action_expr = ctx.reduce_expr_or_return_same(expr);
        let typed_expr = ExprValue::LiteralString("".into());
        action.state_expr = Some(ActionStateExprType::SimpleReducerKeyExpr(typed_expr));

        let reducer_entry = output.processing
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
    pub fn process_store_default_scope_node(&mut self,
                                    output: &mut StoreOutputProcessing,
                                    ctx: &mut Context,
                                    bindings: &mut BindingContext,
                                    node: &DefaultScopeNodeType)
                                    -> DocumentProcessingResult<()> {
        match node {
            &DefaultScopeNodeType::LetNode(ref var_name, ref expr) => {
                self.process_let_node(output, ctx, bindings, var_name.as_ref(), expr.as_ref())?;
            }

            &DefaultScopeNodeType::ApiRootNode(ref scope_name, ref api_nodes) => {
                // if let &Some(ref api_nodes) = api_nodes {
                //     self.collect_js_store_api_scope(scope_name, api_nodes)?;
                // }                            // processing_scope.2 = Some(sym.to_owned());
            }

            &DefaultScopeNodeType::ScopeNode(ref scope_name, ref scope_nodes) => {
                for scope_node in scope_nodes {
                    self.process_store_child_scope_node(output, ctx, bindings, scope_name, scope_node)?;
                }
            }
            _ => {}
        };
        Ok(())
    }

    #[inline]
    pub fn process_store_child_scope_node(&mut self,
                                    output: &mut StoreOutputProcessing,
                                    ctx: &mut Context,
                                    bindings: &mut BindingContext,
                                    reducer_key: &str,
                                    node: &ScopeNodeType)
                                    -> DocumentProcessingResult<()> {
        match node {
            &ScopeNodeType::LetNode(ref var_name, ref expr) => {
                self.process_let_node(output, ctx, bindings, var_name.as_ref(), expr.as_ref())?;
            }

            &ScopeNodeType::ActionNode(ref action_name, ref simple_expr, ref params) => {
                // let expr = if let &Some(ActionStateExprType::SimpleReducerKeyExpr(ref expr)) = simple_expr { Some(expr) } else { None };
                // self.process_action_node(output, ctx, bindings, action_name, expr, params.as_ref())?;
                if let &Some(ActionStateExprType::SimpleReducerKeyExpr(ref expr)) = simple_expr {
                    if let &Some(ref params) = params {
                        // let params = params.as_ref().map(|s| s.as_str());
                        self.process_action_node(output, ctx, bindings, action_name, Some(expr), params.iter().map(|s| s.as_str()))?;
                    } else {
                        self.process_action_node(output, ctx, bindings, action_name, Some(expr), iter::empty())?;
                    };
                }
            }

            &ScopeNodeType::ScopeNode(ref scope_name, ref scope_nodes) => {
                ctx.push_child_scope();
                ctx.append_action_path_str(&scope_name);
                for scope_node in scope_nodes {
                    self.process_store_child_scope_node(output, ctx, bindings, scope_name, scope_node)?;
                }
            }
            _ => {}
        };
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
        let mut output = StoreOutputProcessing::default();
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
            &mut output,
            &mut ctx,
            &mut bindings,
            &store_nodes[0]
        );
        assert!(res.is_ok());

        let res = process_store.process_store_default_scope_node(
            &mut output,
            &mut ctx,
            &mut bindings,
            &store_nodes[1]
        );
        assert!(res.is_ok());

        let output: StoreOutput = output.into();
        assert!(output.processing.reducer_key_data.contains_key("todos"));
        assert_eq!(output.processing.reducer_key_data.get("todos"), Some(
            &ReducerKeyData { reducer_key: "todos".into(), default_expr: Some(ExprValue::LiteralNumber(0)), ty: Some(VarType::Primitive(PrimitiveVarType::Number)), actions: Some(vec![])  }
        ));
    }
}