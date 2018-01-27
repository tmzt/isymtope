
use std::fmt::Debug;
use std::io::{self, Write};
use std::path::Path;
use std::rc::Rc;
use std::sync::Mutex;

use futures;
use futures::future::{self, FutureResult};

use tokio_core::reactor::Core;

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

    // https://users.rust-lang.org/t/questions-about-tokio-futures-and-rwlock/11260/11
    let (sender, receiver) = futures::sync::mpsc::unbounded();
    let t = ::std::thread::spawn(move || {
        use futures::Stream;
        let mut core = Core::new().unwrap();

        let document_provider: Rc<DocumentProvider> = Default::default();
        let mut shared_ctx = DefaultServerContext::new(document_provider);
        core.run(receiver.for_each(|(msg, oneshot): (_, ResponseMsgChannel)| {
            let response = shared_ctx.handle_msg(msg);
            oneshot.send(response).unwrap();

            future::ok(())
        }))
    });

    let factory = IsymtopeServiceFactory::new(sender);
    let server = Http::new().bind(&addr, factory)?;
    server.run().unwrap();

    Ok(())
}