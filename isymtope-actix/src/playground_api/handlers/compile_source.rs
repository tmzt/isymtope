use futures::Future;

use actix::*;
use actix::prelude::*;

use isymtope_generate::*;

use compiler::{Compiler, CompileSource as CompileSourceMsg};
use super::*;

pub struct CompileSource {
    pub api: Addr<Syn, PlaygroundApi>,
    pub compiler: Addr<Syn, Compiler>,
    pub source: String,
    pub base_url: String,
    pub route: String,
    pub slug: String,
}

#[derive(Debug, Message)]
pub struct CompileSourceResponse {
    pub get_template_response: GetTemplateResponse,
    pub body: String
}

impl Message for CompileSource {
    type Result = Result<CompileSourceResponse, PlaygroundApiError>;
}

// impl ResponseType for CompileSource {
//     type Item = CompileSourceResponse;
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

impl Handler<CompileSource> for PlaygroundApi {
    type Result = Response<CompileSourceResponse, PlaygroundApiError>;

    fn handle(&mut self, msg: CompileSource, _: &mut Context<PlaygroundApi>) -> Self::Result {
        let source = msg.source.to_owned();
        let base_url = msg.base_url.to_owned();
        let route = msg.route.to_owned();
        let slug = msg.slug.to_owned();

        let result = msg.api.send(GetTemplate { slug: slug })
            .from_err()
            .and_then(move |res| match res {
                Ok(slug_res) => {
                    let app_name = slug_res.static_template.as_ref().unwrap().to_owned();
                    msg.compiler.send(CompileSourceMsg { source: source, route: route, base_url: base_url })
                        .and_then(move |res| match res {
                            Ok(body) => Ok(CompileSourceResponse {
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
