
use std::fmt::Debug;

use error::*;
use traits::*;
use expressions::*;


impl TryProcessFrom<PipelineValue<SourceExpression>> for ReducedPipelineValue<ProcessedExpression> {
    fn try_process_from(src: &PipelineValue<SourceExpression>, ctx: &mut ProcessingContext) -> DocumentProcessingResult<Self> {
        let head: ExpressionValue<ProcessedExpression> = TryProcessFrom::try_process_from(src.head(), ctx)?;
        let components: Vec<PipelineComponentValue<SourceExpression>> = src.components().map(|c| c.to_owned()).collect();
        let components: Vec<PipelineComponentValue<ProcessedExpression>> = TryProcessFrom::try_process_from(&components, ctx)?;
        let iter = components.iter();

        let reduce = ReducePipelineIter::new(ctx, iter);

        let reduced_components: Vec<_> = reduce.collect();
        let reduced_components: Vec<_> = ok_or_error(reduced_components)?.collect();

        Ok(ReducedPipelineValue::new(head, reduced_components))
    }
}

// impl TryProcessFrom<PathValue<SourceExpression>> for ReducedPipelineValue<ProcessedExpression> {
//     fn try_process_from(src: &PathValue<SourceExpression>, ctx: &mut ProcessingContext) -> DocumentProcessingResult<Self> {
//         let head: ExpressionValue<ProcessedExpression> = TryProcessFrom::try_process_from(src.head(), ctx)?;
//         let components: Vec<_> = src.components().map(|v| v.map(|c| c.to_owned()).collect()).unwrap_or_default();
//         // Convert to pipeline
//         let components: Vec<PipelineComponentValue<ProcessedExpression>> = TryProcessFrom::try_process_from(&components, ctx)?;
//         let iter = components.iter();

//         let reduce = ReducePipelineIter::new(ctx, iter);

//         let reduced_components: Vec<_> = reduce.collect();
//         let reduced_components: Vec<_> = ok_or_error(reduced_components)?.collect();

//         Ok(ReducedPipelineValue::new(head, reduced_components))
//     }
// }

#[derive(Debug)]
pub struct ReducePipelineIter<'ctx, 'a, S: Iterator<Item = &'a PipelineComponentValue<ProcessedExpression>>> {
    ctx: &'ctx mut ProcessingContext,
    iter: S,
    last_state: ReducePipelineIterState,
    buf: Option<Vec<String>>
}

#[allow(dead_code)]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
enum ReducePipelineIterState {
    NoState,
    Symbol,
    PipelineOp
}

impl<'ctx, 'a, S: Iterator<Item = &'a PipelineComponentValue<ProcessedExpression>>> Iterator for ReducePipelineIter<'ctx, 'a, S>
{
    type Item = DocumentProcessingResult<ReducedPipelineComponent<ProcessedExpression>>;

    fn next(&mut self) -> Option<Self::Item> {
        let next = self.yield_next();

        match next {
            Ok(Some(v)) => Some(Ok(v)),
            Ok(None) => None,

            Err(e) => Some(Err(e))
        }
    }
}

impl<'ctx, 'a, S: Iterator<Item = &'a PipelineComponentValue<ProcessedExpression>>> ReducePipelineIter<'ctx, 'a, S>
{
    pub fn new(ctx: &'ctx mut ProcessingContext, iter: S) -> Self {
        ReducePipelineIter {
            ctx: ctx,
            iter: iter,
            last_state: ReducePipelineIterState::NoState,
            buf: None
        }
   }

   fn push_s(&mut self, s: &str, clear: bool) {
        if clear { self.buf = None; }

        if let Some(ref mut buf) = self.buf {
            buf.push(s.to_owned());
        } else {
            self.buf = Some(vec![s.to_owned()]);
        };
    }

