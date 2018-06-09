mod handlers;
mod model;
mod requests;

use std::io::Read;
use std::str;
use actix::*;
use actix_web::*;
use actix_web::fs::NamedFile;
use bytes::*;
use futures::{Future, IntoFuture};
use futures::future::result;

pub use self::handlers::*;
pub use self::model::*;
pub use self::requests::*;
use super::*;

lazy_static! {
    pub static ref GITHUB_CLIENT_ID: String = env::var("GITHUB_CLIENT_ID").expect("GITHUB_CLIENT_ID must provided");
    // pub static ref GITHUB_CLIENT_SECRET: String = env::var("GITHUB_CLIENT_SECRET").expect("GITHUB_CLIENT_SECRET must provided");
}

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

pub fn get_example_index(req: HttpRequest<AppState>) -> FutureResponse<HttpResponse> {
    let items = vec![
        GetExampleIndexItem { slug: "todomvc".to_owned(), title: "TodoMvc".to_owned() }
    ];

    let json = GetExampleIndexResponse {
        index: items,
        defaultSlug: "todomvc".to_owned(),
    };

    result(Ok(HttpResponse::Ok().json(json)))
        .responder()
}

pub fn get_app_meta(slug: &str, app: &GetAppResponse, iframe_base: &str) -> Result<GetAppRestResponse, Error> {
    let static_template = app.static_template.as_ref().map(|s| s.to_owned());
    let base_app_uuid = app.base_app_uuid.as_ref().map(|s| s.to_owned());
    let base_app_slug = app.base_app_slug.as_ref().map(|s| s.to_owned());
    let pathname = format!("/r/{}", slug);

    let static_template = static_template.unwrap();
    let meta_path = format!("../examples/app/{}/app.json", static_template);

    let mut file = NamedFile::open(&meta_path)?;
    let mut data = String::new();
    file.read_to_string(&mut data)?;

    let meta: AppMetadata = serde_json::from_str(&data)?;

    let files: Vec<AppMetadataFile> = meta.files.iter().cloned().collect();

    let json = GetAppRestResponse {
        uuid: app.uuid.to_owned(),
        slug: slug.to_owned(),
        base_app_uuid: base_app_uuid,
        base_app_slug: base_app_slug,
        static_template: Some(static_template),
        pathname: pathname,
        iframe_base: iframe_base.to_owned(),
        files: files
    };

    Ok(json)
}

