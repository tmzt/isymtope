#![recursion_limit = "240"]
#![feature(box_patterns, specialization)]

#[macro_use]
extern crate lazy_static;

extern crate colored;
extern crate dotenv;
extern crate pretty_env_logger;

extern crate log;

#[macro_use]
extern crate failure;

#[cfg(feature = "session_time")]
extern crate time;

extern crate data_encoding;
extern crate futures;
extern crate hyper;
extern crate hyper_staticfile;
extern crate rand;
extern crate regex;
extern crate tokio_core;

extern crate isymtope_ast_common;
extern crate isymtope_build;
extern crate isymtope_generate;

#[cfg(feature = "playground_api")]
extern crate compiler_service;

use dotenv::dotenv;

pub mod app;
pub mod context;
#[cfg(feature = "cookies")]
pub mod cookies;
pub mod sessions;
pub mod default_service;
pub mod errors;
pub mod render_service;
pub mod resource_service;
pub mod message;
pub mod service;
pub mod server;
pub mod srs_generator;
pub mod static_resource_service;

#[cfg(feature = "playground_api")]
pub mod playground_api_service;

pub use self::app::*;
pub use self::context::*;
#[cfg(feature = "cookies")]
pub use self::cookies::*;
pub use self::srs_generator::*;
pub use self::sessions::*;
pub use self::default_service::*;
pub use self::errors::*;
pub use self::render_service::*;
pub use self::resource_service::*;
pub use self::message::*;
pub use self::service::*;
pub use self::server::*;
pub use self::srs_generator::*;
pub use self::static_resource_service::*;

#[cfg(feature = "playground_api")]
pub use self::playground_api_service::*;

pub fn main() {
    dotenv().ok();
    pretty_env_logger::init();

    server::run_server("0.0.0.0:3000").ok();
}
