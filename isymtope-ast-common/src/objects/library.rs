
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct LibraryObject {
    name: String,
    args: Option<Vec<String>>
}

impl LibraryObject {
    pub fn new(name: String, args: Option<Vec<String>>) -> Self {
        LibraryObject {
            name: name,
            args: args
        }
    }
}
