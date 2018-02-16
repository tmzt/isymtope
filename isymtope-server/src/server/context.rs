use std::fmt::{self, Debug, Formatter};
use std::sync::Mutex;
use std::str;

#[cfg(feature = "session_time")]
use time::Duration;

use futures;
use futures::future::{self, Future, FutureResult};

use isymtope_ast_common::*;
use isymtope_build::*;
use server::*;

thread_local! {
    pub static APP_CACHE: Mutex<HashMap<String, DefaultAppContext>> = Default::default();
}

pub trait ServerContext {
    fn handle_msg(&mut self, msg: Msg) -> IsymtopeServerResult<ResponseMsg>;
}

pub trait AppContext {
    fn handle_app_msg(&mut self, msg: AppMsg) -> IsymtopeServerResult<AppResponseMsg>;
}

#[derive(Debug, Default)]
pub struct DefaultServerContext {
    srs: DefaultSecureRandomStringGenerator,
    cookies: Cookies
}

#[derive(Debug)]
pub struct DefaultAppContext {
    sessions: MemorySessions,
    router: Router,
    executor: ServerActionExecutor,
    document_provider: Rc<DocumentProvider>,
}

impl DefaultAppContext {
    pub fn new(document_provider: Rc<DocumentProvider>) -> Self {
        let router = Router::with_document_provider(document_provider.clone());

        DefaultAppContext {
            sessions: Default::default(),
            router: router,
            executor: Default::default(),
            document_provider: document_provider,
        }
    }

    pub fn create(app_name: &str, path: &str) -> IsymtopeServerResult<DefaultAppContext> {
        eprintln!("[app context] creating context for app [{}] with main template path [{}]", app_name, path);

        let trimmed_path = path.trim_left_matches('/').to_owned();
        let template_path = &*APP_DIR.join(app_name).join(trimmed_path);
        let source = TemplateSource::TemplatePathSource(template_path);

        let document_provider = DocumentProvider::create(source)?;
        let app_context = DefaultAppContext::new(Rc::new(document_provider));

        Ok(app_context)
    }
}

#[allow(dead_code)]
#[derive(Debug)]
pub enum Msg {
    // #[cfg(feature = "session_time")] NewSession(usize, Option<Duration>),
    // #[cfg(not(feature = "session_time"))] NewSession(usize),
    // ValidateSession(String),
    // DestroySession(String),
    // SetValueInSession(String, String, ExpressionValue<OutputExpression>, bool),
    RenderAppRoute(String, String, String, String),
}

#[allow(dead_code)]
#[derive(Debug)]
pub enum AppMsg {
    // #[cfg(feature = "session_time")] NewSession(usize, Option<Duration>),
    // #[cfg(not(feature = "session_time"))] NewSession(usize),
    // ValidateSession(String),
    // DestroySession(String),
    // SetValueInSession(String, String, ExpressionValue<OutputExpression>, bool),
    RenderAppRoute(String, String, String, String),
}


#[derive(Debug)]
pub struct RenderResponse(String);

impl RenderResponse {
    pub fn take(self) -> String {
        self.0
    }
}

#[derive(Debug)]
pub enum ResponseMsg {
    RenderComplete(RenderResponse),
}

#[derive(Debug)]
pub enum AppResponseMsg {
    SessionCreated(String),
    SessionValidated,
    SessionDestroyed,
    ValueChanged,
    RenderComplete(RenderResponse),
}


impl ServerContext for DefaultServerContext {
    fn handle_msg(&mut self, msg: Msg) -> IsymtopeServerResult<ResponseMsg> {
        match msg {
            Msg::RenderAppRoute(ref base_url, ref app_name, ref template_path, ref path) => {
                // let template_path = if path == "/" { "/app.ism".to_owned() } else { path.to_owned() };

                let app_key = format!("[appName={}, templatePath={}]", app_name, template_path);
                eprintln!("[server context] get or creating context for app with key ({})", app_key);

                // let route_key = format!("[appName={}, templatePath={}]/{}", app_name, template_path, path);
                // eprintln!("[server context] get or creating context for app with key ({})", app_key);

                let app_response = APP_CACHE.with(|cache| {
                    let mut cache = cache.lock().unwrap();
                    let app_context = cache.entry(app_key.clone())
                        .or_insert_with(|| DefaultAppContext::create(app_name, template_path).unwrap());

                    let app_msg = AppMsg::RenderAppRoute(base_url.to_owned(), app_name.to_owned(), template_path.to_owned(), path.to_owned());
                    app_context.handle_app_msg(app_msg)
                });

                match app_response {
                    Ok(AppResponseMsg::RenderComplete(response)) => Ok(ResponseMsg::RenderComplete(response)),
                    Err(err) => Err(err),
                    _ => Err(try_eval_from_err!("Invalid response from app"))?
                }
            }
        }
    }
}

impl AppContext for DefaultAppContext {
    fn handle_app_msg(&mut self, msg: AppMsg) -> IsymtopeServerResult<AppResponseMsg> {
        match msg {
            // #[cfg(feature = "session_time")]
            // AppMsg::NewSession(bytes, expires) => {
            //     let cookie = self.srs.generate_secure_string(bytes)?;
            //     self.sessions.create(&cookie, expires)?;

            //     Ok(AppResponseMsg::SessionCreated(cookie))
            // }

            // #[cfg(not(feature = "session_time"))]
            // AppMsg::NewSession(bytes) => {
            //     let cookie = self.srs.generate_secure_string(bytes)?;
            //     self.sessions.create(&cookie)?;

            //     Ok(AppResponseMsg::SessionCreated(cookie))
            // }

            // AppMsg::ValidateSession(ref cookie) => {
            //     self.sessions.validate(cookie.as_str())?;
            //     Ok(AppResponseMsg::SessionValidated)
            // }

            // AppMsg::DestroySession(ref cookie) => {
            //     self.sessions.validate(cookie.as_str())?;
            //     Ok(AppResponseMsg::SessionDestroyed)
            // }

            // AppMsg::SetValueInSession(ref session_id, ref key, ref value, ref flag) => {
            //     eprintln!("[DefaultServerContext] Setting value in session [{}] due to request message (flag: {:?}): {} to {:?}", session_id, flag, key, value);
            //     Ok(AppResponseMsg::ValueChanged)
            // }

            AppMsg::RenderAppRoute(ref base_url, ref app_name, ref template_path, ref path) => {
                let ref document_provider = self.document_provider;

                // Create temporary session with default state
                let mut default_state = MemorySession::default();
                let mut default_ctx =
                    DefaultOutputContext::create(document_provider.clone(), None);
                self.executor.initialize_session_data(
                    &mut default_state,
                    document_provider.doc(),
                    &mut default_ctx,
                )?;

                let mut ctx = DefaultOutputContext::create(
                    document_provider.clone(),
                    Some(Rc::new(default_state)),
                );

                // Create temporary session for this route
                let mut state = MemorySession::default();
                self.executor.initialize_session_data(
                    &mut state,
                    document_provider.doc(),
                    &mut ctx,
                )?;
                self.executor.execute_document_route(
                    &mut state,
                    document_provider.doc(),
                    &mut ctx,
                    path,
                )?;

                let factory = InternalTemplateRendererFactory::default();
                let renderer = factory.build(document_provider.clone(), Some(Rc::new(state)), base_url)?;
                let body = renderer.render()?;

                let response = RenderResponse(body);
                Ok(AppResponseMsg::RenderComplete(response))
            }
        }
    }
}
