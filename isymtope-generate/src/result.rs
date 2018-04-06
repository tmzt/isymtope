use std::io::Error as IOError;
use std::str::Utf8Error;

use isymtope_ast_common::*;

#[allow(dead_code)]
#[derive(Debug, Fail)]
pub enum IsymtopeGenerateError {
    #[fail(display = "Internal error: IOError")]
    IOError(#[cause] IOError),
    #[fail(display = "Internal error: Utf8 error")]
    Utf8Error(#[cause] Utf8Error),
    #[fail(display = "Internal error: document processing error")]
    DocumentProcessingError(#[cause] DocumentProcessingError),

    #[fail(display = "Internal error: error parsing template")]
    TemplateParseError(#[cause] TemplateParseError),

    // #[fail(display = "Error rendering internal template")]
    // InternalRenderError(String),

    #[fail(display = "Session error")]
    SessionError(SessionError),
}

impl From<IOError> for IsymtopeGenerateError {
    fn from(err: IOError) -> Self {
        IsymtopeGenerateError::IOError(err)
    }
}

impl From<TemplateParseError> for IsymtopeGenerateError {
    fn from(err: TemplateParseError) -> Self {
        IsymtopeGenerateError::TemplateParseError(err)
    }
}

impl From<DocumentProcessingError> for IsymtopeGenerateError {
    fn from(err: DocumentProcessingError) -> Self {
        IsymtopeGenerateError::DocumentProcessingError(err)
    }
}

impl From<SessionError> for IsymtopeGenerateError {
    fn from(err: SessionError) -> Self {
        IsymtopeGenerateError::SessionError(err)
    }
}

pub type IsymtopeGenerateResult<T> = Result<T, IsymtopeGenerateError>;
