use error::*;
use traits::*;
use expressions::*;

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
        let iter = components.iter();

        let reduce = ReducePipelineIter::new(ctx, iter);

        let reduced_components: Vec<_> = reduce.collect();
        let reduced_components: Vec<_> = ok_or_error(reduced_components)?.collect();

        Ok(ReducedPipelineValue::new(head, reduced_components))
    }
}

#[derive(Debug)]
pub struct ReducePipelineIter<
    'ctx,
    'a,
    S: Iterator<Item = &'a PipelineComponentValue<ProcessedExpression>>,
> {
    ctx: &'ctx mut ProcessingContext,
    iter: S,
    last_state: ReducePipelineIterState,
    buf: Option<Vec<String>>,
}

#[allow(dead_code)]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
enum ReducePipelineIterState {
    NoState,
    Symbol,
    PipelineOp,
}

impl<'ctx, 'a, S: Iterator<Item = &'a PipelineComponentValue<ProcessedExpression>>> Iterator
    for ReducePipelineIter<'ctx, 'a, S>
{
    type Item = DocumentProcessingResult<ReducedPipelineComponent<ProcessedExpression>>;

    fn next(&mut self) -> Option<Self::Item> {
        let next = self.yield_next();

        match next {
            Ok(Some(v)) => Some(Ok(v)),
            Ok(None) => None,

            Err(e) => Some(Err(e)),
        }
    }
}

impl<'ctx, 'a, S: Iterator<Item = &'a PipelineComponentValue<ProcessedExpression>>>
    ReducePipelineIter<'ctx, 'a, S>
{
    pub fn new(ctx: &'ctx mut ProcessingContext, iter: S) -> Self {
        ReducePipelineIter {
            ctx: ctx,
            iter: iter,
            last_state: ReducePipelineIterState::NoState,
            buf: None,
        }
    }

    fn yield_next(
        &mut self,
    ) -> DocumentProcessingResult<Option<ReducedPipelineComponent<ProcessedExpression>>> {
        let next = self.iter.next();
        let was_nostate = self.last_state == ReducePipelineIterState::NoState;
        let was_pathcomp = self.last_state == ReducePipelineIterState::Symbol;

        if let Some(next) = next {
            let is_pathcomp = next.is_member();

            match next {
                &PipelineComponentValue::Member(ref name) => {
                    return Ok(Some(ReducedPipelineComponent::Member(name.to_owned())));
                }

                &PipelineComponentValue::MethodCall(ref mth, ref params, _) => {
                    let params = params.as_ref().map_or(vec![], |v| v.to_owned());
                    // let argc = params.as_ref().map_or(0, |v| v.len());
                    self.ctx.push_child_scope();
                    let op = match mth.as_str() {
                        "map" => {
                            self.ctx.bind_ident(
                                "item".to_owned(),
                                CommonBindings::CurrentItem(Default::default()),
                            )?;
                            let expr = params[0].value().to_owned();

                            if let Some(ref cond) = params.get(1) {
                                let cond = cond.value().to_owned();
                                Some(ReducedMethodCall::MapIf(expr, cond))
                            } else {
                                Some(ReducedMethodCall::Map(expr))
                            }
                        }

                        "filter" => {
                            self.ctx.bind_ident(
                                "item".to_owned(),
                                CommonBindings::CurrentItem(Default::default()),
                            )?;
                            let expr = params[0].value().to_owned();

                            Some(ReducedMethodCall::Filter(expr))
                        }

                        "reduce" => {
                            let expr = params[0].value().to_owned();
                            let initial = params[1].value().to_owned();

                            self.ctx.bind_ident(
                                "item".to_owned(),
                                CommonBindings::CurrentItem(Default::default()),
                            )?;

                            if let Some(ref cond) = params.get(2) {
                                let cond = cond.value().to_owned();
                                Some(ReducedMethodCall::ReduceIf(expr, cond, initial))
                            } else {
                                Some(ReducedMethodCall::Reduce(expr, initial))
                            }
                        }

                        "min" => {
                            self.ctx.bind_ident(
                                "item".to_owned(),
                                CommonBindings::CurrentItem(Default::default()),
                            )?;

                            let expr = params[0].value().to_owned();
                            Some(ReducedMethodCall::MaxBy(expr))
                        }

                        "max" => {
                            self.ctx.bind_ident(
                                "item".to_owned(),
                                CommonBindings::CurrentItem(Default::default()),
                            )?;

                            let expr = params[0].value().to_owned();
                            Some(ReducedMethodCall::MaxBy(expr))
                        }

                        "count" => {
                            self.ctx.bind_ident(
                                "item".to_owned(),
                                CommonBindings::CurrentItem(Default::default()),
                            )?;

                            let expr = params[0].value().to_owned();
                            Some(ReducedMethodCall::Count(expr))
                        }

                        _ => None,
                    };
                    self.ctx.pop_scope();

                    if let Some(op) = op {
                        return Ok(Some(ReducedPipelineComponent::PipelineOp(op)));
                    };
                }

                _ => {}
            };
        }

        Ok(None)
    }
}
