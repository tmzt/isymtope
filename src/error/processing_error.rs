
use failure::{Error, Backtrace};

use std::fmt::{Formatter, Display, Debug, Error as FormatError, Result as FormatResult};
use std::str::Utf8Error;
use std::io::Error as IOError;

use input::parser::token::Error as ParsingError;

use super::*;


#[derive(Debug, Fail)]
pub enum DocumentProcessingError {
    #[fail(display = "Error parsing template")]
    ParsingError(ParsingError),
    #[fail(display = "IO Error occured")]
    IOError(IOError),
    #[fail(display = "Error formatting template")]
    FormatError(FormatError),

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

impl From<Utf8Error> for DocumentProcessingError { fn from(err: Utf8Error) -> Self { DocumentProcessingError::Utf8Error(err) } }
impl From<IOError> for DocumentProcessingError { fn from(err: IOError) -> Self { DocumentProcessingError::IOError(err) } }
impl From<SessionError> for DocumentProcessingError { fn from(err: SessionError) -> Self {  DocumentProcessingError::SessionError(err) } }

impl From<::trimmer::RenderError> for DocumentProcessingError { fn from(err: ::trimmer::RenderError) -> Self { DocumentProcessingError::InternalRenderError(format!("{:?}", err)) } }
impl From<::trimmer::ParseError> for DocumentProcessingError { fn from(err: ::trimmer::ParseError) -> Self { DocumentProcessingError::InternalParseError(format!("{:?}", err)) } }

pub type DocumentProcessingResult<T> = ::std::result::Result<T, DocumentProcessingError>;