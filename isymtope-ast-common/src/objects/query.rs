use std::marker::PhantomData;

use itertools::Itertools;
use itertools::FoldWhile::{Continue, Done};

use error::*;
use traits::*;
use expressions::*;
// use output::*;

#[allow(dead_code)]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum QueryComponent<T> {
    CaseWhere(
        Box<ExpressionValue<T>>,
        Box<ExpressionValue<T>>,
        PhantomData<T>,
    ),
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Query<T> {
    name: String,
    params: FormalParams<T>,
    components: Vec<QueryComponent<T>>,
}

impl<T> Query<T> {
    /// Consumes parameters and returns new Query
    // pub fn new(name: String, params: FormalParams<T>, expression: ExpressionValue<T>) -> Self {
    pub fn new(name: String, params: FormalParams<T>, components: Vec<QueryComponent<T>>) -> Self {
        Query {
            name: name,
            params: params,
            components: components, // expression: expression
        }
    }

    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn params<'a>(&'a self) -> Option<impl Iterator<Item = &'a str>> {
        self.params.params()
    }

    pub fn components<'a>(&'a self) -> impl Iterator<Item = &'a QueryComponent<T>> {
        self.components.iter()
    }

    // pub fn expression(&self) -> &ExpressionValue<T> { &self.expression }
}

// impl<I, O> TryProcessFrom<QueryComponent<I>> for QueryComponent<O> where ExpressionValue<O>: TryProcessFrom<ExpressionValue<I>>, I: ::std::fmt::Debug, O: ::std::fmt::Debug {
//     fn try_process_from(src: &QueryComponent<I>, ctx: &mut ProcessingContext) -> DocumentProcessingResult<Self> {
//         debug!("TryProcess QueryComponent -> QueryComponent: src: {:?}", src);

//         Ok(match *src {
//             QueryComponent::CaseWhere(box ref expr, box ref cond, _) => {
//                 let cond: ExpressionValue<O> = TryProcessFrom::try_process_from(cond, ctx)?;

//                 let expr = match *expr {
//                     // ExpressionValue::Expression(Expression::Path(ref p, _)) => {
//                     ExpressionValue::Expression(Expression::Pipeline(ref p, _)) => {
//                         // Pipeline -> ReducedPipeline
//                         let reduced: ReducedPipelineValue<O> = TryProcessFrom::try_process_from(p, ctx)?;

//                         ExpressionValue::Expression(Expression::ReducedPipeline(reduced, Default::default()))
//                     }

//                     _ => TryProcessFrom::try_process_from(expr, ctx)?
//                 };

//                 QueryComponent::CaseWhere(Box::new(cond), Box::new(expr), Default::default())
//             }
//         })
//     }
// }

impl TryProcessFrom<QueryComponent<SourceExpression>> for QueryComponent<ProcessedExpression> {
    fn try_process_from(
        src: &QueryComponent<SourceExpression>,
        ctx: &mut ProcessingContext,
    ) -> DocumentProcessingResult<Self> {
        debug!(
            "TryProcess QueryComponent -> QueryComponent: src: {:?}",
            src
        );

        Ok(match *src {
            QueryComponent::CaseWhere(box ref expr, box ref cond, _) => {
                let cond: ExpressionValue<ProcessedExpression> =
                    TryProcessFrom::try_process_from(cond, ctx)?;

                let expr = match *expr {
                    ExpressionValue::Expression(Expression::Pipeline(ref p, _)) => {
                        let reduced: ReducedPipelineValue<
                            ProcessedExpression,
                        > = TryProcessFrom::try_process_from(p, ctx)?;

                        ExpressionValue::Expression(Expression::ReducedPipeline(
                            reduced,
                            Default::default(),
                        ))
                    }

                    _ => TryProcessFrom::try_process_from(expr, ctx)?,
                };

                QueryComponent::CaseWhere(Box::new(expr), Box::new(cond), Default::default())
            }
        })
    }
}

