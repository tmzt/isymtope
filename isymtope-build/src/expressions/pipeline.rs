use std::fmt::Debug;
use std::marker::PhantomData;

use itertools::Itertools;
use itertools::FoldWhile::*;

use common::*;
use error::*;
use traits::*;
use expressions::*;
use output::*;

#[allow(dead_code)]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct PipelineValue<T>(Box<ExpressionValue<T>>, Box<Vec<PipelineComponentValue<T>>>);

impl<T> PipelineValue<T> {
    pub fn new(e: ExpressionValue<T>, v: Vec<PipelineComponentValue<T>>) -> Self {
        PipelineValue(Box::new(e), Box::new(v))
    }

    pub fn head(&self) -> &ExpressionValue<T> {
        self.0.as_ref()
    }

    pub fn components<'a>(&'a self) -> impl Iterator<Item = &'a PipelineComponentValue<T>> {
        let box ref v = self.1;
        v.iter()
    }

    pub fn has_components(&self) -> bool {
        let box ref v = self.1;
        !v.is_empty()
    }

    pub fn is_member_path(&self) -> bool {
        let box ref components = self.1;

        components.iter().all(|c| match *c {
            PipelineComponentValue::Member(..) => true,
            _ => false,
        })
    }
}

impl<T: Debug> MapIdents<T> for PipelineValue<T> {
    fn map_idents(self, ctx: &mut ProcessingContext) -> DocumentProcessingResult<Self> {
        let head = self.0.map_idents(ctx)?;
        let box components = self.1;
        let components: Vec<_> = components.into_iter().map(|c| c.map_idents(ctx)).collect();
        let components: Vec<_> = ok_or_error(components)?.collect();

        Ok(PipelineValue(Box::new(head), Box::new(components)))
    }
}

impl<I, O> TryProcessFrom<PipelineValue<I>> for PipelineValue<O>
where
    ExpressionValue<O>: TryProcessFrom<ExpressionValue<I>>,
    I: ::std::fmt::Debug,
    O: ::std::fmt::Debug,
{
    fn try_process_from(
        src: &PipelineValue<I>,
        ctx: &mut ProcessingContext,
    ) -> DocumentProcessingResult<Self> {
        let &box ref expr = &src.0;
        let &box ref pcv = &src.1;

        let expr: ExpressionValue<O> = TryProcessFrom::try_process_from(expr, ctx)?;
        let pcv: Vec<PipelineComponentValue<O>> = TryProcessFrom::try_process_from(pcv, ctx)?;

        Ok(PipelineValue(Box::new(expr), Box::new(pcv)))
    }
}

impl<I, O> TryEvalFrom<PipelineValue<I>> for PipelineValue<O>
where
    ExpressionValue<O>: TryEvalFrom<ExpressionValue<I>>,
    I: ::std::fmt::Debug,
    O: ::std::fmt::Debug,
{
    fn try_eval_from(
        src: &PipelineValue<I>,
        ctx: &mut OutputContext<ProcessedExpression>,
    ) -> DocumentProcessingResult<Self> {
        eprintln!("TryEval PipelineValue -> PipelineValue src: {:?}", src);

        let &box ref expr = &src.0;
        let &box ref pcv = &src.1;

        let expr: ExpressionValue<O> = TryEvalFrom::try_eval_from(expr, ctx)?;
        let pcv: Vec<PipelineComponentValue<O>> = TryEvalFrom::try_eval_from(pcv, ctx)?;

        Ok(PipelineValue(Box::new(expr), Box::new(pcv)))
    }
}

#[allow(dead_code)]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum PipelineComponentValue<T> {
    Member(String),
    MethodCall(String, Option<Vec<ParamValue<T>>>, PhantomData<T>),
}

impl<T> PipelineComponentValue<T> {
    pub fn is_member(&self) -> bool {
        match *self {
            PipelineComponentValue::Member(..) => true,
            _ => false,
        }
    }
}

impl<T: Debug> MapIdents<T> for PipelineComponentValue<T> {
    fn map_idents(self, ctx: &mut ProcessingContext) -> DocumentProcessingResult<Self> {
        Ok(match self {
            PipelineComponentValue::MethodCall(s, Some(params), _) => {
                let params: Vec<_> = params.into_iter().map(|p| p.map_idents(ctx)).collect();
                let params: Vec<_> = ok_or_error(params)?.collect();
                PipelineComponentValue::MethodCall(s, Some(params), Default::default())
            }

            _ => self,
        })
    }
}

impl<I, O> TryProcessFrom<PipelineComponentValue<I>> for PipelineComponentValue<O>
where
    ExpressionValue<O>: TryProcessFrom<ExpressionValue<I>>,
    I: ::std::fmt::Debug,
    O: ::std::fmt::Debug,
{
    fn try_process_from(
        src: &PipelineComponentValue<I>,
        ctx: &mut ProcessingContext,
    ) -> DocumentProcessingResult<Self> {
        match *src {
            PipelineComponentValue::Member(ref s) => {
                Ok(PipelineComponentValue::Member(s.to_owned()))
            }
            PipelineComponentValue::MethodCall(ref s, ref params, _) => {
                let params: Option<Vec<ParamValue<O>>> =
                    TryProcessFrom::try_process_from(params, ctx)?;
                Ok(PipelineComponentValue::MethodCall(
                    s.to_owned(),
                    params,
                    Default::default(),
                ))
            }
        }
    }
}

