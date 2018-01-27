
use std::marker::PhantomData;
use std::borrow::Cow;

use regex::*;

use error::*;
use traits::*;
use expressions::*;
use objects::*;
use ast::*;


fn function_key(name: &str) -> Cow<str> {
    lazy_static! {
        static ref REGEX: Regex = Regex::new("[^a-zA-Z0-9]").unwrap();
    }
    REGEX.replace_all(name, "")
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Route<T> (String, FormalParams<T>, RouteActionValue<T>, PhantomData<T>);

impl<T> Route<T> {
    pub fn new(pattern: String, params: FormalParams<T>, action: RouteActionValue<T>) -> Self {
        Route(pattern, params, action, Default::default())
    }

    pub fn pattern(&self) -> &str { &self.0 }

    pub fn function_key(&self) -> Cow<str> {
        function_key(&self.0)
    }

    pub fn action(&self) -> &RouteActionValue<T> {
        &self.2
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
    Actions(Option<Vec<ActionOp<T>>>, PhantomData<T>)
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
