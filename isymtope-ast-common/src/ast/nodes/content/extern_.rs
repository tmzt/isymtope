use std::marker::PhantomData;

use error::*;
use traits::*;
use expressions::*;
use ast::*;
// use output::*;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct ExternNode<T>(Option<Box<Vec<ContentNode<T>>>>, PhantomData<T>);

impl<T: Clone> ExternNode<T> {
    pub fn new(children: Option<Vec<ContentNode<T>>>) -> Self {
        let children = children.map(Box::new);

        ExternNode(children, Default::default())
    }

    pub fn children<'a>(&'a self) -> Option<impl Iterator<Item = &'a ContentNode<T>>> {
        self.0.as_ref().map(|v| v.iter())
    }
}

impl TryProcessFrom<ExternNode<SourceExpression>> for ExternNode<ProcessedExpression> {
    fn try_process_from(
        src: &ExternNode<SourceExpression>,
        ctx: &mut ProcessingContext,
    ) -> DocumentProcessingResult<Self> {
        let children = match src.0 {
            Some(box ref children) => {
                Some(Box::new(TryProcessFrom::try_process_from(children, ctx)?))
            }
            _ => None,
        };

        Ok(ExternNode(children, Default::default()))
    }
}

impl TryEvalFrom<ExternNode<ProcessedExpression>> for ExternNode<OutputExpression> {
    fn try_eval_from(
        src: &ExternNode<ProcessedExpression>,
        ctx: &mut OutputContext,
    ) -> DocumentProcessingResult<Self> {
        Err(try_eval_from_err!("Cannot evaluate"))
    }
}
