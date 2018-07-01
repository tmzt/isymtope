use std::cell::Cell;
use std::fmt::Debug;
use std::marker::PhantomData;

use itertools::Itertools;
use itertools::FoldWhile::*;

use error::*;
use traits::*;
use expressions::*;

pub enum PipelineState {
    // Indexed(Box<Iterator<Item = PipelineItem<OutputExpression>>>),
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
    // let state = self.state.replace(PipelineState::Empty);

    match state {
        PipelineState::Single(item) => {
            let res = apply_cond_indexed(&item, 0, cond, ctx)?;
            if !res {
                // self.state.replace(PipelineState::Empty);
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

            // self.state.replace(PipelineState::Indexed(next));
            Ok(PipelineState::Indexed(next))
        }

        PipelineState::Empty => {
            Ok(state)
        }
    }

    // eprintln!(
    //     "[pipeline] eval_reduced_pipeline: (filter) pipeline state: {:?}",
    //     value
    // );
    // // let params = pipeline_state_items(&value);

    // eprintln!(
    //     "[pipeline] eval_reduced_pipeline: (filter) cond: {:?}",
    //     cond
    // );
    // // let cond: ExpressionValue<OutputExpression> = match TryEvalFrom::try_eval_from(cond, ctx) {
    // //     Ok(v) => v, Err(e) => { return Done(Err(e)); }
    // // };
    // // let res = params.and_then(|params| apply_filter(&cond, params.as_ref(), ctx));
    // // let res = filter_state(&value, cond, ctx);

    // Ok(PipelineState::Empty)
}

pub fn apply_method(state: PipelineState, method: &ReducedMethodCall<OutputExpression>, ctx: &mut OutputContext) -> DocumentProcessingResult<PipelineState> {
    match *method {
        ReducedMethodCall::Filter(ref cond) => do_filter(state, cond, ctx),

        _ => Err(try_eval_from_err!("Unimplemented reduced pipeline method"))
    }
}

impl<'ctx, I> Iterator for PipelineEval<'ctx, I> where I: Iterator<Item = ReducedPipelineComponent<OutputExpression>> {
    // type Item = PipelineItem<OutputExpression>;
    // type Item = I::Item;
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

// impl<T> From<ParamValue<T>> for PipelineItem<T> {
//     fn from(src: ParamValue<T>) -> Self {
//         let expr = src.value().clone();
//         PipelineItem::Bare(expr, 0, Default::default())
//     }
// }

// impl<T> From<ParamValue<T>> for PipelineState<T> {
//     fn from(src: ParamValue<T>) -> Self {
//         let item = src.clone().into();
//         let expr = src.expr().clone();
//         PipelineState::Single(PipelineItem::Bare(src, ))
//     }
// }

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

// #[derive(Debug)]
// pub enum PipelineItem<T> {
//     Bare(ExpressionValue<T>, usize, PhantomData<T>),
//     Named(String, ExpressionValue<T>, PhantomData<T>),
// }

// impl<T> PipelineItem<T> {
//     pub fn inner(&self) -> &ExpressionValue<T> {
//         match self {
//             PipelineItem::Bare(ref v, _, _) => v,
//             PipelineItem::Named(_, ref v, _) => v,
//         }
//     }

//     pub fn into_inner(self) -> ExpressionValue<T> {
//         match self {
//             PipelineItem::Bare(v, _, _) => v,
//             PipelineItem::Named(_, v, _) => v,
//         }
//     }
// }

// ///
// /// Apply condition to item
// ///
// fn filter_item(
//     cond: &ExpressionValue<OutputExpression>,
//     item: &PipelineItem<OutputExpression>,
//     ctx: &mut OutputContext,
// ) -> DocumentProcessingResult<bool> {
//     ctx.push_child_scope();

//     let cur_item;
//     match item {
//         PipelineItem::Bare(ref v, _, _) => {
//             cur_item = v;
//         }

//         PipelineItem::Named(ref key, ref v, _) => {
//             // TODO: Change to CurrentItemKey
//             let binding = CommonBindings::CurrentItemIndex;
//             eprintln!("[pipeline] apply_filter: item_key: {:?}", key);
//             cur_item = v;

//             let key = ExpressionValue::Primitive(Primitive::StringVal(key.to_owned()));
//             ctx.bind_loop_value(binding, key)?;
//         }
//     };

//     let binding = CommonBindings::CurrentItem(Default::default());
//     let item_value: ExpressionValue<OutputExpression> =
//         TryEvalFrom::try_eval_from(cur_item, ctx)?;
//     eprintln!("[pipeline] apply_filter: item_value: {:?}", item_value);

//     ctx.bind_loop_value(binding, item_value)?;

//     eprintln!("[pipeline] apply_filter: cond (a): {:?}", cond);

//     // Evaluate processed expression
//     let cond: ExpressionValue<OutputExpression> = TryEvalFrom::try_eval_from(cond, ctx)?;
//     eprintln!("[pipeline] apply_filter: cond (b): {:?}", cond);

//     // Evaluate bindings
//     let cond: ExpressionValue<OutputExpression> = TryEvalFrom::try_eval_from(&cond, ctx)?;
//     eprintln!("[pipeline] apply_filter: cond (c): {:?}", cond);

//     // Evaluate condition as boolean
//     let cond: bool = TryEvalFrom::try_eval_from(&cond, ctx)?;
//     eprintln!("[pipeline] apply_filter: cond (d): {:?}", cond);

//     ctx.pop_scope();

//     Ok(cond)
// }

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

    // #[test]
    // fn test_indexed_state_method() {
    //     let mut ctx = TestOutputContext::default();

    //     let expr_0: ExpressionValue<OutputExpression> = ExpressionValue::Primitive(Primitive::StringVal("zero".to_owned()));
    //     let expr_1: ExpressionValue<OutputExpression> = ExpressionValue::Primitive(Primitive::StringVal("one".to_owned()));
    //     let head = ExpressionValue::Expression(Expression::Composite(CompositeValue::ArrayValue(ArrayValue(Some(Box::new(vec![ParamValue::new(expr_0), ParamValue::new(expr_1)]))))));

    //     // let cond = parse_single_expression("idx == 1");
    //     let cond = ExpressionValue::Expression(
    //         Expression::BinaryOp(BinaryOpType::EqualTo,
    //             Box::new(ExpressionValue::Binding(CommonBindings::CurrentItemIndex, Default::default())),
    //             Box::new(ExpressionValue::Primitive(Primitive::Int32Val(1)))
    //         )
    //     );
    //     let method: ReducedMethodCall<OutputExpression> = ReducedMethodCall::Filter(cond);
    //     let components = vec![ReducedPipelineComponent::PipelineOp(method)];

    //     let mut eval = PipelineEval::create(components.into_iter(), head, &mut ctx).unwrap();

    //     let next = eval.next().unwrap();
    //     match next {
    //         PipelineStep::Finished(Ok(state)) => {
    //             let array_value = state.into_array_value().unwrap();

    //             // let res_0: ExpressionValue<OutputExpression> = ExpressionValue::Primitive(Primitive::StringVal("zero".to_owned()));
    //             let res_1: ExpressionValue<OutputExpression> = ExpressionValue::Primitive(Primitive::StringVal("one".to_owned()));
    //             assert_eq!(array_value, ArrayValue(Some(Box::new(vec![ParamValue::new(res_1)]))));
    //         }

    //         _ => {
    //             assert!(false, "Invalid final state for pipeline in test.")
    //         }
    //     }
    // }

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

}
