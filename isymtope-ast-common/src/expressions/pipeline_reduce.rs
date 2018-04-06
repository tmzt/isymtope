use itertools::Itertools;
use itertools::FoldWhile::{Continue, Done};

use error::*;
use traits::*;
use expressions::*;

fn map_method(ctx: &mut ProcessingContext, mth: &str, params: &Vec<ParamValue<ProcessedExpression>>) -> DocumentProcessingResult<ReducedMethodCall<ProcessedExpression>> {
    ctx.push_child_scope();
    let op = match mth {
        "map" => {
            ctx.bind_ident(
                "item".to_owned(),
                CommonBindings::CurrentItem(Default::default()),
            )?;
            let expr = params[0].value().to_owned();

            if let Some(ref cond) = params.get(1) {
                let cond = cond.value().to_owned();
                Ok(ReducedMethodCall::MapIf(expr, cond))
            } else {
                Ok(ReducedMethodCall::Map(expr))
            }
        }

        "filter" => {
            ctx.bind_ident(
                "item".to_owned(),
                CommonBindings::CurrentItem(Default::default()),
            )?;
            let expr = params[0].value().to_owned();

            Ok(ReducedMethodCall::Filter(expr))
        }

        "reduce" => {
            let expr = params[0].value().to_owned();
            let initial = params[1].value().to_owned();

            ctx.bind_ident(
                "item".to_owned(),
                CommonBindings::CurrentItem(Default::default()),
            )?;

            if let Some(ref cond) = params.get(2) {
                let cond = cond.value().to_owned();
                Ok(ReducedMethodCall::ReduceIf(expr, cond, initial))
            } else {
                Ok(ReducedMethodCall::Reduce(expr, initial))
            }
        }

        "min" => {
            ctx.bind_ident(
                "item".to_owned(),
                CommonBindings::CurrentItem(Default::default()),
            )?;

            let expr = params[0].value().to_owned();
            Ok(ReducedMethodCall::MaxBy(expr))
        }

        "max" => {
            ctx.bind_ident(
                "item".to_owned(),
                CommonBindings::CurrentItem(Default::default()),
            )?;

            let expr = params[0].value().to_owned();
            Ok(ReducedMethodCall::MaxBy(expr))
        }

        "count" => {
            ctx.bind_ident(
                "item".to_owned(),
                CommonBindings::CurrentItem(Default::default()),
            )?;

            let expr = params[0].value().to_owned();
            Ok(ReducedMethodCall::Count(expr))
        }

        "first" => {
            if let Some(ref cond) = params.get(0) {
                let cond = cond.value().to_owned();
                Ok(ReducedMethodCall::FirstWhere(cond))
            } else {
                Ok(ReducedMethodCall::First)
            }
        }

        _ => {
            Err(try_process_from_err!(format!("Unsupported pipeline method: {}", mth)))
        }
    }?;
    ctx.pop_scope();

    Ok(op)
}

impl TryProcessFrom<PipelineValue<SourceExpression>> for ReducedPipelineValue<ProcessedExpression> {
    fn try_process_from(
        src: &PipelineValue<SourceExpression>,
        ctx: &mut ProcessingContext,
    ) -> DocumentProcessingResult<Self> {
        let head: ExpressionValue<ProcessedExpression> =
            TryProcessFrom::try_process_from(src.head(), ctx)?;
        let components: Vec<PipelineComponentValue<SourceExpression>> =
            src.components().map(|c| c.to_owned()).collect();
        let components: Vec<PipelineComponentValue<ProcessedExpression>> =
            TryProcessFrom::try_process_from(&components, ctx)?;

        let reduced_components = components.iter()
            .fold_while(Ok(vec![]), |acc: DocumentProcessingResult<Vec<ReducedPipelineComponent<ProcessedExpression>>>, next| {
                let mut acc = acc.unwrap();

                match *next {
                    PipelineComponentValue::Member(ref name) => {
                        acc.push(ReducedPipelineComponent::Member(name.to_owned()));
                        Continue(Ok(acc))
                    }

                    PipelineComponentValue::MethodCall(ref mth, ref params, _) => {
                        let params = params.as_ref().map_or(vec![], |v| v.to_owned());
                        let op = map_method(ctx, mth, &params);

                        match op {
                            Ok(op) => {
                                acc.push(ReducedPipelineComponent::PipelineOp(op));
                                Continue(Ok(acc))
                            }

                            Err(e) => Done(Err(e))
                        }
                    }
                }
            }).into_inner()?;

        Ok(ReducedPipelineValue::new(head, reduced_components))
    }
}
