use std::io;

use super::*;

pub mod writers;
pub use self::writers::*;

pub mod context;
pub use self::context::*;

#[derive(Debug)]
pub struct HtmlOutput {}
#[derive(Debug)]
pub struct JsOutput {}

pub trait ObjectWriter<T, O> {
    fn write_object(&mut self, w: &mut io::Write, ctx: &mut OutputContext, obj: &T) -> DocumentProcessingResult<()>;
}
