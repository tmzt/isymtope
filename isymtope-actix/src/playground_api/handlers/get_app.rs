use futures::Future;

use actix::*;
use actix::prelude::*;
use isymtope_generate::*;
use compiler::*;
use super::*;

#[derive(Debug)]
pub struct GetApp {
    pub slug: String,
}

#[derive(Debug, Message, Clone)]
pub struct GetAppResponse {
    pub uuid: String,
    pub base_app_uuid: Option<String>,
    pub base_app_slug: Option<String>,
    pub static_template: Option<String>,
}

impl Message for GetApp {
    type Result = Result<Option<GetAppResponse>, PlaygroundApiError>;
}

impl Handler<GetApp> for PlaygroundApi {
    type Result = MessageResult<GetApp>;

    fn handle(&mut self, msg: GetApp, _: &mut Self::Context) -> Self::Result {
        let slug = &msg.slug;

        // TODO: Make this lookup and cache
        let result = self.slug_cache.get(slug).map(|entry|
            GetAppResponse {
                uuid: entry.uuid.to_owned(),
                base_app_uuid: entry.base_app_uuid.as_ref().map(|s| s.to_owned()),
                base_app_slug: entry.base_app_slug.as_ref().map(|s| s.to_owned()),
                static_template: entry.static_template.as_ref().map(|s| s.to_owned()),
            }
        );

        MessageResult(Ok(result))
    }
}
