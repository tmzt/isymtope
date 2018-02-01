#![recursion_limit="240"]
#![feature(box_patterns)]
#![feature(conservative_impl_trait)]
#![feature(specialization)]


// #[cfg(not(all(target_arch = "wasm32", target_os = "unknown")))]
// #[macro_use]
// extern crate log;

// #[cfg(all(target_arch = "wasm32", target_os = "unknown"))]
// #[macro_use]
// pub mod log;

// #[cfg(not(all(target_arch = "wasm32", target_os = "unknown")))]
// extern crate uuid;

#[cfg(feature = "session_time")]
extern crate time;

extern crate itertools;
extern crate linked_hash_map;
extern crate trimmer;
extern crate serde_json;
extern crate regex;
extern crate rand;

#[macro_use]
extern crate lazy_static;

#[cfg(test)]
#[macro_use(assert_diff)]
extern crate difference;

#[macro_use]
extern crate failure;

#[macro_use]
extern crate isymtope_ast_common;

use isymtope_ast_common::*;

#[cfg(not(all(target_arch = "wasm32", target_os = "unknown")))]
#[macro_use]
extern crate log;

#[cfg(all(target_arch = "wasm32", target_os = "unknown"))]
#[macro_use]
pub use isymtope_ast_common::log::*;

pub mod common;
pub mod errors;
pub mod input;
pub mod output;

pub use error::*;
pub use traits::*;
pub use expressions::*;
pub use objects::*;

pub use self::common::*;
pub use self::errors::*;
pub use self::input::*;
pub use self::input::processing::*;
pub use self::output::*;
