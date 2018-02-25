use std::env;
use std::io::{Error as IOError, ErrorKind as IOErrorKind, Read};
use std::fs::File;
use std::path::{Path, PathBuf};
use std::result::Result;
use std::sync::Mutex;
use std::error::Error;

use futures::{self, Future};
use hyper::header::{ContentType, Location};
use hyper::mime;

use hyper::{self, Error as HyperError, Method, Request, Response, StatusCode, Uri};
use hyper::server::{Http, NewService, Server, Service};
use hyper_staticfile::Static;

use tokio_core::reactor::Handle;

use server::APP_DIR;
use super::*;

#[derive(Debug)]
pub struct TemplateResourceServiceFactory {
    handle: Handle,
}

impl TemplateResourceServiceFactory {
    pub fn new(handle: Handle) -> Self {
        TemplateResourceServiceFactory { handle: handle }
    }
}

impl IsymtopeAppServiceFactory for TemplateResourceServiceFactory {
    type Request = Request;
    type Response = Response;
    type Error = HyperError;
    type Instance = TemplateResourceService;

    fn create(&self) -> Self::Instance {
        TemplateResourceService {
            handle: self.handle.clone(),
        }
    }
}

#[derive(Debug)]
pub struct TemplateResourceService {
    handle: Handle,
}

impl IsymtopeAppService for TemplateResourceService {
    type Request = Request;
    type Response = Response;
    type Error = HyperError;
    type Future = Box<Future<Item = Response, Error = Self::Error>>;

    fn call(&self, base_url: &str, app_name: &str, req: Request) -> Self::Future {
        let path = req.path().to_owned();

        eprintln!("[resource service] Serving resource path: {:?}", path);
        let serve_files = Static::new(&self.handle, &*APP_DIR.to_path_buf());
        let static_resp = serve_files.call(req);
        let response = static_resp.map(move |response| {
            let mut headers = response.headers().to_owned();
            if path.ends_with(".js") {
                headers.set(ContentType(mime::TEXT_JAVASCRIPT));
            } else if path.ends_with(".css") {
                headers.set(ContentType(mime::TEXT_CSS));
            } else if path.ends_with(".html") {
                headers.set(ContentType(mime::TEXT_HTML));
            }

            Response::new()
                .with_status(response.status())
                .with_headers(headers)
                .with_body(response.body())
        });

        return Box::new(response);
    }
}
