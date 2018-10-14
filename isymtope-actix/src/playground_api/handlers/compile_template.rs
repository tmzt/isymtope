use futures::{future,Future};

use actix::*;

use isymtope_generate::*;
use compiler::*;
use super::*;

pub struct CompileTemplate {
    pub api: Addr<PlaygroundApi>,
    pub compiler: Addr<Compiler>,
    pub base_url: String,
    pub route: String,
    pub template_name: String,
}

impl Message for CompileTemplate {
    type Result = Result<CompileTemplateSourceResponse, Error>;
}

pub enum PlaygroundApiError {
    IsymtopeGenerateError(IsymtopeGenerateError),
    MailboxError(MailboxError)
}

impl From<IsymtopeGenerateError> for PlaygroundApiError {
    fn from(err: IsymtopeGenerateError) -> Self {
        PlaygroundApiError::IsymtopeGenerateError(err)
    }
}
impl From<MailboxError> for PlaygroundApiError {
    fn from(err: MailboxError) -> Self {
        PlaygroundApiError::MailboxError(err)
    }
}

impl Handler<CompileTemplate> for PlaygroundApi {
    type Result = Box<Future<Item = CompileTemplateSourceResponse, Error = Error>>;

    fn handle(&mut self, msg: CompileTemplate, _: &mut Self::Context) -> Self::Result {
        let base_url = msg.base_url.to_owned();
        let route = msg.route.to_owned();
        let template_name = msg.template_name.to_owned();

        let source = "".to_string();
        let result = self::render_named_template_for_app(msg.api.clone(), msg.compiler.clone(), &template_name, &base_url, &route, &source);
        Box::new(result)

        // let result = msg.api.send(GetTemplate { template_name: template_name })
        //     .and_then(move |response| match response {
        //         Ok(response) => {
        //             let template = response.template.to_owned();
        //             let template_name = template.template_name.to_owned();
        //             msg.compiler.send(RenderExampleAppRoute { app_name: template_name, route:  route, base_url: base_url })
        //                 .and_then(move |res| match res {
        //                     Ok(body) => Ok(CompileTemplateResponse {
        //                         template: template.to_owned(),
        //                         body: body
        //                     }),
        //                     _ => panic!("Cannot render template")
        //                 })
        //         },
        //         _ => panic!("Cannot find template")
        //     })
        //     .map_err(Error::from);

        // let result = msg.api.send(GetTemplate { template_name: template_name })
        //     // .map_err(Error::from)
        //     .and_then(move |response| {
        //         let template = response.unwrap().template.to_owned();
        //         let body = "".to_string();
        //         future::ok(CompileTemplateResponse {
        //             template: template.to_owned(),
        //             body: body
        //         }).into_future()
        //     })
        //     .map_err(Error::from);

        // MessageResult(result)
    }
}
