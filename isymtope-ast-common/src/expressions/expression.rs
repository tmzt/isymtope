use std::fmt::Debug;
use std::marker::PhantomData;
use std::collections::HashSet;
use std::iter;

use itertools::Itertools;
use itertools::FoldWhile::{Continue, Done};
use itertools::join;

use common::*;
use error::*;
use traits::*;
use expressions::*;
use objects::*;
use ast::*;
// use output::*;

#[allow(dead_code)]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum CommonBindings<T> {
    CurrentReducerState(PhantomData<T>),
    CurrentItem(PhantomData<T>),
    CurrentItemIndex,
    NamedReducerKey(String, PhantomData<T>),
    NamedReducerActionParam(String, PhantomData<T>),
    NamedQueryParam(String, PhantomData<T>),
    NamedComponentProp(String, PhantomData<T>),
    ComponentPropsObject(PhantomData<T>),
    NamedEventBoundValue(String, PhantomData<T>),
    CurrentElementValue(PhantomData<T>),
    CurrentElementKeyPath,
    PathAlias(String, PhantomData<T>),
}

impl<T> CommonBindings<T> {
    pub fn ident(&self) -> Option<&str> {
        match *self {
            CommonBindings::NamedReducerKey(ref s, _) => Some(s.as_str()),
            CommonBindings::NamedReducerActionParam(ref s, _) => Some(s.as_str()),
            CommonBindings::NamedComponentProp(ref s, _) => Some(s.as_str()),
            CommonBindings::NamedQueryParam(ref s, _) => Some(s.as_str()),
            CommonBindings::NamedEventBoundValue(ref s, _) => Some(s.as_str()),
            CommonBindings::PathAlias(ref s, _) => Some(s.as_str()),
            _ => None,
        }
    }
}

