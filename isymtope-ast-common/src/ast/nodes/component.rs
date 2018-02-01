use expressions::*;
use ast::*;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct ComponentDefinition<T>(String, FormalParams<T>, Option<Vec<ContentNode<T>>>);

impl<T: Clone> ComponentDefinition<T> {
    pub fn new(
        name: String,
        params: FormalParams<T>,
        children: Option<Vec<ContentNode<T>>>,
    ) -> Self {
        ComponentDefinition(name, params, children)
    }

    pub fn name(&self) -> &str {
        self.0.as_str()
    }

    pub fn params<'a>(&'a self) -> Option<impl Iterator<Item = &'a str>> {
        self.1.params()
    }

    pub fn children<'a>(&'a self) -> Option<impl Iterator<Item = &'a ContentNode<T>>> {
        self.2.as_ref().map(|v| v.iter())
    }
}

// impl<I, O> TryProcessFrom<ComponentDefinition<I>> for ComponentDefinition<O> where ExpressionValue<O>: TryProcessFrom<ExpressionValue<I>>, I: ::std::fmt::Debug, O: ::std::fmt::Debug {
//     fn try_process_from(src: &ComponentDefinition<I>) -> DocumentProcessingResult<Self> {
//         eprintln!("TryProcess ComponentDefinition -> ComponentDefinition: src: {:?}", src);

//         let name = src.0.to_owned();
//         let params: FormalParams<O> = TryProcessFrom::try_process_from(&src.1)?;
//         let children: Option<Vec<ContentNode<O>>> = TryProcessFrom::try_process_from(&src.2)?;

//         Ok(ComponentDefinition(name, params, children))
//     }
// }
