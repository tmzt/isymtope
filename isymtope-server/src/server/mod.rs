
use std::fmt::Debug;
use std::io::{self, Write};
use std::path::Path;
use std::rc::Rc;
use std::sync::Mutex;

use futures;
use futures::Stream;
use futures::future::{self, FutureResult};

use tokio_core::reactor::Core;
use tokio_core::net::TcpListener;

use hyper;
use hyper::server::{Server, Http, Request, Response, NewService, Service};
use regex::RegexSet;

use isymtope_build::error::*;
use isymtope_build::ast::*;
use isymtope_build::input::*;
use isymtope_build::processing::*;

pub mod errors;
pub mod srs_generator;

pub mod context;
pub mod executor;
pub mod cookies;
pub mod sessions;
pub mod session;
pub mod service;
pub mod router;

pub use self::context::*;
pub use self::errors::*;
pub use self::srs_generator::*;
pub use self::executor::*;
pub use self::cookies::*;
pub use self::sessions::*;
pub use self::session::*;
pub use self::service::*;
pub use self::router::*;

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
        
        let document_provider: Rc<DocumentProvider> = Default::default();
        let mut shared_ctx = DefaultServerContext::new(document_provider);
        // ctx_handle.spawn(receiver.for_each(move |(msg, oneshot): (_, ResponseMsgChannel)| {
        core.run(receiver.for_each(move |(msg, oneshot): (_, ResponseMsgChannel)| {
            let response = shared_ctx.handle_msg(msg);
            oneshot.send(response).unwrap();

            future::ok(())
        })).ok().unwrap();
    });

    let mut core = Core::new().unwrap();
    let handle = core.handle();
    let factory = IsymtopeServiceFactory::new(sender, handle.clone());

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