impl<I, O> TryProcessFrom<CommonBindings<I>> for CommonBindings<O>
where
    ExpressionValue<O>: TryProcessFrom<ExpressionValue<I>>,
{
    fn try_process_from(
        src: &CommonBindings<I>,
        ctx: &mut ProcessingContext,
    ) -> DocumentProcessingResult<Self> {
        match *src {
            CommonBindings::CurrentReducerState(_) => {
                Ok(CommonBindings::CurrentReducerState(Default::default()))
            }
            CommonBindings::CurrentItem(_) => Ok(CommonBindings::CurrentItem(Default::default())),
            CommonBindings::CurrentItemIndex => Ok(CommonBindings::CurrentItemIndex),
            CommonBindings::NamedReducerKey(ref s, _) => Ok(CommonBindings::NamedReducerKey(
                s.to_owned(),
                Default::default(),
            )),
            CommonBindings::NamedReducerActionParam(ref s, _) => Ok(
                CommonBindings::NamedReducerActionParam(s.to_owned(), Default::default()),
            ),
            CommonBindings::NamedQueryParam(ref s, _) => Ok(CommonBindings::NamedQueryParam(
                s.to_owned(),
                Default::default(),
            )),
            CommonBindings::NamedComponentProp(ref s, _) => Ok(
                CommonBindings::NamedComponentProp(s.to_owned(), Default::default()),
            ),
            CommonBindings::ComponentPropsObject(_) => {
                Ok(CommonBindings::ComponentPropsObject(Default::default()))
            }
            CommonBindings::NamedEventBoundValue(ref s, _) => Ok(
                CommonBindings::NamedEventBoundValue(s.to_owned(), Default::default()),
            ),
            CommonBindings::CurrentElementValue(_) => {
                Ok(CommonBindings::CurrentElementValue(Default::default()))
            }
            CommonBindings::CurrentElementKeyPath => {
                Ok(CommonBindings::CurrentElementKeyPath)
            }
            CommonBindings::PathAlias(ref s, _) => {
                Ok(CommonBindings::PathAlias(s.to_owned(), Default::default()))
            }
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum OuterShape {
    Singleton,
    Array,
    Object,
}

impl Default for OuterShape {
    fn default() -> Self {
        OuterShape::Singleton
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct BindingShape<T>(CommonBindings<T>, OuterShape);

impl<T: Debug> BindingShape<T> {
    pub fn new(binding: CommonBindings<T>, shape: OuterShape) -> Self {
        BindingShape(binding, shape)
    }

    pub fn binding(&self) -> &CommonBindings<T> {
        &self.0
    }
    pub fn shape(&self) -> OuterShape {
        self.1
    }
}

impl<I, O> TryProcessFrom<BindingShape<I>> for BindingShape<O>
where
    ExpressionValue<O>: TryProcessFrom<ExpressionValue<I>>,
{
    fn try_process_from(
        src: &BindingShape<I>,
        ctx: &mut ProcessingContext,
    ) -> DocumentProcessingResult<Self> {
        Ok(BindingShape(
            TryProcessFrom::try_process_from(&src.0, ctx)?,
            src.1,
        ))
    }
}

#[allow(dead_code)]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum ExpressionValue<T> {
    Expression(Expression<T>),
    Primitive(Primitive),
    Binding(CommonBindings<T>, PhantomData<T>),
    BindingShape(BindingShape<T>, PhantomData<T>),
    Lens(LensValue<T>, PhantomData<T>),
    SourceLens(SourceLensValue<T>, PhantomData<T>),
    Content(ContentNode<T>, PhantomData<T>),
}

impl<T> ExpressionValue<T> {
    pub fn is_primitive(&self) -> bool {
        match *self {
            ExpressionValue::Primitive(..) => true,
            _ => false,
        }
    }
    pub fn is_object(&self) -> bool {
        match *self {
            ExpressionValue::Expression(Expression::Composite(CompositeValue::ObjectValue(..))) => {
                true
            }
            _ => false,
        }
    }
    pub fn is_array(&self) -> bool {
        match *self {
            ExpressionValue::Expression(Expression::Composite(CompositeValue::ArrayValue(..))) => {
                true
            }
            _ => false,
        }
    }

    pub fn is_array_of_objects(&self) -> bool {
        if let ExpressionValue::Expression(Expression::Composite(CompositeValue::ArrayValue(
            Some(box ref v),
        ))) = *self
        {
            return v.iter().all(|e| e.value().is_object());
        };

        false
    }

    pub fn shape(&self) -> OuterShape {
        match *self {
            ExpressionValue::Expression(Expression::Composite(CompositeValue::ObjectValue(..))) => {
                OuterShape::Object
            }
            ExpressionValue::Expression(Expression::Composite(CompositeValue::ArrayValue(..))) => {
                OuterShape::Array
            }
            _ => OuterShape::Singleton,
        }
    }

    pub fn ident(&self) -> Option<String> {
        match *self {
            ExpressionValue::Expression(Expression::Path(ref path, _)) => path.components()
                .and_then(|v| v.last().map(|s| s.to_owned())),

            ExpressionValue::Expression(ref expr) => expr.ident(),

            _ => None,
        }
    }
}

impl<T: Debug> MapIdents<T> for ExpressionValue<T> {
    fn map_idents(self, ctx: &mut ProcessingContext) -> DocumentProcessingResult<Self> {
        debug!("map_idents ExpressionValue<T>: src: {:?}", &self);

        Ok(match self {
            ExpressionValue::Expression(e) => ExpressionValue::Expression(match e {
                Expression::Pipeline(p, _) => {
                    Expression::Pipeline(p.map_idents(ctx)?, Default::default())
                }

                Expression::Group(Some(box e)) => {
                    // Expression::Group(Some(Box::new(e.map_idents(formals, formals_object))))
                    Expression::Group(Some(Box::new(e.map_idents(ctx)?)))
                }

                Expression::BinaryOp(op, box a, box b) => {
                    let a = a.map_idents(ctx)?;
                    let b = b.map_idents(ctx)?;

                    Expression::BinaryOp(op, Box::new(a), Box::new(b))
                }

                Expression::UnaryOp(op, box a) => {
                    let a = a.map_idents(ctx)?;

                    Expression::UnaryOp(op, Box::new(a))
                }

                _ => e,
            }),

            _ => self,
        })
    }
}

///
/// Formal parameter list
///

#[allow(dead_code)]
#[derive(Debug, Default, Clone, PartialEq, Eq, Hash)]
pub struct FormalParams<T>(Option<Vec<String>>, PhantomData<T>);

impl<T> FormalParams<T> {
    pub fn new(v: Option<Vec<String>>) -> Self {
        FormalParams(v, Default::default())
    }

    pub fn params<'a>(&'a self) -> Option<impl Iterator<Item = &'a str>> {
        self.0.as_ref().map(|v| v.iter().map(|s| s.as_str()))
    }
}

impl<I, O> TryProcessFrom<FormalParams<I>> for FormalParams<O>
where
    ExpressionValue<O>: TryProcessFrom<ExpressionValue<I>>,
{
    fn try_process_from(
        src: &FormalParams<I>,
        ctx: &mut ProcessingContext,
    ) -> DocumentProcessingResult<Self> {
        Ok(FormalParams(src.0.clone(), Default::default()))
    }
}

impl<I, O> TryEvalFrom<FormalParams<I>> for FormalParams<O>
where
    ExpressionValue<O>: TryEvalFrom<ExpressionValue<I>>,
{
    fn try_eval_from(
        src: &FormalParams<I>,
        ctx: &mut OutputContext,
    ) -> DocumentProcessingResult<Self> {
        Ok(FormalParams(src.0.clone(), Default::default()))
    }
}

#[allow(dead_code)]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct ParamValue<T>(ExpressionValue<T>);

impl<T> ParamValue<T> {
    pub fn new(e: ExpressionValue<T>) -> Self {
        ParamValue(e)
    }

    pub fn value(&self) -> &ExpressionValue<T> {
        &self.0
    }
}

impl<T: Debug> MapIdents<T> for ParamValue<T> {
    fn map_idents(self, ctx: &mut ProcessingContext) -> DocumentProcessingResult<Self> {
        Ok(ParamValue(self.0.map_idents(ctx)?))
    }
}

impl<I, O> TryProcessFrom<ParamValue<I>> for ParamValue<O>
where
    ExpressionValue<O>: TryProcessFrom<ExpressionValue<I>>,
{
    fn try_process_from(
        src: &ParamValue<I>,
        ctx: &mut ProcessingContext,
    ) -> DocumentProcessingResult<Self> {
        Ok(ParamValue(TryProcessFrom::try_process_from(&src.0, ctx)?))
        // Ok(ParamValue(TryProcessFrom::try_process_from(&src.0)?, Default::default()))
    }
}

impl<I, O> TryEvalFrom<ParamValue<I>> for ParamValue<O>
where
    ExpressionValue<O>: TryEvalFrom<ExpressionValue<I>>,
    I: ::std::fmt::Debug,
    O: ::std::fmt::Debug,
{
    fn try_eval_from(
        src: &ParamValue<I>,
        ctx: &mut OutputContext,
    ) -> DocumentProcessingResult<Self> {
        Ok(ParamValue(TryEvalFrom::try_eval_from(&src.0, ctx)?))
    }
}

///
/// Properties and element properties
///

#[allow(dead_code)]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct PropValue<T>(String, ExpressionValue<T>, Option<String>);

impl<T> PropValue<T> {
    pub fn new(key: String, e: ExpressionValue<T>, alias: Option<String>) -> Self {
        PropValue(key, e, alias)
    }

    pub fn key(&self) -> &str {
        self.0.as_str()
    }
    pub fn value(&self) -> &ExpressionValue<T> {
        &self.1
    }
    pub fn alias(&self) -> Option<&str> {
        self.2.as_ref().map(|s| s.as_str())
    }
}

impl<I, O> TryProcessFrom<PropValue<I>> for PropValue<O>
where
    ExpressionValue<O>: TryProcessFrom<ExpressionValue<I>>,
{
    fn try_process_from(
        src: &PropValue<I>,
        ctx: &mut ProcessingContext,
    ) -> DocumentProcessingResult<Self> {
        Ok(PropValue(
            src.0.clone(),
            TryProcessFrom::try_process_from(&src.1, ctx)?,
            src.2.clone()
        ))
    }
}

impl<I, O> TryEvalFrom<PropValue<I>> for PropValue<O>
where
    ExpressionValue<O>: TryEvalFrom<ExpressionValue<I>>,
{
    fn try_eval_from(
        src: &PropValue<I>,
        ctx: &mut OutputContext,
    ) -> DocumentProcessingResult<Self> {
        Ok(PropValue(
            src.0.clone(),
            TryEvalFrom::try_eval_from(&src.1, ctx)?,
            src.2.clone()
        ))
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum ElementAttrValue<T> {
    Prop(ElementPropValue<T>),
    Positional(ExpressionValue<T>),
}

impl<I, O> TryProcessFrom<ElementAttrValue<I>> for ElementAttrValue<O>
where
    ExpressionValue<O>: TryProcessFrom<ExpressionValue<I>>,
    I: ::std::fmt::Debug,
{
    fn try_process_from(
        src: &ElementAttrValue<I>,
        ctx: &mut ProcessingContext,
    ) -> DocumentProcessingResult<Self> {
        eprintln!(
            "TryProcess ElementAttrValue -> ElementAttrValue: src: {:?}",
            src
        );

        match *src {
            ElementAttrValue::Prop(ref p) => Ok(ElementAttrValue::Prop(
                TryProcessFrom::try_process_from(p, ctx)?,
            )),
            ElementAttrValue::Positional(ref e) => Ok(ElementAttrValue::Positional(
                TryProcessFrom::try_process_from(e, ctx)?,
            )),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct ElementPropValue<T>(String, ExpressionValue<T>);

impl<T> ElementPropValue<T> {
    pub fn new(key: String, e: ExpressionValue<T>) -> Self {
        ElementPropValue(key, e)
    }

    pub fn name(&self) -> &str {
        self.0.as_str()
    }
    pub fn expr(&self) -> &ExpressionValue<T> {
        &self.1
    }
}

impl<I, O> TryProcessFrom<ElementPropValue<I>> for ElementPropValue<O>
where
    ExpressionValue<O>: TryProcessFrom<ExpressionValue<I>>,
    I: ::std::fmt::Debug,
{
    fn try_process_from(
        src: &ElementPropValue<I>,
        ctx: &mut ProcessingContext,
    ) -> DocumentProcessingResult<Self> {
        eprintln!(
            "TryProcess ElementPropValue -> ElementPropValue: src: {:?}",
            src
        );

        Ok(ElementPropValue(
            src.0.clone(),
            TryProcessFrom::try_process_from(&src.1, ctx)?,
        ))
    }
}

impl<I, O> TryEvalFrom<ElementPropValue<I>> for ElementPropValue<O>
where
    ExpressionValue<O>: TryEvalFrom<ExpressionValue<I>>,
    I: ::std::fmt::Debug,
{
    fn try_eval_from(
        src: &ElementPropValue<I>,
        ctx: &mut OutputContext,
    ) -> DocumentProcessingResult<Self> {
        eprintln!(
            "TryEval ElementPropValue -> ElementPropValue: src: {:?}",
            src
        );

        Ok(ElementPropValue(
            src.0.clone(),
            TryEvalFrom::try_eval_from(&src.1, ctx)?,
        ))
    }
}

///
/// Path, composite, pipeline, filter
///

#[allow(dead_code)]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum CompositeValue<T> {
    ObjectValue(Option<Box<Vec<PropValue<T>>>>),
    ArrayValue(Option<Box<Vec<ParamValue<T>>>>),
}

#[derive(Debug, Default, Clone, PartialEq, Eq, Hash)]
pub struct SourceExpression {}
#[derive(Debug, Default, Clone, PartialEq, Eq, Hash)]
pub struct ProcessedExpression {}
#[derive(Debug, Default, Clone, PartialEq, Eq, Hash)]
pub struct OutputExpression {}

#[allow(dead_code)]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum Expression<T> {
    Composite(CompositeValue<T>),
    Path(PathValue<T>, PhantomData<T>),
    Ident(String, PhantomData<T>),
    RawPath(String, PhantomData<T>),

    QueryCall(QueryCall<T>, PhantomData<T>),

    Pipeline(PipelineValue<T>, PhantomData<T>),
    Filter(FilterValue<T>, PhantomData<T>),

    ReducedPipeline(ReducedPipelineValue<T>, PhantomData<T>),

    Group(Option<Box<ExpressionValue<T>>>),
    UnaryOp(UnaryOpType, Box<ExpressionValue<T>>),
    BinaryOp(
        BinaryOpType,
        Box<ExpressionValue<T>>,
        Box<ExpressionValue<T>>,
    ),
    ApplyOp(ApplyOpType, Box<ExpressionValue<T>>),
}

impl<T> Expression<T> {
    fn ident(&self) -> Option<String> {
        match *self {
            Expression::Ident(ref s, _) => Some(s.to_owned()),
            _ => None
        }
    }
}

impl<T: Clone> TryProcessFrom<Expression<T>> for Expression<T> {
    fn try_process_from(
        src: &Expression<T>,
        ctx: &mut ProcessingContext,
    ) -> DocumentProcessingResult<Self> {
        Ok(src.to_owned())
    }
}

impl TryProcessFrom<Expression<SourceExpression>> for ExpressionValue<ProcessedExpression> {
    fn try_process_from(
        src: &Expression<SourceExpression>,
        ctx: &mut ProcessingContext,
    ) -> DocumentProcessingResult<Self> {
        let expr = match *src {
            Expression::Composite(ref e) => {
                Expression::Composite(TryProcessFrom::try_process_from(e, ctx)?)
            }
            Expression::Path(ref p, _) => Expression::Path(
                TryProcessFrom::try_process_from(p, ctx)?,
                Default::default(),
            ),

            Expression::Ident(ref ident_key, _) => {
                debug!(
                    "Process Expression -> Expression: Ident: ident_key: {}",
                    ident_key
                );

                if ctx.environment()? == ProcessingScopeEnvironment::ElementActions {
                    eprintln!("Finding element_binding [{}]", ident_key);
                    if let Some(binding) = ctx.find_element_binding(ident_key)? {
                        return Ok(ExpressionValue::Binding(binding, Default::default()));
                    };
                }

                eprintln!("Finding ident with shape [{}]", ident_key);

                if let Some(shaped) = ctx.find_ident_shape(ident_key)? {
                    eprintln!("Found shaped binding: [{:?}]", shaped);

                    return Ok(ExpressionValue::BindingShape(shaped, Default::default()));
                };

                eprintln!("Finding ident [{}]", ident_key);

                let binding = ctx.must_find_ident(ident_key)?;

                return Ok(ExpressionValue::Binding(binding, Default::default()));
            }

            Expression::RawPath(ref s, _) => Expression::RawPath(s.to_owned(), Default::default()),

            Expression::Pipeline(ref pv, _) => {
                let head = TryProcessFrom::try_process_from(pv.head(), ctx)?;
                let components = pv.components();

                let res = match components {
                    _ if !pv.has_components() => head,
                    _ if pv.is_member_path() => {
                        let components: Vec<_> = pv.components()
                            .filter_map(|c| match *c {
                                PipelineComponentValue::Member(ref s) => Some(s.to_owned()),
                                _ => None,
                            })
                            .collect();
                        let path = PathValue::new(head, Some(components));

                        ExpressionValue::Expression(Expression::Path(path, Default::default()))
                    }

                    _ => {
                        let reduced: ReducedPipelineValue<
                            ProcessedExpression,
                        > = TryProcessFrom::try_process_from(pv, ctx)?;
                        ExpressionValue::Expression(Expression::ReducedPipeline(
                            reduced,
                            Default::default(),
                        ))
                    }
                };

                return Ok(res);
            }

            Expression::Filter(ref e, _) => Expression::Filter(
                TryProcessFrom::try_process_from(e, ctx)?,
                Default::default(),
            ),

            Expression::ReducedPipeline(_, _) => {
                Err(try_process_from_err!("ReducedPipeline not supported yet."))?
            }

            Expression::Group(Some(box ref e)) => {
                Expression::Group(Some(Box::new(TryProcessFrom::try_process_from(e, ctx)?)))
            }
            Expression::Group(_) => Expression::Group(None),

            Expression::UnaryOp(ref op, box ref e) => Expression::UnaryOp(
                op.to_owned(),
                Box::new(TryProcessFrom::try_process_from(e, ctx)?),
            ),
            Expression::BinaryOp(ref op, box ref a, box ref b) => Expression::BinaryOp(
                op.to_owned(),
                Box::new(TryProcessFrom::try_process_from(a, ctx)?),
                Box::new(TryProcessFrom::try_process_from(b, ctx)?),
            ),

            Expression::ApplyOp(ref op, box ref e) => Expression::ApplyOp(
                op.to_owned(),
                Box::new(TryProcessFrom::try_process_from(e, ctx)?),
            ),

            _ => Err(try_process_from_err!(format!(
                "Unable to process expression {:?} into ExpressionValue.",
                src
            )))?,
        };

        Ok(ExpressionValue::Expression(expr))
    }
}

fn eval_expression<T>(
    src: &Expression<T>,
    ctx: &mut OutputContext,
) -> DocumentProcessingResult<Option<ExpressionValue<OutputExpression>>>
where
    T: ::std::fmt::Debug,
    ExpressionValue<OutputExpression>: TryEvalFrom<ExpressionValue<T>>,
{
    eprintln!("[eval expression] expr: {:?}", src);

    Ok(match *src {
        Expression::Pipeline(ref e, _) => Some(ExpressionValue::Expression(Expression::Pipeline(
            TryEvalFrom::try_eval_from(e, ctx)?,
            Default::default(),
        ))),
        Expression::Filter(ref e, _) => Some(ExpressionValue::Expression(Expression::Filter(
            TryEvalFrom::try_eval_from(e, ctx)?,
            Default::default(),
        ))),

        Expression::Group(Some(box ref e)) => Some(TryEvalFrom::try_eval_from(e, ctx)?),
        Expression::Group(_) => Some(ExpressionValue::Expression(Expression::Group(None))),

        Expression::UnaryOp(ref op, box ref e) => {
            let e: ExpressionValue<OutputExpression> = TryEvalFrom::try_eval_from(e, ctx)?;

            let res = match (op, &e) {
                (&UnaryOpType::Negate, &ExpressionValue::Primitive(Primitive::BoolVal(b))) => {
                    Some(ExpressionValue::Primitive(Primitive::BoolVal(!b)))
                }

                _ => None,
            };

            if res.is_some() {
                return Ok(res);
            }
            Some(ExpressionValue::Expression(Expression::UnaryOp(
                op.to_owned(),
                Box::new(e),
            )))
        }

        Expression::BinaryOp(ref op, box ref a, box ref b) => {
            let a = TryEvalFrom::try_eval_from(a, ctx)?;
            let b = TryEvalFrom::try_eval_from(b, ctx)?;

            let res = match (op, &a, &b) {
                (
                    _,
                    &ExpressionValue::Primitive(Primitive::Int32Val(a)),
                    &ExpressionValue::Primitive(Primitive::Int32Val(b)),
                ) => {
                    match op {
                        &BinaryOpType::Add => Some(ExpressionValue::Primitive(Primitive::Int32Val(a + b))),
                        &BinaryOpType::Sub => Some(ExpressionValue::Primitive(Primitive::Int32Val(a - b))),
                        &BinaryOpType::Mul => Some(ExpressionValue::Primitive(Primitive::Int32Val(a * b))),
                        &BinaryOpType::Div => Some(ExpressionValue::Primitive(Primitive::Int32Val(a / b))),
                        &BinaryOpType::EqualTo => Some(ExpressionValue::Primitive(Primitive::BoolVal(a == b))),
                        &BinaryOpType::NotEqualTo => Some(ExpressionValue::Primitive(Primitive::BoolVal(a != b))),
                        _ => None
                    }
                }

                (
                    &BinaryOpType::EqualTo,
                    &ExpressionValue::Primitive(Primitive::CharVal(ref a)),
                    &ExpressionValue::Primitive(Primitive::CharVal(ref b)),
                ) => Some(ExpressionValue::Primitive(Primitive::BoolVal(a == b))),

                (
                    &BinaryOpType::NotEqualTo,
                    &ExpressionValue::Primitive(Primitive::CharVal(ref a)),
                    &ExpressionValue::Primitive(Primitive::CharVal(ref b)),
                ) => Some(ExpressionValue::Primitive(Primitive::BoolVal(a != b))),

                (
                    &BinaryOpType::EqualTo,
                    &ExpressionValue::Primitive(Primitive::StringVal(ref a)),
                    &ExpressionValue::Primitive(Primitive::StringVal(ref b)),
                ) => Some(ExpressionValue::Primitive(Primitive::BoolVal(a == b))),

                (
                    &BinaryOpType::NotEqualTo,
                    &ExpressionValue::Primitive(Primitive::StringVal(ref a)),
                    &ExpressionValue::Primitive(Primitive::StringVal(ref b)),
                ) => Some(ExpressionValue::Primitive(Primitive::BoolVal(a != b))),

                (
                    &BinaryOpType::EqualTo,
                    &ExpressionValue::Primitive(Primitive::BoolVal(ref a)),
                    &ExpressionValue::Primitive(Primitive::BoolVal(ref b)),
                ) => Some(ExpressionValue::Primitive(Primitive::BoolVal(a == b))),

                (
                    &BinaryOpType::NotEqualTo,
                    &ExpressionValue::Primitive(Primitive::BoolVal(ref a)),
                    &ExpressionValue::Primitive(Primitive::BoolVal(ref b)),
                ) => Some(ExpressionValue::Primitive(Primitive::BoolVal(a != b))),

                (
                    &BinaryOpType::EqualTo,
                    &ExpressionValue::Primitive(Primitive::NullVal),
                    &ExpressionValue::Primitive(Primitive::NullVal),
                )
                | (
                    &BinaryOpType::EqualTo,
                    &ExpressionValue::Primitive(Primitive::Undefined),
                    &ExpressionValue::Primitive(Primitive::Undefined),
                ) => Some(ExpressionValue::Primitive(Primitive::BoolVal(true))),

                (
                    &BinaryOpType::NotEqualTo,
                    &ExpressionValue::Primitive(Primitive::NullVal),
                    &ExpressionValue::Primitive(Primitive::NullVal),
                )
                | (
                    &BinaryOpType::NotEqualTo,
                    &ExpressionValue::Primitive(Primitive::Undefined),
                    &ExpressionValue::Primitive(Primitive::Undefined),
                ) => Some(ExpressionValue::Primitive(Primitive::BoolVal(false))),

                (&BinaryOpType::EqualTo, _, _) => {
                    Some(ExpressionValue::Primitive(Primitive::BoolVal(false)))
                }

                (&BinaryOpType::NotEqualTo, _, _) => {
                    Some(ExpressionValue::Primitive(Primitive::BoolVal(true)))
                }

                _ => None,
            };

            if res.is_some() {
                return Ok(res);
            }

            Some(ExpressionValue::Expression(Expression::BinaryOp(
                op.to_owned(),
                Box::new(a),
                Box::new(b),
            )))
        }

        Expression::ApplyOp(ref op, box ref e) => Some(ExpressionValue::Expression(
            Expression::ApplyOp(op.to_owned(), Box::new(TryEvalFrom::try_eval_from(e, ctx)?)),
        )),

        _ => None,
    })
}

impl TryEvalFrom<Expression<ProcessedExpression>> for ExpressionValue<OutputExpression> {
    fn try_eval_from(
        src: &Expression<ProcessedExpression>,
        ctx: &mut OutputContext,
    ) -> DocumentProcessingResult<Self> {
        Ok(match *src {
            Expression::QueryCall(ref query, _) => TryEvalFrom::try_eval_from(query, ctx)?,
            Expression::Composite(ref e) => ExpressionValue::Expression(Expression::Composite(
                TryEvalFrom::try_eval_from(e, ctx)?,
            )),
            Expression::Path(ref p, _) => TryEvalFrom::try_eval_from(p, ctx)?,

            Expression::ReducedPipeline(ref p, _) => TryEvalFrom::try_eval_from(p, ctx)?,

            _ => {
                if let Some(expr) = eval_expression(src, ctx)? {
                    return Ok(expr);
                };
                TryEvalFrom::try_eval_from(src, ctx)?
            }
        })
    }
}

impl TryEvalFrom<Expression<OutputExpression>> for ExpressionValue<OutputExpression> {
    fn try_eval_from(
        src: &Expression<OutputExpression>,
        ctx: &mut OutputContext,
    ) -> DocumentProcessingResult<Self> {
        Ok(match *src {
            Expression::Composite(ref e) => ExpressionValue::Expression(Expression::Composite(
                TryEvalFrom::try_eval_from(e, ctx)?,
            )),
            Expression::Path(ref p, _) => TryEvalFrom::try_eval_from(p, ctx)?,

            // Expression::ReducedPipeline(ref p, _) => TryEvalFrom::try_eval_from(p, ctx)?,
            _ => eval_expression(src, ctx)?
                .unwrap_or_else(|| ExpressionValue::Expression(src.clone())),
        })
    }
}

impl TryProcessFrom<ExpressionValue<SourceExpression>> for ExpressionValue<ProcessedExpression> {
    fn try_process_from(
        src: &ExpressionValue<SourceExpression>,
        ctx: &mut ProcessingContext,
    ) -> DocumentProcessingResult<Self> {
        match *src {
            ExpressionValue::Expression(ref e) => Ok(TryProcessFrom::try_process_from(e, ctx)?),
            ExpressionValue::Primitive(ref e) => Ok(ExpressionValue::Primitive(e.to_owned())),
            ExpressionValue::Binding(ref b, _) => Ok(ExpressionValue::Binding(
                TryProcessFrom::try_process_from(b, ctx)?,
                Default::default(),
            )),
            ExpressionValue::BindingShape(ref s, _) => Ok(ExpressionValue::BindingShape(
                TryProcessFrom::try_process_from(s, ctx)?,
                Default::default(),
            )),
            ExpressionValue::Lens(_, _) => {
                Err(try_process_from_err!("Cannot generically process Lens"))
            }
            ExpressionValue::SourceLens(ref l, _) => Ok(ExpressionValue::Lens(
                TryProcessFrom::try_process_from(l, ctx)?,
                Default::default(),
            )),
            ExpressionValue::Content(ref c, _) => Ok(ExpressionValue::Content(
                TryProcessFrom::try_process_from(c, ctx)?,
                Default::default(),
            )),
        }
    }
}

impl TryProcessFrom<CompositeValue<SourceExpression>> for CompositeValue<ProcessedExpression> {
    fn try_process_from(
        src: &CompositeValue<SourceExpression>,
        ctx: &mut ProcessingContext,
    ) -> DocumentProcessingResult<Self> {
        match *src {
            CompositeValue::ObjectValue(Some(box ref props)) => {
                let props: Vec<PropValue<ProcessedExpression>> =
                    TryProcessFrom::try_process_from(props, ctx)?;
                Ok(CompositeValue::ObjectValue(Some(Box::new(props))))
            }

            CompositeValue::ObjectValue(_) => Ok(CompositeValue::ObjectValue(None)),

            CompositeValue::ArrayValue(Some(box ref params)) => {
                let params: Vec<ParamValue<ProcessedExpression>> =
                    TryProcessFrom::try_process_from(params, ctx)?;
                Ok(CompositeValue::ArrayValue(Some(Box::new(params))))
            }

            CompositeValue::ArrayValue(_) => Ok(CompositeValue::ArrayValue(None)),
        }
    }
}

impl<I, O> TryEvalFrom<CompositeValue<I>> for CompositeValue<O>
where
    ExpressionValue<O>: TryEvalFrom<ExpressionValue<I>>,
    I: ::std::fmt::Debug,
    O: ::std::fmt::Debug,
{
    fn try_eval_from(
        src: &CompositeValue<I>,
        ctx: &mut OutputContext,
    ) -> DocumentProcessingResult<CompositeValue<O>> {
        match *src {
            CompositeValue::ObjectValue(Some(box ref props)) => {
                let props: Vec<PropValue<O>> = TryEvalFrom::try_eval_from(props, ctx)?;
                Ok(CompositeValue::ObjectValue(Some(Box::new(props))))
            }

            CompositeValue::ObjectValue(_) => Ok(CompositeValue::ObjectValue(None)),

            CompositeValue::ArrayValue(Some(box ref params)) => {
                let params: Vec<ParamValue<O>> = TryEvalFrom::try_eval_from(params, ctx)?;
                Ok(CompositeValue::ArrayValue(Some(Box::new(params))))
            }

            CompositeValue::ArrayValue(_) => Ok(CompositeValue::ArrayValue(None)),
        }
    }
}

impl<I, O> TryEvalFrom<Option<I>> for Option<O>
where
    O: TryEvalFrom<I>,
{
    fn try_eval_from(src: &Option<I>, ctx: &mut OutputContext) -> DocumentProcessingResult<Self> {
        match *src {
            Some(ref v) => Ok(Some(TryEvalFrom::try_eval_from(v, ctx)?)),
            _ => Ok(None),
        }
    }
}

impl TryEvalFrom<CommonBindings<ProcessedExpression>> for ExpressionValue<OutputExpression> {
    fn try_eval_from(
        src: &CommonBindings<ProcessedExpression>,
        ctx: &mut OutputContext,
    ) -> DocumentProcessingResult<Self> {
        // Evaluate reducer key from provider or defaults
        if let CommonBindings::NamedReducerKey(ref key, _) = *src {
            return ctx.reducer_value(key);
        };

        // Pass through certain values to be evaluated at runtime
        match *src {
            CommonBindings::CurrentElementValue(_) => {
                return Ok(ExpressionValue::Binding(
                    CommonBindings::CurrentElementValue(Default::default()),
                    Default::default(),
                ));
            }

            CommonBindings::CurrentReducerState(_) => {
                return Ok(ExpressionValue::Binding(
                    CommonBindings::CurrentReducerState(Default::default()),
                    Default::default(),
                ));
            }

            CommonBindings::CurrentItem(_) => {
                return Ok(ExpressionValue::Binding(
                    CommonBindings::CurrentItem(Default::default()),
                    Default::default(),
                ));
            }

            _ => {}
        };

        // Otherwise, try to find the value for the binding or return an error
        eprintln!(
            "[Expression eval] Looking for value for binding [{:?}]",
            src
        );
        ctx.must_find_value(src)
    }
}

impl TryEvalFrom<ExpressionValue<ProcessedExpression>> for ExpressionValue<OutputExpression> {
    fn try_eval_from(
        src: &ExpressionValue<ProcessedExpression>,
        ctx: &mut OutputContext,
    ) -> DocumentProcessingResult<Self> {
        match *src {
            ExpressionValue::Expression(ref e) => Ok(TryEvalFrom::try_eval_from(e, ctx)?),

            ExpressionValue::Primitive(ref p) => Ok(ExpressionValue::Primitive(p.to_owned())),

            ExpressionValue::Binding(ref b, _) => TryEvalFrom::try_eval_from(b, ctx),
            ExpressionValue::BindingShape(ref s, _) => TryEvalFrom::try_eval_from(s.binding(), ctx),

            ExpressionValue::Lens(ref l, _) => {
                // Since we are evaluating, we can drop the lens in the result.
                Ok(TryEvalFrom::try_eval_from(l, ctx)?)
            }

            ExpressionValue::Content(ref c, _) => Ok(ExpressionValue::Content(
                TryEvalFrom::try_eval_from(c, ctx)?,
                Default::default(),
            )),

            _ => Err(reduction_err_bt!()),
        }
    }
}

impl TryEvalFrom<ExpressionValue<OutputExpression>> for ExpressionValue<OutputExpression> {
    fn try_eval_from(
        src: &ExpressionValue<OutputExpression>,
        ctx: &mut OutputContext,
    ) -> DocumentProcessingResult<Self> {
        Ok(match *src {
            ExpressionValue::Binding(ref b, _) => {
                // Evaluate reducer key from provider or defaults
                if let CommonBindings::NamedReducerKey(ref key, _) = *b {
                    return ctx.reducer_value(key);
                };

                let expr = ctx.must_find_loop_value(b)?;

                // If we found a binding, evaluate it
                TryEvalFrom::try_eval_from(&expr, ctx)?
            }

            ExpressionValue::Expression(ref e) => {
                eval_expression(e, ctx)?.unwrap_or_else(|| src.clone())
            }
            _ => src.clone(),
        })
    }
}

pub fn ok_or_error<A, I: IntoIterator<Item = DocumentProcessingResult<A>>>(
    iter: I,
) -> DocumentProcessingResult<impl Iterator<Item = A>>
where
    A: ::std::fmt::Debug,
{
    let acc: DocumentProcessingResult<Vec<A>> = Ok(Default::default());

    let res = iter.into_iter().fold_while(acc, |state, x| {
        eprintln!("ok_or_error: x: {:?}", x);

        let mut values = state.ok().unwrap();

        match x {
            Err(e) => {
                eprintln!("ok_or_error: Err: {:?}", e);
                Done(Err(e))
            }

            Ok(value) => {
                values.push(value);
                Continue(Ok(values))
            }
        }
    });

    let res = res.into_inner();

    if let Err(err) = res {
        eprintln!("ok_or_error: completed with Err: {:?}", err);
        return Err(err);
    };

    res.map(|v| v.into_iter())
}

impl<I, O: TryProcessFrom<I>> TryProcessFrom<Vec<I>> for Vec<O>
where
    I: ::std::fmt::Debug,
    O: ::std::fmt::Debug,
{
    fn try_process_from(
        src: &Vec<I>,
        ctx: &mut ProcessingContext,
    ) -> DocumentProcessingResult<Self> {
        debug!("TryProcess Vec -> Vec: src: {:?}", src);
        let src_iter = src.into_iter()
            .map(|s| TryProcessFrom::try_process_from(s, ctx));
        let res: Vec<_> = ok_or_error(src_iter)?.collect();
        debug!("TryProcess Vec -> Vec: res: {:?}", res);

        Ok(res)
    }
}

impl<I, O: TryEvalFrom<I>> TryEvalFrom<Vec<I>> for Vec<O>
where
    I: ::std::fmt::Debug,
    O: ::std::fmt::Debug,
{
    fn try_eval_from(src: &Vec<I>, ctx: &mut OutputContext) -> DocumentProcessingResult<Self> {
        debug!("TryEval Vec -> Vec: src: {:?}", src);
        let src_iter = src.into_iter().map(|s| TryEvalFrom::try_eval_from(s, ctx));
        let res: Vec<_> = ok_or_error(src_iter)?.collect();
        debug!("TryEval Vec -> Vec: res: {:?}", res);

        Ok(res)
    }
}

impl<I, O: TryEvalFrom<I>> TryEvalFrom<Vec<(String, I)>> for Vec<(String, O)>
where
    I: ::std::fmt::Debug,
    O: ::std::fmt::Debug,
{
    fn try_eval_from(
        src: &Vec<(String, I)>,
        ctx: &mut OutputContext,
    ) -> DocumentProcessingResult<Self> {
        debug!("TryEval KeyValueVec -> KeyValueVec: src: {:?}", src);
        let src_iter = src.into_iter().map(|&(ref k, ref v)| {
            TryEvalFrom::try_eval_from(v, ctx).and_then(|e| Ok((k.to_owned(), e)))
        });
        let res: Vec<_> = ok_or_error(src_iter)?.collect();
        debug!("TryEval KeyValueVec -> KeyValueVec: res: {:?}", res);

        Ok(res)
    }
}

impl TryEvalFrom<ExpressionValue<OutputExpression>>
    for Option<Vec<ExpressionValue<OutputExpression>>>
{
    fn try_eval_from(
        src: &ExpressionValue<OutputExpression>,
        ctx: &mut OutputContext,
    ) -> DocumentProcessingResult<Self> {
        match *src {
            ExpressionValue::Expression(Expression::Composite(CompositeValue::ArrayValue(
                Some(box ref arr),
            ))) => {
                let arr: Vec<_> = arr.into_iter()
                    .map(|item| item.value().to_owned())
                    .collect();
                Ok(Some(arr))
            }
            _ => Err(try_process_from_err!(
                "Cannot evaluate non-composite values as vec."
            )),
        }
    }
}
