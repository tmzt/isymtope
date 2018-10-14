
use actix::*;
use actix_web::*;
use super::*;

#[derive(Debug)]
pub struct GetApp {
    pub slug: String,
}

#[derive(Debug, Message, Clone, Serialize)]
pub struct GetAppResponse {
    pub app: StoredApp,
    pub template: TemplateData
}

// #[derive(Debug, Message, Clone)]
// pub struct GetAppResponse {
//     pub app: StoredApp
//     // pub uuid: String,
//     // pub base_app_uuid: Option<String>,
//     // pub base_app_slug: Option<String>,
//     // pub static_template: Option<String>,
// }

impl Message for GetApp {
    type Result = Result<GetAppResponse, Error>;
}

impl Handler<GetApp> for PlaygroundApi {
    type Result = MessageResult<GetApp>;

    fn handle(&mut self, msg: GetApp, _: &mut Self::Context) -> Self::Result {
        let slug = &msg.slug;
        let result = self.get_existing_app(slug)
            .map_err(Error::from)
            .and_then(|app| {
                let template_name = &app.template_name;
                self.get_or_load_template(template_name)
                    .map_err(Error::from)
                    .map(|template| GetAppResponse { app: app.to_owned(), template: template.to_owned() })
                    // .map(|template| RenderableApp::with_app(req, app.to_owned(), template.to_owned()))
            });

        MessageResult(result)
    }
}
