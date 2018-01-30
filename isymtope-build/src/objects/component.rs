use expressions::*;
use objects::*;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Component<T> {
    name: String,
    params: FormalParams<T>,
    block: Block<T>,
}

impl<T> Component<T> {
    /// Consumes parameters and returns new Component
    pub fn new(name: String, params: FormalParams<T>, block: Block<T>) -> Self {
        Component {
            name: name,
            params: params,
            block: block,
        }
    }

    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn params<'a>(&'a self) -> Option<impl Iterator<Item = &'a str>> {
        self.params.params()
    }

    #[allow(dead_code)]
    pub fn block<'a>(&'a self) -> &'a Block<T> {
        &self.block
    }

    // pub fn params_iter<'a>(&'a self) -> Option<impl IntoIterator<Item = FormalPropRef<'a>>> {
    //     self.params.as_ref().map(|v| v.into_iter().map(|s| (s.as_str())))
    // }
}
