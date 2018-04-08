use std::str::FromStr;
use std::io;
use std::result::Result;

use regex::Regex;

use futures::{future, Future};
use hyper::header::{Location, Host};

use hyper::{self, Error as HyperError, Method, Request, Response, StatusCode};
use hyper::server::{NewService, Service};

use tokio_core::reactor::Handle;

use server::{APP_DIR, DEFAULT_APP};
use super::*;

lazy_static! {
    pub static ref APP_ROUTE: Regex = Regex::new(r"app/(?P<app>[a-zA-Z0-9_]+)(?P<path>/*(.*))").unwrap();
    pub static ref STATIC_RESOURCE_ROUTE: Regex = Regex::new(r"resources/static/(?P<path>(.*))").unwrap();
    pub static ref APP_RESOURCE_ROUTE: Regex = Regex::new(r"resources/app/(?P<app>[a-zA-Z0-9_]+)(?P<path>/*(.*))").unwrap();
}

#[cfg(feature = "playground_api")]
lazy_static! {
    pub static ref PLAYGROUND_ROUTE: Regex = Regex::new(r"app/playground/api/(?P<path>/*(.*))").unwrap();
    pub static ref PLAYGROUND_RESOURCE_ROUTE: Regex = Regex::new(r"app/playground/_worker/app/(?P<app>[a-zA-Z0-9_]+)(?P<path>/*(.*))").unwrap();
}

#[derive(Debug)]
pub struct DefaultServiceFactory {
    render_service_factory: TemplateRenderServiceFactory,
    resource_service_factory: TemplateResourceServiceFactory,
    static_resource_service_factory: StaticResourceServiceFactory,
    #[cfg(feature = "playground_api")]
    playground_service_factory: PlaygroundApiServiceFactory,
    handle: Handle,
}

impl DefaultServiceFactory {
    #[cfg(not(feature = "playground_api"))]
    pub fn new(
        render_service_factory: TemplateRenderServiceFactory,
        resource_service_factory: TemplateResourceServiceFactory,
        static_resource_service_factory: StaticResourceServiceFactory,
        handle: Handle,
    ) -> Self {
        DefaultServiceFactory {
            render_service_factory: render_service_factory,
            resource_service_factory: resource_service_factory,
            static_resource_service_factory: static_resource_service_factory,
            handle: handle,
        }
    }

    #[cfg(feature = "playground_api")]
    pub fn new(
        render_service_factory: TemplateRenderServiceFactory,
        resource_service_factory: TemplateResourceServiceFactory,
        static_resource_service_factory: StaticResourceServiceFactory,
        playground_service_factory: PlaygroundApiServiceFactory,
        handle: Handle,
    ) -> Self {
        DefaultServiceFactory {
            render_service_factory: render_service_factory,
            resource_service_factory: resource_service_factory,
            static_resource_service_factory: static_resource_service_factory,
            playground_service_factory: playground_service_factory,
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
        let static_resource_service = self.static_resource_service_factory.create();
        #[cfg(feature = "playground_api")]
        let playground_service = self.playground_service_factory.create();

        Ok(DefaultService {
            render_service: render_service,
            resource_service: resource_service,
            static_resource_service: static_resource_service,
            #[cfg(feature = "playground_api")]
            playground_service: playground_service,
            handle: self.handle.clone(),
        })
    }
}

#[derive(Debug)]
pub struct DefaultService {
    render_service: TemplateRenderService,
    resource_service: TemplateResourceService,
    static_resource_service: StaticResourceService,
    #[cfg(feature = "playground_api")]
    playground_service: PlaygroundApiService,
    handle: Handle,
}

impl Service for DefaultService {
    type Request = Request;
    type Response = Response;
    type Error = HyperError;
    type Future = Box<Future<Item = Response, Error = Self::Error>>;

