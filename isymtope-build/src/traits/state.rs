use std::fmt::Debug;

use error::*;
use expressions::*;

pub trait ReducerStateProvider: Debug {
    fn get(&self, reducer_key: &str) -> SessionResult<Option<&ExpressionValue<OutputExpression>>>;
}
