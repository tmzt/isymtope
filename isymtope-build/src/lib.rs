#![feature(box_patterns)]
#![recursion_limit = "240"]

#[macro_use]
extern crate log;

#[cfg(feature = "wasm")]
#[macro_use]
extern crate wasm_log;

#[cfg(feature = "session_time")]
extern crate time;

extern crate itertools;
extern crate linked_hash_map;
extern crate regex;
extern crate serde_json;
extern crate lalrpop_util;

#[cfg(test)]
#[macro_use(assert_diff)]
extern crate difference;

extern crate failure;

extern crate isymtope_data;
#[macro_use]
extern crate isymtope_ast_common;
extern crate isymtope_parser;

use isymtope_ast_common::*;

pub mod output;

pub use error::*;
pub use traits::*;
pub use expressions::*;
pub use objects::*;

pub use self::output::*;
