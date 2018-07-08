use std::fmt::Debug;
use std::marker::PhantomData;

use itertools::Itertools;
use itertools::FoldWhile::{Continue, Done};

use error::*;
use traits::*;
use expressions::*;
use objects::*;
use ast::*;

#[allow(dead_code)]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum CommonBindings<T> {
    CurrentReducerState(PhantomData<T>),
    CurrentItem(PhantomData<T>),
    CurrentItemIndex,
    CurrentItemKey,
    NamedReducerKey(String, PhantomData<T>),
    NamedReducerActionParam(String, PhantomData<T>),
    NamedQueryParam(String, PhantomData<T>),
    NamedComponentProp(String, PhantomData<T>),
    ComponentPropsObject(PhantomData<T>),
    NamedElementBoundValue(String, PhantomData<T>),
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
        _ctx: &mut ProcessingContext,
    ) -> DocumentProcessingResult<Self> {
        match *src {
            CommonBindings::CurrentReducerState(_) => {
                Ok(CommonBindings::CurrentReducerState(Default::default()))
            }
            CommonBindings::CurrentItem(_) => Ok(CommonBindings::CurrentItem(Default::default())),
            CommonBindings::CurrentItemIndex => Ok(CommonBindings::CurrentItemIndex),
            CommonBindings::CurrentItemKey => Ok(CommonBindings::CurrentItemKey),
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
            CommonBindings::NamedElementBoundValue(ref s, _) => Ok(
                CommonBindings::NamedElementBoundValue(s.to_owned(), Default::default()),
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
    Map,
}

impl Default for OuterShape {
    fn default() -> Self {
        OuterShape::Singleton
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct BindingShape<T>(pub CommonBindings<T>, pub OuterShape);

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

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct ShapedExpressionValue<T>(pub OuterShape, pub ExpressionValue<T>);

#[allow(dead_code)]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum ExpressionValue<T> {
    Composite(CompositeValue<T>),
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
            ExpressionValue::Composite(CompositeValue::ObjectValue(..)) => {
                true
            }
            _ => false,
        }
    }
    pub fn is_array(&self) -> bool {
        match *self {
            ExpressionValue::Composite(CompositeValue::ArrayValue(..)) => {
                true
            }
            _ => false,
        }
    }

    pub fn is_array_of_objects(&self) -> bool {
        if let ExpressionValue::Composite(CompositeValue::ArrayValue(
            ArrayValue(Some(box ref v)),
        )) = *self
        {
            return v.iter().all(|e| e.value().is_object());
        };

        false
    }

    pub fn shape(&self) -> OuterShape {
        match *self {
            ExpressionValue::Composite(CompositeValue::ObjectValue(..)) => {
                OuterShape::Object
            }
            ExpressionValue::Composite(CompositeValue::ArrayValue(..)) => {
                OuterShape::Array
            }
            ExpressionValue::Composite(CompositeValue::MapValue(..)) => {
                OuterShape::Map
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

                Expression::BinaryOp(BinaryOp(op, box a, box b)) => {
                    let a = a.map_idents(ctx)?;
                    let b = b.map_idents(ctx)?;

                    Expression::BinaryOp(BinaryOp(op, Box::new(a), Box::new(b)))
                }

                Expression::UnaryOp(UnaryOp(op, box a)) => {
                    let a = a.map_idents(ctx)?;

                    Expression::UnaryOp(UnaryOp(op, Box::new(a)))
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
        _ctx: &mut ProcessingContext,
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
        _ctx: &mut OutputContext,
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

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct ObjectValue<T>(pub Option<Box<Vec<PropValue<T>>>>);

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct ArrayValue<T>(pub Option<Box<Vec<ParamValue<T>>>>);

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct ArrayOf<T>(pub Option<Box<Vec<T>>>);

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct MapValue<T>(pub Option<String>, pub Option<Box<Vec<ObjectValue<T>>>>);

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum CompositeValue<T> {
    ObjectValue(ObjectValue<T>),
    ArrayValue(ArrayValue<T>),
    MapValue(MapValue<T>)
}

#[derive(Debug, Default, Clone, PartialEq, Eq, Hash)]
pub struct SourceExpression {}
#[derive(Debug, Default, Clone, PartialEq, Eq, Hash)]
pub struct ProcessedExpression {}
#[derive(Debug, Default, Clone, PartialEq, Eq, Hash)]
pub struct OutputExpression {}


#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct UnaryOp<T>(pub UnaryOpType, pub Box<ExpressionValue<T>>);

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct BinaryOp<T>(pub BinaryOpType, pub Box<ExpressionValue<T>>, pub Box<ExpressionValue<T>>);

#[allow(dead_code)]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum Expression<T> {
    // Composite(CompositeValue<T>),
    Path(PathValue<T>, PhantomData<T>),
    Ident(String, PhantomData<T>),
    RawPath(String, PhantomData<T>),

    QueryCall(QueryCall<T>, PhantomData<T>),

    Pipeline(PipelineValue<T>, PhantomData<T>),
    Filter(FilterValue<T>, PhantomData<T>),

    ReducedPipeline(ReducedPipelineValue<T>, PhantomData<T>),

    Group(Option<Box<ExpressionValue<T>>>),
    UnaryOp(UnaryOp<T>),
    BinaryOp(BinaryOp<T>),
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
        _ctx: &mut ProcessingContext,
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
                        debug!("Found element_binding for [{}]: [{:?}]", ident_key, binding);
                        return Ok(ExpressionValue::Binding(binding, Default::default()));
                    };

                    if let Some(binding) = ctx.find_ident(ident_key)? {
                        debug!("Found binding for [{}]: [{:?}]", ident_key, binding);
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

            Expression::UnaryOp(UnaryOp(ref op, box ref e)) => Expression::UnaryOp(UnaryOp(
                op.to_owned(),
                Box::new(TryProcessFrom::try_process_from(e, ctx)?),
            )),
            Expression::BinaryOp(BinaryOp(ref op, box ref a, box ref b)) => Expression::BinaryOp(BinaryOp(
                op.to_owned(),
                Box::new(TryProcessFrom::try_process_from(a, ctx)?),
                Box::new(TryProcessFrom::try_process_from(b, ctx)?),
            )),

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

fn eval_inner_expression(
    src: &Expression<ProcessedExpression>,
    ctx: &mut OutputContext,
) -> DocumentProcessingResult<ExpressionValue<ProcessedExpression>>
// where
//     T: ::std::fmt::Debug,
//     T: Clone,
//     ExpressionValue<ProcessedExpression>: TryEvalFrom<ExpressionValue<T>>,
{
    eprintln!("[eval_inner_expression] expr: {:?}", src);

    Ok(match *src {
        Expression::ReducedPipeline(ref p, _) => eval_reduced_pipeline_to_value(p, ctx)?,

        Expression::Path(ref path, _) => eval_path(path, ctx)?,

        Expression::QueryCall(ref query, _) => eval_inner_query_call(query, ctx)?,

        Expression::Group(Some(box ref e)) => eval_expression(e, ctx)?,
        Expression::Group(_) => ExpressionValue::Expression(Expression::Group(None)),

        // Expression::Composite(..) => Some(ExpressionValue::Expression(src.to_owned())),

        // Expression::Composite(..) => Some(ExpressionValue::Expression(src.to_owned())),
        // Expression::Composite(CompositeValue::ArrayValue(ArrayValue(Some(box ref v)))) => {
        //     let v: Vec<_> = ok_or_error(v.into_iter().map(|e| TryEvalFrom::try_eval_from(e, ctx)))?.collect();
        //     Some(ExpressionValue::Composite(CompositeValue::ArrayValue(ArrayValue(Some(Box::new(v)))))))
        // }

        Expression::UnaryOp(ref op) => eval_unary_expression(op, ctx)?,

        Expression::BinaryOp(ref op) => eval_binary_expression(op, ctx)?,

        _ => {
            eprintln!("[eval_inner_expression] Unable to evaluate expression: {:?}",
                src
            );
            return Err(try_eval_from_err!("Unable to evaluate expression."));
        }
    })
}

fn eval_unary_expression(
    src: &UnaryOp<ProcessedExpression>,
    ctx: &mut OutputContext,
) -> DocumentProcessingResult<ExpressionValue<ProcessedExpression>> {
    let UnaryOp(ref op, box ref e) = *src;

    Ok(match (op, &e) {
        (&UnaryOpType::Negate, &ExpressionValue::Primitive(Primitive::BoolVal(b))) => {
            ExpressionValue::Primitive(Primitive::BoolVal(!b))
        }

        (&UnaryOpType::Negate, ref a) => {
            let a = eval_expression(a, ctx)?;
            let a: bool = TryEvalFrom::try_eval_from(&a, ctx)?;
            ExpressionValue::Primitive(Primitive::BoolVal(!a))
        }
    })
}

fn eval_binary_expression(
    src: &BinaryOp<ProcessedExpression>,
    ctx: &mut OutputContext,
) -> DocumentProcessingResult<ExpressionValue<ProcessedExpression>> {
    let BinaryOp(ref op, box ref a, box ref b) = *src;

    Ok(match (op, a, b) {
        (
            _,
            &ExpressionValue::Expression(..),
            _
        ) |
        (
            _,
            &ExpressionValue::Binding(..),
            _
        ) => {
            eprintln!("[eval_binary_expression] Found expression (operand 1): {:?}", a);
            let a = eval_expression(a, ctx)?;
            eprintln!("[eval_binary_expression] Evaluated expression: result: {:?}", a);

            let expr = Expression::BinaryOp(BinaryOp(op.to_owned(), Box::new(a.to_owned()), Box::new(b.to_owned())));
            eval_inner_expression(&expr, ctx)?
        }

        (
            _,
            _,
            &ExpressionValue::Expression(..),
        ) |
        (
            _,
            _,
            &ExpressionValue::Binding(..),
        ) => {
            eprintln!("[eval_binary_expression] Found expression (operand 2): {:?}", b);
            let b = eval_expression(b, ctx)?;
            eprintln!("[eval_binary_expression] Evaluated expression: result: {:?}", b);

            let expr = Expression::BinaryOp(BinaryOp(op.to_owned(), Box::new(a.to_owned()), Box::new(b.to_owned())));
            eval_inner_expression(&expr, ctx)?
        }

        (
            _,
            &ExpressionValue::Primitive(Primitive::Int32Val(a)),
            &ExpressionValue::Primitive(Primitive::Int32Val(b)),
        ) => {
            match op {
                &BinaryOpType::Add => ExpressionValue::Primitive(Primitive::Int32Val(a + b)),
                &BinaryOpType::Sub => ExpressionValue::Primitive(Primitive::Int32Val(a - b)),
                &BinaryOpType::Mul => ExpressionValue::Primitive(Primitive::Int32Val(a * b)),
                &BinaryOpType::Div => ExpressionValue::Primitive(Primitive::Int32Val(a / b)),
                &BinaryOpType::EqualTo => ExpressionValue::Primitive(Primitive::BoolVal(a == b)),
                &BinaryOpType::NotEqualTo => ExpressionValue::Primitive(Primitive::BoolVal(a != b)),

                _ => {
                    eprintln!("[eval_binary_expression] Unable to evaluate BinaryOp: {:?}",
                        src
                    );
                    return Err(try_eval_from_err!("Unable to evaluate expression."));
                }
            }
        }

        (
            &BinaryOpType::Add,
            &ExpressionValue::Primitive(Primitive::StringVal(ref a)),
            &ExpressionValue::Primitive(Primitive::StringVal(ref b)),
        ) => ExpressionValue::Primitive(Primitive::StringVal(format!("{}{}", a, b))),

        (
            &BinaryOpType::EqualTo,
            &ExpressionValue::Primitive(Primitive::CharVal(ref a)),
            &ExpressionValue::Primitive(Primitive::CharVal(ref b)),
        ) => ExpressionValue::Primitive(Primitive::BoolVal(a == b)),

        (
            &BinaryOpType::NotEqualTo,
            &ExpressionValue::Primitive(Primitive::CharVal(ref a)),
            &ExpressionValue::Primitive(Primitive::CharVal(ref b)),
        ) => ExpressionValue::Primitive(Primitive::BoolVal(a != b)),

        (
            &BinaryOpType::EqualTo,
            &ExpressionValue::Primitive(Primitive::StringVal(ref a)),
            &ExpressionValue::Primitive(Primitive::StringVal(ref b)),
        ) => ExpressionValue::Primitive(Primitive::BoolVal(a == b)),

        (
            &BinaryOpType::NotEqualTo,
            &ExpressionValue::Primitive(Primitive::StringVal(ref a)),
            &ExpressionValue::Primitive(Primitive::StringVal(ref b)),
        ) => ExpressionValue::Primitive(Primitive::BoolVal(a != b)),

        (
            &BinaryOpType::EqualTo,
            &ExpressionValue::Primitive(Primitive::BoolVal(ref a)),
            &ExpressionValue::Primitive(Primitive::BoolVal(ref b)),
        ) => ExpressionValue::Primitive(Primitive::BoolVal(a == b)),

        (
            &BinaryOpType::NotEqualTo,
            &ExpressionValue::Primitive(Primitive::BoolVal(ref a)),
            &ExpressionValue::Primitive(Primitive::BoolVal(ref b)),
        ) => ExpressionValue::Primitive(Primitive::BoolVal(a != b)),

        (
            &BinaryOpType::EqualTo,
            &ExpressionValue::Primitive(Primitive::NullVal),
            &ExpressionValue::Primitive(Primitive::NullVal),
        )
        | (
            &BinaryOpType::EqualTo,
            &ExpressionValue::Primitive(Primitive::Undefined),
            &ExpressionValue::Primitive(Primitive::Undefined),
        ) => ExpressionValue::Primitive(Primitive::BoolVal(true)),

        (
            &BinaryOpType::NotEqualTo,
            &ExpressionValue::Primitive(Primitive::NullVal),
            &ExpressionValue::Primitive(Primitive::NullVal),
        )
        | (
            &BinaryOpType::NotEqualTo,
            &ExpressionValue::Primitive(Primitive::Undefined),
            &ExpressionValue::Primitive(Primitive::Undefined),
        ) => ExpressionValue::Primitive(Primitive::BoolVal(false)),

        (&BinaryOpType::EqualTo, _, _) => {
            ExpressionValue::Primitive(Primitive::BoolVal(false))
        }

        (&BinaryOpType::NotEqualTo, _, _) => {
            ExpressionValue::Primitive(Primitive::BoolVal(true))
        }

        _ => {
            eprintln!("[evaluate_binary_expression] Unable to evaluate BinaryOp: {:?}",
                src
            );
            return Err(try_eval_from_err!("Unable to evaluate expression."));
        }
    })
}

pub fn eval_expression(
    src: &ExpressionValue<ProcessedExpression>,
    ctx: &mut OutputContext,
) -> DocumentProcessingResult<ExpressionValue<ProcessedExpression>> {
    eprintln!("[eval expression] expr: {:?}", src);

    match *src {
        ExpressionValue::Primitive(..) | ExpressionValue::Composite(..) => Ok(src.to_owned()),

        ExpressionValue::Binding(ref binding, _) | ExpressionValue::BindingShape(BindingShape(ref binding, ..), ..) => {
            eval_inner_binding(binding, ctx)
        }

        ExpressionValue::Lens(LensValue::GetLens(_, box ref expr, _), _) => {
            // Since we are evaluating, we can drop the lens in the result.
            eval_expression(expr, ctx)
        }

        ExpressionValue::Expression(ref e) => eval_inner_expression(e, ctx),

        _ => {
            eprintln!("[eval_expression] Unable to evaluate expression: {:?}",
                src
            );
            return Err(try_eval_from_err!("Unable to evaluate expression."));
        }
    }
}

// impl TryEvalFrom<Expression<ProcessedExpression>> for ExpressionValue<ProcessedExpression> {
//     fn try_eval_from(
//         src: &Expression<ProcessedExpression>,
//         ctx: &mut OutputContext,
//     ) -> DocumentProcessingResult<Self> {
//         Ok(match *src {
//             Expression::QueryCall(ref query, _) => TryEvalFrom::try_eval_from(query, ctx)?,
//             // Expression::Composite(ref e) => ExpressionValue::Expression(Expression::Composite(
//             //     TryEvalFrom::try_eval_from(e, ctx)?,
//             // )),

//             Expression::Composite(CompositeValue::ArrayValue(ArrayValue(Some(box ref v)))) => {
//                 let v: Vec<_> = ok_or_error(v.into_iter().map(|e| TryEvalFrom::try_eval_from(e, ctx)))?.collect();
//                 ExpressionValue::Composite(CompositeValue::ArrayValue(ArrayValue(Some(Box::new(v))))))
//             }
//             Expression::Composite(CompositeValue::ArrayValue(ArrayValue(..))) => {
//                 ExpressionValue::Composite(CompositeValue::ArrayValue(ArrayValue(None))))
//             }
//             Expression::Composite(CompositeValue::MapValue(MapValue(ref s, Some(box ref v)))) => {
//                 let s = s.as_ref().map(|s| s.to_owned());
//                 let v: Vec<_> = ok_or_error(v.into_iter().map(|e| TryEvalFrom::try_eval_from(e, ctx)))?.collect();
//                 ExpressionValue::Composite(CompositeValue::MapValue(MapValue(s, Some(Box::new(v))))))
//             }
//             Expression::Composite(CompositeValue::MapValue(MapValue(ref s, ..))) => {
//                 let s = s.as_ref().map(|s| s.to_owned());
//                 ExpressionValue::Composite(CompositeValue::MapValue(MapValue(s, None))))
//             }
//             Expression::Composite(CompositeValue::ObjectValue(ObjectValue(Some(box ref v)))) => {
//                 let v: Vec<_> = ok_or_error(v.into_iter().map(|e| TryEvalFrom::try_eval_from(e, ctx)))?.collect();
//                 ExpressionValue::Composite(CompositeValue::ObjectValue(ObjectValue(Some(Box::new(v))))))
//             }
//             Expression::Composite(CompositeValue::ObjectValue(ObjectValue(..))) => {
//                 ExpressionValue::Composite(CompositeValue::ObjectValue(ObjectValue(None))))
//             }

//             Expression::Path(ref p, _) => TryEvalFrom::try_eval_from(p, ctx)?,

//             // Expression::ReducedPipeline(ref p, _) => TryEvalFrom::try_eval_from(p, ctx)?,

//             _ => {
//                 if let Some(expr) = eval_expression(src, ctx)? {
//                     return Ok(expr);
//                 };
//                 // Return the same object to stop the recursion
//                 // ExpressionValue::Expression(src.to_owned())
//                 // TryEvalFrom::try_eval_from(src, ctx)?
//                 return Err(try_eval_from_err!("Unable to evaluate expression."));
//             }
//         })
//     }
// }

// impl TryEvalFrom<Expression<OutputExpression>> for ExpressionValue<OutputExpression> {
//     fn try_eval_from(
//         src: &Expression<OutputExpression>,
//         ctx: &mut OutputContext,
//     ) -> DocumentProcessingResult<Self> {
//         Ok(match *src {
//             Expression::Composite(ref e) => ExpressionValue::Expression(Expression::Composite(
//                 TryEvalFrom::try_eval_from(e, ctx)?,
//             )),
//             // Expression::Path(ref p, _) => TryEvalFrom::try_eval_from(p, ctx)?,

//             // Expression::ReducedPipeline(ref p, _) => TryEvalFrom::try_eval_from(p, ctx)?,
//             // _ => eval_expression(src, ctx)?
//             //     .unwrap_or_else(|| ExpressionValue::Expression(src.clone())),

//             _ => {
//                 return Err(try_eval_from_err!("Unable to evaluate expression."));
//             }
//         })
//     }
// }

impl TryProcessFrom<ExpressionValue<SourceExpression>> for ExpressionValue<ProcessedExpression> {
    fn try_process_from(
        src: &ExpressionValue<SourceExpression>,
        ctx: &mut ProcessingContext,
    ) -> DocumentProcessingResult<Self> {
        match *src {
            ExpressionValue::Composite(ref e) => Ok(ExpressionValue::Composite(TryProcessFrom::try_process_from(e, ctx)?)),
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

impl<I, O> TryProcessFrom<ObjectValue<I>> for ObjectValue<O>
where
    ExpressionValue<O>: TryProcessFrom<ExpressionValue<I>>,
    I: ::std::fmt::Debug,
    O: ::std::fmt::Debug,
{
    fn try_process_from(
        src: &ObjectValue<I>,
        ctx: &mut ProcessingContext,
    ) -> DocumentProcessingResult<Self> {
        let props = match src.0.as_ref() {
            Some(&box ref props) => {
                let props: Vec<PropValue<O>> =
                    TryProcessFrom::try_process_from(props, ctx)?;

                Some(Box::new(props))
            }

            _ => None
        };

        Ok(ObjectValue(props))
    }
}

impl<I, O> TryEvalFrom<ObjectValue<I>> for ObjectValue<O>
where
    ExpressionValue<O>: TryEvalFrom<ExpressionValue<I>>,
    I: ::std::fmt::Debug,
    O: ::std::fmt::Debug,
{
    fn try_eval_from(
        src: &ObjectValue<I>,
        ctx: &mut OutputContext,
    ) -> DocumentProcessingResult<ObjectValue<O>> {
        let props = match src.0.as_ref() {
            Some(&box ref props) => {
                let props: Vec<PropValue<O>> =
                    TryEvalFrom::try_eval_from(props, ctx)?;

                Some(Box::new(props))
            }

            _ => None
        };

        Ok(ObjectValue(props))
    }
}

impl<I, O> TryProcessFrom<ArrayValue<I>> for ArrayValue<O>
where
    ExpressionValue<O>: TryProcessFrom<ExpressionValue<I>>,
    I: ::std::fmt::Debug,
    O: ::std::fmt::Debug,
{
    fn try_process_from(
        src: &ArrayValue<I>,
        ctx: &mut ProcessingContext,
    ) -> DocumentProcessingResult<ArrayValue<O>> {
        let params = match src.0.as_ref() {
            Some(&box ref params) => {
                let params: Vec<ParamValue<O>> =
                    TryProcessFrom::try_process_from(params, ctx)?;

                Some(Box::new(params))
            }

            _ => None
        };

        Ok(ArrayValue(params))
    }
}

impl<I, O> TryEvalFrom<ArrayValue<I>> for ArrayValue<O>
where
    ExpressionValue<O>: TryEvalFrom<ExpressionValue<I>>,
    I: ::std::fmt::Debug,
    O: ::std::fmt::Debug,
{
    fn try_eval_from(
        src: &ArrayValue<I>,
        ctx: &mut OutputContext,
    ) -> DocumentProcessingResult<ArrayValue<O>> {
        let params = match src.0.as_ref() {
            Some(&box ref params) => {
                let params: Vec<ParamValue<O>> =
                    TryEvalFrom::try_eval_from(params, ctx)?;

                Some(Box::new(params))
            }

            _ => None
        };

        Ok(ArrayValue(params))
    }
}

impl<I, O> TryProcessFrom<MapValue<I>> for MapValue<O>
where
    ExpressionValue<O>: TryProcessFrom<ExpressionValue<I>>,
    I: ::std::fmt::Debug,
    O: ::std::fmt::Debug,
{
    fn try_process_from(
        src: &MapValue<I>,
        ctx: &mut ProcessingContext,
    ) -> DocumentProcessingResult<Self> {
        let entries = match src.1.as_ref() {
            Some(&box ref entries) => {
                let entries: Vec<ObjectValue<O>> =
                    TryProcessFrom::try_process_from(entries, ctx)?;

                Some(Box::new(entries))
            }

            _ => None
        };

        let auto = src.0.as_ref().map(|s| s.to_owned());
        Ok(MapValue(auto, entries))
    }
}

impl<I, O> TryEvalFrom<ArrayOf<I>> for ArrayOf<O>
where
    O: TryEvalFrom<I>,
    I: ::std::fmt::Debug,
    O: ::std::fmt::Debug,
{
    fn try_eval_from(
        src: &ArrayOf<I>,
        ctx: &mut OutputContext,
    ) -> DocumentProcessingResult<ArrayOf<O>> {
        let entries = match src.0.as_ref() {
            Some(&box ref entries) => {
                let entries: Vec<O> =
                    TryEvalFrom::try_eval_from(entries, ctx)?;

                Some(Box::new(entries))
            }

            _ => None
        };

        Ok(ArrayOf(entries))
    }
}

impl<I, O> TryEvalFrom<MapValue<I>> for MapValue<O>
where
    ExpressionValue<O>: TryEvalFrom<ExpressionValue<I>>,
    I: ::std::fmt::Debug,
    O: ::std::fmt::Debug,
{
    fn try_eval_from(
        src: &MapValue<I>,
        ctx: &mut OutputContext,
    ) -> DocumentProcessingResult<MapValue<O>> {
        let entries = match src.1.as_ref() {
            Some(&box ref entries) => {
                let entries: Vec<ObjectValue<O>> =
                    TryEvalFrom::try_eval_from(entries, ctx)?;

                Some(Box::new(entries))
            }

            _ => None
        };

        let auto: Option<String> = src.0.as_ref().map(|s| s.to_owned());
        Ok(MapValue(auto, entries))
    }
}

impl TryProcessFrom<CompositeValue<SourceExpression>> for CompositeValue<ProcessedExpression> {
    fn try_process_from(
        src: &CompositeValue<SourceExpression>,
        ctx: &mut ProcessingContext,
    ) -> DocumentProcessingResult<Self> {
        match *src {
            CompositeValue::ObjectValue(ref value) => Ok(CompositeValue::ObjectValue(TryProcessFrom::try_process_from(value, ctx)?)),
            CompositeValue::ArrayValue(ref value) => Ok(CompositeValue::ArrayValue(TryProcessFrom::try_process_from(value, ctx)?)),
            CompositeValue::MapValue(ref value) => Ok(CompositeValue::MapValue(TryProcessFrom::try_process_from(value, ctx)?)),
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
            CompositeValue::ObjectValue(ref value) => Ok(CompositeValue::ObjectValue(TryEvalFrom::try_eval_from(value, ctx)?)),
            CompositeValue::ArrayValue(ref value) => Ok(CompositeValue::ArrayValue(TryEvalFrom::try_eval_from(value, ctx)?)),
            CompositeValue::MapValue(ref value) => Ok(CompositeValue::MapValue(TryEvalFrom::try_eval_from(value, ctx)?)),
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

// impl TryEvalFrom<CommonBindings<ProcessedExpression>> for ExpressionValue<OutputExpression> {
//     fn try_eval_from(
//         src: &CommonBindings<ProcessedExpression>,
//         ctx: &mut OutputContext,
//     ) -> DocumentProcessingResult<Self> {
//         // Evaluate reducer key from provider or defaults
//         if let CommonBindings::NamedReducerKey(ref key, _) = *src {
//             let reducer_value =  ctx.defaults().reducer_value(key)?;
//             return TryEvalFrom::try_eval_from(&reducer_value, ctx);
//         };

//         // Pass through certain values to be evaluated at runtime
//         match *src {
//             CommonBindings::CurrentElementValue(_) => {
//                 return Ok(ExpressionValue::Binding(
//                     CommonBindings::CurrentElementValue(Default::default()),
//                     Default::default(),
//                 ));
//             }

//             CommonBindings::CurrentReducerState(_) => {
//                 return Ok(ExpressionValue::Binding(
//                     CommonBindings::CurrentReducerState(Default::default()),
//                     Default::default(),
//                 ));
//             }

//             // CommonBindings::CurrentItem(_) => {
//             //     return Ok(ExpressionValue::Binding(
//             //         CommonBindings::CurrentItem(Default::default()),
//             //         Default::default(),
//             //     ));
//             // }

//             // // Skip evaluation on first pass
//             // CommonBindings::NamedComponentProp(ref name, _) => {
//             //     return Ok(ExpressionValue::Binding(
//             //         CommonBindings::NamedComponentProp(name.clone(), Default::default()),
//             //         Default::default(),
//             //     ));
//             // }

//             CommonBindings::NamedElementBoundValue(ref element_key, _) => {
//                 return Ok(ExpressionValue::Binding(
//                     CommonBindings::NamedElementBoundValue(element_key.clone(), Default::default()),
//                     Default::default(),
//                 ));
//             }

//             _ => {}
//         };

//         // Otherwise, try to find the value for the binding or return an error
//         eprintln!(
//             "[Expression eval] Looking for value for binding [{:?}]",
//             src
//         );
//         let expr = ctx.must_find_value(src)?;
//         TryEvalFrom::try_eval_from(&expr, ctx)
//     }
// }

// impl TryEvalFrom<ExpressionValue<ProcessedExpression>> for ExpressionValue<OutputExpression> {
//     fn try_eval_from(
//         src: &ExpressionValue<ProcessedExpression>,
//         ctx: &mut OutputContext,
//     ) -> DocumentProcessingResult<Self> {
//         match *src {
//             ExpressionValue::Expression(ref e) => Ok(TryEvalFrom::try_eval_from(e, ctx)?),

//             ExpressionValue::Primitive(ref p) => Ok(ExpressionValue::Primitive(p.to_owned())),

//             ExpressionValue::Binding(ref b, _) => TryEvalFrom::try_eval_from(b, ctx),
//             ExpressionValue::BindingShape(ref s, _) => TryEvalFrom::try_eval_from(s.binding(), ctx),

//             ExpressionValue::Lens(ref l, _) => {
//                 // Since we are evaluating, we can drop the lens in the result.
//                 Ok(TryEvalFrom::try_eval_from(l, ctx)?)
//             }

//             ExpressionValue::Content(ref c, _) => Ok(ExpressionValue::Content(
//                 TryEvalFrom::try_eval_from(c, ctx)?,
//                 Default::default(),
//             )),

//             _ => Err(reduction_err_bt!()),
//         }
//     }
// }

// impl TryEvalFrom<ExpressionValue<OutputExpression>> for ExpressionValue<OutputExpression> {
//     fn try_eval_from(
//         src: &ExpressionValue<OutputExpression>,
//         ctx: &mut OutputContext,
//     ) -> DocumentProcessingResult<Self> {
//         Ok(match *src {
//             ExpressionValue::Binding(ref b, _) => {
//                 // Evaluate reducer key from provider or defaults
//                 if let CommonBindings::NamedReducerKey(ref key, _) = *b {
//                     let reducer_value =  ctx.defaults().reducer_value(key)?;
//                     return TryEvalFrom::try_eval_from(&reducer_value, ctx);
//                 };

//                 let expr = ctx.must_find_loop_value(b)?;

//                 // If we found a binding, evaluate it
//                 TryEvalFrom::try_eval_from(&expr, ctx)?
//             }

//             ExpressionValue::Expression(ref e) => {
//                 eval_expression(e, ctx)?.unwrap_or_else(|| src.clone())
//             }
//             _ => src.clone(),
//         })
//     }
// }

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

impl<T: Clone + Debug> TryEvalFrom<ExpressionValue<T>>
    for Option<Vec<ExpressionValue<T>>>
{
    fn try_eval_from(
        src: &ExpressionValue<T>,
        _ctx: &mut OutputContext,
    ) -> DocumentProcessingResult<Self> {
        eprintln!("TryEval ExpressionValue -> Option<Vec<_>>: src: {:?}", src);
        match *src {
            ExpressionValue::Composite(CompositeValue::ArrayValue(
                ArrayValue(Some(box ref arr)),
            )) => {
                let arr: Vec<_> = arr.into_iter()
                    .map(|item| item.value().to_owned())
                    .collect();
                Ok(Some(arr))
            }

            ExpressionValue::Composite(CompositeValue::ArrayValue(
                ArrayValue(None),
            )) => {
                Ok(None)
            }

            ExpressionValue::Composite(CompositeValue::MapValue(
                MapValue(_, Some(box ref arr)),
            )) => {
                let arr: Vec<_> = arr.into_iter()
                    .map(|item| ExpressionValue::Composite(CompositeValue::ObjectValue(item.to_owned())))
                    .collect();
                Ok(Some(arr))
            }

            ExpressionValue::Composite(CompositeValue::MapValue(
                MapValue(None, None),
            )) => {
                Ok(None)
            }

            // Cannot evaluate object value as an array
            ExpressionValue::Composite(CompositeValue::ObjectValue(..)) => {
                eprintln!("TryEval ExpressionValue -> Option<Vec<_>>: cannot evaluate as array: {:?}", src);
                Ok(None)
            }

            _ => Err(try_process_from_err!(
                "Cannot evaluate non-composite values as vec."
            )),
        }
    }
}

pub fn eval_inner_binding(
    binding: &CommonBindings<ProcessedExpression>,
    ctx: &mut OutputContext,
) -> DocumentProcessingResult<ExpressionValue<ProcessedExpression>> {
    eprintln!(
        "[expression] eval_inner_binding: binding: {:?}",
        binding
    );

    // Evaluate reducer key from provider or defaults
    if let CommonBindings::NamedReducerKey(ref key, _) = *binding {
        let reducer_value = ctx.defaults().reducer_value(key)?;
        match reducer_value {
            ReducerValue::ProcessedExpression(e) => {
                return eval_expression(&e, ctx);
            }

            ReducerValue::OutputExpression(..) => {
                return Err(try_eval_from_err!("OutputExpression reducer expression not supported in `eval_inner_binding`"));
            }
        };
    };

    // Handle bindings only available in the frontend application
    if let CommonBindings::CurrentElementValue(..) = *binding {
        return Ok(ExpressionValue::Binding(binding.to_owned(), Default::default()));
    }

    if let Some(expr) = ctx.find_value(binding)? {
        return eval_expression(&expr, ctx);
    };

    let expr = ctx.must_find_loop_value(binding)?;
    eval_expression(&expr, ctx)
}