pub fn get_app(req: HttpRequest<AppState>, slug: Path<(String,)>) -> FutureResponse<HttpResponse> {
    let slug = slug.0.to_owned();

    req.state().api.send(GetApp {
            slug: slug.to_owned(),
    })
    .from_err()
    .and_then(move |res| {
        match res {
            Ok(Some(app)) => {
                let scheme = req.connection_info().scheme();
                let host = req.connection_info().host();
                // let port = req.uri().port().map_or(Default::default(), |p| format!(":{}", p));

                let port = &DEV_PORT.map(|p| format!(":{}", p)).unwrap_or_default();
                let iframe_base = format!("{}://{}.f.r{}{}/", scheme, slug, &*PLAYGROUND_APP_DNS_SUFFIX, port);

                get_app_meta(&slug, &app, &iframe_base)
                    .and_then(|json| Ok(HttpResponse::Ok().json(json)))
            },
            Ok(None) => Ok(HttpResponse::NotFound().into()),
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

pub fn github_auth(mut req: HttpRequest<AppState>, path: Path<(String,)>) -> FutureResponse<HttpResponse> {
        let state = ::uuid::Uuid::new_v4().to_string();
        let request_id = ::uuid::Uuid::new_v4().to_string();
        eprintln!("State: {}", state);
        eprintln!("RequestId: {}", request_id);

        req.session().set("auth_state", state.to_owned()).unwrap();
        req.session().set("auth_request_id", request_id.to_owned()).unwrap();

        // req.session().set("auth_request_id", path.1.to_owned()).unwrap();

        let scheme = req.connection_info().scheme();
        let host = req.connection_info().host();
        let port = &DEV_PORT.map(|p| format!(":{}", p)).unwrap_or_default();

        let auth_url = "https://github.com/login/oauth/authorize";
        let token_url = "https://github.com/login/oauth/access_token";

        // let redirect_url = format!("{}://{}/api/auth/github/complete/apps/{}/requests/{}", scheme, host, path.0, path.1);
        // let popup_url = format!("{}?client_id={}&redirect_uri={}&scope=gist&state={}", auth_url, &*GITHUB_CLIENT_ID, redirect_url, state);
        let popup_url = format!("{}?client_id={}&scope=gist&state={}", auth_url, &*GITHUB_CLIENT_ID, state);

        let json = GithubAuthRestResponse {
            state: state,
            auth_url: popup_url,
            request_id: request_id
        };

        result(Ok(HttpResponse::Ok().json(json)))
            .responder()

        // let redirect_url = format!("{}://{}{}/api/apps/{}/github_auth/{}/complete", scheme, host, port, path.0, path.1);
        // let redirect_url = format!("{}://{}{}/api/auth/github/complete", scheme, host, port);
        // let body = format!("response_type=redirect&client_id={}&client_secret={}&redirect_url={}", &*GITHUB_CLIENT_ID, &*GITHUB_CLIENT_SECRET, redirect_url);
        // let url = format!("{}?response_type=redirect&client_id={}&redirect_url={}", &*GITHUB_CLIENT_ID, redirect_url);


        // client::ClientRequest::post(auth_url)
        //     .content_type("application/x-www-form-urlencoded")
        //     .header("accept", "application/json")
        //     .body(body)
        //     .unwrap()
        //     .send()
        //     .map_err(|err| {
        //         eprintln!("Github auth error: {:?}", err);
        //         Error::from(err)
        //     })
        //     .and_then(|resp| {
        //         // resp.json::<GithubOAuthResponse>()
        //         //     .from_err()
        //         //     .and_then(|msg| {
        //         //         eprintln!("Received from Github: {:?}", msg);

        //         //         let state = "".into();
        //         //         let auth_url = "".into();
        //         //         let json = GithubAuthRestResponse {
        //         //             state: state,
        //         //             auth_url: auth_url,
        //         //         };
        //         //         Ok(HttpResponse::Ok().json(json))
        //         //     })
        //     })
        //     .responder()
}

pub fn github_auth_complete(mut req: HttpRequest<AppState>, query: Query<GithubAuthComplete>) -> FutureResponse<HttpResponse> {
    let request_id = req.session().get::<String>("auth_request_id").ok().unwrap().unwrap_or_default();
    let auth_state = req.session().get::<String>("auth_state").ok().unwrap().unwrap_or_default();
    eprintln!("[auth complete] RequestId: {}", request_id);

    let state = &query.state;

    if state != &auth_state {
        return result(Ok(HttpResponse::InternalServerError()
            .content_type("text/html; charset=utf-8")
            .body(format!("<html><body><h1>Failed to match state in Github authentication, please retry.</h1><script>console.log(location); localStorage.setItem('{}', JSON.stringify({{ success: false, error: 'Failed to match state in Github authentication, please retry.' }}));</script></body></html>", request_id))
        )).responder();
    }

    let code = query.code.to_owned();
    req.session().set("auth_code", code).unwrap();

    result(Ok(HttpResponse::Ok()
        .content_type("text/html; charset=utf-8")
        .body(format!("<html><body><h1>Finishing authentication with Github.</h1><script>console.log(location); localStorage.setItem('{}', JSON.stringify({{ success: true, query: location.search }})); /*setTimeout(function() {{ window.close() }}, 10000)*/</script></body></html>", request_id))
    )).responder()
}
