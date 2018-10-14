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

        // let result = self.get_existing_app(&slug)
        //     .map_err(Error::from)
        //     .map(move |app| {
        //         let template_name = app.template_name.to_owned();

            //     msg.api.send(GetTemplate { template_name: template_name })
            //         .map_err(Error::from)
            //         .and_then(move |res| {
            //             res.and_then(move |template_response| {
            //                 let template = template_response.template;
            //                 msg.compiler.send(CompileSourceMsg { source: source, route: route, base_url: base_url })
            //                     // .map_err(|err| ErrorInternalServerError(err))
            //                     .map_err(Error::from)
            //                     .and_then(move |res| {
            //                         res.and_then(move |body| {
            //                             Ok(CompileSourceResponse {
            //                                 template: template,
            //                                 body: body
            //                             })
            //                         })
            //                     })
            //             })
            //         })
            //         .map_err(Error::from)
            // })
            // .map_err(|err| ErrorInternalServerError(err));

        let result = future::result(self.get_existing_app(&slug.to_owned()))
            .map_err(Error::from)
            .and_then(move |app| {
                let source = "".to_string();
                let template_name = app.template_name.to_string();
                let response = self::compile_named_template_for_app(msg.api.clone(), msg.compiler.clone(), &template_name, &base_url, &route, &source);

                response

                // let template_name = app.template_name.to_owned();

                // msg.api.send(GetTemplate { template_name: template_name })
                //     .map_err(Error::from)
                //     .and_then(move |res| {
                //         let template = res.unwrap().template.to_owned();
                //         let body = "".to_string();

                //         future::ok(CompileSourceResponse {
                //             template: template,
                //             body: body
                //         })
                //     })
            });
            // .map_err(Error::from);

        // MessageResult(result)
        // result
        Box::new(result)
    }
}
