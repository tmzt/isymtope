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
    Indexed(Vec<ExpressionValue<OutputExpression>>),
    Single(ExpressionValue<OutputExpression>),
    Empty
}

pub struct PipelineEval<'ctx, I> where I: Iterator<Item = ReducedPipelineComponent<OutputExpression>> {
    components: I,
    state: Cell<PipelineState>,
    ctx: &'ctx mut OutputContext
}

pub enum PipelineStep {
    Finished(DocumentProcessingResult<PipelineState>),
    Continue
}

impl<'ctx, I> PipelineEval<'ctx, I> where I: Iterator<Item = ReducedPipelineComponent<OutputExpression>> {
    pub fn create(
        components: I,
        head: ExpressionValue<OutputExpression>,
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
    cond: &ExpressionValue<OutputExpression>,
    ctx: &mut OutputContext,
) -> DocumentProcessingResult<PipelineState> {
    eprintln!(
        "[pipeline] eval_reduced_pipeline: (do_filter) pipeline state: {:?}",
        state
    );

    match state {
        PipelineState::Single(item) => {
            let res = apply_cond_indexed(&item, 0, cond, ctx)?;
            if !res {
                return Ok(PipelineState::Empty)
            }
            Ok(PipelineState::Single(item))
        }

        PipelineState::Indexed(v) => {
            let iter = v.into_iter().enumerate();
            let mut next = Vec::with_capacity(16);
            let mut ctr = 0;
            for (idx, expr) in iter {
                let res = apply_cond_indexed(&expr, idx, cond, ctx)?;
                if res {
                    next.push(expr);
                }
            }

            Ok(PipelineState::Indexed(next))
        }

        PipelineState::Empty => {
            Ok(state)
        }
    }
}

pub fn apply_method(state: PipelineState, method: &ReducedMethodCall<OutputExpression>, ctx: &mut OutputContext) -> DocumentProcessingResult<PipelineState> {
    match *method {
        ReducedMethodCall::Filter(ref cond) => do_filter(state, cond, ctx),

        _ => Err(try_eval_from_err!("Unimplemented reduced pipeline method"))
    }
}

impl<'ctx, I> Iterator for PipelineEval<'ctx, I> where I: Iterator<Item = ReducedPipelineComponent<OutputExpression>> {
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
    pub fn try_from_expression(expr: ExpressionValue<OutputExpression>) -> DocumentProcessingResult<Self> {
        match expr {
            ExpressionValue::Expression(Expression::Composite(CompositeValue::ArrayValue(ArrayValue(Some(v))))) => {
                let v: Vec<_> = v.into_iter()
                    .map(move |e| e.value().clone())
                    .collect();

                Ok(PipelineState::Indexed(v))
            }

            _ => Err(try_eval_from_err!("Invalid expression to initialize pipeline"))
        }
    }

