use ast::*;
use server::*;

pub struct ServerActionHandler {}

impl ServerActionHandler {
    pub fn handle_server_action_route(doc: &Document, state: &mut Sessions, path: &str) -> Result {
        if let Some(route) = doc.get_route(path) {
            if let Some(action_ops) = route.action_ops_iter() {
                for action_op in action_ops {
                    match *action_op {
                        ActionOp::DispatchAction(ref action_key, ref action_params)
                        | ActionOp::DispatchActionTo(ref action_key, ref action_params, _) => {
                            let path = match *action_op {
                                ActionOp::DispatchActionTo(_, _, ref path) => Some(path),
                                _ => None,
                            };
                        }
                    }
                }
            };
        };

        Ok(())
    }

    pub fn execute_store_action(doc: &Document, state: &mut Sessions, action_ty: &str) -> Result {
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
