#![feature(box_patterns)]

#[cfg(feature = "wasm")]
#[macro_use]
extern crate wasm_log;

#[macro_use]
extern crate failure;
extern crate isymtope_ast_common;
extern crate isymtope_build;
#[macro_use]
extern crate lazy_static;
#[macro_use]
extern crate regex;

// mod actions;
pub mod executor;
pub mod context;
pub mod matcher;
pub mod message;
pub mod router;
pub mod result;
pub mod session;

// pub use actions::*;
pub use executor::*;
pub use context::*;
pub use matcher::*;
pub use message::*;
pub use router::*;
pub use result::*;
pub use session::*;

// use std::env;
// use std::path::PathBuf;

// lazy_static! {
//     pub static ref APP_DIR: Box<PathBuf> = Box::new(env::var_os("APP_DIR").expect("APP_DIR must be provided").into());
//     pub static ref DEFAULT_APP: String = env::var_os("DEFAULT_APP").expect("DEFAULT_APP must be provided").to_string_lossy().to_string();
// }
