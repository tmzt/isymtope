use std::str;
use std::rc::Rc;
use std::error::Error;
use std::collections::HashMap;

use trimmer::{Context as TrimmerContext, Template as TrimmerTemplate};

use error::*;
use ast::*;
use util::*;
use traits::*;
use input::*;
use output::*;

include!(concat!(env!("OUT_DIR"), "/templates.rs"));
use self::templates::*;

#[derive(Debug)]
pub struct InternalTemplateSource {
    template: TrimmerTemplate,
    preload_src: String,
}


// fn render_template(template: &TrimmerTemplate, tctx: &TrimmerContext) -> DocumentProcessingResult<String> {
//     let res = page
//     match template.render(tctx) {
//         Err(err) => Err(DocumentProcessingError::InternalRenderError(err.description().to_owned())),
//         Ok(v) => Ok(v)
//     }
// }

#[derive(Debug, Default)]
pub struct InternalTemplateRendererFactory;

#[derive(Debug)]
pub struct InternalTemplateRenderer {
    data: InternalTemplateData
}

impl InternalTemplateRendererFactory {
    pub fn build(
        &self,
        document_provider: Rc<DocumentProvider>,
        state_provider: Option<Rc<ReducerStateProvider>>,
    ) -> DocumentProcessingResult<InternalTemplateRenderer> {
        let renderer = InternalTemplateRenderer::build(document_provider, state_provider)?;

        eprintln!("[page_template_factory] created renderer");
        Ok(renderer)
    }
}

impl InternalTemplateRenderer {
    pub fn build(
        document_provider: Rc<DocumentProvider>,
        state_provider: Option<Rc<ReducerStateProvider>>,
    ) -> DocumentProcessingResult<InternalTemplateRenderer> {
        let page_data_builder = InternalTemplateDataBuilder::new(document_provider.clone(), state_provider.map(|s| s.clone()));
        let page_data = page_data_builder.build()?;

        Ok(InternalTemplateRenderer {
            data: page_data
        })
    }

    pub fn render(&self) -> DocumentProcessingResult<String> {
        let mut bytes: Vec<u8> = Vec::with_capacity(8192);
        self::page(&mut bytes, &self.data)?;

        let out_buf = str::from_utf8(bytes.as_slice())?.to_owned();
        Ok(out_buf)
    }
}
