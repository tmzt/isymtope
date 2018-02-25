use failure;

use std::fmt::Error as FormatError;
use std::io::Error as IOError;
use std::str::Utf8Error;
use std::net::AddrParseError;

use isymtope_ast_common::*;

#[allow(dead_code)]
#[derive(Debug, Fail)]
pub enum IsymtopeGenerateError {
    #[fail(display = "Internal error: IOError")]
    IOError(IOError),
    #[fail(display = "Internal error: Utf8 error")]
    Utf8Error(Utf8Error),
    #[fail(display = "Internal error: document processing error")]
    DocumentProcessingError(DocumentProcessingError),

    #[fail(display = "Error rendering internal template")]
    InternalRenderError(String),

    #[fail(display = "Session error")]
    SessionError(SessionError),
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
