use std::fmt::{self, Debug, Formatter};
use std::str;

#[cfg(feature = "session_time")]
use time::Duration;

use futures;
use futures::future::{self, Future, FutureResult};

use isymtope_ast_common::*;
use isymtope_build::*;
use server::*;

pub trait ServerContext {
    fn handle_msg(&mut self, msg: Msg) -> IsymtopeServerResult<ResponseMsg>;
}

#[derive(Debug)]
pub struct DefaultServerContext {
    srs: DefaultSecureRandomStringGenerator,
    cookies: Cookies,
    sessions: MemorySessions,
    router: Router,
    executor: ServerActionExecutor,
    document_provider: Rc<DocumentProvider>,
}

impl DefaultServerContext {
    pub fn new(document_provider: Rc<DocumentProvider>) -> Self {
        let router = Router::with_document_provider(document_provider.clone());

        DefaultServerContext {
            srs: Default::default(),
            cookies: Default::default(),
            sessions: Default::default(),
            router: router,
            executor: Default::default(),
            document_provider: document_provider,
        }
    }
}

#[allow(dead_code)]
#[derive(Debug)]
pub enum Msg {
    #[cfg(feature = "session_time")] NewSession(usize, Option<Duration>),
    #[cfg(not(feature = "session_time"))] NewSession(usize),
    ValidateSession(String),
    DestroySession(String),
    SetValueInSession(String, String, ExpressionValue<OutputExpression>, bool),
    ExecuteRoute(String),
    Render,
    RenderRoute(String),
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
    SessionCreated(String),
    SessionValidated,
    SessionDestroyed,
    RouteComplete,
    ValueChanged,
    RenderComplete(RenderResponse),
}

// impl Debug for ResponseMsg {
//     fn fmt(&self, f: &mut Formatter) -> fmt::Result {
//         match *self {
//             ResponseMsg::SessionCreated(ref s) => write!(f, "SessionCreated({})", s),
//             ResponseMsg::SessionValidated => write!(f, "SessionValidated"),
//             ResponseMsg::RouteComplete => write!(f, "RouteComplete"),
//             // ResponseMsg::RenderComplete(..) => write!(f, "RenderComplete(<response future>)"),
//             ResponseMsg::RenderComplete(..) => write!(f, "RenderComplete(<buffer>)"),
//         }
//     }
// }

impl ServerContext for DefaultServerContext {
    fn handle_msg(&mut self, msg: Msg) -> IsymtopeServerResult<ResponseMsg> {
        match msg {
            #[cfg(feature = "session_time")]
            Msg::NewSession(bytes, expires) => {
                let cookie = self.srs.generate_secure_string(bytes)?;
                self.sessions.create(&cookie, expires)?;

                Ok(ResponseMsg::SessionCreated(cookie))
            }

            #[cfg(not(feature = "session_time"))]
            Msg::NewSession(bytes) => {
                let cookie = self.srs.generate_secure_string(bytes)?;
                self.sessions.create(&cookie)?;

                Ok(ResponseMsg::SessionCreated(cookie))
            }

            Msg::ValidateSession(ref cookie) => {
                self.sessions.validate(cookie.as_str())?;
                Ok(ResponseMsg::SessionValidated)
            }

            Msg::DestroySession(ref cookie) => {
                self.sessions.validate(cookie.as_str())?;
                Ok(ResponseMsg::SessionDestroyed)
            }

            Msg::ExecuteRoute(ref path) => {
                eprintln!(
                    "[DefaultServerContext] Executing route against session: {}",
                    path
                );
                Ok(ResponseMsg::RouteComplete)
            }

            Msg::SetValueInSession(ref session_id, ref key, ref value, ref flag) => {
                eprintln!("[DefaultServerContext] Setting value in session [{}] due to request message (flag: {:?}): {} to {:?}", session_id, flag, key, value);
                Ok(ResponseMsg::ValueChanged)
            }

            Msg::Render => {
                let internal_renderer =
                    InternalTemplateRenderer::build(self.document_provider.clone(), None)?;
                let body = internal_renderer.render()?;
                Ok(ResponseMsg::RenderComplete(RenderResponse((body))))
            }

            Msg::RenderRoute(ref path) => {
                let document_provider = self.document_provider.clone();

                // Create temporary session with default state
                let mut default_state = MemorySession::default();
                let mut default_ctx =
                    DefaultOutputContext::create(self.document_provider.clone(), None);
                self.executor.initialize_session_data(
                    &mut default_state,
                    document_provider.doc(),
                    &mut default_ctx,
                )?;

                let mut ctx = DefaultOutputContext::create(
                    self.document_provider.clone(),
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

                let internal_renderer = InternalTemplateRenderer::build(
                    self.document_provider.clone(),
                    Some(Rc::new(state)),
                )?;
                let body = internal_renderer.render()?;
                Ok(ResponseMsg::RenderComplete(RenderResponse((body))))
            }
        }
    }

    // fn handle_render(&mut self, msg: Msg) -> IsymtopeServerResult<RenderResponse> {
    //     let response = RenderResponse(Cow::new("Cowsay hi!!"));

    //     Ok(Some(response))
    // }
}
