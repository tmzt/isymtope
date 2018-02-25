use std::fmt;
use std::error::Error;

use error::*;

#[derive(Debug)]
pub enum TemplateParseError {
    Unexpected(usize),
    UnterminatedString(usize),
    InvalidNumber(usize),
}

impl Error for TemplateParseError {
    fn description(&self) -> &str {
        match self {
            &TemplateParseError::Unexpected(_) => "Unexpected token",
            &TemplateParseError::UnterminatedString(_) => "Unterminated string",
            &TemplateParseError::InvalidNumber(_) => "Invalid number",
        }
    }
}

impl fmt::Display for TemplateParseError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            &TemplateParseError::Unexpected(ref s) => write!(f, "Unexpected token at pos {0}", s),
            &TemplateParseError::UnterminatedString(ref s) => {
                write!(f, "Unterminated string at start {0}", s)
            }
            &TemplateParseError::InvalidNumber(ref s) => {
                write!(f, "Invalid number at start {0}", s)
            }
        }
    }
}

impl Into<ParsingError> for TemplateParseError {
    fn into(self) -> ParsingError {
        let description = self.description();
        ParsingError::new(description.to_owned())
    }
}

#[allow(dead_code)]
#[derive(Clone, Debug, PartialEq)]
pub enum Token {
    UseKeyword,
    LetKeyword,
    ForKeyword,
    InKeyword,
    BindKeyword,
    AsKeyword,
    WhereKeyword,
    ToKeyword,

    QueryKeyword,

    SetKeyword,
    DeleteKeyword,
    UniqueKeyword,
    AndKeyword,

    ComponentKeyword,
    RouteKeyword,
    StoreKeyword,
    ActionKeyword,
    ExternKeyword,
    ApiKeyword,
    ResourceKeyword,
    MethodsKeyword,

    GetKeyword,
    PostKeyword,
    PutKeyword,
    DelKeyword,
    PatchKeyword,

    EventKeyword,
    DispatchKeyword,
    NavigateKeyword,

    StateKeyword,
    ValueKeyword,
    ItemKeyword,

    HashRocket,
    EqualTo,
    NotEqualTo,
    LessThan,
    GreaterThan,
    LessThanOrEqualTo,
    GreaterThanOrEqualTo,

    Pipe,
    OpenBrace,
    CloseBrace,
    OpenBracket,
    CloseBracket,
    OpenParen,
    CloseParen,
    Dot,
    Comma,
    Equals,
    Colon,
    Semi,
    Bang,
    Plus,
    Minus,
    Mul,
    Div,
    Identifier(String),
    LiteralNumber(i32),
    LiteralString(String),
    LiteralBool(bool),
    VariableReference(String),
}

pub type Result<T> = ::std::result::Result<T, TemplateParseError>;
