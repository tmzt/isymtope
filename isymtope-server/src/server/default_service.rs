use std::env;
use std::str::{self, FromStr};
use std::io::{Error as IOError, ErrorKind as IOErrorKind, Read};
use std::fs::File;
use std::path::{Path, PathBuf};
use std::result::Result;
use std::sync::Mutex;
use std::error::Error;

use regex::Regex;

use futures::{self, Future};
use hyper::header::{ContentType, Location};
use hyper::mime;

use hyper::{self, Error as HyperError, Method, Request, Response, StatusCode, Uri};
use hyper::server::{Http, NewService, Server, Service};
use hyper_staticfile::Static;

use tokio_core::reactor::Handle;

use super::*;

lazy_static! {
    pub static ref APP_ROUTE: Regex = Regex::new(r"app/(?P<app>[a-zA-Z0-9_]+)(?P<path>/*(.*))").unwrap();
    pub static ref APP_RESOURCE_ROUTE: Regex = Regex::new(r"resources/app/(?P<app>[a-zA-Z0-9_]+)(?P<path>/*(.*))").unwrap();
}

#[derive(Debug)]
pub struct DefaultServiceFactory {
    render_service_factory: TemplateRenderServiceFactory,
    resource_service_factory: TemplateResourceServiceFactory,
    handle: Handle,
}

impl DefaultServiceFactory {
    pub fn new(
        render_service_factory: TemplateRenderServiceFactory,
        resource_service_factory: TemplateResourceServiceFactory,
        handle: Handle,
    ) -> Self {
        DefaultServiceFactory {
            render_service_factory: render_service_factory,
            resource_service_factory: resource_service_factory,
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
        let render_service = self.render_service_factory.create();
        let resource_service = self.resource_service_factory.create();

        Ok(DefaultService {
            render_service: render_service,
            resource_service: resource_service,
            handle: self.handle.clone(),
        })
    }
}

#[derive(Debug)]
pub struct DefaultService {
    render_service: TemplateRenderService,
    resource_service: TemplateResourceService,
    handle: Handle,
}

impl Service for DefaultService {
    type Request = Request;
    type Response = Response;
    type Error = HyperError;
    type Future = Box<Future<Item = Response, Error = Self::Error>>;

    fn call(&self, req: Self::Request) -> Self::Future {
        let original_path = req.path().to_owned();
        let trimmed_path = req.path().trim_left_matches('/').to_owned();

        // let default_app_str = default_app.to_string_lossy().to_string();
        // let default_app_trimmed = default_app_str.trim_left_matches('/').to_owned();
        // let default_app_trailing = format!("{}/", default_app_trimmed);
        let default_workspace_str = format!("/app/{}/preview-1bcx1", &*DEFAULT_APP);

        // Redirect to default app
        if (trimmed_path == "") {
            let response = Response::new()
                .with_status(StatusCode::Found)
                .with_header(Location::new(format!("/app/{}/", &*DEFAULT_APP)));

            return Box::new(future::ok(response));
        };

        eprintln!("Request uri: {:?}", req.uri());

        let host = req.headers().get_raw("host").unwrap().one().unwrap();
        let host = str::from_utf8(&host).unwrap();

        let forwarded_proto = req.headers().get_raw("x-forwarded-proto").and_then(|s| s.one()).and_then(|s| str::from_utf8(s).ok());
        eprintln!("Forwarded proto: {:?}", forwarded_proto);

        let mut proto = "http";
        if let Some(forwarded_proto) = forwarded_proto {
            if forwarded_proto.to_lowercase() == "https" {
                proto = "https";
            };
        };

        let base_url = format!("{}://{}/", proto, host);

        if let Some(captures) = APP_RESOURCE_ROUTE.captures(&trimmed_path) {
            let app_name = captures.name("app").unwrap().as_str().to_owned();

            // let (scheme, authority) = (req.uri().scheme().unwrap(), req.uri().authority().unwrap());
            // let base_url = format!("{}://{}/app/{}/", scheme, authority, app_name);
            let base_url = format!("{}app/{}/", base_url, app_name);

            eprintln!("Base uri: {:?}", base_url);

            let path = captures.name("path").map(|m| m.as_str()).unwrap_or_default();
            let path = if path == "" { "/" } else { path }.to_owned();
            let trimmed_resource_path = path.trim_left_matches('/').to_owned();

            // Handle resource file case
            let app_resource_path = &*APP_DIR.join(&app_name).join(&trimmed_resource_path);

            // Serve resource
            if app_resource_path.is_file() {
                let resource_path = format!("/{}/{}", app_name, trimmed_resource_path);
                let resource_req = Request::new(Method::Get, FromStr::from_str(&resource_path).unwrap());
                let response = self.resource_service
                    .call(&base_url, &app_name, resource_req);
                return Box::new(response);
            };
        }

        if let Some(captures) = APP_ROUTE.captures(&trimmed_path) {
            // let captures: Vec<_> = captures.into_iter().collect();
            let app_name = captures.name("app").unwrap().as_str().to_owned();

            // let (scheme, authority) = (req.uri().scheme().unwrap(), req.uri().authority().unwrap());
            // let base_url = format!("{}://{}/app/{}/", scheme, authority, app_name);
            let base_url = format!("{}resources/app/{}/", base_url, app_name);

            eprintln!("Base uri: {:?}", base_url);

            let path = captures.name("path").map(|m| m.as_str()).unwrap_or_default();
            let path = if path == "" { "/" } else { path }.to_owned();
            let trimmed_path_in_app = path.trim_left_matches('/').to_owned();

            eprintln!("[default service] requested path [{:?}] in app {}", path, app_name);
            let trimmed_relative_path = path.trim_left_matches('/').to_owned();

            // Handle resource file case
            let app_resource_path = &*APP_DIR.join(&app_name).join(&trimmed_path_in_app);

            // Serve resource
            if app_resource_path.is_file() {
                let resource_path = format!("/{}/{}", app_name, trimmed_path_in_app);
                let resource_req = Request::new(Method::Get, FromStr::from_str(&resource_path).unwrap());
                let response = self.resource_service
                    .call(&base_url, &app_name, resource_req);
                return Box::new(response);
            };

            // Render route
            // let template_path = if path == "/" { "/app.ism".to_owned() } else { path }.to_owned();
            let isymtope_req = Request::new(Method::Get, FromStr::from_str(&path).unwrap());
            let response = self.render_service
                .call(&base_url, &app_name, isymtope_req);
            return Box::new(response);
        };

        // Return not found error
        let response = hyper::Response::new()
            .with_status(StatusCode::NotFound)
            .with_body("Resource not found");
        Box::new(future::ok(response))
    }
}
