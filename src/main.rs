#![recursion_limit="2000"]
#![feature(box_patterns)]
#![feature(conservative_impl_trait)]
#![feature(specialization)]

extern crate futures;
extern crate hyper;
extern crate wren;
extern crate uuid;
extern crate itertools;
extern crate linked_hash_map;
extern crate broadcast;

#[cfg(test)]
#[macro_use(assert_diff)]
extern crate difference;

mod model;
mod parser;
mod scope;
mod processing;
mod output;

use std::io;
use std::path::Path;
use futures::future::Future;
use hyper::server::{Http, Request, Response, Service};

use model::*;
use scope::*;
use processing::*;
use output::*;


fn prepare_document(template: &Template) -> Document {
    let mut ctx = Context::default();
    let mut bindings = BindingContext::default();
    let mut processing = ProcessDocument::from_template(&template);
    processing.process_document(&mut ctx, &mut bindings).unwrap();
    processing.into()
}

pub fn write_html(w: &mut io::Write, template: &Template) -> Result {
    let mut ctx = Context::default();
    let bindings = BindingContext::default();
    let doc = prepare_document(template);
    let mut page_writer = PageWriter::with_doc(&doc);
    page_writer.write_page(w, &mut ctx, &bindings)
}

struct AppServer;

impl Service for AppServer {
    type Request = Request;
    type Response = Response;
    type Error = hyper::Error;
    type Future = Box<Future<Item = Self::Response, Error = Self::Error>>;

    fn call(&self, _req: Request) -> Self::Future {
        let source_path = "./res/tests/app/todomvc/app.ism";
        let template = ::parser::parse_file(Path::new(&source_path)).unwrap();

        let mut s: Vec<u8> = Default::default();
        write_html(&mut s, &template).unwrap();

        Box::new(futures::future::ok(
            Response::new()
                .with_body(s)
        ))
    }
}

pub fn main() {
    let addr = "0.0.0.0:3000".parse().unwrap();
    let server = Http::new().bind(&addr, || Ok(AppServer)).unwrap();
    server.run().unwrap();
}