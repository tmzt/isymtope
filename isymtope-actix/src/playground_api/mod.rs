mod handlers;
mod model;
mod requests;

use std::str;
use actix::*;
use actix_web::*;
use bytes::*;
use futures::{Future, IntoFuture};

pub use self::handlers::*;
pub use self::model::*;
pub use self::requests::*;
use super::*;

// pub struct PlaygroundApiState {
//     pub api: Addr<Syn, PlaygroundApi>
// }

pub fn create_example(req: HttpRequest<AppState>, body: Json<CreateExampleRequest>) -> FutureResponse<HttpResponse> {
    let template_name = body.template_name.to_owned();
    req.state().api.send(CreateExample {
            base_app_uuid: "abcd".to_owned(),
            template_name: template_name,
    })
    .from_err()
    .and_then(move |res| {
        match res {
            Ok(example) => {
                let scheme = req.connection_info().scheme();
                let host = req.connection_info().host();
                // let port = req.uri().port().map_or(Default::default(), |p| format!(":{}", p));
                let port = &DEV_PORT.map(|p| format!(":{}", p)).unwrap_or_default();
                let path = format!("/r/{}", example.slug);
                let redirect = format!("{}://{}/r/{}", scheme, host, example.slug);
                let iframe_base = format!("{}://{}.f.r{}{}/", scheme, example.slug, &*PLAYGROUND_APP_DNS_SUFFIX, port);

                let json = CreateExampleResponse {
                    uuid: example.uuid.to_owned(),
                    slug: example.slug.to_owned(),
                    base_app_uuid: example.base_app_uuid.to_owned(),
                    base_app_slug: example.base_app_slug.to_owned(),
                    template_name: example.template_name.to_owned(),
                    path: path,
                    redirect: redirect.to_owned(),
                    iframe_base: iframe_base.to_owned(),
                };
                Ok(HttpResponse::Ok().json(json))
            },
            _ => Ok(HttpResponse::InternalServerError().into())
        }
    }).responder()
}

pub fn get_app(req: HttpRequest<AppState>, slug: Path<(String,)>) -> FutureResponse<HttpResponse> {
    let slug = slug.0.to_owned();

    req.state().api.send(GetApp {
            slug: slug.to_owned(),
    })
    .from_err()
    .and_then(move |res| {
        match res {
            Ok(app) => {
                let scheme = req.connection_info().scheme();
                let host = req.connection_info().host();
                // let port = req.uri().port().map_or(Default::default(), |p| format!(":{}", p));
                let port = &DEV_PORT.map(|p| format!(":{}", p)).unwrap_or_default();
                let pathname = format!("/r/{}", slug);
                let iframe_base = format!("{}://{}.f.r{}{}/", scheme, slug, &*PLAYGROUND_APP_DNS_SUFFIX, port);

                let static_template = app.static_template.as_ref().map(|s| s.to_owned());
                let base_app_uuid = app.base_app_uuid.as_ref().map(|s| s.to_owned());
                let base_app_slug = app.base_app_slug.as_ref().map(|s| s.to_owned());

                let json = GetAppRestResponse {
                    uuid: app.uuid.to_owned(),
                    slug: slug,
                    base_app_uuid: base_app_uuid,
                    base_app_slug: base_app_slug,
                    static_template: static_template,
                    pathname: pathname,
                    iframe_base: iframe_base.to_owned(),
                };
                Ok(HttpResponse::Ok().json(json))
            },
            _ => Ok(HttpResponse::InternalServerError().into())
        }
    }).responder()
}

pub fn compile_app_source(req: HttpRequest<AppState>, slug: Path<(String,)>) -> FutureResponse<HttpResponse> {
    let api = req.state().api.clone();
    let compiler = req.state().compiler.clone();
    let slug = slug.0.to_owned();
    let route = "/".to_owned();

    let scheme = req.connection_info().scheme().to_owned();
    let port = &DEV_PORT.map(|p| format!(":{}", p)).unwrap_or_default();
    let pathname = format!("/r/{}", slug);
    let iframe_base = format!("{}://{}.f.r{}{}/", scheme, slug, &*PLAYGROUND_APP_DNS_SUFFIX, port);

    let api_ = req.state().api.clone();

    req.body()
       .limit(4096)
       .and_then(|bytes: Bytes| {
           let bytes: &[u8] = bytes.as_ref();
           String::from_utf8(bytes.to_vec()).map_err(|err| ::std::io::Error::new(::std::io::ErrorKind::InvalidInput, "Unable to parse payload from bytes").into()).into_future()
       })
       .from_err()
       .and_then(move |source|
                api_.send(CompileSource { api: api, compiler: compiler, source: source, base_url: iframe_base.clone(), route: route, slug: slug })
                    .from_err()
                    .and_then(|res| match res {
                        Ok(res) => {
                            Ok(HttpResponse::Ok()
                                .content_type("text/html")
                                .body(res.body))
                        },
                        _ => Ok(HttpResponse::InternalServerError().into())
                    })
       )
       .responder()
}
