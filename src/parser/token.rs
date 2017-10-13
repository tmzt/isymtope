
use std::fmt;
use std::error;


#[derive(Debug)]
pub enum Error {
    Unexpected(usize),
    UnterminatedString(usize),
    InvalidNumber(usize)
}

impl error::Error for Error {
    fn description(&self) -> &str {
        match self {
            &Error::Unexpected(_) => "Unexpected token",
            &Error::UnterminatedString(_) => "Unterminated string",
            &Error::InvalidNumber(_) => "Invalid number"
        }
    }
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            &Error::Unexpected(ref s) => write!(f, "Unexpected token at pos {0}", s),
            &Error::UnterminatedString(ref s) => write!(f, "Unterminated string at start {0}", s),
            &Error::InvalidNumber(ref s) => write!(f, "Invalid number at start {0}", s)
        }
    }
}


#[allow(dead_code)]
#[derive(Clone, Debug, PartialEq)]
pub enum Token {
    UseKeyword,
    LetKeyword,
    ValueKeyword,
    ForKeyword,
    InKeyword,
    BindKeyword,
    AsKeyword,
    WhereKeyword,
    ToKeyword,

    QueryKeyword,

    SetKeyword,
    UniqueKeyword,
    AndKeyword,

    ComponentKeyword,
    RouteKeyword,
    StoreKeyword,
    ActionKeyword,
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

pub type Result<T> = ::std::result::Result<T, Error>;
