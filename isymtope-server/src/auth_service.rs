use std::collections::HashMap;
use std::env;

use futures::{future, Future};
use hyper::header::Location;

use hyper::{Error as HyperError, Method, Request, Response, StatusCode};
use hyper::header::ContentType;
use hyper::mime;
use hyper::server::Service;

use url::Url;
use oauth2::Config;

use super::*;

lazy_static! {
    // pub static ref APP_ROUTE: Regex = Regex::new(r"app/(?P<app>[a-zA-Z0-9_-]+)(?P<path>/*(.*))").unwrap();
    pub static ref GITHUB_CLIENT_ID: String = env::var("GITHUB_CLIENT_ID").expect("GITHUB_CLIENT_ID must provided");
    pub static ref GITHUB_CLIENT_SECRET: String = env::var("GITHUB_CLIENT_SECRET").expect("GITHUB_CLIENT_SECRET must provided");
}

#[derive(Debug, Default)]
pub struct AuthService {}

impl AuthService {
    pub fn redirect(&self, req: &<Self as Service>::Request) -> <Self as Service>::Future {
        let auth_url = "https://github.com/login/oauth/authorize";
        let token_url = "https://github.com/login/oauth/access_token";

        let config = Config::new(&*GITHUB_CLIENT_ID.to_owned(), &*GITHUB_CLIENT_SECRET.to_owned(), auth_url, token_url)
            .add_scope("gist")
            // .set_redirect_url(format!("{}/auth/github", base_url))
            .set_state("1abc2");

        let authorize_url = config.authorize_url().to_string();

        // let response = Response::new()
        //     .with_status(StatusCode::Found)
        //     .with_header(Location::new(authorize_url));

        let response = Response::new()
            .with_header(ContentType(mime::APPLICATION_JSON))
            .with_body(format!("{{\"redirect\": \"{}\"}}", authorize_url));

        Box::new(future::ok(response))
    }
}

impl Service for AuthService {
    type Request = Request;
    type Response = Response;
    type Error = HyperError;
    type Future = Box<Future<Item = Response, Error = Self::Error>>;

    fn call(&self, req: Self::Request) -> Self::Future {
        let base_url = self.get_base_url(&req).unwrap();
        let main_url = format!("{}/", base_url);

        let query = req.query().unwrap();
        let qs: HashMap<_, _> = ::url::form_urlencoded::parse(query.as_bytes()).into_owned().collect();

        let code = qs.get("code");
        let state = qs.get("state");

        let response = Response::new()
            .with_status(StatusCode::Found)
            .with_header(Location::new(main_url.to_owned()));

        Box::new(future::ok(response))
    }
}
