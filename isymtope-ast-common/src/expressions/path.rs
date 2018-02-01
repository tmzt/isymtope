use std::fmt::Debug;
use std::marker::PhantomData;

use itertools::Itertools;
use itertools::FoldWhile::Done;
use itertools::join;

use common::*;
use error::*;
use traits::*;
use expressions::*;
use objects::*;
use ast::*;
// use output::*;

#[allow(dead_code)]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum PathComponentValue<T> {
    Member(String, PhantomData<T>),
    MethodCall(String, Option<Vec<ParamValue<T>>>, PhantomData<T>),
}

impl<I, O> TryProcessFrom<PathComponentValue<I>> for PathComponentValue<O>
where
    ExpressionValue<O>: TryProcessFrom<ExpressionValue<I>>,
    I: ::std::fmt::Debug,
    O: ::std::fmt::Debug,
{
    fn try_process_from(
        src: &PathComponentValue<I>,
        ctx: &mut ProcessingContext,
    ) -> DocumentProcessingResult<Self> {
        match *src {
            PathComponentValue::Member(ref s, _) => {
                Ok(PathComponentValue::Member(s.to_owned(), Default::default()))
            }
            PathComponentValue::MethodCall(ref s, ref v, _) => Ok(PathComponentValue::MethodCall(
                s.to_owned(),
                TryProcessFrom::try_process_from(v, ctx)?,
                Default::default(),
            )),
        }
    }
}

impl<I, O> TryEvalFrom<PathComponentValue<I>> for PathComponentValue<O>
where
    ExpressionValue<O>: TryEvalFrom<ExpressionValue<I>>,
    I: ::std::fmt::Debug,
    O: ::std::fmt::Debug,
{
    fn try_eval_from(
        src: &PathComponentValue<I>,
        ctx: &mut OutputContext,
    ) -> DocumentProcessingResult<Self> {
        match *src {
            PathComponentValue::Member(ref s, _) => {
                Ok(PathComponentValue::Member(s.to_owned(), Default::default()))
            }
            PathComponentValue::MethodCall(ref s, ref v, _) => Ok(PathComponentValue::MethodCall(
                s.to_owned(),
                TryEvalFrom::try_eval_from(v, ctx)?,
                Default::default(),
            )),
        }
    }
}

#[allow(dead_code)]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct PathValue<T>(Box<ExpressionValue<T>>, Option<Vec<String>>);

impl<T> PathValue<T> {
    pub fn new(head: ExpressionValue<T>, v: Option<Vec<String>>) -> Self {
        PathValue(Box::new(head), v)
    }

    pub fn head(&self) -> &ExpressionValue<T> {
        self.0.as_ref()
    }

    pub fn components<'a>(&'a self) -> Option<impl Iterator<Item = &'a str>> {
        self.1.as_ref().map(|ref v| v.iter().map(|s| s.as_str()))
    }

    pub fn component_string(&self) -> String {
        self.components().map(|v| join(v, ".")).unwrap_or("".into())
    }

    pub fn complete_string(&self) -> String {
        let component_string = self.component_string();
        let head_string = match *self.head() {
            ExpressionValue::Binding(CommonBindings::NamedComponentProp(ref s, _), _) => {
                Some(format!("props.{}", s))
            }
            _ => None,
        }.unwrap_or_default();

        format!("{}.{}", head_string, component_string)
    }
}

impl<I, O> TryProcessFrom<PathValue<I>> for PathValue<O>
where
    ExpressionValue<O>: TryProcessFrom<ExpressionValue<I>>,
    I: ::std::fmt::Debug,
    O: ::std::fmt::Debug,
{
    fn try_process_from(
        src: &PathValue<I>,
        ctx: &mut ProcessingContext,
    ) -> DocumentProcessingResult<PathValue<O>> {
        let head: ExpressionValue<O> = TryProcessFrom::try_process_from(&src.0, ctx)?;
        let components: Option<Vec<_>> = src.1
            .as_ref()
            .map(|v| v.iter().map(|s| s.to_owned()).collect());

        Ok(PathValue(Box::new(head), components))
    }
}

fn eval_path<T>(
    src: &PathValue<T>,
    ctx: &mut OutputContext,
) -> DocumentProcessingResult<ExpressionValue<OutputExpression>>
where
    ExpressionValue<OutputExpression>: TryEvalFrom<ExpressionValue<T>>,
    T: Debug,
{
    let head = src.head();
    let components: Vec<_> = src.components().map(|v| v.collect()).unwrap_or_default();

    eprintln!("[path] eval_path: head (a): {:?}", head);
    eprintln!("[path] eval_path: components (a): {:?}", components);

    // Evaluate processed expression into output expression
    let head: ExpressionValue<OutputExpression> = TryEvalFrom::try_eval_from(head, ctx)?;

    // Evaluate binding if any
    let head: ExpressionValue<OutputExpression> = TryEvalFrom::try_eval_from(&head, ctx)?;

    eprintln!("[path] eval_path: head (b): {:?}", head);
    eprintln!("[path] eval_path: components (b): {:?}", components);
    let acc = match head {
        ExpressionValue::Expression(Expression::Composite(CompositeValue::ObjectValue(Some(
            ..
        )))) => Ok(&head),
        _ => Err(try_eval_from_err!(
            "Path head must evaluate to composite ObjectValue"
        )),
    }?;

    let res = components
        .into_iter()
        .fold_while(Ok(acc), |acc, key| {
            eprintln!("TryEval PathValue -> OutputExpression: member: {}", key);

            if let Ok(value) = acc {
                eprintln!("TryEval PathValue -> OutputExpression: value: {:?}", value);

                if let ExpressionValue::Expression(Expression::Composite(
                    CompositeValue::ObjectValue(Some(box ref fields)),
                )) = *value
                {
                    let next = fields.into_iter().filter(|e| e.key() == key).nth(0);
                    if let Some(ref value) = next.map(|e| e.value()) {
                        return Done(Ok(value));
                    };
                };

                return Done(Ok(value));
            };

            Done(Err(try_process_from_err!("Missing next object.")))
        })
        .into_inner();

    res.map(|r| r.to_owned())
}

impl TryEvalFrom<PathValue<ProcessedExpression>> for ExpressionValue<OutputExpression> {
    fn try_eval_from(
        src: &PathValue<ProcessedExpression>,
        ctx: &mut OutputContext,
    ) -> DocumentProcessingResult<Self> {
        eval_path(src, ctx)
    }
}

impl TryEvalFrom<PathValue<OutputExpression>> for ExpressionValue<OutputExpression> {
    fn try_eval_from(
        src: &PathValue<OutputExpression>,
        ctx: &mut OutputContext,
    ) -> DocumentProcessingResult<Self> {
        eval_path(src, ctx)
    }
}
