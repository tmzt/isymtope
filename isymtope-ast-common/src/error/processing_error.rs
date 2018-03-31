use std::error::Error;
use std::fmt::{self, Display};
use std::str::Utf8Error;
use std::io::Error as IOError;

use failure::Backtrace;

// use input::parser::token::Error as ParsingError;

use super::*;

#[derive(Debug)]
pub struct ParsingError(String);

impl ParsingError {
    pub fn new(msg: String) -> Self {
        ParsingError(msg)
    }
}

impl Error for ParsingError {
    fn description(&self) -> &str {
        &self.0
    }
}

impl Display for ParsingError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}

#[derive(Debug, Fail)]
pub enum DocumentProcessingError {
    #[fail(display = "Error parsing template")]
    ParsingError(#[cause] ParsingError),

    #[fail(display = "IO Error occured")]
    IOError(IOError),
    #[fail(display = "Error formatting template")]
    FormatError(fmt::Error),

    #[cfg(feature = "types")]
    #[fail(display = "Type error")]
    TypeError(DocumentTypeError),

    #[fail(display = "Cannot reduce value")]
    ReductionErrorWithBacktrace(Backtrace),

    #[fail(display = "Error processing value")]
    TryProcessFromError(String, Backtrace),
    #[fail(display = "Evaluating error")]
    TryEvalFromError(String, Backtrace),

    #[fail(display = "Error rendering internal template")]
    InternalRenderError(String),

    #[fail(display = "Error parsing internal template")]
    InternalParseError(String),

    #[fail(display = "Utf8Error")]
    Utf8Error(Utf8Error),

    #[fail(display = "Session error")]
    SessionError(SessionError),
}

impl From<Utf8Error> for DocumentProcessingError {
    fn from(err: Utf8Error) -> Self {
        DocumentProcessingError::Utf8Error(err)
    }
}
impl From<IOError> for DocumentProcessingError {
    fn from(err: IOError) -> Self {
        DocumentProcessingError::IOError(err)
    }
}
impl From<SessionError> for DocumentProcessingError {
    fn from(err: SessionError) -> Self {
        DocumentProcessingError::SessionError(err)
    }
}

pub type DocumentProcessingResult<T> = ::std::result::Result<T, DocumentProcessingError>;
