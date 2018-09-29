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
extern crate isymtope_parser;
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
use actix_web::middleware::{Middleware, Started, Response};
use actix_web::middleware::session::{SessionStorage, RequestSession, CookieSessionBackend};
use actix_web::server;
use actix_web::fs::{NamedFile, DefaultConfig};
use futures::Future;
use futures::future::ok;

#[cfg(feature="playground_api")]
use self::playground_api::*;

pub use self::compiler::*;

lazy_static! {
    // pub static ref STATIC_APP_ROOT: Box<PathBuf> = Box::new(env::var_os("STATIC_APP_ROOT").expect("STATIC_APP_ROOT must be provided").into());
    // pub static ref STATIC_RESOURCE_ROOT: Box<PathBuf> = Box::new(env::var_os("STATIC_RESOURCE_ROOT").expect("STATIC_RESOURCE_ROOT must be provided").into());
    pub static ref APP_NAME: String = env::var_os("APP_NAME").expect("APP_NAME must be provided").to_string_lossy().to_string();
    pub static ref DEV_PORT: Option<u16> = env::var_os("DEV_PORT").and_then(|p| FromStr::from_str(&p.to_string_lossy().to_string()).ok());

    pub static ref EXAMPLES_DIR: PathBuf = PathBuf::from("./examples/app");
}

#[cfg(feature="playground_api")]
lazy_static! {
    pub static ref PLAYGROUND_APP_DNS_SUFFIX: String = env::var_os("PLAYGROUND_APP_DNS_SUFFIX").expect("PLAYGROUND_APP_DNS_SUFFIX must be provided").to_string_lossy().to_string();
    pub static ref PLAYGROUND_APP_HOST: Regex = Regex::new(&format!(r"(?P<slug>[a-zA-Z0-9]+).f.r{}(?P<port>:\d*)", &*PLAYGROUND_APP_DNS_SUFFIX)).unwrap();
}

// fn render_example_app_route(state: &AppState, static_template: &str, base_url: &str, route: &str) -> impl Responder { //FutureResponse<HttpResponse> {
//     let msg = RenderExampleAppRoute { app_name: static_template.to_owned(), route:  route.to_owned(), base_url: base_url.to_owned() };
//     state.compiler.send(msg)
//         .from_err()
//         .and_then(move |res| {
//             if let Ok(body) = res {
//                 return ok(HttpResponse::Ok()
//                     .content_type("text/html")
//                     .body(body))
//             };
//             ok(HttpResponse::InternalServerError().finish().into())
//         })
//         .responder()
// }

fn render_route(req: &HttpRequest<AppState>) -> impl Responder { //AsyncResult<HttpResponse> {
    eprintln!("[isymtope-actix] render_route: {:?}", req);

    let playground_session_id = ::uuid::Uuid::new_v4().to_string();
    req.session().set("playground_session_id", playground_session_id).unwrap();

    let state = req.state();
    let route = req.uri().path().to_owned();

    let scheme = req.connection_info().scheme().to_owned();
    let host = req.connection_info().host().to_owned();
    let base_url = format!("{}://{}/", scheme, host);

    // #[cfg(feature = "playground_api")]
    // {
    //     if let Some(captures) = PLAYGROUND_APP_HOST.captures(&host) {
    //         let api = state.api.clone();
    //         let compiler = state.compiler.clone();
    //         let slug = captures.name("slug").map(|m| m.as_str()).unwrap();

    //         return state.api.send(CompileTemplate { api: api, compiler: compiler, base_url: base_url.to_owned(), route: route.clone(), slug: slug.to_owned() })
    //             .from_err()
    //             .and_then(move |res| {
    //                 if let Ok(res) = res {
    //                     return ok(HttpResponse::Ok()
    //                         .content_type("text/html")
    //                         .body(res.body));
    //                 };
    //                 ok(HttpResponse::InternalServerError().finish().into())
    //             })
    //             .responder();
    //     };
    // }

    // let slug = req.match_info().get("slug");
    // render_example_app_route(state, &*APP_NAME, &base_url, &path)

    let app_name: String = format!("{}", &*APP_NAME);
    let msg = RenderExampleAppRoute { app_name: app_name, route:  route.to_owned(), base_url: base_url.to_owned() };
    state.compiler.send(msg)
        .map_err(Error::from)
        .and_then(move |res| {
            if let Ok(body) = res {
                return ok(HttpResponse::Ok()
                    .content_type("text/html")
                    .body(body))
            };
            ok(HttpResponse::InternalServerError().finish().into())
        })
        .responder()
}

pub struct AppState {
    compiler: Addr<Compiler>,
    api: Addr<PlaygroundApi>,
}

