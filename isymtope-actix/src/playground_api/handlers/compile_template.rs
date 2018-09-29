use futures::Future;

use actix::*;
use actix::prelude::*;
use isymtope_generate::*;
use compiler::*;
use super::*;

pub struct CompileTemplate {
    pub api: Addr<PlaygroundApi>,
    pub compiler: Addr<Compiler>,
    pub base_url: String,
    pub route: String,
    pub slug: String,
}

#[derive(Debug, Message)]
pub struct CompileTemplateResponse {
    pub get_template_response: GetTemplateResponse,
    pub body: String
}

impl Message for CompileTemplate {
    type Result = Result<CompileTemplateResponse, PlaygroundApiError>;
}

// impl ResponseType for CompileTemplate {
//     type Item = CompileTemplateResponse;
//     type Error = IsymtopeGenerateError;
// }

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
    type Result = Response<CompileTemplateResponse, PlaygroundApiError>;

    fn handle(&mut self, msg: CompileTemplate, _: &mut Self::Context) -> Self::Result {
        let base_url = msg.base_url.to_owned();
        let route = msg.route.to_owned();
        let slug = msg.slug.to_owned();

        let result = msg.api.send(GetTemplate { slug: slug })
            .from_err()
            .and_then(move |res| match res {
                Ok(slug_res) => {
                    let app_name = slug_res.static_template.as_ref().unwrap().to_owned();
                    msg.compiler.send(RenderExampleAppRoute { app_name: app_name, route:  route, base_url: base_url })
                        .and_then(move |res| match res {
                            Ok(body) => Ok(CompileTemplateResponse {
                                get_template_response: slug_res.to_owned(),
                                body: body
                            }),
                            _ => panic!("Cannot render template")
                        })
                },
                _ => panic!("Cannot find template")
            });

        Response::async(result.from_err())
    }
}
