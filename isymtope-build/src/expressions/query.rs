
use error::*;
use traits::*;
use expressions::*;
use ast::*;
use output::*;


#[allow(dead_code)]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct QueryParamValue<T>(String, ExpressionValue<T>);

impl<T> QueryParamValue<T> {
    pub fn new(name: String, value: ExpressionValue<T>) -> Self {
        QueryParamValue(name, value)
    }

    pub fn key(&self) -> &str { self.0.as_str() }
    pub fn value(&self) -> &ExpressionValue<T> { &self.1 }
}

impl<I, O> TryProcessFrom<QueryParamValue<I>> for QueryParamValue<O> where ExpressionValue<O>: TryProcessFrom<ExpressionValue<I>>, I: ::std::fmt::Debug, O: ::std::fmt::Debug {
    fn try_process_from(src: &QueryParamValue<I>, ctx: &mut ProcessingContext) -> DocumentProcessingResult<Self> {
        Ok(QueryParamValue(src.0.to_owned(), TryProcessFrom::try_process_from(&src.1, ctx)?))
    }
}

// impl<I, O> TryProcessFrom<QueryParamValue<I>> for QueryParamValue<O> where ExpressionValue<O>: TryProcessFrom<ExpressionValue<I>>, I: ::std::fmt::Debug, O: ::std::fmt::Debug {
//     fn try_process_from(src: &QueryParamValue<I>) -> DocumentProcessingResult<Self> {
//         Ok(match *src {
//             QueryParamValue::Lens(ref l) => QueryParamValue::Lens(TryProcessFrom::try_process_from(l)?),
//             QueryParamValue::ParamValue(ref v, _) => QueryParamValue(TryProcessFrom::try_process_from(v)?, Default::default())
//         })
//     }
// }

#[allow(dead_code)]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct QueryCall<T>(String, Box<Vec<QueryParamValue<T>>>);

impl<T> QueryCall<T> {
    pub fn new(name: String, params: Vec<QueryParamValue<T>>) -> Self {
        QueryCall(name, Box::new(params))
    }

    pub fn name(&self) -> &str { self.0.as_str() }

    pub fn params<'a>(&'a self) -> impl Iterator<Item = &'a QueryParamValue<T>> {
        self.1.as_ref().iter()
    }
}