    fn yield_next(&mut self) -> DocumentProcessingResult<Option<ReducedPipelineComponent<ProcessedExpression>>> {
        let next = self.iter.next();
        let was_nostate = self.last_state == ReducePipelineIterState::NoState;
        let was_pathcomp = self.last_state == ReducePipelineIterState::Symbol;
        
        if let Some(next) = next {
            let is_pathcomp = next.is_member();

            match next {
                &PipelineComponentValue::Member(ref name) => {
                    return Ok(Some(ReducedPipelineComponent::Member(name.to_owned())));
                }

                // &PipelineComponentValue::Member(ref s)  if was_nostate => {
                //     self.push_s(s, true);
                // }

                // &PipelineComponentValue::Member(ref s) if was_pathcomp => {
                //     self.push_s(s, false);
                // }

                &PipelineComponentValue::MethodCall(ref mth, ref params, _) => {
                    let params = params.as_ref().map_or(vec![], |v| v.to_owned());
                    // let argc = params.as_ref().map_or(0, |v| v.len());
                    self.ctx.push_child_scope();
                    let op = match mth.as_str() {
                        "map" => {
                            // let param_ref = &params[0];

                            self.ctx.bind_ident("item".to_owned(), CommonBindings::CurrentItem(Default::default()))?;

                            // self.ctx.add_sym("item", Symbol::binding(&BindingType::MapItemBinding));
                            // self.ctx.add_sym("index", Symbol::binding(&BindingType::MapIndexBinding));

                            // let expr = self.ctx.reduce_expr_or_return_same(&param_ref.0);
                            // let expr = &param_ref.0.to_owned();

                            let expr = params[0].value().to_owned();
                            // let expr: ExpressionValue<ProcessedExpression> = TryProcessFrom::try_process_from(params[0].value(), &mut self.ctx)?;

                            if let Some(ref cond) = params.get(1) {
                                let cond = cond.value().to_owned();
                                Some(ReducedMethodCall::MapIf(expr, cond))
                            } else {
                                Some(ReducedMethodCall::Map(expr))
                            }
                        }

                        "filter" => {
                            // let param_ref = &params[0];
                            self.ctx.bind_ident("item".to_owned(), CommonBindings::CurrentItem(Default::default()))?;
                            // self.ctx.add_sym("item", Symbol::binding(&BindingType::MapItemBinding));

                            // let expr: ExpressionValue<ProcessedExpression> = TryProcessFrom::try_process_from(params[0].value(), &mut self.ctx)?;
                            let expr = params[0].value().to_owned();
                            Some(ReducedMethodCall::Filter(expr))
                        }

                        "reduce" => {
                            let expr = params[0].value().to_owned();
                            let initial = params[1].value().to_owned();

                            // let expr: ExpressionValue<ProcessedExpression> = TryProcessFrom::try_process_from(params[0].value(), &mut self.ctx)?;
                            // let initial: ExpressionValue<ProcessedExpression> = TryProcessFrom::try_process_from(params[1].value(), &mut self.ctx)?;

                            self.ctx.bind_ident("item".to_owned(), CommonBindings::CurrentItem(Default::default()))?;

                            if let Some(ref cond) = params.get(2) {
                                // let cond = self.ctx.reduce_expr_or_return_same(cond);
                                // let cond: ExpressionValue<ProcessedExpression> = TryProcessFrom::try_process_from(cond.value(), &mut self.ctx)?;
                                let cond = cond.value().to_owned();
                                Some(ReducedMethodCall::ReduceIf(expr, cond, initial))
                            } else {
                                Some(ReducedMethodCall::Reduce(expr, initial))
                            }
                        }

                        "min" => {
                            self.ctx.bind_ident("item".to_owned(), CommonBindings::CurrentItem(Default::default()))?;

                            let expr = params[0].value().to_owned();
                            Some(ReducedMethodCall::MaxBy(expr))
                        }

                        "max" => {
                            self.ctx.bind_ident("item".to_owned(), CommonBindings::CurrentItem(Default::default()))?;

                            let expr = params[0].value().to_owned();
                            Some(ReducedMethodCall::MaxBy(expr))
                        }

                        "count" => {
                            self.ctx.bind_ident("item".to_owned(), CommonBindings::CurrentItem(Default::default()))?;

                            let expr = params[0].value().to_owned();
                            Some(ReducedMethodCall::Count(expr))
                        }

                        _ => None
                    };
                    self.ctx.pop_scope();

                    if let Some(op) = op {
                        return Ok(Some(ReducedPipelineComponent::PipelineOp(op)));
                    };
                }

                // _ if !is_pathcomp && was_pathcomp => {
                //     if let Some(ref buf) = self.buf {
                //         let expr = match buf.len() {
                //             1 => ExpressionValue::Expression(Expression::Ident(buf[0].to_owned(), Default::default())),
                //             _ => {
                //                 let head = ExpressionValue::Expression(Expression::Ident(buf[0].to_owned(), Default::default()));
                //                 let rest: Vec<_> = buf.iter().skip(1)
                //                     .map(|p| PathComponentValue::Member(p.to_owned(), Default::default()))
                //                     .collect();
                //                 let path = PathValue::new(head, Some(rest));

                //                 ExpressionValue::Expression(Expression::Path(path, Default::default()))
                //             }
                //         };

                //         return Ok(Some(ReducedPipelineComponent::ExpressionValue(expr)));
                //         // return Some(ReducedPipelineComponent: Debug +:Symbol(sym));
                //     };
                // },

                _ => {}
            };
        }

        Ok(None)
    }
}