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

#[cfg(feature="server")]
extern crate futures;
#[cfg(feature="server")]
extern crate tokio_core;
#[cfg(feature="server")]
extern crate hyper;
#[cfg(feature="server")]
extern crate data_encoding;

#[cfg(test)]
#[macro_use(assert_diff)]
extern crate difference;

extern crate backtrace;

#[macro_use]
extern crate failure;

#[macro_use]
mod error;
mod common;
mod util;
mod traits;

mod expressions;
mod ast;
mod objects;

mod input;
mod output;

#[cfg(feature="server")]
mod server;

use dotenv::dotenv;
use self::input::*;


pub fn main() {
    dotenv().ok();

    // let document_provider: Rc<DocumentProvider> = Default::default();
    // let _ = document_provider.doc();

    #[cfg(feature="server")]
    server::run_server("0.0.0.0:3000").ok();
}