use std::cell::Cell;
use std::fmt::Debug;
use std::marker::PhantomData;

use itertools::Itertools;
use itertools::FoldWhile::*;

use error::*;
use traits::*;
use expressions::*;

#[derive(Debug)]
pub enum PipelineState {
    Indexed(Vec<ExpressionValue<ProcessedExpression>>),
    Keyed(Vec<(String, ExpressionValue<ProcessedExpression>)>),
    Single(ExpressionValue<ProcessedExpression>),
    Empty
}

pub struct PipelineEval<'ctx, I> where I: Iterator<Item = ReducedPipelineComponent<ProcessedExpression>> {
    components: I,
    state: Cell<PipelineState>,
    ctx: &'ctx mut OutputContext
}

pub enum PipelineStep {
    Finished(DocumentProcessingResult<PipelineState>),
    Continue
}

impl<'ctx, I> PipelineEval<'ctx, I> where I: Iterator<Item = ReducedPipelineComponent<ProcessedExpression>> {
    pub fn create(
        components: I,
        head: ExpressionValue<ProcessedExpression>,
        ctx: &'ctx mut OutputContext,
    ) -> DocumentProcessingResult<Self> {
        let state = Cell::new(PipelineState::try_from_expression(head)?);

        Ok(Self {
            components: components,
            state: state,
            ctx: ctx,
        })
    }
}

fn do_filter(
    state: PipelineState,
    cond: &ExpressionValue<ProcessedExpression>,
    ctx: &mut OutputContext,
) -> DocumentProcessingResult<PipelineState> {
    eprintln!(
        "[pipeline] eval_reduced_pipeline: (do_filter) pipeline state: {:?}",
        state
    );

    match state {
        PipelineState::Single(item) => {
            let res = apply_cond(&item, None, None, cond, ctx)?;
            if !res {
                return Ok(PipelineState::Empty)
            }
            Ok(PipelineState::Single(item))
        }

        PipelineState::Indexed(v) => {
            let iter = v.into_iter().enumerate();
            let mut next = Vec::with_capacity(16);
            for (idx, expr) in iter {
                let res = apply_cond(&expr, Some(idx), None, cond, ctx)?;
                if res {
                    next.push(expr);
                }
            }

            Ok(PipelineState::Indexed(next))
        }

        PipelineState::Keyed(v) => {
            let iter = v.into_iter().enumerate();
            let mut next = Vec::with_capacity(16);
            for (idx, item) in iter {
                let res = apply_cond(&item.1, Some(idx), Some(&item.0), cond, ctx)?;
                if res {
                    next.push(item);
                }
            }

            Ok(PipelineState::Keyed(next))
        }

        PipelineState::Empty => {
            Ok(state)
        }
    }
}

fn do_count(
    state: PipelineState,
    cond: Option<&ExpressionValue<ProcessedExpression>>,
    ctx: &mut OutputContext,
) -> DocumentProcessingResult<PipelineState> {
    eprintln!(
        "[pipeline] eval_reduced_pipeline: (do_count) pipeline state: {:?}",
        state
    );

    match state {
        PipelineState::Single(item) => {
            if let Some(cond) = cond {
                let res = apply_cond(&item, None, None, cond, ctx)?;
                let count = if res { 1 } else { 0 };
                return Ok(PipelineState::Single(ExpressionValue::Primitive(Primitive::Int32Val(count))))
            }
            Ok(PipelineState::Single(ExpressionValue::Primitive(Primitive::Int32Val(1))))
        }

        PipelineState::Indexed(v) => {
            let iter = v.into_iter().enumerate();
            let mut count = 0;
            for (idx, expr) in iter {
                if let Some(cond) = cond {
                    let res = apply_cond(&expr, Some(idx), None, cond, ctx)?;
                    if res {
                        count += 1;
                    }
                } else { count += 1; }
            }
            Ok(PipelineState::Single(ExpressionValue::Primitive(Primitive::Int32Val(count))))
        }

        PipelineState::Keyed(v) => {
            let iter = v.into_iter().enumerate();
            let mut count = 0;
            for (idx, item) in iter {
                if let Some(cond) = cond {
                    let res = apply_cond(&item.1, Some(idx), Some(&item.0), cond, ctx)?;
                    if res {
                        count += 1;
                    }
                } else { count += 1; }
            }

            Ok(PipelineState::Single(ExpressionValue::Primitive(Primitive::Int32Val(count))))
        }

        PipelineState::Empty => {
            Ok(PipelineState::Single(ExpressionValue::Primitive(Primitive::Int32Val(0))))
        }
    }
}

