#![allow(dead_code)]
#![allow(unused_variables)]
#![allow(unused_macros)]

use std::str::{CharIndices, FromStr};
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

    fn identifier(&mut self, start: usize) -> Result<(usize, Token, usize)> {
        let (end, content) = take!(self, start, 'a'...'z' | 'A'...'Z' | '_' | '0'...'9');

        let token = match content {
            "component" => {
                self.block_name_mode = true;
                Token::ComponentKeyword
            }

            "use" => Token::UseKeyword,
            "let" => Token::LetKeyword,
            "for" => Token::ForKeyword,
            "in" => Token::InKeyword,
            "value" => Token::ValueKeyword,
            "bind" => Token::BindKeyword,
            "as" => Token::AsKeyword,
            "where" => Token::WhereKeyword,

            "set" => Token::SetKeyword,
            "unique" => Token::UniqueKeyword,
            "and" => Token::AndKeyword,

            "store" => Token::StoreKeyword,
            "action" => Token::ActionKeyword,
            "api" => Token::ApiKeyword,
            "resource" => Token::ResourceKeyword,
            "methods" => Token::MethodsKeyword,

            "get" => Token::GetKeyword,
            "post" => Token::PostKeyword,
            "put" => Token::PutKeyword,
            "del" => Token::DelKeyword,
            "patch" => Token::PatchKeyword,

            "event" => Token::EventKeyword,
            "dispatch" => Token::DispatchKeyword,

            "true" => Token::LiteralBool(true),
            "false" => Token::LiteralBool(false),

            identifier => Token::Identifier(identifier.into()),
        };

        return Ok((start, token, end));
    }

    fn string(&mut self, start: usize) -> Result<(usize, Token, usize)> {
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

        Err(Error::UnterminatedString(start).into())
    }

    fn numeric(&mut self, start: usize) -> Result<(usize, Token, usize)> {
        // TODO: Support negative numbers
        let (end, content) = take!(self, start, '0'...'9');
        if let Ok(num) = i32::from_str(content) {
            return Ok((start, Token::LiteralNumber(num), end));
        }

        Err(Error::InvalidNumber(start).into())
    }

    fn normal(&mut self) -> Option<Result<(usize, Token, usize)>> {
        loop {
            if let Some((start, a, b)) = self.two() {
                let token = match (a, b) {
                    ('=', '>') => Some(Token::HashRocket),
                    ('=', '=') => Some(Token::EqualTo),
                    ('!', '=') => Some(Token::NotEqualTo),
                    ('>', '=') => Some(Token::GreaterThanOrEqualTo),
                    ('<', '=') => Some(Token::LessThanOrEqualTo),
                    _ => None,
                };

                if let Some(token) = token {
                    let end = self.step_n(2);
                    return Some(Ok((start, token, end)));
                };
            };

            if let Some((start, c)) = self.one() {

                // println!("Char: {:?}", c);

                let token = match c {
                    '>' => Token::GreaterThan,
                    '<' => Token::LessThan,

                    '|' => Token::Pipe,
                    '{' => Token::OpenBrace,
                    '}' => Token::CloseBrace,
                    '[' => Token::OpenBracket,
                    ']' => Token::CloseBracket,
                    '(' => {
                        self.param_list_mode = true;
                        Token::OpenParen
                    }
                    ')' => {
                        self.param_list_mode = false;
                        Token::CloseParen
                    }
                    '"' => return Some(self.string(start)),
                    '.' => Token::Dot,
                    ',' => Token::Comma,
                    '=' => Token::Equals,
                    ':' => Token::Colon,
                    ';' => Token::Semi,

                    '!' => Token::Bang,

                    // TODO: Support uniary minus (two char match)
                    '+' => Token::Plus,
                    '-' => Token::Minus,
                    '*' => Token::Mul,
                    '/' => Token::Div,

                    // TODO: Support negative numbers
                    '0'...'9' => {
                        return Some(self.numeric(start));
                    }

                    'A'...'Z' | 'a'...'z' => {
                        return Some(self.identifier(start));
                    }

                    ' ' | '\n' | '\r' | '\t' => {
                        self.step();
                        continue;
                    }

                    _ => break,
                };

                let end = self.step_n(1);
                return Some(Ok((start, token, end)));
            } else {
                return None;
            }
        }

        Some(Err(Error::Unexpected(self.pos())))
    }
}

impl<'input> Iterator for Lexer<'input> {
    type Item = Result<(usize, Token, usize)>;

    fn next(&mut self) -> Option<Self::Item> {
        if let Some(Ok((start, token, end))) = self.normal() {
            println!("Token: {:?} ({:?}, {:?})", token, start, end);
            Some(Ok((start, token, end)))
        } else {
            None
        }
    }
}

pub fn lex<'input>(input: &'input str) -> Lexer<'input> {
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
        param_list_mode: false,
    }
}
