use futures::{future,Future};

use actix::*;
use actix_web::*;
use actix_web::error::*;

use isymtope_generate::*;

use compiler::{Compiler, CompileSource as CompileSourceMsg};
use super::*;

pub struct CompileSource {
    pub api: Addr<PlaygroundApi>,
    pub compiler: Addr<Compiler>,
    pub source: String,
    pub base_url: String,
    pub route: String,
    pub slug: String,
}

impl Message for CompileSource {
    type Result = Result<CompileTemplateSourceResponse, Error>;
}

impl Handler<CompileSource> for PlaygroundApi {
    type Result = Box<Future<Item = CompileTemplateSourceResponse, Error = Error>>;

    fn handle(&mut self, msg: CompileSource, _: &mut Self::Context) -> Self::Result {
        let source = msg.source.to_owned();
        let base_url = msg.base_url.to_owned();
        let route = msg.route.to_owned();
        let slug = msg.slug.to_owned();

        let result = future::result(self.get_existing_app(&slug.to_owned()))
            .map_err(Error::from)
            .and_then(move |app| {
                let source = "".to_string();
                let template_name = app.template_name.to_string();
                let response = self::render_named_template_for_app(msg.api.clone(), msg.compiler.clone(), &template_name, &base_url, &route, &source);

                response
            });

        Box::new(result)
    }
}
