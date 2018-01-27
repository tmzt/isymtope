
use failure;

use std::fmt::Error as FormatError;
use std::io::Error as IOError;
use std::str::Utf8Error;
use std::net::AddrParseError;

use hyper::Error as HyperError;
use futures::Canceled as FutureCanceled;
use futures::sync::oneshot::Canceled as OneshotCanceled;
use ::error::processing_error::DocumentProcessingError;

use error::*;


#[allow(dead_code)]
#[derive(Debug, Fail)]
pub enum IsymtopeServerError {
    #[fail(display = "Error parsing address")]
    AddrParseError(AddrParseError),
    #[fail(display = "Session with the same randomly generated key exists, this should not happen.")]
    SessionKeyConflict,
    #[fail(display = "Session cannot be created")]
    SessionAllocationError,
    #[fail(display = "Sesison not found")]
    CannotFindSession,

    #[fail(display = "Internal error: IOError")]
    IOError(IOError),
    #[fail(display = "Internal error: Utf8 error")]
    Utf8Error(Utf8Error),
    #[fail(display = "Internal error: document processing error")]
    DocumentProcessingError(DocumentProcessingError),

    #[fail(display = "Internal error: HyperError")]
    HyperError(HyperError),
    #[fail(display = "Internal error: future canceled")]
    FutureCanceled(FutureCanceled),
    #[fail(display = "Internal error: oneshot future canceled")]
    OneshotCanceled(OneshotCanceled),
    #[fail(display = "Internal error: render request failed")]
    RenderRequestFailed,

    #[fail(display = "Error rendering internal template")]
    InternalRenderError(String),

    #[fail(display = "Error parsing internal template")]
    InternalParseError(String),

    #[fail(display = "Session error")]
    SessionError(SessionError),
}

impl From<AddrParseError> for IsymtopeServerError { fn from(err: AddrParseError) -> Self { IsymtopeServerError::AddrParseError(err) } }
impl From<HyperError> for IsymtopeServerError { fn from(err: HyperError) -> Self { IsymtopeServerError::HyperError(err) } }
impl From<FutureCanceled> for IsymtopeServerError { fn from(err: FutureCanceled) -> Self { IsymtopeServerError::FutureCanceled(err) } }
impl From<Utf8Error> for IsymtopeServerError { fn from(err: Utf8Error) -> Self { IsymtopeServerError::Utf8Error(err) } }
impl From<DocumentProcessingError> for IsymtopeServerError { fn from(err: DocumentProcessingError) -> Self { IsymtopeServerError::DocumentProcessingError(err) } }
impl From<SessionError> for IsymtopeServerError { fn from(err: SessionError) -> Self {  IsymtopeServerError::SessionError(err) } }

pub type IsymtopeServerResult<T> = Result<T, IsymtopeServerError>;
pub type IsymtopeServerVoidResult = Result<(), IsymtopeServerError>;