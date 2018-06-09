use std::path::Path;
use std::rc::Rc;

use isymtope_build::*;
use super::*;

pub trait TemplateContext {
    fn handle_msg(
        &mut self,
        msg: TemplateRequestMsg,
    ) -> IsymtopeGenerateResult<TemplateResponseMsg>;
}

#[derive(Debug)]
pub struct DefaultTemplateContext {
    router: Router,
    executor: ActionExecutor,
    document_provider: Rc<DocumentProvider>,
}

impl DefaultTemplateContext {
    pub fn new(document_provider: Rc<DocumentProvider>) -> Self {
        let router = Router::with_document_provider(document_provider.clone());

        DefaultTemplateContext {
            router: router,
            executor: Default::default(),
            document_provider: document_provider,
        }
    }

    pub fn create(
        app_root: &Path,
        template_path: &str
    ) -> IsymtopeGenerateResult<DefaultTemplateContext> {
        eprintln!(
            "[template context] creating context for app root [{:?}], with main template path [{}]",
            app_root, template_path
        );

        let trimmed_path = template_path.trim_left_matches('/').to_owned();
        let template_file = app_root.join(trimmed_path);
        let source = TemplateSource::TemplatePathSource(&template_file);

        let document_provider = DocumentProvider::create(source)?;
        let template_context = DefaultTemplateContext::new(Rc::new(document_provider));

        Ok(template_context)
    }
}

// #[derive(Debug)]
// pub struct RouteState(Rc<Route>);

// impl RouteStateProvider for RouteState {
//     fn route(&self) -> SessionResult<Option<Rc<Route>>> {
//         Ok(self.0)
//     }    
// }

impl TemplateContext for DefaultTemplateContext {
    fn handle_msg(
        &mut self,
        msg: TemplateRequestMsg,
    ) -> IsymtopeGenerateResult<TemplateResponseMsg> {
        match msg {
            TemplateRequestMsg::RenderAppRoute(
                ref base_url,
                ref _app_name,
                ref _template_path,
                ref path,
            ) => {
                let ref document_provider = self.document_provider;

                // Create temporary session with default state
                let mut default_state = MemorySession::default();
                let mut default_ctx = DefaultOutputContext::create(document_provider.clone(), None);
                self.executor.initialize_session_data(
                    &mut default_state,
                    document_provider.doc(),
                    &mut default_ctx,
                )?;

                let mut ctx = DefaultOutputContext::create(
                    document_provider.clone(),
                    Some(Rc::new(default_state)),
                );

                eprintln!("Processing route: {} in document", path);

                let doc = document_provider.doc();

                // Create temporary session for this route
                let mut state = MemorySession::default();
                self.executor.initialize_session_data(
                    &mut state,
                    doc,
                    &mut ctx,
                )?;

                let matcher = RouteMatcher::default();
                let route = matcher.with_route(
                    doc,
                    &mut ctx,
                    path,
                )?;
                self.executor.execute_route(
                    &mut state,
                    doc,
                    &mut ctx,
                    &route,
                )?;
                let route = route.route();

                let factory = InternalTemplateRendererFactory::default();
                let route_state = Rc::new(route.to_owned());
                let renderer =
                    factory.build(document_provider.clone(), Some(Rc::new(state)), route_state.clone(), base_url, path)?;
                let body = renderer.render()?;

                let response = RenderResponse::new(body);
                Ok(TemplateResponseMsg::RenderComplete(response))
            }
        }
    }
}
