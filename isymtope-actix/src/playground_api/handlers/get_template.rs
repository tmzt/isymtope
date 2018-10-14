use futures::Future;

use actix::*;
use super::*;

#[derive(Debug)]
pub struct GetTemplate {
    pub template_name: String,
}

#[derive(Debug, Message, Clone)]
pub struct GetTemplateResponse {
    pub template: TemplateData,
}

impl Message for GetTemplate {
    type Result = Result<GetTemplateResponse, Error>;
}

impl Handler<GetTemplate> for PlaygroundApi {
    type Result = MessageResult<GetTemplate>;

    fn handle(&mut self, msg: GetTemplate, _: &mut Self::Context) -> Self::Result {
        // let slug = &msg.slug;

        // TODO: Make this lookup and cache
        // let entry = self.slug_cache.get(slug).unwrap();
        let res = self.get_or_load_template(&msg.template_name)
            .map_err(Error::from)
            .map(|template| GetTemplateResponse { template: template });

        MessageResult(res)
    }
}
