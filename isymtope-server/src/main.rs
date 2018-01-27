#![recursion_limit="240"]
#![feature(box_patterns)]
#![feature(conservative_impl_trait)]
#![feature(specialization)]

extern crate pretty_env_logger;
extern crate colored;
extern crate dotenv;

#[macro_use]
extern crate log;

#[macro_use]
extern crate failure;

extern crate time;
extern crate rand;
extern crate hyper;
extern crate regex;
extern crate futures;
extern crate data_encoding;
extern crate tokio_core;

#[macro_use]
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