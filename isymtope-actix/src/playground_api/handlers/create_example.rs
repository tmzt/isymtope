use uuid::Uuid;

use actix::*;
use isymtope_ast_common::*;
use isymtope_generate::*;
use super::*;

#[derive(Debug)]
pub struct CreateExample {
    pub base_app_uuid: String,
    pub template_name: String,
}

impl Message for CreateExample {
    type Result = IsymtopeGenerateResult<Example>;
}

#[derive(Debug, Message)]
pub struct Example {
    pub uuid: String,
    pub slug: String,
    pub base_app_uuid: String,
    pub base_app_slug: String,
    pub template_name: String,
}

impl Handler<CreateExample> for PlaygroundApi {
    type Result = MessageResult<CreateExample>;

    fn handle(&mut self, msg: CreateExample, _: &mut Context<PlaygroundApi>) -> Self::Result {
        let uuid = format!("{}", Uuid::new_v4());
        let slug = allocate_element_key();
        let template_name = msg.template_name.to_owned();

        let base_app_uuid = "abcd".to_owned();
        let base_app_slug = "4wxtz".to_owned();

        let entry  = CachedStoredApp {
            uuid: uuid.to_string(),
            slug: slug.clone(),
            base_app_uuid: Some(base_app_uuid.clone()),
            base_app_slug: Some(base_app_slug.clone()),
            static_template: Some(template_name.to_owned()),
        };

        let result = Example {
            uuid: uuid,
            slug: slug.clone(),
            base_app_uuid: base_app_uuid,
            base_app_slug: base_app_slug,
            template_name: template_name,
        };

        self.slug_cache.insert(slug, entry);
        MessageResult(Ok(result))
    }
}