    fn call(&self, req: Self::Request) -> Self::Future {
        let trimmed_path = req.path().trim_left_matches('/').to_owned();

        // Redirect to default app
        if trimmed_path == "" {
            let response = Response::new()
                .with_status(StatusCode::Found)
                .with_header(Location::new(format!("/app/{}/", &*DEFAULT_APP)));

            return Box::new(future::ok(response));
        };

        eprintln!("Request uri: {:?}", req.uri());

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

        // let default_port = if proto == "http" { 80 } else { 443 };

        let host = req.headers().get::<Host>().map(|h| h.hostname().to_owned()).unwrap();
        let host = req.headers().get::<Host>().and_then(|h| h.port())
            .map(|port| format!("{}:{}", host, port))
            .unwrap_or_else(|| format!("{}", host));

        let base_url = format!("{}://{}/", proto, host);

        #[cfg(feature = "playground_api")]
        {
            if let Some(captures) = PLAYGROUND_ROUTE.captures(&trimmed_path) {
                let path = captures
                    .name("path")
                    .map(|m| m.as_str())
                    .unwrap_or_default();

                if path == "compile" {
                    let base_url = "";
                    let app_name = "/app.ism";
                    // let mut api_req =
                    //     Request::new(Method::Post, FromStr::from_str("/").unwrap());
                    // if let Some(body_ref) = req.body_ref() {
                    //     api_req.set_body(body_ref);
                    // };

                    let response = self.playground_service
                        .call(&base_url, &app_name, req);
                    return Box::new(response);
                }
            };

            if let Some(captures) = PLAYGROUND_RESOURCE_ROUTE.captures(&trimmed_path) {
                let app_name = captures.name("app").unwrap().as_str().to_owned();
                let base_url = format!("{}app/{}/", base_url, app_name);

                eprintln!("Base uri: {:?}", base_url);

                let path = captures
                    .name("path")
                    .map(|m| m.as_str())
                    .unwrap_or_default();
                let trimmed_path = path.trim_left_matches('/').to_owned();
                let trimmed_resource_path = if trimmed_path == "" { "index.html".to_owned() } else { trimmed_path };

                // Handle resource file case
                let app_resource_path = &*APP_DIR.join(&app_name).join(&trimmed_resource_path);

                println!("default_service: looking for worker resource at path {:?}", app_resource_path);

                // Serve resource
                if app_resource_path.is_file() {
                    let resource_path = format!("/{}/{}", app_name, trimmed_resource_path);
                    let resource_req =
                        Request::new(Method::Get, FromStr::from_str(&resource_path).unwrap());
                    let response = self.resource_service
                        .call(&base_url, &app_name, resource_req);
                    return Box::new(response);
                };

                // Return not found error
                let response = hyper::Response::new()
                    .with_status(StatusCode::NotFound)
                    .with_body("Resource not found");
                return Box::new(future::ok(response));
            }
        }

        if let Some(captures) = STATIC_RESOURCE_ROUTE.captures(&trimmed_path) {
            let path = captures.name("path").unwrap().as_str().to_owned();
            let trimmed_resource_path = path.trim_left_matches('/').to_owned();

            // Handle resource file case
            let resource_file_path = &*STATIC_RESOURCE_DIR.join(&trimmed_resource_path);

            // Serve resource
            if resource_file_path.is_file() {
                let resource_path = format!("/{}", trimmed_resource_path);
                let resource_req =
                    Request::new(Method::Get, FromStr::from_str(&resource_path).unwrap());
                let response = self.static_resource_service
                    .call(resource_req);
                return Box::new(response);
            };
        }

        if let Some(captures) = APP_RESOURCE_ROUTE.captures(&trimmed_path) {
            let app_name = captures.name("app").unwrap().as_str().to_owned();

            // let (scheme, authority) = (req.uri().scheme().unwrap(), req.uri().authority().unwrap());
            // let base_url = format!("{}://{}/app/{}/", scheme, authority, app_name);
            let base_url = format!("{}app/{}/", base_url, app_name);

            eprintln!("Base uri: {:?}", base_url);

            let path = captures
                .name("path")
                .map(|m| m.as_str())
                .unwrap_or_default();
            let trimmed_path = path.trim_left_matches('/').to_owned();
            let trimmed_resource_path = if trimmed_path == "" { "index.html".to_owned() } else { trimmed_path };

            // Handle resource file case
            let app_resource_path = &*APP_DIR.join(&app_name).join(&trimmed_resource_path);

            println!("default_service: looking for resource at path {:?}", app_resource_path);

            // Serve resource
            if app_resource_path.is_file() {
                let resource_path = format!("/{}/{}", app_name, trimmed_resource_path);
                let resource_req =
                    Request::new(Method::Get, FromStr::from_str(&resource_path).unwrap());
                let response = self.resource_service
                    .call(&base_url, &app_name, resource_req);
                return Box::new(response);
            };

            // Default file
            // let default_resource_path = app_resource_path.join("index.html");

            // println!("default_service: looking for default resource at path {:?}", default_resource_path);

            // if default_resource_path.is_file() {
            //     let resource_path = format!("/{}/{}/index.html", app_name, trimmed_resource_path);
            //     let resource_req =
            //         Request::new(Method::Get, FromStr::from_str(&resource_path).unwrap());
            //     let response = self.resource_service
            //         .call(&base_url, &app_name, resource_req);
            //     return Box::new(response);
            // };

            let response = hyper::Response::new()
                .with_status(StatusCode::NotFound)
                .with_body("Resource not found");
            return Box::new(future::ok(response));
        }

        if let Some(captures) = APP_ROUTE.captures(&trimmed_path) {
            // let captures: Vec<_> = captures.into_iter().collect();
            let app_name = captures.name("app").unwrap().as_str().to_owned();

            // let (scheme, authority) = (req.uri().scheme().unwrap(), req.uri().authority().unwrap());
            // let base_url = format!("{}://{}/app/{}/", scheme, authority, app_name);
            let base_url = format!("{}resources/app/{}/", base_url, app_name);

            eprintln!("Base uri: {:?}", base_url);

            let path = captures
                .name("path")
                .map(|m| m.as_str())
                .unwrap_or_default();
            let path = if path == "" { "/" } else { path }.to_owned();
            let trimmed_path_in_app = path.trim_left_matches('/').to_owned();

            eprintln!(
                "[default service] requested path [{:?}] in app {}",
                path, app_name
            );

            // Handle resource file case
            let app_resource_path = &*APP_DIR.join(&app_name).join(&trimmed_path_in_app);

            // Serve resource
            if app_resource_path.is_file() {
                let resource_path = format!("/{}/{}", app_name, trimmed_path_in_app);
                let resource_req =
                    Request::new(Method::Get, FromStr::from_str(&resource_path).unwrap());
                let response = self.resource_service
                    .call(&base_url, &app_name, resource_req);
                return Box::new(response);
            };

            // Render route
            // let template_path = if path == "/" { "/app.ism".to_owned() } else { path }.to_owned();
            let isymtope_req = Request::new(Method::Get, FromStr::from_str(&path).unwrap());
            let response = self.render_service.call(&base_url, &app_name, isymtope_req);
            return Box::new(response);
        };

        // Return not found error
        let response = hyper::Response::new()
            .with_status(StatusCode::NotFound)
            .with_body("Resource not found");
        Box::new(future::ok(response))
    }
}
