use std::collections::HashMap;
use std::env;
use std::path::PathBuf;

use actix::prelude::*;

use isymtope_generate::*;
// use isymtope_actix::STATIC_APP_ROOT;
use super::*;

lazy_static! {
    pub static ref STATIC_APP_ROOT: Box<PathBuf> = Box::new(env::var_os("STATIC_APP_ROOT").expect("STATIC_APP_ROOT must be provided").into());
}

// #[derive(Debug)]
// struct CompilerActorMsg(pub CompilerRequestMsg);
// #[derive(Debug)]
// struct CompilerActorResult(pub CompilerResponseMsg);

#[derive(Debug)]
pub struct RenderExampleAppRoute {
    pub app_name: String,
    pub route: String,
    pub base_url: String
}

impl Message for RenderExampleAppRoute {
    type Result = IsymtopeGenerateResult<String>;
}

impl Handler<RenderExampleAppRoute> for Compiler {
    type Result = MessageResult<RenderExampleAppRoute>;

    fn handle(&mut self, msg: RenderExampleAppRoute, _: &mut Context<Self>) -> Self::Result {
        let app_name = &msg.app_name;
        let ism_path = "/app.ism";
        let route = &msg.route;
        let base_url = &msg.base_url;

        let template_key = format!("[appName={:?}, ismPath={}, baseUrl={}]", app_name, ism_path, base_url);
        debug!(
            "[compiler] get or creating context for app with key ({})",
            template_key);

        let app_dir = &*STATIC_APP_ROOT.join(app_name);

        let template_context = self.template_file_cache
            .entry(template_key)
            .or_insert_with(|| DefaultTemplateContext::create(&app_dir, &ism_path).unwrap());

        let req = TemplateRequestMsg::RenderAppRoute(
            base_url.to_owned(),
            app_name.to_owned(),
            ism_path.to_owned(),
            route.to_owned(),
        );

        let result = template_context.handle_msg(req)
            .map(|response| {
                    let TemplateResponseMsg::RenderComplete(result) = response;
                    let body = result.into_inner();

                    body
                });

        MessageResult(result)
    }
}
