use std::marker::PhantomData;
use std::borrow::Cow;

use regex::*;

use expressions::*;
use objects::*;
use ast::*;

fn function_key(name: &str) -> Cow<str> {
    lazy_static! {
        static ref REGEX: Regex = Regex::new("[^a-zA-Z0-9]").unwrap();
    }
    REGEX.replace_all(name, "_")
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Route(String, FormalParams<ProcessedExpression>, Option<Vec<ActionOp<ProcessedExpression>>>, Option<Block<ProcessedExpression>>, bool);

impl Route {
    pub fn new(pattern: String, params: FormalParams<ProcessedExpression>, actions: Option<Vec<ActionOp<ProcessedExpression>>>, content: Option<Block<ProcessedExpression>>, client_only: bool) -> Self {
        Route(pattern, params, actions, content, client_only)
    }

    pub fn pattern(&self) -> &str {
        &self.0
    }

    pub fn function_key(&self) -> Cow<str> {
        function_key(&self.0)
    }

    pub fn actions<'r>(&'r self) -> Option<impl IntoIterator<Item = &'r ActionOp<ProcessedExpression>>> {
        self.2.as_ref().map(|s| s.iter())
    }

    pub fn content(&self) -> Option<&Block<ProcessedExpression>> {
        self.3.as_ref()
    }

    pub fn client_only(&self) -> bool {
        self.4
    }
}

// impl<I, O> TryProcessFrom<Route<I>> for Route<O> where ExpressionValue<O>: TryProcessFrom<ExpressionValue<I>>, I: ::std::fmt::Debug, O: ::std::fmt::Debug {
//     fn try_process_from(src: &Route<I>) -> DocumentProcessingResult<Self> {
//         eprintln!("TryProcess Route<I>: Params: {:?}", src.1);
//         eprintln!("TryProcess Route<I>: Action: {:?}", src.2);

//         let params: FormalParams<O> = TryProcessFrom::try_process_from(&src.1)?;
//         let action: RouteActionValue<O> = TryProcessFrom::try_process_from(&src.1)?;

//         Ok(Route(src.0.to_owned(), params, action))
//     }
// }

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum RouteActionValue<T> {
    Block(Block<T>, PhantomData<T>),
    Actions(Option<Vec<ActionOp<T>>>, PhantomData<T>),
}

// impl<I, O> TryProcessFrom<RouteActionValue<I>> for RouteActionValue<O> where ExpressionValue<O>: TryProcessFrom<ExpressionValue<I>>, I: ::std::fmt::Debug, O: ::std::fmt::Debug {
//     fn try_process_from(src: &RouteActionValue<I>) -> DocumentProcessingResult<Self> {
//         eprintln!("TryProcess RouteActionValue -> RouteActionValue: src: {:?}", src);

//         Ok(match *src {
//             RouteActionValue::Content(ref a, _) => RouteActionValue::Content(TryProcessFrom::try_process_from(a)?, Default::default()),
//             RouteActionValue::Actions(ref a, _) => RouteActionValue::Content(TryProcessFrom::try_process_from(a)?, Default::default()),
//         })
//     }
// }
