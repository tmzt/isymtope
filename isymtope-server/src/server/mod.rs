use std::env;
use std::fmt::Debug;
use std::io::{self, Write};
use std::path::{Path, PathBuf};
use std::rc::Rc;
use std::sync::{Arc, Mutex};

use std::collections::HashMap;

use futures;
use futures::Stream;
use futures::future::{self, FutureResult};

use tokio_core::reactor::Core;
use tokio_core::net::TcpListener;

use hyper;
use hyper::server::{Http, NewService, Request, Response, Server, Service};
use regex::RegexSet;

use isymtope_ast_common::*;
use isymtope_build::*;

pub mod errors;
pub mod srs_generator;

pub mod context;
pub mod executor;
pub mod cookies;
pub mod sessions;
pub mod session;
pub mod default_service;
pub mod render_service;
pub mod resource_service;
pub mod service;
pub mod router;

pub use self::context::*;
pub use self::errors::*;
pub use self::srs_generator::*;
pub use self::executor::*;
pub use self::cookies::*;
pub use self::sessions::*;
pub use self::session::*;
pub use self::default_service::*;
pub use self::render_service::*;
pub use self::resource_service::*;
pub use self::service::*;
pub use self::router::*;

lazy_static! {
    pub static ref APP_DIR: Box<PathBuf> = Box::new(env::var_os("APP_DIR").expect("APP_DIR must be provided").into());
    pub static ref DEFAULT_APP: String = env::var_os("DEFAULT_APP").expect("DEFAULT_APP must be provided").to_string_lossy().to_string();
}

pub trait ServiceInject: Debug {
    type ServiceImpl: Service;
}

pub fn run_server(addr: &str) -> IsymtopeServerResult<()> {
    let addr = addr.parse()?;
    // let mut core = Core::new().unwrap();
    // let handle = core.handle();
    // let ctx_handle = handle.clone();

    // https://users.rust-lang.org/t/questions-about-tokio-futures-and-rwlock/11260/11
    let (sender, receiver) = futures::sync::mpsc::unbounded();
    let t = ::std::thread::spawn(move || {
        use futures::Stream;
        let mut core = Core::new().unwrap();
        // let ctx_handle = core.handle();

        // let document_provider: Rc<DocumentProvider> = Default::default();
        // let mut shared_ctx = DefaultServerContext::new(document_provider);
        // ctx_handle.spawn(receiver.for_each(move |(msg, oneshot): (_, ResponseMsgChannel)| {

        let mut shared_ctx = DefaultServerContext::default();
        core.run(
            receiver.for_each(move |(msg, oneshot): (_, ResponseMsgChannel)| {
                let response = shared_ctx.handle_msg(msg);
                oneshot.send(response).unwrap();

                future::ok(())
            }),
        ).ok()
            .unwrap();
    });

    // let default_app = env::var_os("DEFAULT_APP").expect("DEFAULT_APP must be provided");
    // let default_app_str: String = default_app.to_string_lossy().to_string();

    let default_app_str = &*DEFAULT_APP.to_owned();

    let mut core = Core::new().unwrap();
    let handle = core.handle();

    let render_service_factory =
        TemplateRenderServiceFactory::new(sender, handle.clone(), default_app_str.to_owned());
    let resource_service_factory =
        TemplateResourceServiceFactory::new(handle.clone());

    let factory = DefaultServiceFactory::new(
        render_service_factory,
        resource_service_factory,
        handle.clone(),
    );

    let listener = TcpListener::bind(&addr, &handle).unwrap();
    let server = listener.incoming().for_each(|(sock, addr)| {
        let s = factory.new_service().unwrap();
        Http::new().bind_connection(&handle, sock, addr, s);

        Ok(())
    });
    core.run(server).unwrap();

    // let server = Http::new().bind(&addr, factory)?;
    // server.run().unwrap();

    Ok(())
}
