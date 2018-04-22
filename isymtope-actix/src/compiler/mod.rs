use std::collections::HashMap;
use actix::*;

mod render_example_app_route;
mod compile_source;

pub use self::render_example_app_route::*;
pub use self::compile_source::*;

use isymtope_generate::*;

#[derive(Debug, Default)]
pub struct Compiler {
    template_file_cache: HashMap<String, DefaultTemplateContext>
}

impl Actor for Compiler {
    type Context = Context<Self>;
}
