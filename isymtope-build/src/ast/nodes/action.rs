use std::marker::PhantomData;
use std::fmt::Debug;

use error::*;
use traits::*;
use expressions::*;
use output::*;

#[allow(dead_code)]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum ActionOp<T> {
    DispatchAction(String, Option<Box<Vec<PropValue<T>>>>, PhantomData<T>),
    DispatchActionTo(
        String,
        Option<Box<Vec<PropValue<T>>>>,
        String,
        PhantomData<T>,
    ),
    Navigate(ExpressionValue<T>, PhantomData<T>),
}

impl<I, O> TryProcessFrom<ActionOp<I>> for ActionOp<O>
where
    ExpressionValue<O>: TryProcessFrom<ExpressionValue<I>>,
    I: ::std::fmt::Debug,
    O: ::std::fmt::Debug,
{
    fn try_process_from(
        src: &ActionOp<I>,
        ctx: &mut ProcessingContext,
    ) -> DocumentProcessingResult<Self> {
        match *src {
            ActionOp::DispatchAction(ref a, Some(box ref b), _) => Ok(ActionOp::DispatchAction(
                a.to_owned(),
                Some(Box::new(TryProcessFrom::try_process_from(b, ctx)?)),
                Default::default(),
            )),
            ActionOp::DispatchAction(ref a, _, _) => Ok(ActionOp::DispatchAction(
                a.to_owned(),
                None,
                Default::default(),
            )),

            ActionOp::DispatchActionTo(ref a, Some(box ref b), ref c, _) => {
                Ok(ActionOp::DispatchActionTo(
                    a.to_owned(),
                    Some(Box::new(TryProcessFrom::try_process_from(b, ctx)?)),
                    c.to_owned(),
                    Default::default(),
                ))
            }
            ActionOp::DispatchActionTo(ref a, _, ref c, _) => Ok(ActionOp::DispatchActionTo(
                a.to_owned(),
                None,
                c.to_owned(),
                Default::default(),
            )),

            ActionOp::Navigate(ref path, _) => Ok(ActionOp::Navigate(
                TryProcessFrom::try_process_from(path, ctx)?,
                Default::default(),
            )),
        }
    }
}

impl<T> TryEvalFrom<ActionOp<T>> for ActionOp<OutputExpression>
where
    ExpressionValue<OutputExpression>: TryEvalFrom<ExpressionValue<T>>,
    T: Debug,
{
    fn try_eval_from(
        src: &ActionOp<T>,
        ctx: &mut OutputContext<ProcessedExpression>,
    ) -> DocumentProcessingResult<Self> {
        match *src {
            ActionOp::DispatchAction(ref a, Some(box ref b), _) => Ok(ActionOp::DispatchAction(
                a.to_owned(),
                Some(Box::new(TryEvalFrom::try_eval_from(b, ctx)?)),
                Default::default(),
            )),
            ActionOp::DispatchAction(ref a, _, _) => Ok(ActionOp::DispatchAction(
                a.to_owned(),
                None,
                Default::default(),
            )),

            ActionOp::DispatchActionTo(ref a, Some(box ref b), ref c, _) => {
                Ok(ActionOp::DispatchActionTo(
                    a.to_owned(),
                    Some(Box::new(TryEvalFrom::try_eval_from(b, ctx)?)),
                    c.to_owned(),
                    Default::default(),
                ))
            }
            ActionOp::DispatchActionTo(ref a, _, ref c, _) => Ok(ActionOp::DispatchActionTo(
                a.to_owned(),
                None,
                c.to_owned(),
                Default::default(),
            )),

            ActionOp::Navigate(ref path, _) => Ok(ActionOp::Navigate(
                TryEvalFrom::try_eval_from(path, ctx)?,
                Default::default(),
            )),
        }
    }
}
