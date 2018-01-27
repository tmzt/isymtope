
use std::fmt::Debug;
use std::collections::HashSet;

use error::*;
use traits::*;
use expressions::*;
use ast::*;


#[derive(Debug, Default, Clone, PartialEq, Eq, Hash)]
pub struct Reducer<T>(String, Option<Vec<ReducerAction<T>>>, Option<ExpressionValue<T>>, Option<OuterShape>);

impl<T> Reducer<T> {
    pub fn new(name: String, actions: Option<Vec<ReducerAction<T>>>, default_value: Option<ExpressionValue<T>>, shape: Option<OuterShape>) -> Self {
        Reducer(name, actions, default_value, shape)
    }

    pub fn key(&self) -> &str { &self.0.as_str() }

    pub fn actions<'a>(&'a self) -> Option<impl Iterator<Item = &'a ReducerAction<T>>> {
        self.1.as_ref().map(|v| v.iter())
    }

    pub fn default_value(&self) -> Option<&ExpressionValue<T>> { self.2.as_ref() }

    pub fn shape(&self) -> Option<OuterShape> { self.3 }
}

impl<I, O> TryProcessFrom<Reducer<I>> for Reducer<O> where ExpressionValue<O>: TryProcessFrom<ExpressionValue<I>>, I: ::std::fmt::Debug, O: ::std::fmt::Debug {
    fn try_process_from(src: &Reducer<I>, ctx: &mut ProcessingContext) -> DocumentProcessingResult<Self> {
        eprintln!("TryProcess Reducer<I>: Actions: {:?}", src.1);

        ctx.push_child_scope_with_environment(ProcessingScopeEnvironment::Reducer(src.0.to_owned()));
        let actions: Option<Vec<ReducerAction<O>>> = TryProcessFrom::try_process_from(&src.1, ctx)?;
        eprintln!("TryProcess Reducer<O>: Actions: {:?}", actions);
        ctx.pop_scope();

        let default_value: Option<ExpressionValue<O>> = TryProcessFrom::try_process_from(&src.2, ctx)?;


        Ok(Reducer(src.0.to_owned(), actions, default_value, src.3))
    }
}

#[derive(Debug, Default, Clone, PartialEq, Eq, Hash)]
pub struct ReducerAction<T>(String, FormalParams<T>, Option<ExpressionValue<T>>);

impl<T> ReducerAction<T> {
    pub fn new(name: String, params: FormalParams<T>, expr: Option<ExpressionValue<T>>) -> Self {
        ReducerAction(name, params, expr)
    }

    pub fn name(&self) -> &str { &self.0 }

    pub fn params<'a>(&'a self) -> Option<impl Iterator<Item = &'a str>> { self.1.params() }

    pub fn expr(&self) -> Option<&ExpressionValue<T>> { self.2.as_ref() }

}

impl<T: Debug> MapIdents<T> for ReducerAction<T> {
    fn map_idents(self, ctx: &mut ProcessingContext) -> DocumentProcessingResult<Self> {
        let formals: HashSet<String> = (&self.1).params().map(|v| v.map(|s| s.to_owned()).collect())
            .unwrap_or_default();

        for formal in formals {
            let binding = CommonBindings::NamedReducerActionParam(formal.to_owned(), Default::default());
            ctx.bind_ident(formal, binding)?;
        }

        // let expr = self.2.map(|e| e.map_idents(&formals, "action"));
        // let expr = self.2.map(|e| e.map_idents(ctx));
        let expr = match self.2 { Some(e) => Some(e.map_idents(ctx)?), _ => None };

        Ok(ReducerAction(self.0, self.1, expr))
    }
}

impl<I, O> TryProcessFrom<ReducerAction<I>> for ReducerAction<O> where ExpressionValue<O>: TryProcessFrom<ExpressionValue<I>>, I: ::std::fmt::Debug, O: ::std::fmt::Debug {
    fn try_process_from(src: &ReducerAction<I>, ctx: &mut ProcessingContext) -> DocumentProcessingResult<Self> {
        eprintln!("TryProcess ReducerAction: src: {:?}", src);
        let params: FormalParams<O> = TryProcessFrom::try_process_from(&src.1, ctx)?;

        let expr: Option<ExpressionValue<O>> = TryProcessFrom::try_process_from(&src.2, ctx)?;
        let action = ReducerAction(src.0.to_owned(), params, expr);
        eprintln!("TryProcess ReducerAction: action: {:?}", action);

        Ok(action)
        // Ok(ReducerAction(src.0.to_owned(), TryProcessFrom::try_process_from(&src.1)?, TryProcessFrom::try_process_from(&src.2)?))
    }
}
