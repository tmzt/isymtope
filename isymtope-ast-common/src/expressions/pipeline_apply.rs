use error::*;
use traits::*;
use expressions::*;

///
/// Apply condition to item
///
pub fn apply_cond_indexed(
    item: &ExpressionValue<OutputExpression>,
    idx: usize,
    cond: &ExpressionValue<OutputExpression>,
    ctx: &mut OutputContext,
) -> DocumentProcessingResult<bool> {
    ctx.push_child_scope();

    // // TODO: Change to CurrentItemKey
    // let binding = CommonBindings::CurrentItemIndex;
    // eprintln!("[pipeline] apply_filter: item_key: {:?}", key);

    // let key = ExpressionValue::Primitive(Primitive::StringVal(key.to_owned()));
    // ctx.bind_loop_value(binding, key)?;

    let index_value = ExpressionValue::Primitive(Primitive::Int32Val(idx as i32));
    let index_binding = CommonBindings::CurrentItemIndex;
    ctx.bind_loop_value(index_binding, index_value)?;

    let binding = CommonBindings::CurrentItem(Default::default());
    // let item_value: ExpressionValue<OutputExpression> =
    //     TryEvalFrom::try_eval_from(cur_item, ctx)?;
    eprintln!("[pipeline] apply_filter: item: {:?}", item);

    ctx.bind_loop_value(binding, item.clone())?;

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
