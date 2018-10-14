use super::*;

#[derive(Debug, Serialize, Deserialize)]
pub struct CreateExampleRequest {
    pub template_name: String,
}

// #[derive(Debug, Serialize)]
// pub struct CreateExampleResponse {
//     // pub uuid: String,
//     // pub slug: String,
//     // pub base_app_uuid: String,
//     // pub base_app_slug: String,
//     // pub template_name: String,
//     pub app: StoredApp,
//     pub path: String,
//     pub redirect: String,
//     pub iframe_base: String,
// }

#[derive(Debug, Message, Clone, Serialize)]
pub struct RenderableApp {
    pub app: StoredApp,
    pub template: TemplateData,
    pub path: String,
    pub redirect: String,
    pub iframe_base: String,
}

impl RenderableApp {
    pub fn with_app(req: &HttpRequest<AppState>, app: StoredApp, template: TemplateData) -> Self {
            let scheme = req.connection_info().scheme().to_owned();
            let host = req.connection_info().host().to_owned();
            // let port = req.uri().port().map_or(Default::default(), |p| format!(":{}", p));

            let port = &DEV_PORT.map(|p| format!(":{}", p)).unwrap_or_default();
            let iframe_base = format!("{}://{}.f.r{}{}/", scheme, app.slug, &*PLAYGROUND_APP_DNS_SUFFIX, port);

            let path = format!("/r/{}", app.slug);
            let redirect = format!("{}://{}/r/{}", scheme, host, app.slug);
            let iframe_base = format!("{}://{}.f.r{}{}/", scheme, app.slug, &*PLAYGROUND_APP_DNS_SUFFIX, port);

            RenderableApp {
                app: app,
                template: template,
                path: path,
                redirect: redirect,
                iframe_base: iframe_base,
            }
    }
}

#[derive(Debug, Serialize)]
pub struct GetExampleIndexItem {
    pub slug: String,
    pub title: String,
}

#[derive(Debug, Serialize)]
pub struct GetExampleIndexResponse {
    pub index: Vec<GetExampleIndexItem>,
    pub default_slug: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct GetAppRequest {
    pub template_name: String,
}

// #[derive(Debug, Serialize)]
// pub struct GetAppRestResponse {
//     pub uuid: String,
//     pub slug: String,
//     pub base_app_uuid: Option<String>,
//     pub base_app_slug: Option<String>,
//     pub static_template: Option<String>,
//     pub pathname: String,
//     pub iframe_base: String,
//     pub files: Vec<AppMetadataFile>,
// }

#[derive(Debug, Serialize)]
pub struct GithubAuthRestResponse {
    pub state: String,
    pub auth_url: String,
    pub request_id: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct GithubOAuthResponse {
    pub access_token: String,
    pub scope: String,
    pub token_type: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct GithubAuthComplete {
    pub code: String,
    pub state: String,
}
