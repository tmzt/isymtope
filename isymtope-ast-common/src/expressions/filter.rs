use std::fmt::Debug;
use std::marker::PhantomData;

use error::*;
use traits::*;
use expressions::*;
// use output::*;

#[allow(dead_code)]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct FilterValue<T>(Box<ExpressionValue<T>>, Box<Vec<FilterComponentValue<T>>>);

impl<T> FilterValue<T> {
    pub fn new(e: ExpressionValue<T>, v: Vec<FilterComponentValue<T>>) -> Self {
        FilterValue(Box::new(e), Box::new(v))
    }

    pub fn head(&self) -> &ExpressionValue<T> {
        self.0.as_ref()
    }

    pub fn components<'a>(&'a self) -> impl Iterator<Item = &'a FilterComponentValue<T>> {
        let box ref v = self.1;
        v.iter()
    }
}

impl<T: Debug> MapIdents<T> for FilterValue<T> {
    fn map_idents(self, ctx: &mut ProcessingContext) -> DocumentProcessingResult<Self> {
        let head = self.0.map_idents(ctx)?;
        let box components = self.1;
        let components: Vec<_> = components.into_iter().map(|c| c.map_idents(ctx)).collect();
        let components: Vec<_> = ok_or_error(components)?.collect();

        Ok(FilterValue(Box::new(head), Box::new(components)))
    }
}

impl<I, O> TryProcessFrom<FilterValue<I>> for FilterValue<O>
where
    ExpressionValue<O>: TryProcessFrom<ExpressionValue<I>>,
    I: ::std::fmt::Debug,
    O: ::std::fmt::Debug,
{
    fn try_process_from(
        src: &FilterValue<I>,
        ctx: &mut ProcessingContext,
    ) -> DocumentProcessingResult<Self> {
        let &box ref expr = &src.0;
        let &box ref fcv = &src.1;

        let expr: ExpressionValue<O> = TryProcessFrom::try_process_from(expr, ctx)?;
        let fcv: Vec<FilterComponentValue<O>> = TryProcessFrom::try_process_from(fcv, ctx)?;

        Ok(FilterValue(Box::new(expr), Box::new(fcv)))
    }
}

impl<I, O> TryEvalFrom<FilterValue<I>> for FilterValue<O>
where
    ExpressionValue<O>: TryEvalFrom<ExpressionValue<I>>,
    I: ::std::fmt::Debug,
    O: ::std::fmt::Debug,
{
    fn try_eval_from(
        src: &FilterValue<I>,
        ctx: &mut OutputContext,
    ) -> DocumentProcessingResult<Self> {
        let &box ref expr = &src.0;
        let &box ref fcv = &src.1;

        let expr: ExpressionValue<O> = TryEvalFrom::try_eval_from(expr, ctx)?;
        let fcv: Vec<FilterComponentValue<O>> = TryEvalFrom::try_eval_from(fcv, ctx)?;

        Ok(FilterValue(Box::new(expr), Box::new(fcv)))
    }
}

#[allow(dead_code)]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum FilterComponentValue<T> {
    Where(FilterWhereClause<T>, PhantomData<T>),
    Set(
        Vec<FilterSetAssignment<T>>,
        Option<FilterWhereClause<T>>,
        PhantomData<T>,
    ),
    Delete(String),
    Unique(String),
}

impl<T: Debug> MapIdents<T> for FilterComponentValue<T> {
    fn map_idents(self, ctx: &mut ProcessingContext) -> DocumentProcessingResult<Self> {
        Ok(match self {
            FilterComponentValue::Where(wc, _) => {
                let wc = wc.map_idents(ctx)?;
                FilterComponentValue::Where(wc, Default::default())
            }

            FilterComponentValue::Set(v, wc, _) => {
                let v: Vec<_> = v.into_iter().map(|s| s.map_idents(ctx)).collect();
                let v: Vec<_> = ok_or_error(v)?.collect();

                // let wc = wc.map(|wc| wc.map_idents(formals, formals_object));
                let wc = match wc {
                    Some(wc) => Some(wc.map_idents(ctx)?),
                    _ => None,
                };

                FilterComponentValue::Set(v, wc, Default::default())
            }

            _ => self,
        })
    }
}

impl<I, O> TryProcessFrom<FilterComponentValue<I>> for FilterComponentValue<O>
where
    ExpressionValue<O>: TryProcessFrom<ExpressionValue<I>>,
    I: ::std::fmt::Debug,
    O: ::std::fmt::Debug,
{
    fn try_process_from(
        src: &FilterComponentValue<I>,
        ctx: &mut ProcessingContext,
    ) -> DocumentProcessingResult<Self> {
        match *src {
            FilterComponentValue::Where(ref w, _) => Ok(FilterComponentValue::Where(
                TryProcessFrom::try_process_from(w, ctx)?,
                Default::default(),
            )),

            FilterComponentValue::Set(ref v, ref w, _) => {
                let v: Vec<FilterSetAssignment<O>> = TryProcessFrom::try_process_from(v, ctx)?;
                let w: Option<FilterWhereClause<O>> = TryProcessFrom::try_process_from(w, ctx)?;

                Ok(FilterComponentValue::Set(v, w, Default::default()))
            }

            FilterComponentValue::Delete(ref s) => Ok(FilterComponentValue::Delete(s.to_owned())),
            FilterComponentValue::Unique(ref s) => Ok(FilterComponentValue::Unique(s.to_owned())),
        }
    }
}

