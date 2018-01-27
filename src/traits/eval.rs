
use error::*;
use expressions::*;
use scope::*;
use output::*;


pub trait TryEvalFrom<I> {
    fn try_eval_from(src: &I, ctx: &mut OutputContext<ProcessedExpression>) -> DocumentProcessingResult<Self> where Self: Sized;
}

// impl<I, O: TryEvalFrom<I>> TryEvalFrom<Option<Box<I>>> for Option<O> {
//     fn try_eval_from(src: &Box<I>, ctx: &mut OutputContext<ProcessedExpression>) -> DocumentProcessingResult<Self> where Self: Sized {
//         Ok(match src.as_ref() {
//             Some(box ref v) => v,
//             _ => None
//         })
//     }
// }

impl<I, O: TryEvalFrom<I>> TryEvalFrom<Box<I>> for O {
    fn try_eval_from(src: &Box<I>, ctx: &mut OutputContext<ProcessedExpression>) -> DocumentProcessingResult<Self> where Self: Sized {
        Ok(TryEvalFrom::try_eval_from(src.as_ref(), ctx)?)
    }
}
