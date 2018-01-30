use std::marker::PhantomData;

use error::*;
use traits::*;
use expressions::*;
use objects::*;
use ast::*;
use util::*;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct RouteDefinition<T>(String, FormalParams<T>, RouteAction<T>, PhantomData<T>);

impl<T> RouteDefinition<T> {
    pub fn new(pattern: String, params: FormalParams<T>, action: RouteAction<T>) -> Self {
        RouteDefinition(pattern, params, action, Default::default())
    }

    pub fn pattern(&self) -> &str {
        &self.0
    }

    pub fn function_key(&self) -> &str {
        "key"
    }
}

impl<I, O> TryProcessFrom<RouteDefinition<I>> for Route<O>
where
    ExpressionValue<O>: TryProcessFrom<ExpressionValue<I>>,
    I: ::std::fmt::Debug,
    O: ::std::fmt::Debug,
{
    fn try_process_from(
        src: &RouteDefinition<I>,
        ctx: &mut ProcessingContext,
    ) -> DocumentProcessingResult<Self> {
        eprintln!("TryProcess RouteDefinition<I>: Pattern: {}", &src.0);
        eprintln!("TryProcess RouteDefinition<I>: Params: {:?}", src.1);
        eprintln!("TryProcess RouteDefinition<I>: Action: {:?}", src.2);

        let params: FormalParams<O> = TryProcessFrom::try_process_from(&src.1, ctx)?;
        let action: RouteActionValue<O> = TryProcessFrom::try_process_from(&src.2, ctx)?;

        Ok(Route::new(src.0.to_owned(), params, action))
    }
}

// impl<I, O> TryProcessFrom<RouteDefinition<I>> for RouteDefinition<O> where ExpressionValue<O>: TryProcessFrom<ExpressionValue<I>>, I: ::std::fmt::Debug, O: ::std::fmt::Debug {
//     fn try_process_from(src: &RouteDefinition<I>) -> DocumentProcessingResult<Self> {
//         eprintln!("TryProcess RouteDefinition<I>: Params: {:?}", src.1);
//         eprintln!("TryProcess RouteDefinition<I>: Action: {:?}", src.2);

//         let params: FormalParams<O> = TryProcessFrom::try_process_from(&src.1)?;
//         let action: RouteAction<O> = TryProcessFrom::try_process_from(&src.1)?;

//         Ok(RouteDefinition(src.0.to_owned(), params, action))
//     }
// }

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum RouteAction<T> {
    Content(Option<Vec<ContentNode<T>>>, PhantomData<T>),
    Actions(Option<Vec<ActionOp<T>>>, PhantomData<T>),
}

impl<I, O> TryProcessFrom<RouteAction<I>> for RouteActionValue<O>
where
    ExpressionValue<O>: TryProcessFrom<ExpressionValue<I>>,
    I: ::std::fmt::Debug,
    O: ::std::fmt::Debug,
{
    fn try_process_from(
        src: &RouteAction<I>,
        ctx: &mut ProcessingContext,
    ) -> DocumentProcessingResult<Self> {
        eprintln!(
            "TryProcess RouteAction (ast) -> RouteActionValue: src: {:?}",
            src
        );

        Ok(match *src {
            RouteAction::Content(ref a, _) => {
                let block_id = allocate_element_key();
                let ops: Vec<ElementOp<O>> = Default::default();

                let block = Block::new(block_id, None, Some(ops));

                RouteActionValue::Block(block, Default::default())
            }

            RouteAction::Actions(ref a, _) => RouteActionValue::Actions(
                TryProcessFrom::try_process_from(a, ctx)?,
                Default::default(),
            ),
        })
    }
}

// impl<I, O> TryProcessFrom<RouteAction<I>> for RouteAction<O> where ExpressionValue<O>: TryProcessFrom<ExpressionValue<I>>, I: ::std::fmt::Debug, O: ::std::fmt::Debug {
//     fn try_process_from(src: &RouteAction<I>) -> DocumentProcessingResult<Self> {
//         eprintln!("TryProcess RouteAction -> RouteAction: src: {:?}", src);

//         Ok(match *src {
//             RouteAction::Content(ref a, _) => RouteAction::Content(TryProcessFrom::try_process_from(a)?, Default::default()),
//             RouteAction::Actions(ref a, _) => RouteAction::Content(TryProcessFrom::try_process_from(a)?, Default::default()),
//         })
//     }
// }
