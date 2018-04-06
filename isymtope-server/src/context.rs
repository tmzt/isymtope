use std::collections::hash_map::Entry;
use std::path::{Path, PathBuf};

use isymtope_generate::*;
use super::*;

pub trait ServerContext {
    fn handle_msg(&mut self, msg: Msg) -> IsymtopeServerResult<ResponseMsg>;
}

#[derive(Debug)]
pub struct DefaultServerContext {
    app_dir: PathBuf,
    srs: DefaultSecureRandomStringGenerator,
    #[cfg(feature = "cookies")]
    cookies: Cookies,
}

impl DefaultServerContext {
    #[cfg(not(feature = "cookies"))]
    pub fn new(app_dir: &Path) -> Self {
        DefaultServerContext {
            app_dir: app_dir.to_owned(),
            srs: Default::default(),
        }
    }

    #[cfg(feature = "cookies")]
    pub fn new(app_dir: &Path) -> Self {
        DefaultServerContext {
            app_dir: app_dir.to_owned(),
            srs: Default::default(),
            cookies: Default::default(),
        }
    }
}

#[derive(Debug)]
pub enum Msg {
    RenderAppRoute(String, String, String, String),
}

#[derive(Debug)]
pub enum ResponseMsg {
    RenderComplete(RenderResponse),
}

impl ServerContext for DefaultServerContext {
    fn handle_msg(&mut self, msg: Msg) -> IsymtopeServerResult<ResponseMsg> {
        match msg {
            Msg::RenderAppRoute(ref base_url, ref app_name, ref template_path, ref path) => {
                // let template_path = if path == "/" { "/app.ism".to_owned() } else { path.to_owned() };

                // let trimmed_path = path.trim_left_matches('/').to_owned();
                // let template_path = app_dir.join(app_name).join(trimmed_path);

                let app_root = &*APP_DIR.join(app_name);

                let app_key = format!("[appName={}, templatePath={}]", app_name, template_path);
                eprintln!(
                    "[server context] get or creating context for app with key ({})",
                    app_key
                );

                // let route_key = format!("[appName={}, templatePath={}]/{}", app_name, template_path, path);
                // eprintln!("[server context] get or creating context for app with key ({})", app_key);

                let app_response = APP_CACHE.with(|cache| {
                    let mut cache = cache.borrow_mut();

                    let app_context = match cache.entry(app_key.clone()) {
                        Entry::Occupied(e) => e.into_mut(),
                        Entry::Vacant(v) => {
                            v.insert(DefaultAppContext::create(&app_root, template_path)?)
                        }
                    };

                    // let app_context = cache.entry(app_key.clone()).or_insert_with(|| {
                    //     DefaultAppContext::create(&app_root, template_path).unwrap()
                    // });

                    let template_req_msg = TemplateRequestMsg::RenderAppRoute(
                        base_url.to_owned(),
                        app_name.to_owned(),
                        template_path.to_owned(),
                        path.to_owned(),
                    );
                    let app_req_msg = AppRequestMsg::TemplateRequest(template_req_msg);
                    app_context.handle_msg(app_req_msg)
                })?;

                let AppResponseMsg::TemplateResponse(template_response) = app_response;
                let TemplateResponseMsg::RenderComplete(render_response) = template_response;

                Ok(ResponseMsg::RenderComplete(render_response))
            }
        }
    }
}
