use std::env;
use std::fmt::Debug;
use std::io::{self, Write};
use std::path::{Path, PathBuf};
use std::rc::Rc;
use std::sync::{Arc, Mutex};
use std::thread;

use std::collections::HashMap;

use futures::{self, Stream};
use futures::future::{self, FutureResult};

use tokio_core::reactor::Core;
use tokio_core::net::TcpListener;

// use hyper;
use hyper::server::{Http, NewService, Request, Response, Server, Service};
// use hyper::Service;
use regex::RegexSet;

use isymtope_ast_common::*;
use isymtope_build::*;
use isymtope_generate::*;
#[cfg(feature = "playground_api")]
use compiler_service::*;
use super::*;

lazy_static! {
    pub static ref APP_DIR: Box<PathBuf> = Box::new(env::var_os("APP_DIR").expect("APP_DIR must be provided").into());
    pub static ref STATIC_RESOURCE_DIR: Box<PathBuf> = Box::new(env::var_os("STATIC_RESOURCE_DIR").expect("STATIC_RESOURCE_DIR must be provided").into());
    pub static ref DEFAULT_APP: String = env::var_os("DEFAULT_APP").expect("DEFAULT_APP must be provided").to_string_lossy().to_string();
}

pub trait ServiceInject: Debug {
    type ServiceImpl: Service;
}

pub fn spawn_server_msg_handler(app_dir: &Path) -> IsymtopeServerResult<RequestMsgChannel> {
    let (sender, receiver) = futures::sync::mpsc::unbounded();
    let app_dir = app_dir.to_owned();

    thread::spawn(move || {
        use futures::Stream;
        let mut core = Core::new().unwrap();
        let mut shared_ctx = DefaultServerContext::new(&app_dir);

        core.run(
            receiver.for_each(move |(msg, oneshot): (_, ResponseMsgChannel)| {
                let response = shared_ctx.handle_msg(msg);
                oneshot.send(response).unwrap();

                future::ok(())
            }),
        ).ok()
            .unwrap();
    });

    Ok(sender)
}


pub fn run_server(addr: &str) -> IsymtopeServerResult<()> {
    let addr = addr.parse()?;
    let app_dir = &*APP_DIR;

    let server_msg_handler = spawn_server_msg_handler(app_dir)?;

    #[cfg(feature = "playground_api")]
    let compiler_msg_handler = spawn_compiler_service()?;

    // let default_app = env::var_os("DEFAULT_APP").expect("DEFAULT_APP must be provided");
    // let default_app_str: String = default_app.to_string_lossy().to_string();

    let default_app_str = &*DEFAULT_APP.to_owned();

    let mut core = Core::new().unwrap();
    let handle = core.handle();

    let render_service_factory =
        TemplateRenderServiceFactory::new(server_msg_handler, handle.clone(), default_app_str.to_owned());
    let static_resource_service_factory = StaticResourceServiceFactory::new(handle.clone());
    let resource_service_factory = TemplateResourceServiceFactory::new(handle.clone());

    #[cfg(feature = "playground_api")]
    let playground_api_service = PlaygroundApiServiceFactory::new(compiler_msg_handler, handle.clone());

    let factory = DefaultServiceFactory::new(
        render_service_factory,
        resource_service_factory,
        static_resource_service_factory,
        #[cfg(feature = "playground_api")]
        playground_api_service,
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
