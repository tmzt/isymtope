#[cfg(feature = "session_time")]
use time::{Duration, Timespec};

use error::*;
use ast::*;
use expressions::*;

pub trait Sessions {
    #[cfg(not(feature = "session_time"))]
    fn create(&mut self, session_id: &str) -> SessionResult<()>;
    #[cfg(feature = "session_time")]
    fn create(&mut self, session_id: &str, expires: Option<Duration>) -> SessionResult<()>;
    fn validate(&mut self, session_id: &str) -> SessionResult<()>;
    fn destroy(&mut self, session_id: &str) -> SessionResult<()>;
    fn execute_action(
        &mut self,
        session_id: &str,
        action_op: &ActionOp<ProcessedExpression>,
    ) -> SessionResult<()>;
}

pub trait Session {
    fn set_value(
        &mut self,
        key: &str,
        value: ExpressionValue<OutputExpression>,
        update: bool,
    ) -> SessionResult<()>;
    fn remove_value(&mut self, key: &str) -> SessionResult<()>;
    fn get_value(&self, key: &str) -> SessionResult<Option<&ExpressionValue<OutputExpression>>>;

    #[cfg(feature = "session_time")]
    fn created(&self) -> &Timespec;
    #[cfg(feature = "session_time")]
    fn expires(&self) -> Option<&Timespec>;

    fn execute_action(&mut self, action_op: &ActionOp<ProcessedExpression>) -> SessionResult<()>;

    #[cfg(feature = "types")]
    fn set_value_with_type(
        &mut self,
        key: &str,
        value: ExpressionValue<OutputExpression>,
    ) -> SessionResult<()>;
}
