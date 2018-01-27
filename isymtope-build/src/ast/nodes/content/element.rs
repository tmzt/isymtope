
use std::marker::PhantomData;

use util::*;
use error::*;
use traits::*;
use expressions::*;
use objects::*;
use ast::*;
use output::*;


#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct ElementNode<T>(String, String, Option<String>, Option<Vec<ElementAttrValue<T>>>, Option<Vec<ElementBinding<T>>>, Option<Box<Vec<ContentNode<T>>>>, PhantomData<T>);

impl<T: Clone> ElementNode<T> {
    pub fn new(tag: String, props: Option<Vec<ElementAttrValue<T>>>, children: Option<Vec<ContentNode<T>>>, bindings: Option<Vec<ElementBinding<T>>>) -> Self {
        let key = allocate_element_key();
        let children = children.map(Box::new);

        // FIXME: Support parent_tag and bindings
        ElementNode(tag, key, None, props, bindings, children, Default::default())
    }

    pub fn tag(&self) -> &str { &self.0 }
    pub fn key(&self) -> &str { &self.1 }

    pub fn attrs<'a>(&'a self) -> Option<impl Iterator<Item = &'a ElementAttrValue<T>>> {
        self.3.as_ref().map(|v| v.iter())
    }

    pub fn bindings<'a>(&'a self) -> Option<impl Iterator<Item = &'a ElementBinding<T>>> {
        self.4.as_ref().map(|v| v.iter())
    }

    pub fn children<'a>(&'a self) -> Option<impl Iterator<Item = &'a ContentNode<T>>> {
        self.5.as_ref().map(|v| v.iter())
    }

    pub fn parent_tag(&self) -> Option<&str> { self.2.as_ref().map(|s| s.as_str()) }
}

impl TryProcessFrom<ElementNode<SourceExpression>> for ElementNode<ProcessedExpression> {
    fn try_process_from(src: &ElementNode<SourceExpression>, ctx: &mut ProcessingContext) -> DocumentProcessingResult<Self> {
        let bindings: Option<Vec<_>> = match src.4 { Some(ref bindings) => Some(TryProcessFrom::try_process_from(bindings, ctx)?), _ => None };

        ctx.push_child_scope();

        if let Some(ref bindings) = bindings {
            for binding in bindings {
                if let &ElementBinding::Value(ref v, _) = binding {
                    if let Some(key) = v.ident() {
                        let binding = CommonBindings::CurrentElementValue(Default::default());

                        eprintln!("Adding element_binding with key [{}]: {:?}", key, binding);
                        ctx.bind_element_binding(key.to_owned(), binding)?;
                    };
                };
            }
        };

        let props = match src.3 { Some(ref props) => Some(TryProcessFrom::try_process_from(props, ctx)?), _ => None };
        let children = match src.5 { Some(box ref children) => Some(Box::new(TryProcessFrom::try_process_from(children, ctx)?)), _ => None };

        ctx.pop_scope();

        Ok(ElementNode(src.0.to_owned(), src.1.to_owned(), src.2.to_owned(), props, bindings, children, Default::default()))
    }
}

impl TryEvalFrom<ElementNode<ProcessedExpression>> for ElementNode<OutputExpression> {
    fn try_eval_from(src: &ElementNode<ProcessedExpression>, ctx: &mut OutputContext<ProcessedExpression>) -> DocumentProcessingResult<Self> {
        Err(try_eval_from_err!("Cannot evaluate"))
    }
}

// impl<I, O> TryProcessFrom<ElementNode<I>> for ElementNode<O> where ExpressionValue<O>: TryProcessFrom<ExpressionValue<I>> {
//     fn try_process_from(src: &ElementNode<I>, ctx: &mut ProcessingContext) -> DocumentProcessingResult<Self> {
//         Err(reduction_err_bt!())

//         // Ok(ElementNode(src.0.clone(), Default::default()))
//     }
// }

// impl<I, O> TryEvalFrom<ElementNode<I>> for ElementNode<O> where ExpressionValue<O>: TryEvalFrom<ExpressionValue<I>>, I: ::std::fmt::Debug, O: ::std::fmt::Debug {
//     fn try_eval_from(src: &ElementNode<I>, ctx: &mut OutputContext<ProcessedExpression>) -> DocumentProcessingResult<Self> {
//         Err(reduction_err_bt!())
//     }
// }