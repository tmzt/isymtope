#![recursion_limit = "240"]
#![feature(box_patterns)]
#![feature(conservative_impl_trait)]
#![feature(specialization)]

extern crate colored;
extern crate dotenv;
extern crate pretty_env_logger;

#[macro_use]
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

#[macro_use]
extern crate isymtope_ast_common;
extern crate isymtope_build;

use dotenv::dotenv;

mod server;
use self::server::*;

pub fn main() {
    dotenv().ok();

    // let document_provider: Rc<DocumentProvider> = Default::default();
    // let _ = document_provider.doc();

    server::run_server("0.0.0.0:3000").ok();
}
