
use std::iter;
use model::*;
use processing::*;
use scope::*;


pub struct EvalReducedPipeline<'ctx, 'doc, 'a, I: IntoIterator<Item = &'a ReducedPipelineComponent>> {
    iter: <I as iter::IntoIterator>::IntoIter,
    next_comp: Option<&'a ReducedPipelineComponent>,
    cur_val: Option<ExprValue>,
    doc: &'doc Document,
    ctx: &'ctx mut Context
}

impl<'ctx, 'doc, 'a, I: IntoIterator<Item = &'a ReducedPipelineComponent>> EvalReducedPipeline<'ctx, 'doc, 'a, I>
{
    pub fn new(head: Option<&ExprValue>, iter: I, doc: &'doc Document, ctx: &'ctx mut Context) -> Self {
        let cur_val = head.map(|head| head.to_owned());

        EvalReducedPipeline {
            iter: iter.into_iter(),
            next_comp: None,
            cur_val: cur_val,
            doc: doc,
            ctx: ctx
        }
    }

    fn apply_filter(&mut self, cond: &ExprValue, expr: &ExprValue) -> Option<ExprValue> {
        // let expr = self.cur_val.as_ref().and_then(|e| self.ctx.eval_expr(self.doc, e).unwrap_or_else(|| e.to_owned()));
        let expr = self.ctx.eval_expr(self.doc, expr).unwrap_or_else(|| expr.to_owned());
        let cond = self.ctx.eval_expr(self.doc, cond).unwrap_or_else(|| cond.to_owned());
        let arr = match expr { ExprValue::LiteralArray(Some(ref arr)) => Some(arr.iter()), _ => None };

        let binding = BindingType::MapItemBinding;
        let mut ctx = &mut self.ctx;

        if let Some(arr) = arr {
            let arr: Vec<_> = arr.filter_map(move |el| {
                ctx.push_child_scope();
                ctx.add_binding_value(&binding, el.to_owned());

                let b = cond.bool_value().map_or(false, |b| b);
                let res = if b { Some(el.to_owned()) } else { None };

                ctx.pop_scope();
                res
            }).collect();

            return Some(ExprValue::LiteralArray(Some(arr)));
        };

        None
    }

    fn apply_pipeline_op(&mut self, pipeline_op: &'a ReducedMethodType, expr: &ExprValue) -> Option<ExprValue> {
        match *pipeline_op {
            ReducedMethodType::Filter(ref cond) => self.apply_filter(cond, expr),
            _ => None
        }
    }

    fn apply_component(&mut self, comp: &'a ReducedPipelineComponent, expr: Option<&ExprValue>) -> Option<ExprValue> {
        match (comp, expr) {
            (&ReducedPipelineComponent::Symbol(ref sym), _) => Some(ExprValue::SymbolReference(sym.to_owned())),

            (&ReducedPipelineComponent::PipelineOp(ref op), Some(ref expr)) => {
                self.apply_pipeline_op(op, expr)
            }

            _ => None
        }
    }
}

impl<'ctx, 'doc, 'a, I: IntoIterator<Item = &'a ReducedPipelineComponent>> Iterator for EvalReducedPipeline<'ctx, 'doc, 'a, I>
{
    type Item = Option<ExprValue>;

    fn next(&mut self) -> Option<Self::Item> {

        // First iteration
        if self.next_comp.is_none() {
            self.next_comp = self.iter.next();
        };

        let expr = self.cur_val.to_owned();

        let res = self.next_comp
            .and_then(|comp| self.apply_component(comp, expr.as_ref()))
            .map(|expr| self.ctx.reduce_expr_or_return_same(&expr));

        if res.is_some() {
            // Update current value
            self.cur_val = res;
        };

        self.next_comp = self.iter.next();

        if self.next_comp.is_none() {
            // Return current value as result
            return Some(self.cur_val.to_owned());
        }

        // Continue iterating
        Some(None)
    }
}