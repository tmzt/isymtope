use std::collections::HashMap;
use std::path::{Path, PathBuf};
use std::sync::Mutex;
use std::str;

#[cfg(feature = "session_time")]
use time::Duration;

use isymtope_generate::*;
use super::*;

thread_local! {
    pub static APP_CACHE: Mutex<HashMap<String, DefaultAppContext>> = Default::default();
}

pub trait AppContext {
    fn handle_msg(&mut self, msg: AppRequestMsg) -> IsymtopeServerResult<AppResponseMsg>;
}

#[derive(Debug)]
pub struct DefaultAppContext {
    app_root: PathBuf,
    template_context: DefaultTemplateContext,
    sessions: MemorySessions,
}

impl DefaultAppContext {
    pub fn new(app_root: &Path, template_context: DefaultTemplateContext) -> Self {
        DefaultAppContext {
            app_root: app_root.to_owned(),
            template_context: template_context,
            sessions: Default::default(),
        }
    }

    pub fn create(
        app_root: &Path,
        template_path: &str
    ) -> IsymtopeServerResult<DefaultAppContext> {
        eprintln!(
            "[app context] creating context for app root [{:?}] with main template path [{}]",
            app_root, template_path
        );

        let template_context = DefaultTemplateContext::create(app_root, template_path).unwrap();
        let app_context = DefaultAppContext::new(app_root, template_context);

        Ok(app_context)
    }
}

impl AppContext for DefaultAppContext {
    fn handle_msg(&mut self, msg: AppRequestMsg) -> IsymtopeServerResult<AppResponseMsg> {
        match msg {
            AppRequestMsg::TemplateRequest(template_req_msg) => {
                let response_msg = self.template_context.handle_msg(template_req_msg)?;
                Ok(AppResponseMsg::TemplateResponse(response_msg))
            }
        }
    }
}
