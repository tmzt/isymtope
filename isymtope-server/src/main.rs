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

#[cfg(feature = "github_auth")]
extern crate url;
#[cfg(feature = "github_auth")]
extern crate oauth2;

use dotenv::dotenv;

pub mod app;
pub mod context;
#[cfg(feature = "cookies")]
pub mod cookies;
pub mod sessions;
pub mod default_service;
#[cfg(feature = "github_auth")]
pub mod auth_service;
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
#[cfg(feature = "github_auth")]
pub use self::auth_service::*;
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

use std::env;

use hyper::header::Host;
use hyper::server::{Service, Request};

lazy_static! {
    pub static ref PLAYGROUND_HOST: String = env::var("PLAYGROUND_HOST").expect("PLAYGROUND_HOST must provided");
    pub static ref PLAYGROUND_PORT: u16 = env::var("PLAYGROUND_PORT").map(|s| s.parse::<u16>().unwrap()).expect("PLAYGROUND_PORT must provided");
}

pub trait IsymtopeServiceExt<S: Service> {
    fn get_proto(&self, req: &Request) -> IsymtopeServerResult<String>;
    fn get_host_port(&self, req: &Request) -> IsymtopeServerResult<(String, u16)>;
    fn get_base_url(&self, req: &Request) -> IsymtopeServerResult<String>;
}

impl<S: Service> IsymtopeServiceExt<S> for S {
    fn get_proto(&self, req: &Request) -> IsymtopeServerResult<String> {
            let forwarded_proto = req.headers()
                .get_raw("x-forwarded-proto")
                .and_then(|s| s.one())
                .and_then(|s| String::from_utf8(s.to_vec()).ok());
            eprintln!("Forwarded proto: {:?}", forwarded_proto);

            let mut proto = "http";
            if let Some(forwarded_proto) = forwarded_proto {
                if forwarded_proto.to_lowercase() == "https" {
                    proto = "https";
                };
            };

            Ok(proto.to_owned())
    }

    fn get_host_port(&self, req: &Request) -> IsymtopeServerResult<(String, u16)> {
        let default_port = PLAYGROUND_PORT.to_owned();
        let default_host = &*PLAYGROUND_HOST;

        let hostname = req.headers().get::<Host>().map(|h| h.hostname())
            .or_else(|| req.uri().host()).unwrap_or_else(|| default_host);
        let port = req.headers().get::<Host>().and_then(|h| h.port())
            .or_else(|| req.uri().port()).unwrap_or_else(|| default_port);

        Ok((hostname.to_owned(), port.clone()))
    }

    fn get_base_url(&self, req: &Request) -> IsymtopeServerResult<String> {
        let proto = self.get_proto(req)?;
        let (hostname, port) = self.get_host_port(req)?;
        let s_port = match (proto.as_str(), port) {
            ("https", 443) => "".to_owned(),
            ("http", 80) => "".to_owned(),
            _ => format!(":{}", port)
        };

        Ok(format!("{}://{}{}/", proto, hostname, s_port))
    }
}

pub fn main() {
    dotenv().ok();
    pretty_env_logger::init();

    server::run_server("0.0.0.0:3000").ok();
}