impl<I, O> TryEvalFrom<PipelineComponentValue<I>> for PipelineComponentValue<O>
where
    ExpressionValue<O>: TryEvalFrom<ExpressionValue<I>>,
    I: ::std::fmt::Debug,
    O: ::std::fmt::Debug,
{
    fn try_eval_from(
        src: &PipelineComponentValue<I>,
        ctx: &mut OutputContext<ProcessedExpression>,
    ) -> DocumentProcessingResult<Self> {
        match *src {
            PipelineComponentValue::Member(ref s) => {
                Ok(PipelineComponentValue::Member(s.to_owned()))
            }
            PipelineComponentValue::MethodCall(ref s, ref params, _) => {
                let params: Option<Vec<ParamValue<O>>> = TryEvalFrom::try_eval_from(params, ctx)?;
                Ok(PipelineComponentValue::MethodCall(
                    s.to_owned(),
                    params,
                    Default::default(),
                ))
            }
        }
    }
}

///
/// Evaluate reduced pipeline
///
fn apply_filter(
    cond: &ExpressionValue<ProcessedExpression>,
    expr: &ExpressionValue<OutputExpression>,
    ctx: &mut OutputContext<ProcessedExpression>,
) -> DocumentProcessingResult<Vec<ExpressionValue<OutputExpression>>> {
    let arr = match *expr {
        ExpressionValue::Expression(Expression::Composite(CompositeValue::ArrayValue(Some(
            box ref arr,
        )))) => Ok(Some(arr)),
        ExpressionValue::Expression(Expression::Composite(CompositeValue::ArrayValue(None))) => {
            Ok(None)
        }
        _ => Err(try_eval_from_err!(
            "Can only filter on array values at this time."
        )),
    }?;

    ctx.push_child_scope();

    if !arr.is_some() {
        return Ok(vec![]);
    }

    let res: Vec<_> = arr.unwrap()
        .iter()
        .map(move |param| {
            let value = param.value();

            let binding = CommonBindings::CurrentItem(Default::default());
            let item_value: ExpressionValue<OutputExpression> =
                TryEvalFrom::try_eval_from(value, ctx)?;
            eprintln!("[pipeline] apply_filter: item_value: {:?}", item_value);

            ctx.push_child_scope();
            ctx.bind_loop_value(binding, item_value)?;

            eprintln!("[pipeline] apply_filter: cond (a): {:?}", cond);

            // Evaluate processed expression
            let cond: ExpressionValue<OutputExpression> = TryEvalFrom::try_eval_from(cond, ctx)?;
            eprintln!("[pipeline] apply_filter: cond (b): {:?}", cond);

            // Evaluate bindings
            let cond: ExpressionValue<OutputExpression> = TryEvalFrom::try_eval_from(&cond, ctx)?;
            eprintln!("[pipeline] apply_filter: cond (c): {:?}", cond);

            // Evaluate condition as boolean
            let cond: bool = TryEvalFrom::try_eval_from(&cond, ctx)?;
            eprintln!("[pipeline] apply_filter: cond (d): {:?}", cond);

            ctx.pop_scope();

            if cond {
                return Ok(Some(value.to_owned()));
            };

            Ok(None)
        })
        .collect();

    let res: Vec<_> = ok_or_error(res)?.filter_map(|el| el).collect();

    Ok(res)
}

fn eval_reduced_pipeline(
    src: &ReducedPipelineValue<ProcessedExpression>,
    ctx: &mut OutputContext<ProcessedExpression>,
) -> DocumentProcessingResult<ExpressionValue<OutputExpression>> {
    let head: ExpressionValue<OutputExpression> = TryEvalFrom::try_eval_from(src.head(), ctx)?;

    let res = src.components()
        .fold_while(Ok(head), |acc, x| {
            let value = acc.unwrap();

            if let ReducedPipelineComponent::PipelineOp(ref method) = *x {
                return match *method {
                    ReducedMethodCall::Filter(ref cond) => {
                        eprintln!(
                            "[pipeline] eval_reduced_pipeline: (filter) cond: {:?}",
                            cond
                        );
                        let res = apply_filter(cond, &value, ctx);

                        match res {
                            Ok(res) => {
                                let params: Vec<_> =
                                    res.into_iter().map(|el| ParamValue::new(el)).collect();
                                let expr = ExpressionValue::Expression(Expression::Composite(
                                    CompositeValue::ArrayValue(Some(Box::new(params))),
                                ));
                                Continue(Ok(expr))
                            }
                            Err(e) => Done(Err(e)),
                        }
                    }

                    ReducedMethodCall::Count(ref cond) => {
                        eprintln!("[pipeline] eval_reduced_pipeline: (count) cond: {:?}", cond);
                        let res = apply_filter(cond, &value, ctx);

                        match res {
                            Ok(res) => {
                                let count = res.into_iter().count();
                                let expr =
                                    ExpressionValue::Primitive(Primitive::Int32Val(count as i32));
                                Continue(Ok(expr))
                            }
                            Err(e) => Done(Err(e)),
                        }
                    }

                    _ => Continue(Ok(value)),
                };
            };

            Continue(Ok(value))
        })
        .into_inner();

    res
}

impl TryEvalFrom<ReducedPipelineValue<ProcessedExpression>> for ExpressionValue<OutputExpression> {
    fn try_eval_from(
        src: &ReducedPipelineValue<ProcessedExpression>,
        ctx: &mut OutputContext<ProcessedExpression>,
    ) -> DocumentProcessingResult<Self> {
        eval_reduced_pipeline(src, ctx)
    }
}
