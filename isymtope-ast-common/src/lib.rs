#![feature(box_patterns, specialization, conservative_impl_trait)]

#[macro_use]
extern crate log;

#[cfg(all(target_arch = "wasm32", target_os = "unknown"))]
#[macro_use]
extern crate wasm_log;

#[macro_use]
extern crate lazy_static;

#[cfg(not(all(target_arch = "wasm32", target_os = "unknown")))]
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
pub mod expressions;
pub mod objects;
pub mod traits;
pub mod util;

pub use self::ast::*;
pub use self::error::*;
pub use self::expressions::*;
pub use self::objects::*;
pub use self::traits::*;
pub use self::util::*;
