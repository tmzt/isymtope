extern crate dotenv;
#[macro_use]
extern crate log;
extern crate env_logger;
#[macro_use]
extern crate lazy_static;
extern crate regex;

extern crate openssl_probe;

#[cfg(feature="playground_api")]
#[macro_use]
extern crate serde_derive;
#[cfg(feature="playground_api")]
#[macro_use]
extern crate serde;
extern crate serde_json;
#[cfg(feature="playground_api")]
#[macro_use]
extern crate chrono;
#[cfg(feature="playground_api")]
#[macro_use]
extern crate redis_async;
#[cfg(feature="playground_api")]
extern crate uuid;

extern crate actix;
extern crate actix_web;
extern crate futures;
extern crate bytes;

extern crate isymtope_ast_common;
extern crate isymtope_build;
extern crate isymtope_generate;

mod compiler;

#[cfg(feature="playground_api")]
mod playground_api;

use std::env;
use std::path::PathBuf;
use std::str::FromStr;
use dotenv::dotenv;

use regex::Regex;

use actix::prelude::*;
use actix_web::*;
use actix_web::http::Method;
use actix_web::middleware::{Logger, SessionStorage, RequestSession, CookieSessionBackend, ErrorHandlers, Response};
use actix_web::server::HttpServer;
use futures::Future;
use futures::future::{FutureResult, ok};

use isymtope_generate::*;

#[cfg(feature="playground_api")]
use self::playground_api::*;

pub use self::compiler::*;

lazy_static! {
    // pub static ref STATIC_APP_ROOT: Box<PathBuf> = Box::new(env::var_os("STATIC_APP_ROOT").expect("STATIC_APP_ROOT must be provided").into());
    // pub static ref STATIC_RESOURCE_ROOT: Box<PathBuf> = Box::new(env::var_os("STATIC_RESOURCE_ROOT").expect("STATIC_RESOURCE_ROOT must be provided").into());
    pub static ref APP_NAME: String = env::var_os("APP_NAME").expect("APP_NAME must be provided").to_string_lossy().to_string();
    pub static ref DEV_PORT: Option<u16> = env::var_os("DEV_PORT").and_then(|p| FromStr::from_str(&p.to_string_lossy().to_string()).ok());
}

#[cfg(feature="playground_api")]
lazy_static! {
    pub static ref PLAYGROUND_APP_DNS_SUFFIX: String = env::var_os("PLAYGROUND_APP_DNS_SUFFIX").expect("PLAYGROUND_APP_DNS_SUFFIX must be provided").to_string_lossy().to_string();
    pub static ref PLAYGROUND_APP_HOST: Regex = Regex::new(&format!(r"(?P<slug>[a-zA-Z0-9]+).f.r{}(?P<port>:\d*)", &*PLAYGROUND_APP_DNS_SUFFIX)).unwrap();
}

fn render_example_app_route(state: &AppState, static_template: &str, base_url: &str, route: &str) -> FutureResponse<HttpResponse> {
    let msg = RenderExampleAppRoute { app_name: static_template.to_owned(), route:  route.to_owned(), base_url: base_url.to_owned() };
    state.compiler.send(msg)
        .from_err()
        .and_then(|res| {
            match res {
                Ok(body) => {
                    Ok(HttpResponse::Ok()
                        .content_type("text/html")
                        .body(body))
                },
                _ => Ok(HttpResponse::InternalServerError().into())
            }
        })
        .responder()
}

fn render_route(mut req: HttpRequest<AppState>) -> FutureResponse<HttpResponse> {
    let playground_session_id = ::uuid::Uuid::new_v4().to_string();
    req.session().set("playground_session_id", playground_session_id).unwrap();

    let state = req.state();
    let path = req.uri().path().to_owned();

    let scheme = req.connection_info().scheme().to_owned();
    let host = req.connection_info().host().to_owned();
    let base_url = format!("{}://{}/", scheme, host);

    #[cfg(feature = "playground_api")]
    {
        if let Some(captures) = PLAYGROUND_APP_HOST.captures(&host) {
            let api = state.api.clone();
            let compiler = state.compiler.clone();
            let slug = captures.name("slug").map(|m| m.as_str()).unwrap();

            return state.api.send(CompileTemplate { api: api, compiler: compiler, base_url: base_url.to_owned(), route: path.clone(), slug: slug.to_owned() })
                .from_err()
                .and_then(|res| match res {
                    Ok(res) => {
                        Ok(HttpResponse::Ok()
                            .content_type("text/html")
                            .body(res.body))
                    },
                    _ => Ok(HttpResponse::InternalServerError().into())
                })
                .responder();
        };
    }

    // let slug = req.match_info().get("slug");
    render_example_app_route(state, &*APP_NAME, &base_url, &path)
}

pub struct AppState {
    compiler: Addr<Syn, Compiler>,
    api: Addr<Syn, PlaygroundApi>,
}

fn render_500<S>(_: &mut HttpRequest<S>, resp: HttpResponse) -> Result<Response> {
    Ok(Response::Done(resp.into_builder()
        .body("Error occurred parsing page template")))
}

fn main() {
    openssl_probe::init_ssl_cert_env_vars();
    // dotenv().ok();
    // env::set_var("RUST_LOG", "actix_web=debug");
    // env::set_var("RUST_BACKTRACE", "1");host:3000/api/auth/github/complete?code=e78f8f0ecebf16b21c65&state=7ee77eb9-974c-448f-af6b-
    env_logger::init();
    let sys = actix::System::new("server");

    let compiler: Addr<Syn, _> = Arbiter::start(|_| Compiler::default());
    let api: Addr<Syn, _> = Arbiter::start(|_| PlaygroundApi::default());

    HttpServer::new(
        move || {
            let state = AppState { compiler: compiler.clone(), api: api.clone() };

            let mut app = App::with_state(state)
                .middleware(ErrorHandlers::new()
                    .handler(http::StatusCode::INTERNAL_SERVER_ERROR, render_500)
                )
                .middleware(SessionStorage::new(
                    CookieSessionBackend::signed(&[0; 32])
                        .secure(false)
                ));

            #[cfg(feature="playground_api")]
            {
                app = app
                    .resource("/r/{slug:[0-9a-zA-Z]+}", |r| r.method(Method::GET).a(render_route))
                    .resource("/api/create_example", |r| r.method(Method::POST).with2(create_example))
                    .resource("/api/examples", |r| r.method(Method::GET).a(get_example_index))
                    .resource("/api/apps/{slug:[0-9a-zA-Z]+}", |r| r.method(Method::GET).with2(get_app))
                    .resource("/api/apps/{slug:[0-9a-zA-Z]+}/compile", |r| r.method(Method::POST).with2(compile_app_source))
                    .resource("/api/apps/{slug:[0-9a-zA-Z]+}/github_auth", |r| r.method(Method::POST).with2(github_auth))
                    .resource("/api/auth/github/complete", |r| r.method(Method::GET).with2(github_auth_complete));
            }

            app = app
                .resource("/favicon.ico", |r| r.f(|_| HttpResponse::NotFound()))

                .handler("/resources/app", fs::StaticFiles::new("../examples/app/")
                    .default_handler(|_| HttpResponse::NotFound()))

                .resource("/", |r| r.method(Method::GET).a(render_route))

                .handler("/", fs::StaticFiles::new("../examples/app/playground/")
                    .default_handler(|_| HttpResponse::NotFound()));

            app
        })
        .bind("0.0.0.0:3000").expect("cannot bind to 0.0.0.0:3000")
        .shutdown_timeout(0)
        .start();

    println!("Starting http server: 0.0.0.0:3000");
    let _ = sys.run();
}
