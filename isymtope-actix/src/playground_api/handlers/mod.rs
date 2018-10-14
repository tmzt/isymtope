use std::collections::hash_map::{HashMap, Entry};
use actix::*;
use actix_web::error::*;

use isymtope_ast_common::*;

mod create_example;
mod compile_source;
mod compile_template;
mod get_app;
mod get_template;

pub use self::create_example::*;
pub use self::compile_source::*;
pub use self::compile_template::*;
pub use self::get_app::*;
pub use self::get_template::*;

#[derive(Debug, Default)]
pub struct PlaygroundApiContext {}

pub use super::*;

use futures::future;

// #[derive(Debug, Default, Serialize)]
// pub struct CachedStoredApp {
//     pub slug: String,
//     pub uuid: String,
//     pub base_app_uuid: Option<String>,
//     pub base_app_slug: Option<String>,
//     pub static_template: Option<String>,
// }

#[derive(Debug, Default)]
pub struct PlaygroundApi {
    slug_cache: HashMap<String, StoredApp>,
    template_cache: HashMap<String, TemplateData>,
}

#[derive(Debug, Message)]
pub struct CompileTemplateSourceResponse {
    pub template: TemplateData,
    pub body: String
}

impl PlaygroundApi {
    pub fn get_or_load_template(&mut self, template_name: &str) -> Result<TemplateData> {
        // let entry = self.template_cache.entry(template_name.to_owned())
        //     .or_insert_with(|| TemplateData::read_file(template_name));

        match self.template_cache.entry(template_name.to_owned()) {
            Entry::Occupied(o) => { return Ok(o.get().to_owned()) },
            Entry::Vacant(v) => {
                let template = TemplateData::read_file(template_name)?;
                v.insert(template.to_owned());
                Ok(template)
            }
        }
    }

    pub fn get_existing_app(&mut self, slug: &str) -> Result<StoredApp> {
        self.slug_cache.get(slug)
            .map(|app| Ok(app.to_owned()))
            .unwrap_or_else(|| Err(ErrorNotFound("Cannot find application in cache")))
    }

    pub fn create_app(&mut self, base: AppBase) -> Result<StoredApp> {
        let slug = allocate_element_key();

        self.get_or_create_app(&slug, base)
    }

    pub fn get_or_create_app(&mut self, slug: &str, base: AppBase) -> Result<StoredApp> {
        if let Some(app) = self.slug_cache.get(slug) {
            return Ok(app.to_owned());
        };

        // Look for base
        let template = match base {
            AppBase::Template(ref template_name) => self.get_or_load_template(template_name)?,
            AppBase::App(ref base_slug) => {
                let base_app = self.slug_cache.get(base_slug).map(|base_app| base_app.to_owned());
                if base_app.is_none() {
                    return Err(ErrorNotFound("Base application not found"))?;
                };
                self.get_or_load_template(&base_app.unwrap().template_name)?
            }
        };

        let app = StoredApp::create(slug, base, &template.template_name);
        self.slug_cache.insert(slug.to_owned(), app.to_owned());
        Ok(app)
    }
}

fn compile_named_template_for_app(api: &Addr<PlaygroundApi>, template_name: &str, source: &str) -> impl Future<Item = CompileTemplateSourceResponse, Error = Error> {
    let template_name = template_name.to_string();

    api.send(GetTemplate { template_name: template_name })
        .map_err(Error::from)
        .and_then(move |res| {
            let template = res.unwrap().template.to_owned();
            let body = "".to_string();

            future::ok(CompileTemplateSourceResponse {
                template: template,
                body: body
            })
        })
}

impl Actor for PlaygroundApi {
    type Context = Context<Self>;
}
