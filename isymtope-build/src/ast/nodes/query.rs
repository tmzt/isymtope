use std::marker::PhantomData;

use error::*;
use traits::*;
use expressions::*;
use objects::*;

#[allow(dead_code)]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct QueryDefinition<T>(String, FormalParams<T>, Vec<QueryComponent<T>>);

impl<T> QueryDefinition<T> {
    pub fn new(name: String, params: FormalParams<T>, children: Vec<QueryComponent<T>>) -> Self {
        QueryDefinition(name, params, children)
    }
}

impl TryProcessFrom<QueryDefinition<SourceExpression>> for Query<ProcessedExpression> {
    fn try_process_from(
        src: &QueryDefinition<SourceExpression>,
        ctx: &mut ProcessingContext,
    ) -> DocumentProcessingResult<Self> {
        debug!("TryProcess QueryDefinition -> Query: src: {:?}", src);

        let name = src.0.to_owned();
        let params: FormalParams<ProcessedExpression> =
            TryProcessFrom::try_process_from(&src.1, ctx)?;

        ctx.push_child_scope();
        if let Some(params) = params.params() {
            for param in params {
                let binding = CommonBindings::NamedQueryParam(param.to_owned(), Default::default());
                eprintln!("Binding QueryParam [{:?}] as [{}]", binding, param);
                ctx.bind_ident(param.to_owned(), binding)?;
            }
        };

        let components: Vec<QueryComponent<ProcessedExpression>> =
            TryProcessFrom::try_process_from(&src.2, ctx)?;
        ctx.pop_scope();

        Ok(Query::new(name, params, components))
    }
}

#[allow(dead_code)]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum ParsedQueryParamValue<T> {
    Lens(SourceLensValue<T>),
    Prop(PropValue<T>, PhantomData<T>),
}

// impl<I, O> TryProcessFrom<ParsedQueryParamValue<I>> for ParsedQueryParamValue<O> where ExpressionValue<O>: TryProcessFrom<ExpressionValue<I>>, I: ::std::fmt::Debug, O: ::std::fmt::Debug {
//     fn try_process_from(src: &ParsedQueryParamValue<I>, ctx: &mut ProcessingContext) -> DocumentProcessingResult<Self> {
//         Ok(match *src {
//             ParsedQueryParamValue::Lens(ref l) => ParsedQueryParamValue::Lens(TryProcessFrom::try_process_from(l, ctx)?),
//             ParsedQueryParamValue::Prop(ref p, _) => ParsedQueryParamValue::Prop(TryProcessFrom::try_process_from(p, ctx)?, Default::default())
//         })
//     }
// }

impl TryProcessFrom<ParsedQueryParamValue<SourceExpression>>
    for QueryParamValue<ProcessedExpression>
{
    fn try_process_from(
        src: &ParsedQueryParamValue<SourceExpression>,
        ctx: &mut ProcessingContext,
    ) -> DocumentProcessingResult<Self> {
        eprintln!(
            "[Query] TryProcess ParsedQueryParamValue -> QueryParamValue: src: {:?}",
            src
        );

        Ok(match *src {
            ParsedQueryParamValue::Lens(ref l) => {
                let default_alias = l.default_alias()?.to_owned();
                eprintln!("[Query] TryProcess ParsedQueryParamValue -> QueryParamValue: default_alias: {:?}", default_alias);
                let lens = TryProcessFrom::try_process_from(l, ctx)?;
                eprintln!(
                    "[Query] TryProcess ParsedQueryParamValue -> QueryParamValue: lens: {:?}",
                    lens
                );

                QueryParamValue::new(
                    default_alias,
                    ExpressionValue::Lens(lens, Default::default()),
                )
            }

            ParsedQueryParamValue::Prop(ref p, _) => {
                let key = p.key().to_owned();
                eprintln!(
                    "[Query] TryProcess ParsedQueryParamValue -> QueryParamValue: key: {:?}",
                    key
                );
                let value = p.value();
                eprintln!(
                    "[Query] TryProcess ParsedQueryParamValue -> QueryParamValue: value: {:?}",
                    value
                );

                QueryParamValue::new(key, TryProcessFrom::try_process_from(value, ctx)?)
            }
        })
    }
}

// impl<I, O> TryProcessFrom<ParsedQueryParamValue<I>> for ParsedQueryParamValue<O> where ExpressionValue<O>: TryProcessFrom<ExpressionValue<I>>, I: ::std::fmt::Debug, O: ::std::fmt::Debug {
//     fn try_process_from(src: &ParsedQueryParamValue<I>) -> DocumentProcessingResult<Self> {
//         Ok(match *src {
//             ParsedQueryParamValue::Lens(ref l) => ParsedQueryParamValue::Lens(TryProcessFrom::try_process_from(l)?),
//             ParsedQueryParamValue::PropValue(ref v, _) => ParsedQueryParamValue(TryProcessFrom::try_process_from(v)?, Default::default())
//         })
//     }
// }

#[allow(dead_code)]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct LensQueryCall<T>(String, Box<Vec<ParsedQueryParamValue<T>>>);

impl<T> LensQueryCall<T> {
    pub fn new(name: String, params: Vec<ParsedQueryParamValue<T>>) -> Self {
        LensQueryCall(name, Box::new(params))
    }

    pub fn name(&self) -> &str {
        &self.0.as_str()
    }
}

// impl<I, O> TryProcessFrom<LensQueryCall<I>> for LensQueryCall<O> where ExpressionValue<O>: TryProcessFrom<ExpressionValue<I>>, I: ::std::fmt::Debug, O: ::std::fmt::Debug {
//     fn try_process_from(src: &LensQueryCall<I>, ctx: &mut ProcessingContext) -> DocumentProcessingResult<Self> {
//         let params: Vec<ParsedQueryParamValue<O>> = TryProcessFrom::try_process_from(&src.1, ctx)?;
//         Ok(LensQueryCall(src.0.to_owned(), Box::new(params)))
//     }
// }

impl TryProcessFrom<LensQueryCall<SourceExpression>> for QueryCall<ProcessedExpression> {
    fn try_process_from(
        src: &LensQueryCall<SourceExpression>,
        ctx: &mut ProcessingContext,
    ) -> DocumentProcessingResult<Self> {
        eprintln!(
            "[Query] TryProcess LensQueryCall -> QueryCall: src: {:?}",
            src
        );
        let params: Vec<QueryParamValue<ProcessedExpression>> =
            TryProcessFrom::try_process_from(&src.1, ctx)?;
        eprintln!(
            "[Query] TryProcess LensQueryCall -> QueryCall: params: {:?}",
            params
        );

        // let params = src.1.clone();

        // let query_call: QueryCall<SourceExpression> = QueryCall::new(src.0.to_owned(), params);
        // eprintln!("[Query] TryProcess LensQueryCall -> QueryCall: query_call: {:?}", query_call);

        // let query_call: QueryCall<ProcessedExpression> = TryProcessFrom::try_process_from(&query_call, ctx)?;
        let query_call = QueryCall::new(src.name().to_owned(), params);
        eprintln!(
            "[Query] TryProcess LensQueryCall -> QueryCall: query_call: {:?}",
            query_call
        );

        Ok(query_call)
    }
}
