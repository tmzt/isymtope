use error::*;
use traits::*;
use expressions::*;

///
/// Apply condition to item
///
pub fn apply_cond(
    item: &ExpressionValue<OutputExpression>,
    idx: Option<usize>,
    key: Option<&str>,
    cond: &ExpressionValue<OutputExpression>,
    ctx: &mut OutputContext,
) -> DocumentProcessingResult<bool> {
    ctx.push_child_scope();

    // Bind `index` if provided
    if let Some(idx) = idx {
        let index_value = ExpressionValue::Primitive(Primitive::Int32Val(idx as i32));
        let index_binding = CommonBindings::CurrentItemIndex;
        eprintln!("[pipeline] apply_filter: index: {:?}", index_value);
        ctx.bind_loop_value(index_binding, index_value)?;
    }

    // Bind `key` if provided
    if let Some(key) = key {
        let key_value = ExpressionValue::Primitive(Primitive::StringVal(key.to_owned()));
        let key_binding = CommonBindings::CurrentItemKey;
        eprintln!("[pipeline] apply_filter: key: {:?}", key_value);
        ctx.bind_loop_value(key_binding, key_value)?;
    }

    // Always bind `item`
    let binding = CommonBindings::CurrentItem(Default::default());
    eprintln!("[pipeline] apply_filter: item: {:?}", item);

    ctx.bind_loop_value(binding, item.to_owned())?;

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
