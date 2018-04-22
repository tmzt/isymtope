use std::collections::HashMap;
use actix::*;

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

#[derive(Debug, Default, Serialize)]
pub struct CachedStoredApp {
    pub slug: String,
    pub uuid: String,
    pub base_app_uuid: Option<String>,
    pub base_app_slug: Option<String>,
    pub static_template: Option<String>,
}

#[derive(Debug, Default)]
pub struct PlaygroundApi {
    slug_cache: HashMap<String, CachedStoredApp>
}

impl Actor for PlaygroundApi {
    type Context = Context<Self>;
}
