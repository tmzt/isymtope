

#[macro_use]
use failure::{Error, Backtrace};

use std::fmt::{Formatter, Display, Debug, Error as FormatError, Result as FormatResult};
use std::str::Utf8Error;
use std::io::Error as IOError;

#[derive(Debug, Fail)]
pub enum SessionError {
    #[fail(display = "Error getting session value")]
    ValueGetError,
}

pub type SessionResult<T> = ::std::result::Result<T, SessionError>;
