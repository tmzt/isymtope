#![allow(dead_code)]

#[derive(Debug)]
pub enum Error {
    Unexpected { pos: usize },
    UnterminatedString { start: usize }
}

#[derive(Clone, Debug, PartialEq)]
pub enum Token<'input> {
    ComponentKeyword,
    HashRocket,
    OpenBrace,
    CloseBrace,
    OpenParen,
    CloseParen,
    Dot,
    Comma,
    Equals,
    Identifier(&'input str),
    ComponentIdentifier(&'input str),
    LiteralString(String),
    VariableReference(&'input str),
    // InputVariable(&'input str),
    // BlockName(&'input str),
    // ElementName(&'input str)
}

pub type Result<T> = ::std::result::Result<T, Error>;