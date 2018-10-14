use std::rc::Rc;

use failure::*;

use actix_web::{Error, HttpResponse};
use actix_web::error::{ResponseError, ErrorInternalServerError};
use actix_web::http::StatusCode;
use super::*;

use isymtope_ast_common::*;
use isymtope_parser::*;
use isymtope_generate::*;

fn parse_template(src: &str) -> IsymtopeGenerateResult<impl DocumentProvider> {
    let template = Rc::new(parser::parse_str(&src)?);

    // Create document provider
    let mut ctx: DefaultProcessingContext<ProcessedExpression> =
        DefaultProcessingContext::for_template(template.clone());

    let document: Document = TryProcessFrom::try_process_from(template.as_ref(), &mut ctx)?;
    let document_provider = DefaultDocumentProvider::create(document)?;

    Ok(document_provider)
}

fn compile_template(src: &str, base_url: &str) -> IsymtopeGenerateResult<String> {
    let template = parse_template(&src)?;
    let mut template_context = DefaultTemplateContext::new(Rc::new(template));

    let app_name = "/app.ism";
    let template_path = "/app.ism";
    let path = "/";

    let req = TemplateRequestMsg::RenderAppRoute(
        base_url.to_string(),
        app_name.to_string(),
        template_path.to_string(),
        path.to_string(),
    );

    let response = template_context.handle_msg(req)?;
    let TemplateResponseMsg::RenderComplete(result) = response;
    let body = result.into_inner();

    Ok(body)
}

#[derive(Debug)]
pub struct CompileSource {
    pub source: String,
    pub route: String,
    pub base_url: String
}

impl Message for CompileSource {
    // type Result = IsymtopeGenerateResult<String>;
    type Result = Result<String, Error>;
}

#[derive(Fail, Debug)]
pub enum CompilerError {
    #[fail(display = "Generate error: {:?}", _0)]
    GenerateError(IsymtopeGenerateError)
}

impl From<IsymtopeGenerateError> for CompilerError {
    fn from(err: IsymtopeGenerateError) -> Self {
        CompilerError::GenerateError(err)
    }
}

impl ResponseError for CompilerError {
    fn error_response(&self) -> HttpResponse {
        HttpResponse::new(StatusCode::INTERNAL_SERVER_ERROR)
    }
}

impl Handler<CompileSource> for Compiler {
    type Result = MessageResult<CompileSource>;

    fn handle(&mut self, msg: CompileSource, _: &mut Self::Context) -> Self::Result {
        // let app_name = &msg.app_name;
        let ism_path = "/app.ism";
        let route = &msg.route;
        let base_url = &msg.base_url;

        let source = &msg.source;
        let body = compile_template(source, base_url)
            .map_err(CompilerError::from)
            .map_err(|err| ErrorInternalServerError(err));

        MessageResult(body)
    }
}
