
use std::marker::PhantomData;

use colored::*;

use error::*;
use common::*;
use traits::*;
use expressions::*;
use ast::*;
use output::*;


#[allow(dead_code)]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum SourceLensValue<T> {
    ForLens(Option<String>, Box<ExpressionValue<T>>, PhantomData<T>),
    GetLens(Option<String>, Box<ExpressionValue<T>>, PhantomData<T>),
    QueryLens(Option<String>, LensQueryCall<T>, PhantomData<T>)
}

#[allow(dead_code)]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum LensValue<T> {
    ForLens(Option<String>, Box<ExpressionValue<T>>, PhantomData<T>),
    GetLens(Option<String>, Box<ExpressionValue<T>>, PhantomData<T>),
    QueryLens(Option<String>, QueryCall<T>, PhantomData<T>)
}

impl<T> SourceLensValue<T> {
    pub fn default_alias(&self) -> DocumentProcessingResult<String> {

        // Alias is provided within lens expression
        match *self {
            SourceLensValue::ForLens(Some(ref s), _, _) |
            SourceLensValue::GetLens(Some(ref s), _, _) |
            SourceLensValue::QueryLens(Some(ref s), _, _) => {
                Ok(s.to_owned())
            }

            SourceLensValue::GetLens(None, box ExpressionValue::Expression(Expression::Ident(ref s, _)), _) => {
                Ok(s.to_owned())
            }

            _ => Err(reduction_err_bt!())
        }
    }
}

impl<T> LensValue<T> {
    pub fn default_alias(&self) -> DocumentProcessingResult<String> where T: ::std::fmt::Debug {

        // Alias is provided within lens expression
        match *self {
            LensValue::ForLens(Some(ref s), _, _) |
            LensValue::GetLens(Some(ref s), _, _) |
            LensValue::QueryLens(Some(ref s), _, _) => {
                Ok(s.to_owned())
            }

            LensValue::GetLens(None, box ExpressionValue::Binding(CommonBindings::NamedReducerKey(ref s, _), _), _) => {
                Ok(s.to_owned())
            }

            LensValue::GetLens(None, box ExpressionValue::Expression(Expression::Ident(ref s, _)), _) => {
                Ok(s.to_owned())
            }

            _ => {
                eprintln!("Unsupported LensValue for default alias: {:?}", self);
                Err(try_process_from_err!("Unsupported LensValue for default alias"))
            }
        }
    }

    pub fn expr(&self) -> DocumentProcessingResult<ExpressionValue<T>> where T: Clone {
        match *self {
            LensValue::ForLens(_, box ref expr, _) |
            LensValue::GetLens(_, box ref expr, _) => {
                Ok(expr.to_owned())
            }

            LensValue::QueryLens(_, ref query_call, _) => {
                Ok(ExpressionValue::Expression(Expression::QueryCall(query_call.to_owned(), Default::default())))
            }
        }
    }
}

impl TryProcessFrom<SourceLensValue<SourceExpression>> for LensValue<ProcessedExpression> {
    default fn try_process_from(src: &SourceLensValue<SourceExpression>, ctx: &mut ProcessingContext) -> DocumentProcessingResult<Self> {
        match *src {
            SourceLensValue::ForLens(ref s, box ref a, _) => {
                let lens = LensValue::ForLens(s.to_owned(), Box::new(TryProcessFrom::try_process_from(a, ctx)?), Default::default());
                Ok(lens)
            }

            SourceLensValue::GetLens(ref s, box ref a, _) => {
                if let ExpressionValue::Expression(Expression::Ident(ref ident_key, _)) = *a {
                    if ctx.is_reducer_key(ident_key)? {
                        let expr = ExpressionValue::Binding(CommonBindings::NamedReducerKey(ident_key.to_owned(), Default::default()), Default::default());
                        return Ok(LensValue::GetLens(s.to_owned(), Box::new(expr), Default::default()));
                    }
                };

                let lens = LensValue::GetLens(s.to_owned(), Box::new(TryProcessFrom::try_process_from(a, ctx)?), Default::default());
                Ok(lens)
            }

            SourceLensValue::QueryLens(ref alias, ref query_call, _) => {
                let query_call: QueryCall<ProcessedExpression> = TryProcessFrom::try_process_from(query_call, ctx)?;
                let lens = LensValue::QueryLens(alias.to_owned(), query_call, Default::default());
                Ok(lens)
            }
        }
    }
}

impl TryEvalFrom<LensValue<ProcessedExpression>> for ExpressionValue<OutputExpression> {
    fn try_eval_from(src: &LensValue<ProcessedExpression>, ctx: &mut OutputContext<ProcessedExpression>) -> DocumentProcessingResult<Self> {
        match *src {
            LensValue::ForLens(ref s, box ref a, _) => {
                Err(try_eval_from_err!("For lens cannot be evaluated as a value"))
            }

            LensValue::GetLens(ref s, box ref a, _) => {
                eprintln!("[GetLens] a: {:?}", a);

                // TODO: Only resolve to reducer keys at top level
                if let ExpressionValue::Binding(CommonBindings::NamedReducerKey(..), _) = *a {
                    // Resolve reducer value from state or default
                    return TryEvalFrom::try_eval_from(a, ctx);
                };

                Err(try_process_from_err!("Error processing GetLens"))
            }

            LensValue::QueryLens(ref alias, ref query_call, _) => {
                TryEvalFrom::try_eval_from(query_call, ctx)
            }
        }
    }
}
