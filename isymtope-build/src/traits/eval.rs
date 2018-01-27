
use error::*;
use expressions::*;
use output::*;


pub trait TryEvalFrom<I> {
    fn try_eval_from(src: &I, ctx: &mut OutputContext<ProcessedExpression>) -> DocumentProcessingResult<Self> where Self: Sized;
}

impl<I, O: TryEvalFrom<I>> TryEvalFrom<Box<I>> for O {
    fn try_eval_from(src: &Box<I>, ctx: &mut OutputContext<ProcessedExpression>) -> DocumentProcessingResult<Self> where Self: Sized {
        Ok(TryEvalFrom::try_eval_from(src.as_ref(), ctx)?)
    }
}
