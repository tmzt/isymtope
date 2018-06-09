use std::fmt::Debug;
use std::marker::PhantomData;

use itertools::Itertools;
use itertools::FoldWhile::*;

use error::*;
use traits::*;
use expressions::*;

#[allow(dead_code)]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct PipelineValue<T>(Box<ExpressionValue<T>>, Box<Vec<PipelineComponentValue<T>>>);

impl<T> PipelineValue<T> {
    pub fn new(e: ExpressionValue<T>, v: Vec<PipelineComponentValue<T>>) -> Self {
        PipelineValue(Box::new(e), Box::new(v))
    }

    pub fn from_components(e: ExpressionValue<T>, v: Vec<PipelineComponentValue<T>>) -> Self {
        let mut iter = v.into_iter().peekable();
        // Collect member components at start
        let path_components: Vec<_> = iter.peeking_take_while(|e| match e { PipelineComponentValue::Member(..) => true, _ => false })
            .map(|e| match e { PipelineComponentValue::Member(ref s) => Some(s.to_owned()), _ => None })
            .map(|e| e.unwrap())
            .collect();

        // Collect method calls and remaining member components
        let rest: Vec<_> = iter.collect();
        let len = path_components.len();
        let head = match e {
            ExpressionValue::Expression(Expression::Ident(..)) |
            ExpressionValue::Binding(CommonBindings::NamedQueryParam(..), _)
                if len > 0 => {
                ExpressionValue::Expression(Expression::Path(PathValue::new(e, Some(path_components)), Default::default()))
            },
            _ => e
        };
        PipelineValue(Box::new(head), Box::new(rest))
    }

    pub fn head(&self) -> &ExpressionValue<T> {
        self.0.as_ref()
    }

    pub fn components<'a>(&'a self) -> impl Iterator<Item = &'a PipelineComponentValue<T>> {
        let box ref v = self.1;
        v.iter()
    }

    pub fn has_components(&self) -> bool {
        let box ref v = self.1;
        !v.is_empty()
    }

    pub fn is_member_path(&self) -> bool {
        let box ref components = self.1;

        components.iter().all(|c| match *c {
            PipelineComponentValue::Member(..) => true,
            _ => false,
        })
    }
}

impl<T: Debug> MapIdents<T> for PipelineValue<T> {
    fn map_idents(self, ctx: &mut ProcessingContext) -> DocumentProcessingResult<Self> {
        let head = self.0.map_idents(ctx)?;
        let box components = self.1;
        let components: Vec<_> = components.into_iter().map(|c| c.map_idents(ctx)).collect();
        let components: Vec<_> = ok_or_error(components)?.collect();

        Ok(PipelineValue(Box::new(head), Box::new(components)))
    }
}

impl<I, O> TryProcessFrom<PipelineValue<I>> for PipelineValue<O>
where
    ExpressionValue<O>: TryProcessFrom<ExpressionValue<I>>,
    I: ::std::fmt::Debug,
    O: ::std::fmt::Debug,
{
    fn try_process_from(
        src: &PipelineValue<I>,
        ctx: &mut ProcessingContext,
    ) -> DocumentProcessingResult<Self> {
        let &box ref expr = &src.0;
        let &box ref pcv = &src.1;

        let expr: ExpressionValue<O> = TryProcessFrom::try_process_from(expr, ctx)?;
        let pcv: Vec<PipelineComponentValue<O>> = TryProcessFrom::try_process_from(pcv, ctx)?;

        Ok(PipelineValue(Box::new(expr), Box::new(pcv)))
    }
}

impl<I, O> TryEvalFrom<PipelineValue<I>> for PipelineValue<O>
where
    ExpressionValue<O>: TryEvalFrom<ExpressionValue<I>>,
    I: ::std::fmt::Debug,
    O: ::std::fmt::Debug,
{
    fn try_eval_from(
        src: &PipelineValue<I>,
        ctx: &mut OutputContext,
    ) -> DocumentProcessingResult<Self> {
        eprintln!("TryEval PipelineValue -> PipelineValue src: {:?}", src);

        let &box ref expr = &src.0;
        let &box ref pcv = &src.1;

        let expr: ExpressionValue<O> = TryEvalFrom::try_eval_from(expr, ctx)?;
        let pcv: Vec<PipelineComponentValue<O>> = TryEvalFrom::try_eval_from(pcv, ctx)?;

        Ok(PipelineValue(Box::new(expr), Box::new(pcv)))
    }
}

#[allow(dead_code)]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum PipelineComponentValue<T> {
    Member(String),
    MethodCall(String, Option<Vec<ParamValue<T>>>, PhantomData<T>),
}

impl<T> PipelineComponentValue<T> {
    pub fn is_member(&self) -> bool {
        match *self {
            PipelineComponentValue::Member(..) => true,
            _ => false,
        }
    }
}

impl<T: Debug> MapIdents<T> for PipelineComponentValue<T> {
    fn map_idents(self, ctx: &mut ProcessingContext) -> DocumentProcessingResult<Self> {
        Ok(match self {
            PipelineComponentValue::MethodCall(s, Some(params), _) => {
                let params: Vec<_> = params.into_iter().map(|p| p.map_idents(ctx)).collect();
                let params: Vec<_> = ok_or_error(params)?.collect();
                PipelineComponentValue::MethodCall(s, Some(params), Default::default())
            }

            _ => self,
        })
    }
}

impl<I, O> TryProcessFrom<PipelineComponentValue<I>> for PipelineComponentValue<O>
where
    ExpressionValue<O>: TryProcessFrom<ExpressionValue<I>>,
    I: ::std::fmt::Debug,
    O: ::std::fmt::Debug,
{
    fn try_process_from(
        src: &PipelineComponentValue<I>,
        ctx: &mut ProcessingContext,
    ) -> DocumentProcessingResult<Self> {
        match *src {
            PipelineComponentValue::Member(ref s) => {
                Ok(PipelineComponentValue::Member(s.to_owned()))
            }
            PipelineComponentValue::MethodCall(ref s, ref params, _) => {
                let params: Option<Vec<ParamValue<O>>> =
                    TryProcessFrom::try_process_from(params, ctx)?;
                Ok(PipelineComponentValue::MethodCall(
                    s.to_owned(),
                    params,
                    Default::default(),
                ))
            }
        }
    }
}

impl<I, O> TryEvalFrom<PipelineComponentValue<I>> for PipelineComponentValue<O>
where
    ExpressionValue<O>: TryEvalFrom<ExpressionValue<I>>,
    I: ::std::fmt::Debug,
    O: ::std::fmt::Debug,
{
    fn try_eval_from(
        src: &PipelineComponentValue<I>,
        ctx: &mut OutputContext,
    ) -> DocumentProcessingResult<Self> {
        match *src {
            PipelineComponentValue::Member(ref s) => {
                Ok(PipelineComponentValue::Member(s.to_owned()))
            }
            PipelineComponentValue::MethodCall(ref s, ref params, _) => {
                let params: Option<Vec<ParamValue<O>>> = TryEvalFrom::try_eval_from(params, ctx)?;
                Ok(PipelineComponentValue::MethodCall(
                    s.to_owned(),
                    params,
                    Default::default(),
                ))
            }
        }
    }
}

///
/// Evaluate reduced pipeline
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

enum PipelineState<'p, T: Debug + 'p> {
    Iterable(Box<Iterator<Item = &'p PipelineItem<T>> + 'p>, PhantomData<T>),
    Single(PipelineItem<T>, PhantomData<T>),
    Empty(PhantomData<T>)
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
