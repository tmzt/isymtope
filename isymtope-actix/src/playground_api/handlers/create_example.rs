use actix::*;
use actix_web::*;
use super::*;
use isymtope_generate::*;

#[derive(Debug)]
pub struct CreateExample {
    pub base_app_uuid: String,
    pub template_name: String,
}

impl Message for CreateExample {
    type Result = Result<CreateAppResponse>;
}

#[derive(Debug, Message)]
pub struct CreateAppResponse {
    pub app: StoredApp,
    pub template: TemplateData,
}

impl Handler<CreateExample> for PlaygroundApi {
    type Result = MessageResult<CreateExample>;

    fn handle(&mut self, msg: CreateExample, _: &mut Self::Context) -> Self::Result {
        let template_name = msg.template_name.to_owned();
        let res = self.get_or_load_template(&template_name)
            .map_err(Error::from)
            .and_then(|template| {
                self.create_app(AppBase::Template(msg.template_name.to_owned()))
                    .map_err(Error::from)
                    .map(|app| CreateAppResponse { app: app, template: template })
            });

        MessageResult(res)
    }
}
