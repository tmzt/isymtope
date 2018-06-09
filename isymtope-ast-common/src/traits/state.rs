use std::fmt::Debug;

use error::*;
use expressions::*;
use objects::*;

pub trait ReducerStateProvider: Debug {
    fn get(&self, reducer_key: &str) -> SessionResult<Option<&ExpressionValue<OutputExpression>>>;
}

pub trait RouteStateProvider: Debug {
    fn route(&self) -> SessionResult<Option<&Route>>;
}
