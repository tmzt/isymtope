#![allow(dead_code)]

#[derive(Debug)]
pub enum Error {
    Unexpected { pos: usize },
    UnterminatedString { start: usize },
    InvalidNumber { start: usize },
}

#[derive(Clone, Debug, PartialEq)]
pub enum Token {
    UseKeyword,
    LetKeyword,

    ComponentKeyword,
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

    HashRocket,
    OpenBrace,
    CloseBrace,
    OpenParen,
    CloseParen,
    Dot,
    Comma,
    Equals,
    Semi,
    Plus,
    Minus,
    Mul,
    Div,
    Identifier(String),
    LiteralNumber(i32),
    LiteralString(String),
    VariableReference(String),
}

pub type Result<T> = ::std::result::Result<T, Error>;