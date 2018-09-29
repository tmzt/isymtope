use std::collections::hash_map::{HashMap, Entry};
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

fn render(template_file_cache: &mut HashMap<String, DefaultTemplateContext>, base_url: &str, app_name: &str, ism_path: &str, route: &str) -> IsymtopeGenerateResult<String> {
    let template_key = format!("[appName={:?}, ismPath={}, baseUrl={}]", app_name, ism_path, base_url);
    debug!(
        "[compiler] get or creating context for app with key ({})",
        template_key);

    let app_dir = &*STATIC_APP_ROOT.join(app_name);

    let template_context = match template_file_cache.entry(template_key) {
        Entry::Occupied(v) => v.into_mut(),
        Entry::Vacant(v) => v.insert(DefaultTemplateContext::create(&app_dir, ism_path)?)
    };

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

    result
}

impl Handler<RenderExampleAppRoute> for Compiler {
    type Result = MessageResult<RenderExampleAppRoute>;

    fn handle(&mut self, msg: RenderExampleAppRoute, _: &mut Self::Context) -> Self::Result {
        let base_url = &msg.base_url;
        let app_name = &msg.app_name;
        let ism_path = "/app.ism";
        let route = &msg.route;

        let result = render(&mut self.template_file_cache, base_url, app_name, ism_path, route);
        MessageResult(result)
    }
}
