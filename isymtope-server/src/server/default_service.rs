use std::env;
use std::str::FromStr;
use std::io::{Error as IOError, ErrorKind as IOErrorKind, Read};
use std::fs::File;
use std::path::Path;
use std::result::Result;
use std::sync::Mutex;
use std::error::Error;

use futures::{self, Future};
use hyper::header::{ContentType, Location};
use hyper::mime;

use hyper::{self, StatusCode, Error as HyperError, Request, Response, Method, Uri};
use hyper::server::{Http, NewService, Server, Service};
use hyper_staticfile::Static;

use tokio_core::reactor::Handle;

use super::*;

#[derive(Debug)]
pub struct DefaultServiceFactory {
    isymtope_service_factory: IsymtopeServiceFactory,
    handle: Handle,
}

impl DefaultServiceFactory {
    pub fn new(isymtope_service_factory: IsymtopeServiceFactory, handle: Handle) -> Self {
        DefaultServiceFactory {
            isymtope_service_factory: isymtope_service_factory,
            handle: handle,
        }
    }
}

impl NewService for DefaultServiceFactory {
    type Request = <Self::Instance as Service>::Request;
    type Response = <Self::Instance as Service>::Response;
    type Error = <Self::Instance as Service>::Error;
    type Instance = DefaultService;

    fn new_service(&self) -> Result<Self::Instance, io::Error> {
        let isymtope_service = self.isymtope_service_factory.new_service()?;

        Ok(DefaultService {
            isymtope_service: isymtope_service,
            handle: self.handle.clone(),
        })
    }
}

#[derive(Debug)]
pub struct DefaultService {
    isymtope_service: IsymtopeService,
    handle: Handle,
}

impl Service for DefaultService {
    type Request = Request;
    type Response = Response;
    type Error = HyperError;
    type Future = Box<Future<Item = Response, Error = Self::Error>>;

    fn call(&self, req: Self::Request) -> Self::Future {

        let default_app = env::var_os("DEFAULT_APP").expect("DEFAULT_APP must be provided");
        let default_app_str: String = default_app.to_string_lossy().to_string();
        let default_app_trimmed = default_app_str.trim_left_matches('/').to_owned();
        let default_app_trailing = format!("{}/", default_app_trimmed);
        let default_workspace_str = format!("{}/preview-1bcx1", default_app_str);

        let resource_dir = env::var_os("RESOURCE_DIR").expect("RESOURCE_DIR must be provided");
        let resource_dir = Path::new(&resource_dir);

        let original_path = req.path().to_owned();
        let trimmed_path = req.path().trim_left_matches('/').to_owned();

        // Redirect to default app
        if (trimmed_path == "") {
            let response = Response::new()
                .with_status(StatusCode::Found)
                .with_header(Location::new(default_app_trailing));

            return Box::new(future::ok(response));
        };

        if (trimmed_path.starts_with(&default_app_trimmed)) {
            eprintln!("[default service] requested path in default app");

            // Strip off workspace prefix
            // TODO: support actually serving modified files
            let original_path = req.path().to_owned();
            let prefix_len = if original_path.starts_with(&default_workspace_str) { default_workspace_str.len() } else { default_app_str.len() };
            let relative_path = original_path[prefix_len..].to_owned();
            let trimmed_relative_path = relative_path.trim_left_matches('/').to_owned();

            // Handle resource file case
            let app_resource_path = resource_dir.join(&default_app_trimmed).join(&trimmed_relative_path);

            if app_resource_path.is_file() {
                eprintln!("[default app] Serving resource path: {:?}", app_resource_path);
                let serve_files = Static::new(&self.handle, &resource_dir);
                let static_resp = serve_files.call(req);
                let response = static_resp.map(move |response| {
                    let mut headers = response.headers().to_owned();
                    if trimmed_relative_path.ends_with(".js") {
                        headers.set(ContentType(mime::TEXT_JAVASCRIPT));
                    }

                    Response::new()
                        .with_status(response.status())
                        .with_headers(headers)
                        .with_body(response.body())
                });

                return Box::new(response);
            };

            // Pass request to isymtope service
            let isymtope_path = if relative_path == "" { "/" } else { &relative_path };
            let isymtope_req = Request::new(Method::Get, FromStr::from_str(isymtope_path).unwrap());
            let response = self.isymtope_service.call(isymtope_req);
            return Box::new(response);
        };

        // Return not found error
        let response = hyper::Response::new()
            .with_status(StatusCode::NotFound)
            .with_body("Resource not found");
        Box::new(future::ok(response))
    }
}
