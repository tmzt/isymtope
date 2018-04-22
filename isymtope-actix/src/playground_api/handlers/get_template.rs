use futures::Future;

use actix::*;
use actix::prelude::*;
use isymtope_generate::*;
use compiler::*;
use super::*;

#[derive(Debug)]
pub struct GetTemplate {
    pub slug: String,
}

#[derive(Debug, Message, Clone)]
pub struct GetTemplateResponse {
    pub uuid: String,
    pub base_app_uuid: Option<String>,
    pub static_template: Option<String>,
}

impl Message for GetTemplate {
    type Result = Result<GetTemplateResponse, PlaygroundApiError>;
}

impl Handler<GetTemplate> for PlaygroundApi {
    type Result = MessageResult<GetTemplate>;

    fn handle(&mut self, msg: GetTemplate, _: &mut Context<PlaygroundApi>) -> Self::Result {
        let slug = &msg.slug;

        // TODO: Make this lookup and cache
        let entry = self.slug_cache.get(slug).unwrap();

        let result = GetTemplateResponse {
            uuid: entry.uuid.to_owned(),
            base_app_uuid: entry.base_app_uuid.as_ref().map(|s| s.to_owned()),
            static_template: entry.static_template.as_ref().map(|s| s.to_owned()),
        };

        MessageResult(Ok(result))
    }
}
