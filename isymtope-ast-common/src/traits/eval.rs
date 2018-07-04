use std::fmt::Debug;

use error::*;
use expressions::*;
use objects::*;

#[derive(Debug, Clone, Hash, PartialEq)]
pub enum OutputScopeEnvironment {
    Normal,
    Reducer(String),
    Component,
    ComponentInstance,
    MappedComponentInstance,
    SubComponentInstance,
    MappedSubComponentInstance,
    RouteDispatchAction,
}

pub trait DocumentProvider: Debug {
    fn doc(&self) -> &Document;
}

pub enum ReducerValue {
    ProcessedExpression(ExpressionValue<ProcessedExpression>),
    OutputExpression(ExpressionValue<OutputExpression>),
}

impl TryEvalFrom<ReducerValue> for ExpressionValue<OutputExpression> {
    fn try_eval_from(
        src: &ReducerValue,
        ctx: &mut OutputContext,
    ) -> DocumentProcessingResult<Self> {
        match src {
            ReducerValue::ProcessedExpression(ref expr) => {
                TryEvalFrom::try_eval_from(expr, ctx)
            }

            ReducerValue::OutputExpression(ref expr) => {
                Ok(expr.to_owned())
            }
        }
    }
}

pub trait ContextDefaultsProvider: Debug {
    fn doc(&self) -> &Document;

    fn reducer_value(
        &mut self,
        key: &str,
    ) -> DocumentProcessingResult<ReducerValue>;
}

pub trait OutputContext: Debug {
    fn defaults(&mut self) -> &mut ContextDefaultsProvider;
    // fn doc(&self) -> &Document;

    fn push_child_scope_with_environment(&mut self, environment: OutputScopeEnvironment);
    fn push_child_scope(&mut self);
    fn pop_scope(&mut self);

    fn bind_value(
        &mut self,
        binding: CommonBindings<ProcessedExpression>,
        value: ExpressionValue<ProcessedExpression>,
    ) -> DocumentProcessingResult<()>;

    fn find_value(
        &mut self,
        binding: &CommonBindings<ProcessedExpression>,
    ) -> DocumentProcessingResult<Option<ExpressionValue<ProcessedExpression>>>;

    fn must_find_value(
        &mut self,
        binding: &CommonBindings<ProcessedExpression>,
    ) -> DocumentProcessingResult<ExpressionValue<ProcessedExpression>>;

    fn bind_loop_value(
        &mut self,
        binding: CommonBindings<ProcessedExpression>,
        value: ExpressionValue<ProcessedExpression>,
    ) -> DocumentProcessingResult<()>;

    fn must_find_loop_value(
        &mut self,
        binding: &CommonBindings<ProcessedExpression>,
    ) -> DocumentProcessingResult<ExpressionValue<ProcessedExpression>>;

    fn bind_element_key(&mut self, key: &str, idx: Option<i32>) -> DocumentProcessingResult<()>;

    fn get_element_key(&mut self) -> DocumentProcessingResult<Option<String>>;
    fn must_get_element_key(&mut self) -> DocumentProcessingResult<String>;

    fn environment(&mut self) -> DocumentProcessingResult<Option<OutputScopeEnvironment>>;
}

pub trait TryEvalFrom<I> {
    fn try_eval_from(src: &I, ctx: &mut OutputContext) -> DocumentProcessingResult<Self>
    where
        Self: Sized;
}

impl<I, O: TryEvalFrom<I>> TryEvalFrom<Box<I>> for O {
    fn try_eval_from(src: &Box<I>, ctx: &mut OutputContext) -> DocumentProcessingResult<Self>
    where
        Self: Sized,
    {
        Ok(TryEvalFrom::try_eval_from(src.as_ref(), ctx)?)
    }
}
