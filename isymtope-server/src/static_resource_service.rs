
use futures::Future;
use hyper::{Error as HyperError, Request, Response};
use hyper::server::Service;
use hyper::header::ContentType;
use hyper::mime;

use hyper_staticfile::Static;

use tokio_core::reactor::Handle;

use server::STATIC_RESOURCE_DIR;

#[derive(Debug)]
pub struct StaticResourceServiceFactory {
    handle: Handle,
}

impl StaticResourceServiceFactory {
    pub fn new(handle: Handle) -> Self {
        StaticResourceServiceFactory { handle: handle }
    }

    pub fn create(&self) -> StaticResourceService {
        StaticResourceService {
            handle: self.handle.clone(),
        }
    }
}

#[derive(Debug)]
pub struct StaticResourceService {
    handle: Handle,
}

impl Service for StaticResourceService {
    type Request = Request;
    type Response = Response;
    type Error = HyperError;
    type Future = Box<Future<Item = Response, Error = Self::Error>>;

    fn call(&self, req: Request) -> Self::Future {
        let path = req.path().to_owned();

        eprintln!("[resource service] Serving resource path: {:?}", path);
        let serve_files = Static::new(&self.handle, &*STATIC_RESOURCE_DIR.to_path_buf());
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