fn do_first(
    state: PipelineState,
    cond: Option<&ExpressionValue<ProcessedExpression>>,
    ctx: &mut OutputContext,
) -> DocumentProcessingResult<PipelineState> {
    eprintln!(
        "[pipeline] eval_reduced_pipeline: (do_first) pipeline state: {:?}",
        state
    );

    match state {
        PipelineState::Single(item) => {
            if let Some(cond) = cond {
                let res = apply_cond(&item, None, None, cond, ctx)?;
                if res {
                    return Ok(PipelineState::Single(item.clone()));
                }
            }
            Ok(PipelineState::Empty)
        }

        PipelineState::Indexed(v) => {
            let iter = v.into_iter().enumerate();
            let mut count = 0;
            for (idx, expr) in iter {
                if let Some(cond) = cond {
                    let res = apply_cond(&expr, Some(idx), None, cond, ctx)?;
                    if res {
                        return Ok(PipelineState::Single(expr.to_owned()));
                    }
                }
                break;
            }

            Ok(PipelineState::Empty)
        }

        PipelineState::Keyed(v) => {
            let iter = v.into_iter().enumerate();
            let mut count = 0;
            for (idx, item) in iter {
                if let Some(cond) = cond {
                    let res = apply_cond(&item.1, Some(idx), Some(&item.0), cond, ctx)?;
                    if res {
                        return Ok(PipelineState::Single(item.1.to_owned()));
                    }
                }
                break;
            }

            Ok(PipelineState::Empty)
        }

        PipelineState::Empty => {
            Ok(PipelineState::Single(ExpressionValue::Primitive(Primitive::Int32Val(0))))
        }
    }
}

pub fn apply_method(state: PipelineState, method: &ReducedMethodCall<ProcessedExpression>, ctx: &mut OutputContext) -> DocumentProcessingResult<PipelineState> {
    match *method {
        ReducedMethodCall::Filter(ref cond) => do_filter(state, cond, ctx),

        ReducedMethodCall::CountIf(ref cond) => do_count(state, Some(cond), ctx),
        ReducedMethodCall::Count => do_count(state, None, ctx),

        ReducedMethodCall::First => do_first(state, None, ctx),

        _ => Err(try_eval_from_err!("Unimplemented reduced pipeline method"))
    }
}

impl<'ctx, I> Iterator for PipelineEval<'ctx, I> where I: Iterator<Item = ReducedPipelineComponent<ProcessedExpression>> {
    type Item = PipelineStep;

    fn next(&mut self) -> Option<Self::Item> {
        let next = self.components.next();
        match next {
            Some(ReducedPipelineComponent::PipelineOp(method)) => {
                let state = self.state.replace(PipelineState::Empty);
                let res = apply_method(state, &method, self.ctx);

                match res {
                    Ok(state) => {
                        self.state.replace(state);
                        Some(PipelineStep::Continue)
                    },

                    Err(e) => {
                        Some(PipelineStep::Finished(Err(e)))
                    }
                }
            }

            Some(_) => {
                let err = Err(try_eval_from_err!("Invalid ReducedPipelineComponent in pipeline."));
                Some(PipelineStep::Finished(err))
            }

            None => {
                let state = self.state.replace(PipelineState::Empty);
                Some(PipelineStep::Finished(Ok(state)))
            }
        }
    }
}

