use isymtope_ast_common::*;

pub struct ActionHandler {}

impl ActionHandler {
    pub fn handle_server_action_route(doc: &Document, state: &mut Session, path: &str) -> IsymtopeGenerateResult<()> {
        if let Some(route) = doc.get_route(path) {
            if route.client_only() {
                return Ok(());
            }

            if let Some(action_ops) = route.action_ops_iter() {
                for action_op in action_ops {
                    match *action_op {
                        ActionOp::DispatchAction(ref action_key, ref action_params, _)
                        | ActionOp::DispatchActionTo(ref action_key, ref action_params, _, _) => {
                            let path = match *action_op {
                                ActionOp::DispatchActionTo(_, _, ref path, _) => Some(path),
                                _ => None,
                            };
                        }
                    }
                }
            };
        };

        Ok(())
    }

    pub fn execute_store_action(doc: &Document, state: &mut Session, action_ty: &str) -> IsymtopeGenerateResult<()> {
        if let Some(reducer) = doc.get_reducer(action_ty) {
            for ref action in reducer.actions {
                println!(
                    "Executing action [{}] against server state: {:?}",
                    action_ty, action
                );
                state.execute_action(action_ty, action)?;
            }
        };

        Ok(())
    }
}