fn render_500<S>(_: &mut HttpRequest<S>, resp: HttpResponse) -> Result<Response> {
    Ok(Response::Done(resp.into_builder()
        .body("Error occurred parsing page template")))
}

fn serve_static_file(req: &HttpRequest<AppState>) ->Result<NamedFile> {
    let route = req.uri().path().to_owned();
    // let route: PathBuf = req.match_info().query("route")?;
    let path = &*EXAMPLES_DIR
        .join(&*APP_NAME)
        .join(route);

    Ok(NamedFile::open(path)?)
}

// fn serve_route(req: HttpRequest<AppState>) -> impl Responder {
//     let route = req.uri().path().to_owned();
//     // let route: PathBuf = req.match_info().query("route")?;
//     let path = &*EXAMPLES_DIR
//         .join(&*APP_NAME)
//         .join(route);

//     let config = DefaultConfig::default();
//     if let Ok(file) = NamedFile::open_with_config(path, config) {
//         // return Box::new(ok(file));
//         return file;
//     }

//     render_route(req)
// }

#[derive(Default)]
struct ExampleStaticFiles;
impl<S: 'static> Middleware<S> for ExampleStaticFiles {
    fn start(&self, req: &HttpRequest<S>) -> Result<Started> {
        let route = req.uri().path().to_owned();
        // let route: PathBuf = req.match_info().query("route")?;
        let path = &*EXAMPLES_DIR
            .join(&*APP_NAME)
            .join(route);

        if path.exists() {
            let file = NamedFile::open(path);
            if let Ok(file) = file {
                let resp = file.respond_to(req)?;

                return Ok(Started::Response(resp));
            }
        }

        // Default
        Ok(Started::Done)
    }
}

fn main() {
    openssl_probe::init_ssl_cert_env_vars();
    dotenv().ok();
    // env::set_var("RUST_LOG", "actix_web=debug");
    // env::set_var("RUST_BACKTRACE", "1");host:3000/api/auth/github/complete?code=e78f8f0ecebf16b21c65&state=7ee77eb9-974c-448f-af6b-
    env_logger::init();
    let sys = actix::System::new("server");

    let compiler = Arbiter::start(|_| Compiler::default());
    let api = Arbiter::start(|_| PlaygroundApi::default());

    server::new(
        move || {
            let state = AppState { compiler: compiler.clone(), api: api.clone() };

            let mut app = App::with_state(state)
                // .middleware(ErrorHandlers::new()
                //     .handler(http::StatusCode::INTERNAL_SERVER_ERROR, render_500)
                // )
                .middleware(SessionStorage::new(
                    CookieSessionBackend::signed(&[0; 32])
                        .secure(false)
                ))
                .middleware(ExampleStaticFiles::default());

            #[cfg(feature="playground_api")]
            {
                app = app
                    // .resource("/r/{slug:[0-9a-zA-Z]+}", |r| r.method(Method::GET).a(render_route))
                    .resource("/api/create_example", |r| r.method(Method::POST).with(create_example))
                    .resource("/api/examples", |r| r.method(Method::GET).with(get_example_index))
                    .resource("/api/apps/{slug:[0-9a-zA-Z]+}", |r| r.method(Method::GET).with(get_app))
                    // .resource("/api/apps/{slug:[0-9a-zA-Z]+}/compile", |r| r.method(Method::POST).a(compile_app_source))
                    // .resource("/api/apps/{slug:[0-9a-zA-Z]+}/github_auth", |r| r.method(Method::POST).a(github_auth))
                    // .resource("/api/auth/github/complete", |r| r.method(Method::GET).a(github_auth_complete));
            }

            app = app
                .resource("/favicon.ico", |r| r.f(|_| HttpResponse::NotFound()))

                .resource("/{route:.*}", |r| r.method(Method::GET).f(render_route));
                
                // .default_resource(|r| r.method(Method::GET).a(serve_route));

                // .handler("/resources/app", fs::StaticFiles::new("../examples/app/")
                //     .default_handler(|_| HttpResponse::NotFound()))

                // .handler("/", fs::StaticFiles::new(static_base_path)
                //     .default_handler(|rq: HttpRequest<_>| rq.resource("/{route:.*}", |r| r.method(Method::GET).a(render_route))));

                // .resource("/{route:.*}", |r| r.method(Method::GET).a(render_route));

                // .handler("/", fs::StaticFiles::new("../examples/app/playground/")
                //     .default_handler(|_| HttpResponse::NotFound()));

            app
        })
        .bind("0.0.0.0:3000").expect("cannot bind to 0.0.0.0:3000")
        .shutdown_timeout(0)
        .start();

    println!("Starting http server: 0.0.0.0:3000");
    let _ = sys.run();
}
