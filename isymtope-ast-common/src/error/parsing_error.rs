use std::error::Error;
use std::fmt;

#[derive(Debug)]
pub enum TemplateParseError {
    UnrecognizedToken(usize),
    UnexpectedToken(usize),
    UnterminatedString(usize),
    InvalidNumber(usize),
    Other
}

impl Error for TemplateParseError {
    fn description(&self) -> &str {
        match *self {
            TemplateParseError::UnrecognizedToken(_) => "Unrecognized token",
            TemplateParseError::UnexpectedToken(_) => "Unexpected token",
            TemplateParseError::UnterminatedString(_) => "Unterminated string",
            TemplateParseError::InvalidNumber(_) => "Invalid number",
            TemplateParseError::Other => "Other parsing error"
        }
    }
}

impl fmt::Display for TemplateParseError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            TemplateParseError::UnrecognizedToken(n) => write!(f, "Unrecognized token at pos {0}", n),
            TemplateParseError::UnexpectedToken(n) => write!(f, "Unexpected token at pos {0}", n),
            TemplateParseError::UnterminatedString(n) => write!(f, "Unterminated string starting at pos {0}", n),
            TemplateParseError::InvalidNumber(n) => write!(f, "Invalid number starting at pos {0}", n),
            TemplateParseError::Other => write!(f, "Other parsing error")
        }
    }
}

pub type TemplateParseResult<T> = ::std::result::Result<T, TemplateParseError>;
