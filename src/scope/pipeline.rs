// #![allow(dead_code)]

use itertools::Itertools;

use model::*;
use scope::*;


#[allow(dead_code)]
#[derive(Debug)]
pub struct ReducePipelineIter<'ctx, 'head, 'a, S: Iterator<Item = &'a IterMethodPipelineComponent>> {
    ctx: &'ctx mut Context,
    head: Option<&'head ExprValue>,
    last_state: ReducePipelineIterState,
    buf: Option<Vec<String>>,
    iter: S
}

#[allow(dead_code)]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
enum ReducePipelineIterState {
    NoState,
    Symbol,
    PipelineOp
}

impl<'ctx, 'head, 'a, S: Iterator<Item = &'a IterMethodPipelineComponent>> ReducePipelineIter<'ctx, 'head, 'a, S>
{
    #[allow(dead_code)]
    pub fn new(ctx: &'ctx mut Context, head: Option<&'head ExprValue>, iter: S) -> Self {
        ReducePipelineIter {
            ctx: ctx,
            head: head,
            last_state: ReducePipelineIterState::NoState,
            buf: None,
            iter: iter
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
}

impl<'ctx, 'head, 'a, S: Iterator<Item = &'a IterMethodPipelineComponent>> Iterator for ReducePipelineIter<'ctx, 'head, 'a, S>
{
    type Item = ReducedPipelineComponent;

    fn next(&mut self) -> Option<Self::Item> {
        let next = self.iter.next();
        let was_nostate = self.last_state == ReducePipelineIterState::NoState;
        let was_pathcomp = self.last_state == ReducePipelineIterState::Symbol;
        
        if let Some(next) = next {
            let is_pathcomp = next.is_path();

            match next {
                &IterMethodPipelineComponent::PathComponent(ref s)  if was_nostate => {
                    self.push_s(s, true);
                }

                &IterMethodPipelineComponent::PathComponent(ref s) if was_pathcomp => {
                    self.push_s(s, false);
                }

                &IterMethodPipelineComponent::Method(ref mth, ref params) => {
                    let params = params.as_ref().map_or(vec![], |v| v.to_owned());
                    // let argc = params.as_ref().map_or(0, |v| v.len());
                    self.ctx.push_child_scope();
                    let op = match mth.as_str() {
                        "map" => {
                            let param_ref = &params[0];
                            self.ctx.add_sym("item", Symbol::binding(&BindingType::MapItemBinding));
                            self.ctx.add_sym("index", Symbol::binding(&BindingType::MapIndexBinding));

                            let expr = self.ctx.reduce_expr_or_return_same(&param_ref.0);

                            if let Some(ref cond) = param_ref.1 {
                                let cond = self.ctx.reduce_expr_or_return_same(cond);
                                Some(ReducedMethodType::MapIf(expr, cond))
                            } else {
                                Some(ReducedMethodType::Map(expr))
                            }
                        }

                        "filter" => {
                            let param_ref = &params[0];
                            self.ctx.add_sym("item", Symbol::binding(&BindingType::MapItemBinding));

                            let expr = self.ctx.reduce_expr_or_return_same(&param_ref.0);
                            Some(ReducedMethodType::Filter(expr))
                        }

                        "reduce" => {
                            let param_ref = &params[0];
                            let initial = params.get(1).and_then(|p| p.1.as_ref().map(|p| p.to_owned()));
                            let sym = Symbol::binding(&BindingType::MapItemBinding);
                            self.ctx.add_sym("item", sym);

                            let expr = self.ctx.reduce_expr_or_return_same(&param_ref.0);

                            if let Some(ref cond) = param_ref.1 {
                                let cond = self.ctx.reduce_expr_or_return_same(cond);
                                Some(ReducedMethodType::ReduceIf(expr, cond, initial))
                            } else {
                                Some(ReducedMethodType::Reduce(expr, initial))
                            }
                        }

                        "max" => Some(ReducedMethodType::Max),
                        "min" => Some(ReducedMethodType::Min),

                        _ => None
                    };
                    self.ctx.pop_scope();

                    if let Some(op) = op {
                        return Some(ReducedPipelineComponent::PipelineOp(op));
                    };
                }

                _ if !is_pathcomp && was_pathcomp => {
                    if let Some(ref buf) = self.buf {
                        let path = buf.join(".");
                        let sym = match buf.len() {
                            1 => Symbol::unresolved(&path),
                            _ => Symbol::unresolved_path(&path)
                        };

                        return Some(ReducedPipelineComponent::Symbol(sym));
                    };
                },

                _ => {}
            };
        }

        None
    }
}
