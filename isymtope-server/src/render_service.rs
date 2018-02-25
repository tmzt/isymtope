use std::env;
use std::io::{Error as IOError, ErrorKind as IOErrorKind, Read};
use std::fs::File;
use std::path::{Path, PathBuf};
use std::result::Result;
use std::sync::Mutex;
use std::error::Error;

#[cfg(feature = "session_time")]
use time::Duration;
use futures::{self, future, Future};
use hyper::{Request, Response};
use hyper::header::ContentType;
use hyper::mime;
use hyper::Error as HyperError;
use hyper_staticfile::Static;
use regex::RegexSet;

use tokio_core::reactor::Handle;

use isymtope_build::input::*;
use isymtope_build::processing::*;
use super::*;

pub type ResponseMsgChannel = futures::sync::oneshot::Sender<IsymtopeServerResult<ResponseMsg>>;
pub type RequestMsgChannel = futures::sync::mpsc::UnboundedSender<(Msg, ResponseMsgChannel)>;

#[derive(Debug)]
pub struct TemplateRenderServiceFactory {
    sender: RequestMsgChannel,
    handle: Handle,
    prefix: String,
}

impl TemplateRenderServiceFactory {
    pub fn new(sender: RequestMsgChannel, handle: Handle, prefix: String) -> Self {
        TemplateRenderServiceFactory {
            sender: sender,
            handle: handle,
            prefix: prefix,
        }
    }
}

impl IsymtopeAppServiceFactory for TemplateRenderServiceFactory {
    type Request = Request;
    type Response = Response;
    type Error = HyperError;
    type Instance = TemplateRenderService;

    fn create(&self) -> Self::Instance {
        TemplateRenderService {
            sender: self.sender.clone(),
            handle: self.handle.clone(),
            prefix: self.prefix.clone(),
        }
    }
}

#[derive(Debug)]
pub struct TemplateRenderService {
    sender: RequestMsgChannel,
    handle: Handle,
    prefix: String,
}

impl IsymtopeAppService for TemplateRenderService {
    type Request = Request;
    type Response = Response;
    type Error = HyperError;
    type Future = Box<Future<Item = Self::Response, Error = Self::Error>>;

    fn call(&self, base_url: &str, app_name: &str, req: Request) -> Self::Future {
        let (tx, rx) = futures::sync::oneshot::channel::<IsymtopeServerResult<ResponseMsg>>();
        let template_path = "/app.ism".to_owned();

        let render = Msg::RenderAppRoute(
            base_url.to_owned(),
            app_name.to_owned(),
            template_path,
            req.path().to_owned(),
        );
        self.sender.unbounded_send((render, tx)).unwrap();

        let work = rx.map_err(|_| {
            HyperError::Io(IOError::new(
                IOErrorKind::Other,
                "Failed making request on ServerContext.",
            ))
        }).and_then(move |rendered| {
            eprintln!("Got render complete.");
            eprintln!("Got render result: {:?}", rendered);

            match rendered {
                Ok(ResponseMsg::RenderComplete(response)) => {
                    let body = response.into_inner();
                    let response = Response::new().with_body(body);
                    future::ok(response)
                }

                _ => {
                    let body = format!("Unknown response message from render task: {:?}", rendered);
                    let response = Response::new().with_body(body);
                    future::ok(response)
                }
            }
        });

        Box::new(work)
    }
}

// impl Service for IsymtopeService {
//     type Request = Request;
//     type Response = Response;
//     type Error = HyperError;
//     type Future = Box<Future<Item = Response, Error = Self::Error>>;

//     fn call(&self, req: Self::Request) -> Self::Future {
//     }
// }