impl PipelineState {
    pub fn try_from_expression(expr: ExpressionValue<ProcessedExpression>) -> DocumentProcessingResult<Self> {
        eprintln!(
            "[pipeline state] create pipeline state: try_from_expression: expr: {:?}",
            expr
        );

        match expr {
            ExpressionValue::Composite(CompositeValue::ArrayValue(ArrayValue(Some(v)))) => {
                let v: Vec<_> = v.into_iter()
                    .map(move |e| e.value().clone())
                    .collect();

                Ok(PipelineState::Indexed(v))
            }

            ExpressionValue::Composite(CompositeValue::ObjectValue(ObjectValue(Some(v)))) => {
                let v: Vec<_> = v.into_iter()
                    .map(move |e| (e.key().to_owned(), e.value().to_owned()))
                    // .map(move |e| e.value().clone())
                    .collect();

                Ok(PipelineState::Keyed(v))
            }

            ExpressionValue::Composite(CompositeValue::MapValue(MapValue(ref keyname, None))) => {
                let v = vec![];

                Ok(PipelineState::Keyed(v))
            }

            ExpressionValue::Composite(CompositeValue::MapValue(MapValue(ref keyname, Some(box ref v)))) => {
                let keyname = keyname.as_ref().map_or_else(|| "id".to_owned(), |s| s.to_owned());
                let v: Vec<_> = ok_or_error(v.into_iter()
                    .map(move |obj| {
                        if let Some(ExpressionValue::Primitive(Primitive::StringVal(ref key))) = obj.get(&keyname) {
                            Ok((key.to_owned(), ExpressionValue::Composite(CompositeValue::ObjectValue(obj.to_owned()))))
                        } else {
                            eprintln!("Missing or unsupported key {} on object in map with keyname.", keyname);
                            Err(try_eval_from_err!(format!("Missing key {} on object in map with keyname.", keyname)))
                        }
                    }))?
                    .collect();

                Ok(PipelineState::Keyed(v))
            }

            _ => Err(try_eval_from_err!("Invalid expression to initialize pipeline"))
        }
    }

    pub fn into_single_value(self) -> DocumentProcessingResult<ExpressionValue<ProcessedExpression>> {
        match self {
            PipelineState::Single(e) => {
                Ok(e)
            }
            _ => Err(try_eval_from_err!("Invalid state to convert to single value"))
        }
    }

    pub fn into_array_value(self) -> DocumentProcessingResult<ArrayValue<ProcessedExpression>> {
        match self {
            PipelineState::Single(e) => {
                let v = vec![ParamValue::new(e)];
                Ok(ArrayValue(Some(Box::new(v))))
            }

            PipelineState::Indexed(v) => {
                let v: Vec<_> = v.into_iter().map(move |e| ParamValue::new(e)).collect();
                Ok(ArrayValue(Some(Box::new(v))))
            }

            PipelineState::Keyed(v) => {
                let v: Vec<_> = v.into_iter().map(move |(_, value)| ParamValue::new(value)).collect();
                Ok(ArrayValue(Some(Box::new(v))))
            }

            PipelineState::Empty => {
                Ok(ArrayValue(None))
            }

            // _ => Err(try_eval_from_err!("Invalid final pipeline state"))
        }
    }
}

pub fn eval_reduced_pipeline(
    src: &ReducedPipelineValue<ProcessedExpression>,
    ctx: &mut OutputContext,
) -> DocumentProcessingResult<PipelineState> {
    eprintln!(
        "[pipeline] eval_reduced_pipeline: src: {:?}",
        src
    );

    let head = eval_expression(src.head(), ctx)?;
    let iter = src.components().into_iter().cloned();
    let eval = PipelineEval::create(iter, head, ctx)?;

    for step in eval {
        if let PipelineStep::Finished(res) = step {
            return res;
        }
    }

    Err(try_eval_from_err!("Reached end of pipeline evalutation without PipelineStep::Finished, this should not happen."))
}

