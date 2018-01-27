#![recursion_limit="240"]
#![feature(box_patterns)]
#![feature(conservative_impl_trait)]
#![feature(specialization)]

extern crate pretty_env_logger;
extern crate colored;
extern crate dotenv;

#[macro_use]
extern crate log;

extern crate uuid;
extern crate itertools;
extern crate linked_hash_map;
extern crate rand;
extern crate trimmer;
extern crate serde_json;
extern crate regex;
extern crate time;


#[macro_use]
extern crate lazy_static;

#[cfg(test)]
#[macro_use(assert_diff)]
extern crate difference;

extern crate backtrace;

#[macro_use]
extern crate failure;

#[macro_use]
pub mod error;
pub mod common;
pub mod util;
pub mod traits;

pub mod expressions;
pub mod ast;
pub mod objects;

pub mod input;
pub mod output;

pub use input::processing;