    pub fn into_array_value(self) -> DocumentProcessingResult<ArrayValue<OutputExpression>> {
        match self {
            PipelineState::Single(e) => {
                let v = vec![ParamValue::new(e)];
                Ok(ArrayValue(Some(Box::new(v))))
            }

            PipelineState::Indexed(v) => {
                let v: Vec<_> = v.into_iter().map(move |e| ParamValue::new(e)).collect();
                Ok(ArrayValue(Some(Box::new(v))))
            },

            _ => Err(try_eval_from_err!("Invalid final pipeline state"))
        }
    }
}

fn eval_reduced_pipeline(
    src: &ReducedPipelineValue<OutputExpression>,
    ctx: &mut OutputContext,
) -> DocumentProcessingResult<PipelineState> {
    Ok(PipelineState::Empty)
}

impl TryEvalFrom<ReducedPipelineValue<OutputExpression>> for ExpressionValue<OutputExpression> {
    fn try_eval_from(
        src: &ReducedPipelineValue<OutputExpression>,
        ctx: &mut OutputContext,
    ) -> DocumentProcessingResult<Self> {
        let final_state = eval_reduced_pipeline(src, ctx)?;

        match final_state {
            PipelineState::Indexed(iter) => {
                // let iter = *iter;
                let v: Vec<_> = iter.into_iter().map(move |e| ParamValue::new(e)).collect();
                Ok(ExpressionValue::Expression(Expression::Composite(CompositeValue::ArrayValue(ArrayValue(Some(Box::new(v)))))))
            },

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
        let expr: ExpressionValue<OutputExpression> = ExpressionValue::Primitive(Primitive::StringVal("test".to_owned()));
        let state = PipelineState::Single(expr);

        let array_value = state.into_array_value().unwrap();

        let el = ExpressionValue::Primitive(Primitive::StringVal("test".to_owned()));
        assert_eq!(array_value, ArrayValue(Some(Box::new(vec![ParamValue::new(el)]))));
    }

    #[test]
    fn test_indexed_state_as_array() {
        let expr_0: ExpressionValue<OutputExpression> = ExpressionValue::Primitive(Primitive::StringVal("zero".to_owned()));
        let expr_1: ExpressionValue<OutputExpression> = ExpressionValue::Primitive(Primitive::StringVal("one".to_owned()));
        let v = vec![expr_0, expr_1];
        let state = PipelineState::Indexed(v);

        let array_value = state.into_array_value().unwrap();

        let res_0: ExpressionValue<OutputExpression> = ExpressionValue::Primitive(Primitive::StringVal("zero".to_owned()));
        let res_1: ExpressionValue<OutputExpression> = ExpressionValue::Primitive(Primitive::StringVal("one".to_owned()));
        assert_eq!(array_value, ArrayValue(Some(Box::new(vec![ParamValue::new(res_0), ParamValue::new(res_1)]))));
    }

    #[test]
    fn test_indexed_state_method() {
        let defaults: Rc<TestDefaults> = Default::default();
        let mut ctx = DefaultOutputContext::create(defaults);

        let expr_0: ExpressionValue<OutputExpression> = ExpressionValue::Primitive(Primitive::StringVal("zero".to_owned()));
        let expr_1: ExpressionValue<OutputExpression> = ExpressionValue::Primitive(Primitive::StringVal("one".to_owned()));
        let v = vec![expr_0, expr_1];
        let state = PipelineState::Indexed(v);

        let cond = ExpressionValue::Expression(
            Expression::BinaryOp(BinaryOpType::EqualTo,
                Box::new(ExpressionValue::Binding(CommonBindings::CurrentItemIndex, Default::default())),
                Box::new(ExpressionValue::Primitive(Primitive::Int32Val(1)))
            )
        );
        let method: ReducedMethodCall<OutputExpression> = ReducedMethodCall::Filter(cond);

        let state = apply_method(state, &method, &mut ctx).unwrap();
        let array_value = state.into_array_value().unwrap();

        let res_1: ExpressionValue<OutputExpression> = ExpressionValue::Primitive(Primitive::StringVal("one".to_owned()));
        assert_eq!(array_value, ArrayValue(Some(Box::new(vec![ParamValue::new(res_1)]))));
    }

    #[test]
    fn test_indexed_state_method_with_value() {
        let defaults: Rc<TestDefaults> = Default::default();
        let mut ctx = DefaultOutputContext::create(defaults);

        let expr_0: ExpressionValue<OutputExpression> = ExpressionValue::Primitive(Primitive::StringVal("zero".to_owned()));
        let expr_1: ExpressionValue<OutputExpression> = ExpressionValue::Primitive(Primitive::StringVal("one".to_owned()));
        let v = vec![expr_0, expr_1];
        let state = PipelineState::Indexed(v);

        let cond = ExpressionValue::Expression(
            Expression::BinaryOp(BinaryOpType::EqualTo,
                Box::new(ExpressionValue::Binding(CommonBindings::CurrentItem(Default::default()), Default::default())),
                Box::new(ExpressionValue::Primitive(Primitive::StringVal("zero".into())))
            )
        );
        let method: ReducedMethodCall<OutputExpression> = ReducedMethodCall::Filter(cond);

        let state = apply_method(state, &method, &mut ctx).unwrap();
        let array_value = state.into_array_value().unwrap();

        let res_0: ExpressionValue<OutputExpression> = ExpressionValue::Primitive(Primitive::StringVal("zero".to_owned()));
        assert_eq!(array_value, ArrayValue(Some(Box::new(vec![ParamValue::new(res_0)]))));
    }
}
