
use std::iter;
use model::*;
use processing::*;
use scope::*;


pub struct EvalProps<'ctx, 'doc, 'a, I: IntoIterator<Item = ActualPropRef<'a>>> {
    iter: <I as iter::IntoIterator>::IntoIter,
    doc: &'doc Document,
    ctx: &'ctx mut Context
}

impl<'ctx, 'doc, 'a, I: IntoIterator<Item = ActualPropRef<'a>>> EvalProps<'ctx, 'doc, 'a, I>
{
    pub fn new(iter: I, doc: &'doc Document, ctx: &'ctx mut Context) -> Self {
        EvalProps {
            iter: iter.into_iter(),
            doc: doc,
            ctx: ctx
        }
    }
}

impl<'ctx, 'doc, 'a, I: IntoIterator<Item = ActualPropRef<'a>>> Iterator for EvalProps<'ctx, 'doc, 'a, I>
{
    type Item = Prop;

    fn next(&mut self) -> Option<Self::Item> {
        let doc = self.doc;
        self.iter.next()
            .and_then(|(key, expr)| {
                let expr = expr.and_then(|e| self.ctx.eval_expr(doc, e)).or_else(|| expr.map(|e| e.to_owned()));
                Some((key.to_owned(), expr))
            })
    }
}