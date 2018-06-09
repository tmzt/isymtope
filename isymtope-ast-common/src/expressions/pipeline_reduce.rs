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
        let head = src.head();
        // let head: ExpressionValue<ProcessedExpression> =
        //     TryProcessFrom::try_process_from(src.head(), ctx)?;
        let components: Vec<PipelineComponentValue<SourceExpression>> =
            src.components().map(|c| c.to_owned()).collect();
        let components: Vec<PipelineComponentValue<ProcessedExpression>> =
            TryProcessFrom::try_process_from(&components, ctx)?;

        let mut iter = components.into_iter();
        let mut member_path: Vec<String> = Vec::with_capacity(16);
        let mut components: Vec<ReducedPipelineComponent<ProcessedExpression>> = Vec::with_capacity(16);

        loop {
            if let Some(PipelineComponentValue::Member(ref s)) = iter.next() {
                member_path.push(s.to_owned());
            } else {
                break;
            };
        };

        loop {
            if let Some(PipelineComponentValue::MethodCall(ref mth, ref params, _)) = iter.next() {
                let params = params.as_ref().map_or(vec![], |v| v.to_owned());
                let op = map_method(ctx, mth, &params)?;

                components.push(ReducedPipelineComponent::PipelineOp(op));
            } else {
                break;
            };
        };

        let n = member_path.len();
        if n > 0 {
            // if let ExpressionValue::Expression(Expression::Ident(ref s, _)) = head {
            if let ExpressionValue::Expression(Expression::Ident(..)) = head {
                let head = ExpressionValue::Expression(Expression::Path(PathValue::new(head.to_owned(), Some(member_path)), Default::default()));
                // let path_components: Vec<_> = vec![s.to_owned()].into_iter()
                //     .chain(member_path.into_iter()).map(|s| PathComponentValue::Member(s, Default::default())).collect();
                // let head = ExpressionValue::Expression(Expression::Path(PathValue::new(path_components, Default::default()), Default::default()));

                let head: ExpressionValue<ProcessedExpression> =
                    TryProcessFrom::try_process_from(&head, ctx)?;
                
                return Ok(ReducedPipelineValue::new(head, components));
            };
        };

        let head: ExpressionValue<ProcessedExpression> =
            TryProcessFrom::try_process_from(head, ctx)?;

        Ok(ReducedPipelineValue::new(head, components))
    }
}
