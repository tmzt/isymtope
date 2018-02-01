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
}

pub trait OutputContext: Debug {
    fn doc(&self) -> &Document;

    fn reducer_value(
        &mut self,
        key: &str,
    ) -> DocumentProcessingResult<ExpressionValue<OutputExpression>>;

    fn push_child_scope_with_environment(&mut self, environment: OutputScopeEnvironment);
    fn push_child_scope(&mut self);
    fn pop_scope(&mut self);

    fn bind_value(
        &mut self,
        binding: CommonBindings<ProcessedExpression>,
        value: ExpressionValue<OutputExpression>,
    ) -> DocumentProcessingResult<()>;

    fn find_value(
        &mut self,
        binding: &CommonBindings<ProcessedExpression>,
    ) -> DocumentProcessingResult<Option<ExpressionValue<OutputExpression>>>;

    fn must_find_value(
        &mut self,
        binding: &CommonBindings<ProcessedExpression>,
    ) -> DocumentProcessingResult<ExpressionValue<OutputExpression>>;

    fn bind_loop_value(
        &mut self,
        binding: CommonBindings<OutputExpression>,
        value: ExpressionValue<OutputExpression>,
    ) -> DocumentProcessingResult<()>;

    fn must_find_loop_value(
        &mut self,
        binding: &CommonBindings<OutputExpression>,
    ) -> DocumentProcessingResult<ExpressionValue<OutputExpression>>;

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
