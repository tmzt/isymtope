use std::fmt::Debug;
use std::marker::PhantomData;

use itertools::Itertools;
use itertools::FoldWhile::*;

use error::*;
use traits::*;
use expressions::*;


enum PipelineState<'p, T: Debug + 'p> {
    Iterable(Box<Iterator<Item = &'p PipelineItem<T>> + 'p>, PhantomData<T>),
    Single(PipelineItem<T>, PhantomData<T>),
    Empty(PhantomData<T>)
}

#[derive(Debug)]
enum PipelineItem<T> {
    Bare(ExpressionValue<T>, usize, PhantomData<T>),
    Named(String, ExpressionValue<T>, PhantomData<T>),
}

impl<T> PipelineItem<T> {
    pub fn inner(&self) -> &ExpressionValue<T> {
        match self {
            PipelineItem::Bare(ref v, _, _) => v,
            PipelineItem::Named(_, ref v, _) => v,
        }
    }

    pub fn into_inner(self) -> ExpressionValue<T> {
        match self {
            PipelineItem::Bare(v, _, _) => v,
            PipelineItem::Named(_, v, _) => v,
        }
    }
}

///
/// Apply condition to item
///
fn filter_item(
    cond: &ExpressionValue<OutputExpression>,
    item: &PipelineItem<OutputExpression>,
    ctx: &mut OutputContext,
) -> DocumentProcessingResult<bool> {
    ctx.push_child_scope();

    let cur_item;
    match item {
        PipelineItem::Bare(ref v, _, _) => {
            cur_item = v;
        }

        PipelineItem::Named(ref key, ref v, _) => {
            // TODO: Change to CurrentItemKey
            let binding = CommonBindings::CurrentItemIndex;
            eprintln!("[pipeline] apply_filter: item_key: {:?}", key);
            cur_item = v;

            let key = ExpressionValue::Primitive(Primitive::StringVal(key.to_owned()));
            ctx.bind_loop_value(binding, key)?;
        }
    };

    let binding = CommonBindings::CurrentItem(Default::default());
    let item_value: ExpressionValue<OutputExpression> =
        TryEvalFrom::try_eval_from(cur_item, ctx)?;
    eprintln!("[pipeline] apply_filter: item_value: {:?}", item_value);

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

    Ok(cond)
}

fn eval_reduced_pipeline<'p>(
    src: &ReducedPipelineValue<OutputExpression>,
    ctx: &mut OutputContext,
) -> DocumentProcessingResult<PipelineState<'p, OutputExpression>> {
    Ok(PipelineState::Empty(Default::default()))
}

impl TryEvalFrom<ReducedPipelineValue<OutputExpression>> for ExpressionValue<OutputExpression> {
    fn try_eval_from(
        src: &ReducedPipelineValue<OutputExpression>,
        ctx: &mut OutputContext,
    ) -> DocumentProcessingResult<Self> {
        let final_state = eval_reduced_pipeline(src, ctx)?;

        match final_state {
            PipelineState::Iterable(iter, _) => {
                // let iter = *iter;
                let v: Vec<_> = iter.map(move |e| ParamValue::new(e.inner().to_owned())).collect();
                Ok(ExpressionValue::Expression(Expression::Composite(CompositeValue::ArrayValue(ArrayValue(Some(Box::new(v)))))))
            },

            _ => Err(try_eval_from_err!("Invalid final pipeline state"))
        }
    }
}
