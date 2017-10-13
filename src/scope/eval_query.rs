
use std::iter;
use model::*;
use processing::*;
use scope::*;


pub struct EvalQuery<'ctx, 'doc, 'a, I: IntoIterator<Item = &'a QueryComponent>> {
    iter: <I as iter::IntoIterator>::IntoIter,
    next_comp: Option<&'a QueryComponent>,
    doc: &'doc Document,
    ctx: &'ctx mut Context
}

impl<'ctx, 'doc, 'a, I: IntoIterator<Item = &'a QueryComponent>> EvalQuery<'ctx, 'doc, 'a, I>
{
    pub fn new(iter: I, doc: &'doc Document, ctx: &'ctx mut Context) -> Self {
        EvalQuery {
            iter: iter.into_iter(),
            next_comp: None,
            doc: doc,
            ctx: ctx
        }
    }

    fn apply_rule(&mut self, comp: &'a QueryComponent) -> Option<ExprValue> {
        match *comp {
            QueryComponent::CaseWhere(box ref val, box ref cond) => {
                let cond = self.ctx.eval_expr(self.doc, cond).unwrap_or_else(|| cond.to_owned());

                if let Some(b) = cond.bool_value() {
                    let val = self.ctx.eval_expr(self.doc, val).unwrap_or_else(|| val.to_owned());
                    if b { return Some(val); };
                };

                None
            }

            _ => None
        }
    }
}

impl<'ctx, 'doc, 'a, I: IntoIterator<Item = &'a QueryComponent>> Iterator for EvalQuery<'ctx, 'doc, 'a, I>
{
    type Item = Option<ExprValue>;

    fn next(&mut self) -> Option<Self::Item> {

        // First iteration
        if self.next_comp.is_none() {
            self.next_comp = self.iter.next();
        };

        let res = self.next_comp
            .and_then(|comp| self.apply_rule(comp))
            .map(|expr| self.ctx.reduce_expr_or_return_same(&expr));

        if res.is_some() {
            // Got result
            return Some(res);
        };

        self.next_comp = self.iter.next();

        if self.next_comp.is_none() {
            // No match found
            return None;
        }

        // Continue iterating
        Some(None)
    }
}