fn case_where(
    ctx: &mut OutputContext,
    cond: &ExpressionValue<ProcessedExpression>,
    expr: &ExpressionValue<ProcessedExpression>,
) -> DocumentProcessingResult<(bool, ExpressionValue<OutputExpression>)> {
    let cond: ExpressionValue<OutputExpression> = TryEvalFrom::try_eval_from(cond, ctx)?;
    let expr: ExpressionValue<OutputExpression> = TryEvalFrom::try_eval_from(expr, ctx)?;

    // let cond: Primitive = TryProcessFrom::try_process_from(cond)?;
    // let cond: bool = TryProcessFrom::try_process_from(&cond, ctx)?;
    let cond: bool = TryEvalFrom::try_eval_from(&cond, ctx)?;
    eprintln!(
        "[Query] TryEval QueryCall -> ExpressionValue: cond value: {:?}",
        cond
    );

    Ok((cond, expr))
}

// impl TryProcessFrom<QueryCall<SourceExpression>> for QueryCall<ProcessedExpression> {
//     fn try_process_from(src: &QueryCall<SourceExpression>, ctx: &mut ProcessingContext) -> DocumentProcessingResult<Self> {
//         eprintln!("[Query] TryProcess QueryCall -> QueryCall: src: {:?}", src);
//         let params: Vec<QueryParamValue<ProcessedExpression>> = TryProcessFrom::try_process_from(&src.1, ctx)?;
//         eprintln!("[Query] TryProcess QueryCall -> QueryCall: params: {:?}", params);

//         Ok(QueryCall::new(src.0.to_owned(), params))
//     }
// }

impl TryEvalFrom<QueryCall<ProcessedExpression>> for ExpressionValue<OutputExpression> {
    fn try_eval_from(
        query_call: &QueryCall<ProcessedExpression>,
        ctx: &mut OutputContext,
    ) -> DocumentProcessingResult<Self> {
        eprintln!(
            "[Query] TryEval QueryCall -> ExpressionValue: query_call: {:?}",
            query_call
        );

        let name = query_call.name();
        eprintln!(
            "[Query] TryEval QueryCall -> ExpressionValue: name: {:?}",
            name
        );

        let params: Vec<_> = query_call.params().collect();
        eprintln!(
            "[Query] TryEval QueryCall -> ExpressionValue: params: {:?}",
            params
        );

        let query = ctx.doc()
            .query(name)
            .map(|v| Ok(v.to_owned()))
            .unwrap_or_else(|| Err(try_process_from_err!("Could not locate query by name.")))?;

        info!("Query: {:?}", query);

        // Prepare context
        ctx.push_child_scope();

        eprintln!("[Query] params: {:?}", params);

        for prop in params {
            let binding =
                CommonBindings::NamedQueryParam(prop.key().to_owned(), Default::default());
            let value: ExpressionValue<OutputExpression> =
                TryEvalFrom::try_eval_from(prop.value(), ctx)?;

            eprintln!(
                "[Query] Adding binding {:?} with value [{:?}]",
                binding, value
            );
            ctx.bind_value(binding, value)?;
        }

        let acc: DocumentProcessingResult<Option<ExpressionValue<OutputExpression>>> = Ok(None);

        let res = query
            .components()
            .fold_while(acc, |acc, component| match *component {
                QueryComponent::CaseWhere(box ref expr, box ref cond, _) => {
                    eprintln!(
                        "[Query] TryEval QueryCall -> ExpressionValue: case cond: {:?} expr {:?}",
                        cond, expr
                    );
                    let res = case_where(ctx, cond, expr);
                    eprintln!(
                        "[Query] TryEval QueryCall -> ExpressionValue: res: {:?}",
                        res
                    );

                    match res {
                        Err(e) => Done(Err(e)),

                        Ok((true, expr)) => Done(Ok(Some(expr))),

                        _ => Continue(Ok(None)),
                    }
                }
            })
            .into_inner()?;

        eprintln!(
            "[Query] TryEval QueryCall -> ExpressionValue: res: {:?}",
            res
        );

        if res.is_none() {
            return Err(try_eval_from_err!(
                "Unable to evaluate query, result is None."
            ));
        }
        let res = res.unwrap();

        ctx.pop_scope();
        Ok(res)
    }
}
