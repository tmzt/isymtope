use expressions::*;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct ExternComponentDefinition<T>(String, FormalParams<T>);

impl<T: Clone> ExternComponentDefinition<T> {
    pub fn new(
        name: String,
        params: FormalParams<T>
    ) -> Self {
        ExternComponentDefinition(name, params)
    }

    pub fn name(&self) -> &str {
        self.0.as_str()
    }

    pub fn params<'a>(&'a self) -> Option<impl Iterator<Item = &'a str>> {
        self.1.params()
    }
}
