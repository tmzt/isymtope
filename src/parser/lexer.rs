#![allow(dead_code)]
#![allow(unused_variables)]
#![allow(unused_macros)]

use std::str::CharIndices;
use parser::token::{Token, Result, Error};

macro_rules! take_until {
    ($slf:expr, $start:expr, $first:pat $(| $rest:pat)*) => {{
        let mut __end = $start;
        let mut __content_end = $start;

        loop {
            if let Some((e, c)) = $slf.one() {

                __content_end = e;

                match c {
                    $first $(| $rest)* => {
                        let (e, _) = take!($slf, e, $first (| $rest)*);
                        __end = e;
                        break;
                    },
                    _ => {
                        __end = e;
                        $slf.step();
                    }
                }
            } else {
                __content_end = $slf.source_len;
                break;
            }
        }

        (__end, &$slf.source_str[$start .. __content_end])
    }}
}

macro_rules! take {
    ($slf: expr, $start: expr, $first:pat $(| $rest:pat)*) => {{
        let mut __end: usize = $start;

        loop {
            if let Some((__pos, __c)) = $slf.one() {
                __end = __pos;

                match __c {
                    $first $(| $rest)* => {},
                    _ => break
                }

                $slf.step();
            } else {
                __end = $slf.source_len;
                break;
            }
        }

        (__end, &$slf.source_str[$start..__end])
    }}
}

pub struct Lexer<'input> {
    source: CharIndices<'input>,
    source_len: usize,
    source_str: &'input str,
    n0: Option<(usize, char)>,
    n1: Option<(usize, char)>,
    n2: Option<(usize, char)>,
    buffer: String,
    identifier_str: Option<&'input str>,
    detect_element_name_mode: bool,
    element_block: Option<(usize, usize)>,
    element_close: Option<(usize, usize)>,
    block_name_mode: bool,
    param_list_mode: bool,
    ref_mode: bool,
    ref_buffer: String,
}

impl<'input> Lexer<'input> {
    #[inline]
    fn step(&mut self) {
        self.n0 = self.n1;
        self.n1 = self.n2;
        self.n2 = self.source.next();
    }

    #[inline]
    fn step_n(&mut self, n: usize) -> usize {
        for _ in 0..n {
            self.step();
        }

        self.n0.map(|n| n.0).unwrap_or_else(|| self.source_str.len())
    }

    #[inline]
    fn one(&mut self) -> Option<(usize, char)> {
        self.n0
    }

    #[inline]
    fn two(&mut self) -> Option<(usize, char, char)> {
        if let (Some((pos, a)), Some((_, b))) = (self.n0, self.n1) {
            Some((pos, a, b))
        } else {
            None
        }
    }

    #[inline]
    fn pos(&self) -> usize {
        self.n0.map(|n| n.0).unwrap_or(self.source_len)
    }

    fn identifier(&mut self, start: usize) -> Result<(usize, Token<'input>, usize)> {
        let (end, content) = take!(self, start, 'a'...'z' | 'A'...'Z' | '_' | '0'...'9');

/*        let token = match content {
            "component" => {
                self.block_name_mode = true;
                Some(Token::ComponentKeyword)
            },
            _ => None
        };

        if let Some(token) = token {
            return Ok((start, token, end));
        }

        let identifer = content;

        if self.block_name_mode {
            self.block_name_mode = false;
            return Ok((start, Token::BlockName(identifier), end));
        }

        if self.param_list_mode {
            return Ok((start, Token::InputVariable(identifier), end));
        }

*/
        /*
        self.identifier_str = Some(identifier);
        self.detect_element_name_mode = true;
        continue;
        */

        let token = match content {
            "component" => {
                self.block_name_mode = true;
                Token::ComponentKeyword
            }

            identifier => Token::Identifier(identifier)
        };

        return Ok((start, token, end));
    }

    fn string(&mut self, start: usize) -> Result<(usize, Token<'input>, usize)> {
        self.buffer.clear();
        self.step();

        while let Some((pos, c)) = self.one() {
            if c == '"' {
                let end = self.step_n(1);
                return Ok((start, Token::LiteralString(self.buffer.clone()), end));
            };

            self.buffer.push(c);
            self.step();
        }

        Err(Error::UnterminatedString { start: start }.into())
    }

    fn normal(&mut self) -> Option<Result<(usize, Token<'input>, usize)>> {
        loop {
            if let Some((start, a, b)) = self.two() {
                let token = match (a, b) {
                    ('=', '>') => Some(Token::HashRocket),
                    _ => None
                };

                if let Some(token) = token {
                    let end = self.step_n(2);
                    return Some(Ok((start, token, end)));
                };
            };

            if let Some((start, c)) = self.one() {
                println!("Char: {:?}", c);

                let token = match c {
                    '{' => {
                        /*
                        if self.detect_element_name_mode {
                            if let Some(identifier) = self.identifier_str {
                                let end = self.step_n(1);
                                return Some(Ok((start, Token::ElementName(identifier), end)));
                            }
                        }
                        */
                        
                        Token::OpenBrace
                    },
                    '}' => Token::CloseBrace,
                    '(' => {
                        self.param_list_mode = true;
                        Token::OpenParen
                    },
                    ')' => {
                        self.param_list_mode = false;
                        Token::CloseParen
                    },
                    '"' => return Some(self.string(start)),
                    '.' => Token::Dot,
                    ',' => Token::Comma,
                    '=' => Token::Equals,

                    'A'...'Z' | 'a'...'z' => {
                        return Some(self.identifier(start));
                    },

                    ' ' | '\n' | '\r' | '\t' => {
                        self.step();
                        continue;
                    },

                    _ => break
                };

                let end = self.step_n(1);
                return Some(Ok((start, token, end)));
            } else {
                return None;
            }
        };

        Some(Err(Error::Unexpected { pos: self.pos() }))
    }
}

impl<'input> Iterator for Lexer<'input> {
    type Item = Result<(usize, Token<'input>, usize)>;

    fn next(&mut self) -> Option<Self::Item> {
        self.normal()
    }
}

pub fn lex(input: &str) -> Lexer {
    let mut source = input.char_indices();

    let n0 = source.next();
    let n1 = source.next();
    let n2 = source.next();

    Lexer {
        source: source,
        source_len: input.len(),
        source_str: input,
        n0: n0,
        n1: n1,
        n2: n2,
        buffer: String::new(),
        identifier_str: None,
        detect_element_name_mode: false,
        element_block: None,
        element_close: None,
        ref_mode: false,
        ref_buffer: String::new(),
        block_name_mode: false,
        param_list_mode: false
    }
}