impl<I, O> TryEvalFrom<FilterComponentValue<I>> for FilterComponentValue<O>
where
    ExpressionValue<O>: TryEvalFrom<ExpressionValue<I>>,
    I: ::std::fmt::Debug,
    O: ::std::fmt::Debug,
{
    fn try_eval_from(
        src: &FilterComponentValue<I>,
        ctx: &mut OutputContext,
    ) -> DocumentProcessingResult<Self> {
        match *src {
            FilterComponentValue::Where(ref w, _) => Ok(FilterComponentValue::Where(
                TryEvalFrom::try_eval_from(w, ctx)?,
                Default::default(),
            )),

            FilterComponentValue::Set(ref v, ref w, _) => {
                let v: Vec<FilterSetAssignment<O>> = TryEvalFrom::try_eval_from(v, ctx)?;
                let w: Option<FilterWhereClause<O>> = TryEvalFrom::try_eval_from(w, ctx)?;

                Ok(FilterComponentValue::Set(v, w, Default::default()))
            }

            FilterComponentValue::Delete(ref s) => Ok(FilterComponentValue::Delete(s.to_owned())),
            FilterComponentValue::Unique(ref s) => Ok(FilterComponentValue::Unique(s.to_owned())),
        }
    }
}

#[allow(dead_code)]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum FilterSetAssignment<T> {
    SetMemberTo(String, ExpressionValue<T>, PhantomData<T>),
}

impl<T: Debug> MapIdents<T> for FilterSetAssignment<T> {
    fn map_idents(self, ctx: &mut ProcessingContext) -> DocumentProcessingResult<Self> {
        match self {
            FilterSetAssignment::SetMemberTo(s, e, _) => {
                let e = e.map_idents(ctx)?;
                Ok(FilterSetAssignment::SetMemberTo(s, e, Default::default()))
            }
        }
    }
}

impl<I, O> TryProcessFrom<FilterSetAssignment<I>> for FilterSetAssignment<O>
where
    ExpressionValue<O>: TryProcessFrom<ExpressionValue<I>>,
{
    fn try_process_from(
        src: &FilterSetAssignment<I>,
        ctx: &mut ProcessingContext,
    ) -> DocumentProcessingResult<Self> {
        match *src {
            FilterSetAssignment::SetMemberTo(ref s, ref e, _) => {
                Ok(FilterSetAssignment::SetMemberTo(
                    s.to_owned(),
                    TryProcessFrom::try_process_from(e, ctx)?,
                    Default::default(),
                ))
            }
        }
    }
}

impl<I, O> TryEvalFrom<FilterSetAssignment<I>> for FilterSetAssignment<O>
where
    ExpressionValue<O>: TryEvalFrom<ExpressionValue<I>>,
{
    fn try_eval_from(
        src: &FilterSetAssignment<I>,
        ctx: &mut OutputContext,
    ) -> DocumentProcessingResult<Self> {
        match *src {
            FilterSetAssignment::SetMemberTo(ref s, ref e, _) => {
                Ok(FilterSetAssignment::SetMemberTo(
                    s.to_owned(),
                    TryEvalFrom::try_eval_from(e, ctx)?,
                    Default::default(),
                ))
            }
        }
    }
}

#[allow(dead_code)]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct FilterWhereClause<T>(Box<Vec<ExpressionValue<T>>>);

impl<T> FilterWhereClause<T> {
    pub fn new(anded_conditions: Vec<ExpressionValue<T>>) -> Self {
        FilterWhereClause(Box::new(anded_conditions))
    }

    pub fn anded_conditions<'a>(&'a self) -> impl Iterator<Item = &'a ExpressionValue<T>> {
        self.0.iter()
    }
}

impl<T: Debug> MapIdents<T> for FilterWhereClause<T> {
    fn map_idents(self, ctx: &mut ProcessingContext) -> DocumentProcessingResult<Self> {
        let box v = self.0;
        let v: Vec<_> = v.into_iter().map(|e| e.map_idents(ctx)).collect();
        let v: Vec<_> = ok_or_error(v)?.collect();

        Ok(FilterWhereClause(Box::new(v)))
    }
}

impl<I, O> TryProcessFrom<FilterWhereClause<I>> for FilterWhereClause<O>
where
    ExpressionValue<O>: TryProcessFrom<ExpressionValue<I>>,
    I: ::std::fmt::Debug,
    O: ::std::fmt::Debug,
{
    fn try_process_from(
        src: &FilterWhereClause<I>,
        ctx: &mut ProcessingContext,
    ) -> DocumentProcessingResult<Self> {
        let box ref expr = src.0;
        let expr: Vec<ExpressionValue<O>> = TryProcessFrom::try_process_from(expr, ctx)?;

        Ok(FilterWhereClause(Box::new(expr)))
    }
}

impl<I, O> TryEvalFrom<FilterWhereClause<I>> for FilterWhereClause<O>
where
    ExpressionValue<O>: TryEvalFrom<ExpressionValue<I>>,
    I: ::std::fmt::Debug,
    O: ::std::fmt::Debug,
{
    fn try_eval_from(
        src: &FilterWhereClause<I>,
        ctx: &mut OutputContext,
    ) -> DocumentProcessingResult<Self> {
        let box ref expr = src.0;
        let expr: Vec<ExpressionValue<O>> = TryEvalFrom::try_eval_from(expr, ctx)?;

        Ok(FilterWhereClause(Box::new(expr)))
    }
}