pub fn eval_reduced_pipeline_to_value(
    src: &ReducedPipelineValue<ProcessedExpression>,
    ctx: &mut OutputContext,
) -> DocumentProcessingResult<ExpressionValue<ProcessedExpression>> {
    let final_state = eval_reduced_pipeline(src, ctx)?;

    match final_state {
        PipelineState::Indexed(..) | PipelineState::Keyed(..) | PipelineState::Empty => {
            final_state.into_array_value()
                .map(|arr| ExpressionValue::Composite(CompositeValue::ArrayValue(arr)))
        }

        PipelineState::Single(e) => {
            Ok(e)
        }

        _ => Err(try_eval_from_err!("Invalid final pipeline state"))
    }
}

impl TryEvalFrom<ReducedPipelineValue<ProcessedExpression>> for ExpressionValue<ProcessedExpression> {
    fn try_eval_from(
        src: &ReducedPipelineValue<ProcessedExpression>,
        ctx: &mut OutputContext,
    ) -> DocumentProcessingResult<Self> {
        let final_state = eval_reduced_pipeline(src, ctx)?;

        match final_state {
            PipelineState::Indexed(..) => {
                final_state.into_array_value()
                    .map(|arr| ExpressionValue::Composite(CompositeValue::ArrayValue(arr)))
            }

            PipelineState::Single(e) => {
                Ok(e)
            }

            _ => Err(try_eval_from_err!("Invalid final pipeline state"))
        }
    }
}

#[cfg(test)]
mod test {
    use std::rc::Rc;
    use super::*;
    use contexts::*;
    use objects::*;

    #[derive(Default, Debug)]
    struct TestDefaults {}

    impl ContextDefaultsProvider for TestDefaults {
        fn doc(&self) -> &Document {
            unimplemented!()
        }

        fn reducer_value(
            &mut self,
            _: &str,
        ) -> DocumentProcessingResult<ReducerValue> { unimplemented!() }
    }

    #[test]
    fn test_single_value_as_array() {
        let expr: ExpressionValue<ProcessedExpression> = ExpressionValue::Primitive(Primitive::StringVal("test".to_owned()));
        let state = PipelineState::Single(expr);

        let array_value = state.into_array_value().unwrap();

        let el = ExpressionValue::Primitive(Primitive::StringVal("test".to_owned()));
        assert_eq!(array_value, ArrayValue(Some(Box::new(vec![ParamValue::new(el)]))));
    }

    #[test]
    fn test_indexed_state_as_array() {
        let expr_0: ExpressionValue<ProcessedExpression> = ExpressionValue::Primitive(Primitive::StringVal("zero".to_owned()));
        let expr_1: ExpressionValue<ProcessedExpression> = ExpressionValue::Primitive(Primitive::StringVal("one".to_owned()));
        let v = vec![expr_0, expr_1];
        let state = PipelineState::Indexed(v);

        let array_value = state.into_array_value().unwrap();

        let res_0: ExpressionValue<ProcessedExpression> = ExpressionValue::Primitive(Primitive::StringVal("zero".to_owned()));
        let res_1: ExpressionValue<ProcessedExpression> = ExpressionValue::Primitive(Primitive::StringVal("one".to_owned()));
        assert_eq!(array_value, ArrayValue(Some(Box::new(vec![ParamValue::new(res_0), ParamValue::new(res_1)]))));
    }

    #[test]
    fn test_apply_cond_with_single_item() {
        let defaults: Rc<TestDefaults> = Default::default();
        let mut ctx = DefaultOutputContext::create(defaults);

        let expr: ExpressionValue<ProcessedExpression> = ExpressionValue::Primitive(Primitive::StringVal("zero".to_owned()));
        let cond = ExpressionValue::Expression(
            Expression::BinaryOp(BinaryOp(BinaryOpType::EqualTo,
                Box::new(ExpressionValue::Binding(CommonBindings::CurrentItem(Default::default()), Default::default())),
                Box::new(ExpressionValue::Primitive(Primitive::StringVal("zero".into())))
            ))
        );

        let res = apply_cond(&expr, None, None, &cond, &mut ctx).unwrap();
        assert!(res);
    }

