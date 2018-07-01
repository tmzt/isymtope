#![feature(box_patterns, specialization)]

#[macro_use]
extern crate log;

#[cfg(feature = "wasm")]
#[macro_use]
extern crate wasm_log;

#[macro_use]
extern crate lazy_static;

#[cfg(feature = "uuid_v4")]
extern crate uuid;

#[macro_use]
extern crate failure;
extern crate itertools;
extern crate linked_hash_map;
extern crate rand;
extern crate regex;

#[macro_use]
pub mod error;

pub mod ast;
pub mod common;
pub mod contexts;
pub mod expressions;
pub mod objects;
pub mod traits;
pub mod util;

pub use self::ast::*;
pub use self::common::*;
pub use self::contexts::*;
pub use self::error::*;
pub use self::expressions::*;
pub use self::objects::*;
pub use self::traits::*;
pub use self::util::*;
