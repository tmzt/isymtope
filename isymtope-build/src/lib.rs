#![recursion_limit="240"]
#![feature(box_patterns)]
#![feature(conservative_impl_trait)]
#![feature(specialization)]


#[cfg(not(all(target_arch = "wasm32", target_os = "unknown")))]
#[macro_use]
extern crate log;

#[cfg(all(target_arch = "wasm32", target_os = "unknown"))]
#[macro_use]
pub mod log;

extern crate uuid;

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
pub mod error;
pub mod common;
pub mod traits;

pub mod expressions;
pub mod ast;
pub mod objects;

pub mod input;
pub mod output;

pub use input::processing;

#[cfg(all(target_arch = "wasm32", target_os = "unknown"))]
pub mod util_wasm;

#[cfg(not(all(target_arch = "wasm32", target_os = "unknown")))]
pub mod util_uuid;

#[cfg(all(target_arch = "wasm32", target_os = "unknown"))]
pub use self::util_wasm as util;

#[cfg(not(all(target_arch = "wasm32", target_os = "unknown")))]
pub use self::util_uuid as util;