    #[test]
    fn test_apply_cond_with_index() {
        let defaults: Rc<TestDefaults> = Default::default();
        let mut ctx = DefaultOutputContext::create(defaults);

        let expr: ExpressionValue<ProcessedExpression> = ExpressionValue::Primitive(Primitive::StringVal("one".to_owned()));
        let cond = ExpressionValue::Expression(
            Expression::BinaryOp(BinaryOp(BinaryOpType::EqualTo,
                Box::new(ExpressionValue::Binding(CommonBindings::CurrentItemIndex, Default::default())),
                Box::new(ExpressionValue::Primitive(Primitive::Int32Val(1)))
            ))
        );

        let res = apply_cond(&expr, Some(1), None, &cond, &mut ctx).unwrap();
        assert!(res);
    }

    #[test]
    fn test_apply_cond_with_key() {
        let defaults: Rc<TestDefaults> = Default::default();
        let mut ctx = DefaultOutputContext::create(defaults);

        let expr: ExpressionValue<ProcessedExpression> = ExpressionValue::Primitive(Primitive::StringVal("one".to_owned()));
        let key = "b".to_owned();

        let cond = ExpressionValue::Expression(
            Expression::BinaryOp(BinaryOp(BinaryOpType::EqualTo,
                Box::new(ExpressionValue::Binding(CommonBindings::CurrentItemKey, Default::default())),
                Box::new(ExpressionValue::Primitive(Primitive::StringVal("b".into())))
            ))
        );

        let res = apply_cond(&expr, Some(1), Some(&key), &cond, &mut ctx).unwrap();
        assert!(res);
    }

    #[test]
    fn test_indexed_state_method() {
        let defaults: Rc<TestDefaults> = Default::default();
        let mut ctx = DefaultOutputContext::create(defaults);

        let expr_0: ExpressionValue<ProcessedExpression> = ExpressionValue::Primitive(Primitive::StringVal("zero".to_owned()));
        let expr_1: ExpressionValue<ProcessedExpression> = ExpressionValue::Primitive(Primitive::StringVal("one".to_owned()));
        let v = vec![expr_0, expr_1];
        let state = PipelineState::Indexed(v);

        let cond = ExpressionValue::Expression(
            Expression::BinaryOp(BinaryOp(BinaryOpType::EqualTo,
                Box::new(ExpressionValue::Binding(CommonBindings::CurrentItemIndex, Default::default())),
                Box::new(ExpressionValue::Primitive(Primitive::Int32Val(1)))
            ))
        );
        let method: ReducedMethodCall<ProcessedExpression> = ReducedMethodCall::Filter(cond);

        let state = apply_method(state, &method, &mut ctx).unwrap();
        let array_value = state.into_array_value().unwrap();

        let res_1: ExpressionValue<ProcessedExpression> = ExpressionValue::Primitive(Primitive::StringVal("one".to_owned()));
        assert_eq!(array_value, ArrayValue(Some(Box::new(vec![ParamValue::new(res_1)]))));
    }

    #[test]
    fn test_indexed_state_method_with_value() {
        let defaults: Rc<TestDefaults> = Default::default();
        let mut ctx = DefaultOutputContext::create(defaults);

        let expr_0: ExpressionValue<ProcessedExpression> = ExpressionValue::Primitive(Primitive::StringVal("zero".to_owned()));
        let expr_1: ExpressionValue<ProcessedExpression> = ExpressionValue::Primitive(Primitive::StringVal("one".to_owned()));
        let v = vec![expr_0, expr_1];
        let state = PipelineState::Indexed(v);

        let cond = ExpressionValue::Expression(
            Expression::BinaryOp(BinaryOp(BinaryOpType::EqualTo,
                Box::new(ExpressionValue::Binding(CommonBindings::CurrentItem(Default::default()), Default::default())),
                Box::new(ExpressionValue::Primitive(Primitive::StringVal("zero".into())))
            ))
        );
        let method: ReducedMethodCall<ProcessedExpression> = ReducedMethodCall::Filter(cond);

        let state = apply_method(state, &method, &mut ctx).unwrap();
        let array_value = state.into_array_value().unwrap();

        let res_0: ExpressionValue<ProcessedExpression> = ExpressionValue::Primitive(Primitive::StringVal("zero".to_owned()));
        assert_eq!(array_value, ArrayValue(Some(Box::new(vec![ParamValue::new(res_0)]))));
    }

