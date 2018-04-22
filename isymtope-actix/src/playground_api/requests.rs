
#[derive(Debug, Serialize, Deserialize)]
pub struct CreateExampleRequest {
    pub template_name: String,
}

#[derive(Debug, Serialize)]
pub struct CreateExampleResponse {
    pub uuid: String,
    pub slug: String,
    pub base_app_uuid: String,
    pub base_app_slug: String,
    pub template_name: String,
    pub path: String,
    pub redirect: String,
    pub iframe_base: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct GetAppRequest {
    pub template_name: String,
}

#[derive(Debug, Serialize)]
pub struct GetAppRestResponse {
    pub uuid: String,
    pub slug: String,
    pub base_app_uuid: Option<String>,
    pub base_app_slug: Option<String>,
    pub static_template: Option<String>,
    pub pathname: String,
    pub iframe_base: String,
}
