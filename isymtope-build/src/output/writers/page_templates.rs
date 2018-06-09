use std::str;
use std::rc::Rc;

use error::*;
use traits::*;
use input::*;
use output::*;

use isymtope_data::*;

#[derive(Debug, Default)]
pub struct InternalTemplateRendererFactory;

#[derive(Debug)]
pub struct InternalTemplateRenderer {
    data: InternalTemplateData,
}

impl InternalTemplateRendererFactory {
    pub fn build(
        &self,
        document_provider: Rc<DocumentProvider>,
        state_provider: Option<Rc<ReducerStateProvider>>,
        route_state: Rc<Route>,
        base_url: &str,
        path: &str,
    ) -> DocumentProcessingResult<InternalTemplateRenderer> {
        let renderer =
            InternalTemplateRenderer::build(document_provider, state_provider, route_state, base_url, path)?;

        eprintln!("[page_template_factory] created renderer");
        Ok(renderer)
    }
}

impl InternalTemplateRenderer {
    pub fn build(
        document_provider: Rc<DocumentProvider>,
        state_provider: Option<Rc<ReducerStateProvider>>,
        route_state: Rc<Route>,
        base_url: &str,
        path: &str,
    ) -> DocumentProcessingResult<InternalTemplateRenderer> {
        let page_data_builder = InternalTemplateDataBuilder::new(
            document_provider.clone(),
            state_provider.map(|s| s.clone()),
            route_state.clone(),
            base_url,
            path,
        );
        let page_data = page_data_builder.build()?;

        Ok(InternalTemplateRenderer { data: page_data })
    }

    pub fn render(&self) -> DocumentProcessingResult<String> {
        let mut bytes: Vec<u8> = Vec::with_capacity(8192);
        self::page(&mut bytes, &self.data)?;

        let out_buf = str::from_utf8(bytes.as_slice())?.to_owned();
        Ok(out_buf)
    }
}
