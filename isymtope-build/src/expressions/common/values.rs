#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum MethodType {
    Get,
    Post,
    Put,
    Delete,
    Patch,
}
