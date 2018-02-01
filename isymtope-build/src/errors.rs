use std::error::Error;

use trimmer;
use error::*;

#[derive(Debug)]
pub struct InternalParseError(String);
#[derive(Debug)]
pub struct InternalRenderError(String);

impl From<trimmer::ParseError> for InternalParseError {
    fn from(err: ::trimmer::ParseError) -> Self {
        InternalParseError(format!("{:?}", err))
    }
}
impl From<trimmer::RenderError> for InternalRenderError {
    fn from(err: ::trimmer::RenderError) -> Self {
        InternalRenderError(format!("{:?}", err))
    }
}

impl Into<DocumentProcessingError> for InternalParseError {
    fn into(self) -> DocumentProcessingError {
        DocumentProcessingError::InternalRenderError(self.0)
    }
}
impl Into<DocumentProcessingError> for InternalRenderError {
    fn into(self) -> DocumentProcessingError {
        DocumentProcessingError::InternalParseError(self.0)
    }
}
