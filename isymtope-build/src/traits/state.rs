

use std::fmt::Debug;

use error::*;
use ast::*;
use expressions::*;

pub trait ReducerStateProvider : Debug {
    fn get(&self, reducer_key: &str) -> SessionResult<Option<&ExpressionValue<OutputExpression>>>;
}