    #[test]
    fn test_eval_pipeline_condition_with_value() {
        let defaults: Rc<TestDefaults> = Default::default();
        let mut ctx = DefaultOutputContext::create(defaults);

        let expr_0: ExpressionValue<ProcessedExpression> = ExpressionValue::Primitive(Primitive::StringVal("zero".to_owned()));
        let expr_1: ExpressionValue<ProcessedExpression> = ExpressionValue::Primitive(Primitive::StringVal("one".to_owned()));
        let arr = ArrayValue(Some(Box::new(vec![ParamValue::new(expr_0), ParamValue::new(expr_1)])));
        let head = ExpressionValue::Composite(CompositeValue::ArrayValue(arr));

        let cond = ExpressionValue::Expression(
            Expression::BinaryOp(BinaryOp(BinaryOpType::EqualTo,
                Box::new(ExpressionValue::Binding(CommonBindings::CurrentItem(Default::default()), Default::default())),
                Box::new(ExpressionValue::Primitive(Primitive::StringVal("zero".into())))
            ))
        );
        let method: ReducedMethodCall<ProcessedExpression> = ReducedMethodCall::Filter(cond);
        let components = vec![ReducedPipelineComponent::PipelineOp(method)];
        let src = ReducedPipelineValue::new(head, components);

        let state = eval_reduced_pipeline(&src, &mut ctx).unwrap();
        let array_value = state.into_array_value().unwrap();

        let res_0: ExpressionValue<ProcessedExpression> = ExpressionValue::Primitive(Primitive::StringVal("zero".to_owned()));
        assert_eq!(array_value, ArrayValue(Some(Box::new(vec![ParamValue::new(res_0)]))));
    }

    #[test]
    fn test_eval_pipeline_countif_with_value() {
        let defaults: Rc<TestDefaults> = Default::default();
        let mut ctx = DefaultOutputContext::create(defaults);

        let expr_0: ExpressionValue<ProcessedExpression> = ExpressionValue::Primitive(Primitive::StringVal("zero".to_owned()));
        let expr_1: ExpressionValue<ProcessedExpression> = ExpressionValue::Primitive(Primitive::StringVal("one".to_owned()));
        let arr = ArrayValue(Some(Box::new(vec![ParamValue::new(expr_0), ParamValue::new(expr_1)])));
        let head = ExpressionValue::Composite(CompositeValue::ArrayValue(arr));

        let cond = ExpressionValue::Expression(
            Expression::BinaryOp(BinaryOp(BinaryOpType::EqualTo,
                Box::new(ExpressionValue::Binding(CommonBindings::CurrentItem(Default::default()), Default::default())),
                Box::new(ExpressionValue::Primitive(Primitive::StringVal("zero".into())))
            ))
        );
        let method: ReducedMethodCall<ProcessedExpression> = ReducedMethodCall::CountIf(cond);
        let components = vec![ReducedPipelineComponent::PipelineOp(method)];
        let src = ReducedPipelineValue::new(head, components);

        let state = eval_reduced_pipeline(&src, &mut ctx).unwrap();
        let single = state.into_single_value().unwrap();

        let res_0: ExpressionValue<ProcessedExpression> = ExpressionValue::Primitive(Primitive::Int32Val(1));
        assert_eq!(single, res_0);
    }
}
