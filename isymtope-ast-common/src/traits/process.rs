use std::fmt::Debug;

use error::*;
use expressions::*;
use ast::*;

#[derive(Debug, Clone, Hash, PartialEq)]
pub enum ProcessingScopeEnvironment {
    Normal,
    Reducer(String),
    ElementActions,
}

impl Default for ProcessingScopeEnvironment {
    fn default() -> Self {
        ProcessingScopeEnvironment::Normal
    }
}

pub trait ProcessingContext: Debug {
    fn template(&self) -> &Template;
    fn add_reducer_key(&mut self, key: String) -> DocumentProcessingResult<()>;
    fn is_reducer_key(&self, key: &str) -> DocumentProcessingResult<bool>;

    fn push_child_scope_with_environment(&mut self, environment: ProcessingScopeEnvironment);
    fn push_child_scope(&mut self);
    fn pop_scope(&mut self);

    fn bind_ident(
        &mut self,
        key: String,
        binding: CommonBindings<ProcessedExpression>,
    ) -> DocumentProcessingResult<()>;
    fn must_find_ident(
        &mut self,
        key: &str,
    ) -> DocumentProcessingResult<CommonBindings<ProcessedExpression>>;

    fn bind_ident_shape(
        &mut self,
        key: String,
        binding: BindingShape<ProcessedExpression>,
    ) -> DocumentProcessingResult<()>;
    fn find_ident_shape(
        &mut self,
        key: &str,
    ) -> DocumentProcessingResult<Option<BindingShape<ProcessedExpression>>>;

    fn bind_element_binding(
        &mut self,
        key: String,
        binding: CommonBindings<ProcessedExpression>,
    ) -> DocumentProcessingResult<()>;
    fn find_element_binding(
        &mut self,
        key: &str,
    ) -> DocumentProcessingResult<Option<CommonBindings<ProcessedExpression>>>;

    fn environment(&mut self) -> DocumentProcessingResult<ProcessingScopeEnvironment>;
}

pub trait TryProcessFrom<Input> {
    fn try_process_from(src: &Input, ctx: &mut ProcessingContext) -> DocumentProcessingResult<Self>
    where
        Self: Sized;
}

impl<I, O: TryProcessFrom<I>> TryProcessFrom<Option<I>> for Option<O> {
    fn try_process_from(
        src: &Option<I>,
        ctx: &mut ProcessingContext,
    ) -> DocumentProcessingResult<Self> {
        Ok(match *src {
            Some(ref e) => Some(TryProcessFrom::try_process_from(e, ctx)?),
            _ => None,
        })
    }
}

// impl<I, O: TryProcessFrom<I>> TryProcessFrom<Box<I>> for Box<O> {
//     fn try_process_from(src: &Box<I>) -> DocumentProcessingResult<Self> {
//         Ok(Box::new(TryProcessFrom::try_process_from(src.as_ref())?))
//     }
// }

// impl<I, O: TryProcessFrom<I>> TryProcessFrom<I> for Box<O> {
//     fn try_process_from(src: &I) -> DocumentProcessingResult<Self> {
//         Ok(Box::new(TryProcessFrom::try_process_from(src.as_ref())?))
//     }
// }

impl<I, O: TryProcessFrom<I>> TryProcessFrom<Box<I>> for O {
    fn try_process_from(
        src: &Box<I>,
        ctx: &mut ProcessingContext,
    ) -> DocumentProcessingResult<Self> {
        Ok(TryProcessFrom::try_process_from(src.as_ref(), ctx)?)
    }
}

// impl<I, O: TryProcessFrom<I>> TryProcessFrom<Option<Box<I>>> for O {
//     fn try_process_from(src: &Box<I>) -> DocumentProcessingResult<Self> {
//         Ok(TryProcessFrom::try_process_from(src.as_ref().map(|box ref v| v))?)
//     }
// }
