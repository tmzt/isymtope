
use std::collections::HashMap;

use traits::*;
use objects::*;
use expressions::*;
use processing::*;
use output::*;
use server::*;


#[derive(Debug, Default)]
pub struct ServerActionExecutor {}

impl ServerActionExecutor {
    pub fn initialize_session_data(&self, state: &mut Session, doc: &Document, ctx: &mut OutputContext<ProcessedExpression>) -> IsymtopeServerVoidResult {

        // Initial values from reducers
        if let Some(reducers) = doc.reducers() {
            for (key, reducer) in reducers {
                if let Some(default_expr) = reducer.default_value() {
                    let default_expr: ExpressionValue<OutputExpression> = TryEvalFrom::try_eval_from(default_expr, ctx)?;

                    state.set_value(key, default_expr, true)?;
                }
            }
        }

        Ok(())
    }

    pub fn execute_store_action<'p, P: IntoIterator<Item = (&'p str, &'p ExpressionValue<OutputExpression>)>>(&self, state: &mut Session, doc: &Document, ctx: &mut OutputContext<ProcessedExpression>, action_ty: &str, params: Option<P>) -> IsymtopeServerVoidResult {
        let reducers: HashMap<_, _> = doc.reducers().map(|v| v.map(|(key, reducer)| (key.to_owned(), reducer)).collect()).unwrap_or_default();

        let actions: HashMap<_, _> = reducers.iter()
            .flat_map(|(reducer_key, reducer)| {
                let actions: Vec<_> = reducer.actions().map(|v| v.map(|action| (action.name().to_uppercase(), (reducer_key.to_owned(), action))).collect()).unwrap_or_default();
                actions.into_iter()
            })
            .collect();

        if let Some(&(ref reducer_key, ref action)) = actions.get(&action_ty.to_uppercase()) {
            eprintln!("[server/executor] executing action of type [{}].", action_ty);

            eprintln!("[server/executor] action: {:?}", action);
            let expr = action.expr();
            eprintln!("[server/executor] expr: {:?}", expr);

            // FIXME: expr should not be optional
            if expr.is_none() {
                eprintln!("[server/executor] no expression for this action");
                Err(try_eval_from_err!("Missing expression on action"))?;
            };
            let expr = expr.unwrap();

            ctx.push_child_scope();
            if let Some(params) = params {
                for (key, value) in params {
                    eprintln!("[server/executor] executing action of type [{}]: param [{}]: {:?}", action_ty, key, value);
                    let binding = CommonBindings::NamedReducerActionParam(key.to_owned(), Default::default());
                    eprintln!("[server/executor] adding binding [{:?}] with value [{:?}]", binding, value);
                    ctx.bind_value(binding, value.to_owned())?;
                }
            };

            // Evalute processed expression
            let expr: ExpressionValue<OutputExpression> = TryEvalFrom::try_eval_from(expr, ctx)?;
            eprintln!("[server/executor] expr(a): {:?}", expr);
            // Evalute output expression
            let expr: ExpressionValue<OutputExpression> = TryEvalFrom::try_eval_from(&expr, ctx)?;
            eprintln!("[server/executor] expr(b): {:?}", expr);

            eprintln!("[server/executor] setting reducer key [{}] to value [{:?}]", reducer_key, expr);
            state.set_value(&reducer_key, expr, true)?;
            ctx.pop_scope();
        };

        Ok(())
    }

    pub fn execute_action_op(&self, state: &mut Session, doc: &Document, ctx: &mut OutputContext<ProcessedExpression>, action_op: &ActionOp<ProcessedExpression>) -> IsymtopeServerVoidResult {
        match *action_op {
            ActionOp::DispatchAction(ref action_ty, ref params, _) => {
                let params: Vec<_> = params.as_ref().map(|&box ref v| v.iter().map(|param| (param.key().to_owned(), param.value().to_owned())).collect()).unwrap_or_default();
                let params: Vec<(String, ExpressionValue<OutputExpression>)> = TryEvalFrom::try_eval_from(&params, ctx)?;

                self.execute_store_action(state, doc, ctx, action_ty, Some(params.iter().map(|p| (p.0.as_str(), &p.1))))?;
            }

            ActionOp::DispatchActionTo(ref action_ty, ref params, ref target, _) => {
                let action_ty = format!("{}.{}", action_ty, target);
                let params: Vec<_> = params.as_ref().map(|&box ref v| v.iter().map(|param| (param.key().to_owned(), param.value().to_owned())).collect()).unwrap_or_default();
                let params: Vec<(String, ExpressionValue<OutputExpression>)> = TryEvalFrom::try_eval_from(&params, ctx)?;

                self.execute_store_action(state, doc, ctx, &action_ty, Some(params.iter().map(|p| (p.0.as_str(), &p.1))))?;
            }

            _ => {
                return Err(try_eval_from_err!(format!("Unsupported action_op for server execution: {:?}", action_op)))?;
            }
        };

        Ok(())
    }

    pub fn execute_document_route(&self, state: &mut Session, doc: &Document, ctx: &mut OutputContext<ProcessedExpression>, path: &str) -> IsymtopeServerVoidResult {
        let routes: HashMap<String, Route<ProcessedExpression>> = doc.routes().map(|r| (r.pattern().to_owned(), r.to_owned())).collect();
        let route = routes.get(path);

        if route.is_none() {
            return Err(try_eval_from_err!("Invalid route"))?;
        }

        let action = route.unwrap().action();
        match *action {
            RouteActionValue::Actions(ref v, _) => {
                if let Some(ref v) = *v {
                    for action_op in v {
                        eprintln!("[server/executor] Executing action_op: {:?}", action_op);
                        self.execute_action_op(state, doc, ctx, action_op)?;
                    }
                };
            }

            _ => {
                return Err(try_eval_from_err!(format!("Unsupported action for server execution: {:?}", action)))?;
            }
        };

        Ok(())
    }

}