use std::marker::PhantomData;

use error::*;
use traits::*;
use common::*;
use expressions::*;
use output::*;

pub mod bindings;
pub mod attr;
pub mod element;
pub mod extern_;

pub use self::bindings::*;
pub use self::attr::*;
pub use self::element::*;
pub use self::extern_::*;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum ContentNode<T> {
    Element(ElementNode<T>, PhantomData<T>),
    Extern(ExternNode<T>, PhantomData<T>),
    // Value(ExpressionValue<T>, String, PhantomData<T>),
    ForNode(
        Option<String>,
        Box<ExpressionValue<T>>,
        Option<Box<Vec<ContentNode<T>>>>,
        PhantomData<T>,
    ),
    ExpressionValue(Box<ExpressionValue<T>>, String, PhantomData<T>),
    Primitive(Primitive, PhantomData<T>),
}

impl TryProcessFrom<ContentNode<SourceExpression>> for ContentNode<ProcessedExpression> {
    fn try_process_from(
        src: &ContentNode<SourceExpression>,
        ctx: &mut ProcessingContext,
    ) -> DocumentProcessingResult<Self> {
        match *src {
            ContentNode::Element(ref n, _) => Ok(ContentNode::Element(
                TryProcessFrom::try_process_from(n, ctx)?,
                Default::default(),
            )),
            ContentNode::Extern(ref e, _) => Ok(ContentNode::Extern(
                TryProcessFrom::try_process_from(e, ctx)?,
                Default::default(),
            )),

            ContentNode::ForNode(ref s, ref e, Some(ref n), _) => Ok(ContentNode::ForNode(
                s.to_owned(),
                Box::new(TryProcessFrom::try_process_from(e, ctx)?),
                Some(Box::new(TryProcessFrom::try_process_from(n, ctx)?)),
                Default::default(),
            )),
            ContentNode::ForNode(ref s, ref e, _, _) => Ok(ContentNode::ForNode(
                s.to_owned(),
                Box::new(TryProcessFrom::try_process_from(e, ctx)?),
                None,
                Default::default(),
            )),

            ContentNode::ExpressionValue(ref e, ref s, _) => Ok(ContentNode::ExpressionValue(
                Box::new(TryProcessFrom::try_process_from(e, ctx)?),
                s.to_owned(),
                Default::default(),
            )),

            ContentNode::Primitive(ref p, _) => {
                Ok(ContentNode::Primitive(p.to_owned(), Default::default()))
            }
        }
    }
}

impl TryEvalFrom<ContentNode<ProcessedExpression>> for ContentNode<OutputExpression> {
    fn try_eval_from(
        src: &ContentNode<ProcessedExpression>,
        ctx: &mut OutputContext<ProcessedExpression>,
    ) -> DocumentProcessingResult<Self> {
        match *src {
            ContentNode::Element(ref n, _) => Ok(ContentNode::Element(
                TryEvalFrom::try_eval_from(n, ctx)?,
                Default::default(),
            )),
            ContentNode::Extern(ref e, _) => Ok(ContentNode::Extern(
                TryEvalFrom::try_eval_from(e, ctx)?,
                Default::default(),
            )),

            ContentNode::ForNode(ref s, ref e, Some(ref n), _) => Ok(ContentNode::ForNode(
                s.to_owned(),
                Box::new(TryEvalFrom::try_eval_from(e, ctx)?),
                Some(Box::new(TryEvalFrom::try_eval_from(n, ctx)?)),
                Default::default(),
            )),
            ContentNode::ForNode(ref s, ref e, _, _) => Ok(ContentNode::ForNode(
                s.to_owned(),
                Box::new(TryEvalFrom::try_eval_from(e, ctx)?),
                None,
                Default::default(),
            )),

            ContentNode::ExpressionValue(ref e, ref s, _) => Ok(ContentNode::ExpressionValue(
                Box::new(TryEvalFrom::try_eval_from(e, ctx)?),
                s.to_owned(),
                Default::default(),
            )),

            ContentNode::Primitive(ref p, _) => {
                Ok(ContentNode::Primitive(p.to_owned(), Default::default()))
            }
        }
    }
}
