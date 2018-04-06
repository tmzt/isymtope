pub mod token;
pub mod lexer;
pub mod parser;

use std::path::Path;
use std::io::{self, Read};
use std::fs;

use isymtope_ast_common::*;
use lalrpop_util::ParseError;

#[cfg(test)]
type TokenIter = Box<Iterator<Item = TemplateParseResult<(usize, token::Token, usize)>>>;

#[cfg(test)]
pub fn lex(input: &'static str) -> TokenIter {
    Box::new(lexer::lex(input))
}

pub fn parse_str<'input>(input: &str) -> TemplateParseResult<Template> {
    let lexer = lexer::lex(&input);

    match parser::parse_Template(lexer) {
        Ok(template) => Ok(template),
        Err(parse_err) => {
            match parse_err {
                ParseError::UnrecognizedToken { ref token, .. } => Err(TemplateParseError::UnrecognizedToken(token.as_ref().map_or(0, |t| t.0))),
                ParseError::ExtraToken { ref token } => Err(TemplateParseError::UnexpectedToken(token.0)),
                _ => Err(TemplateParseError::Other)
            }
        }
    }
}
