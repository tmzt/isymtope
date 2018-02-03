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

#[derive(Debug)]
pub struct InternalTemplateSource {
    template: TrimmerTemplate,
    preload_src: String,
}

fn parse_template(src: &str) -> DocumentProcessingResult<TrimmerTemplate> {
    let parser = ::trimmer::Parser::new();
    let res = parser.parse(&src);
    match res {
        Err(err) => Err(DocumentProcessingError::InternalParseError(err.description().to_owned())),
        Ok(v) => Ok(v)
    }
}

fn render_template(template: &TrimmerTemplate, tctx: &TrimmerContext) -> DocumentProcessingResult<String> {
    match template.render(tctx) {
        Err(err) => Err(DocumentProcessingError::InternalRenderError(err.description().to_owned())),
        Ok(v) => Ok(v)
    }
}

#[cfg(not(feature = "include_templates"))]
fn template_source() -> DocumentProcessingResult<InternalTemplateSource> {
    use std::io::*;
    use std::path::Path;
    use std::fs::File;

    let mut buf = String::with_capacity(4096);

    // Preload template
    let path = Path::new("../isymtope-build/res/templates/page/page.trimmer");
    buf.truncate(0);

    File::open(path).and_then(|mut f| f.read_to_string(&mut buf))?;
    let template = self::parse_template(&buf)?;

    // Preload scripts
    let scripts = vec![
        Path::new("../isymtope-build/res/templates/page/isymtope-driver-incdom.js"),
        Path::new("../isymtope-build/res/templates/page/isymtope-routing.js"),
        Path::new("../isymtope-build/res/templates/page/isymtope-util.js"),
    ];

    buf.truncate(0);
    for script in scripts {
        File::open(script).and_then(|mut f| f.read_to_string(&mut buf))?;
    }
    let preload_src = format!("{}", buf);

    Ok(InternalTemplateSource {
        template: template,
        preload_src: preload_src,
    })
}

#[cfg(feature = "include_templates")]
fn template_source() -> DocumentProcessingResult<InternalTemplateSource> {
    let template_src = include_str!("../../../res/templates/page/page.trimmer");
    let preload_src = format!(
        "{}\r\n{}\r\n{}",
        include_str!("../../../res/templates/page/isymtope-driver-incdom.js"),
        include_str!("../../../res/templates/page/isymtope-routing.js"),
        include_str!("../../../res/templates/page/isymtope-util.js")
    );
    eprintln!("[page templates] parsing template");
    let template = self::parse_template(&template_src)?;
    eprintln!("[page templates] parsed template: {:?}", template);

    Ok(InternalTemplateSource {
        template: template,
        preload_src: preload_src,
    })
}

#[derive(Debug)]
pub struct InternalTemplateRendererFactory {
    template_src: InternalTemplateSource
}

#[derive(Debug)]
pub struct InternalTemplateRenderer<'template_src> {
    template_src: &'template_src InternalTemplateSource,
    route_keys: Vec<String>,
    route_func_keys: HashMap<String, String>,
    route_bodies: HashMap<String, String>,
    event_keys: Vec<String>,
    event_enterkeyflags: HashMap<String, bool>,
    event_action_keys: HashMap<String, Vec<String>>,
    event_action_bodies: HashMap<String, HashMap<String, String>>,
    reducer_keys: Vec<String>,
    reducer_action_keys: HashMap<String, Vec<String>>,
    reducer_bodies: HashMap<String, HashMap<String, String>>,
    reducer_defaults: HashMap<String, String>,
    extern_reducer_keys: Vec<String>,
    query_names: Vec<String>,
    query_params: HashMap<String, Vec<String>>,
    query_bodies: HashMap<String, String>,
    component_names: Vec<String>,
    component_bodies: HashMap<String, String>,
    page_render_func_body: String,
    page_body_html: String,
}

impl InternalTemplateRendererFactory {
    pub fn create() -> DocumentProcessingResult<InternalTemplateRendererFactory> {
        let template_src = self::template_source()?;
        eprintln!("[page_templates_factory] got template source");
        
        Ok(InternalTemplateRendererFactory { template_src: template_src })
    }

    pub fn build(
        &self,
        document_provider: Rc<DocumentProvider>,
        state_provider: Option<Rc<ReducerStateProvider>>,
    ) -> DocumentProcessingResult<InternalTemplateRenderer> {
        let template_src = &self.template_src;
        let renderer = InternalTemplateRenderer::build(template_src, document_provider, state_provider)?;

        eprintln!("[page_template_factory] created renderer");
        Ok(renderer)
    }
}

impl<'tmpl> InternalTemplateRenderer<'tmpl> {
    pub fn build<'t>(
        template_src: &'t InternalTemplateSource,
        document_provider: Rc<DocumentProvider>,
        state_provider: Option<Rc<ReducerStateProvider>>,
    ) -> DocumentProcessingResult<InternalTemplateRenderer<'t>> {
        // Initialize output context
        let mut ctx: DefaultOutputContext =
            DefaultOutputContext::create(document_provider.clone(), state_provider);
        let doc = document_provider.doc();

        // Buffers
        let mut bytes: Vec<u8> = Vec::with_capacity(8192);
        eprintln!("[page_templates] bytes.len(): {}", bytes.len());

        // Writers

        let mut js_writer = DefaultJsWriter::default();
        let mut html_writer = DefaultHtmlWriter::default();
        eprintln!("[page_templates] writers created");

        // let template_src = self::template_source()?;
        // eprintln!("[page_templates] got template source");

        // Events

        let mut event_keys: Vec<String> = Default::default();
        let mut event_enterkeyflags: HashMap<String, bool> = Default::default();
        let mut event_action_keys: HashMap<String, Vec<String>> = Default::default();
        let mut event_action_bodies: HashMap<String, HashMap<String, String>> = Default::default();

        eprintln!("[page_templates] enumerating event bindings");
        for ref event_binding in doc.event_bindings() {
            eprintln!("[page_templates] event_binding: {:?}", event_binding);

            let key = event_binding.key();
            let event = event_binding.event();

            event_keys.push(key.clone());
            event_enterkeyflags.insert(key.clone(), event_binding.is_enterkey());

            eprintln!("[page_templates] getting or creating event action key");
            let action_keys = event_action_keys
                .entry(key.clone())
                .or_insert_with(|| Default::default());

            eprintln!("[page_templates] getting or creating event action body");
            let action_bodies = event_action_bodies
                .entry(key.clone())
                .or_insert_with(|| Default::default());

            eprintln!("[page_templates] checking for event actions");
            if let Some(actions) = event.actions() {
                let actions: Vec<_> = actions.collect();

                ctx.push_child_scope();

                let event_prop_aliases: Vec<_> = actions
                    .iter()
                    .flat_map(|action| {
                        let dispatch_iter: Vec<_> = match **action {
                            ActionOp::DispatchAction(_, Some(box ref props), _)
                            | ActionOp::DispatchActionTo(_, Some(box ref props), _, _) => Some(
                                props
                                    .into_iter()
                                    .map(|prop| (prop.key().to_owned(), prop.value().to_owned())),
                            ),
                            _ => None,
                        }.into_iter()
                            .flat_map(|v| v)
                            .collect();
                        let navigate_iter: Vec<_> = match **action {
                            ActionOp::Navigate(ref path, _) => {
                                Some(vec![("path".to_owned(), path.to_owned())].into_iter())
                            }
                            _ => None,
                        }.into_iter()
                            .flat_map(|v| v)
                            .collect();
                        dispatch_iter.into_iter().chain(navigate_iter.into_iter())
                    })
                    .filter_map(|(alias, prop)| match prop {
                        ExpressionValue::Expression(Expression::Path(ref path_value, _)) => {
                            Some((alias, prop.to_owned(), path_value.component_string()))
                        }
                        _ => None,
                    })
                    .collect();

                eprintln!("[page_templates] enumerating path aliases");
                for (alias, _, raw_path) in event_prop_aliases {
                    let binding = CommonBindings::PathAlias(alias.to_owned(), Default::default());
                    let expr = ExpressionValue::Expression(Expression::RawPath(
                        raw_path,
                        Default::default(),
                    ));
                    ctx.bind_value(binding, expr)?;
                }

                eprintln!("[page_templates] enumerating actions");
                for action in actions {
                    bytes.truncate(0);
                    js_writer.write_object(&mut bytes, &mut ctx, action)?;

                    let event_action_key = format!("{}_{}", key, allocate_element_key());
                    let event_action_body = str::from_utf8(bytes.as_slice())?.to_owned();

                    action_keys.push(event_action_key.clone());
                    action_bodies.insert(event_action_key, event_action_body);
                }

                ctx.pop_scope();
            };
        }

        let route_func_keys: HashMap<String, String> = doc.routes()
            .map(|r| (r.pattern().to_owned(), r.function_key().to_owned().into()))
            .collect();
        let route_keys: Vec<String> = route_func_keys.keys().map(|s| s.to_owned()).collect();
        let mut route_bodies: HashMap<String, String> = Default::default();

        for route in doc.routes() {
            eprintln!("[page_templates] route: {:?}", route);

            bytes.truncate(0);
            js_writer.write_object(&mut bytes, &mut ctx, route.action())?;

            let pattern = route.pattern().to_owned();
            let body = str::from_utf8(bytes.as_slice())?.to_owned();

            route_bodies.insert(pattern, body);
        }

        // Render reducers

        let reducer_keys: Vec<_> = doc.reducers()
            .map(|v| v.map(|(key, _)| key.to_owned()).collect())
            .unwrap_or_default();

        let mut reducer_bodies: HashMap<String, HashMap<String, String>> = Default::default();
        let mut reducer_defaults: HashMap<String, String> = Default::default();
        let mut reducer_action_keys: HashMap<String, Vec<String>> = Default::default();

        // eprintln!("Document: {:?}", doc);

        if let Some(v) = doc.reducers() {
            for (reducer_key, reducer) in v {
                eprintln!("[page_templates] reducer with key {}: {:?}", reducer_key, reducer);

                let action_keys = reducer_action_keys
                    .entry(reducer_key.to_owned())
                    .or_insert_with(|| Default::default());

                let action_bodies = reducer_bodies
                    .entry(reducer_key.to_owned())
                    .or_insert_with(|| Default::default());

                eprintln!("Reducer: {:?}", reducer);

                if let Some(v) = reducer.actions() {
                    for action in v {
                        eprintln!("Action: {:?}", action);

                        if let Some(expr) = action.expr() {
                            let name = action.name().to_uppercase();

                            bytes.truncate(0);
                            js_writer.write_object(&mut bytes, &mut ctx, expr)?;

                            let value = str::from_utf8(bytes.as_slice())?;
                            eprintln!("Value: {}", value);

                            let action_key = format!("{}.{}", reducer_key.to_uppercase(), name);
                            action_keys.push(action_key.clone());
                            action_bodies.insert(action_key, value.to_owned());
                        };
                    }
                };

                if let Some(expr) = reducer.default_value() {
                    bytes.truncate(0);
                    js_writer.write_object(&mut bytes, &mut ctx, expr)?;

                    let value = str::from_utf8(bytes.as_slice())?.to_owned();
                    let body = match *expr {
                        // FIXME: Special case
                        ExpressionValue::Expression(Expression::Composite(
                            CompositeValue::ArrayValue(..),
                        )) => format!("new Map({}.map(_item => [_item.id, _item]))", value),

                        _ => value,
                    };
                    reducer_defaults.insert(reducer_key.to_owned(), body);
                } else {
                    reducer_defaults.insert(reducer_key.to_owned(), "null".to_owned());
                }
            }
        };

        let extern_reducer_keys: Vec<_> =
            doc.extern_reducers().map(|n| n.name().to_owned()).collect();

        // Render component and root block bodies

        // Query bodies

        let query_names: Vec<_> = doc.queries().map(|(name, _)| name.to_owned()).collect();
        let mut query_params: HashMap<String, Vec<String>> = Default::default();
        let mut query_bodies: HashMap<String, String> = Default::default();

        for (query_name, query) in doc.queries() {
            bytes.truncate(0);
            js_writer.write_object(&mut bytes, &mut ctx, query)?;

            let body = str::from_utf8(bytes.as_slice())?.to_owned();
            let params: Vec<_> = query
                .params()
                .map(|v| v.map(|s| s.to_owned()).collect())
                .unwrap_or_default();

            query_bodies.insert(query_name.to_owned(), body);
            query_params.insert(query_name.to_owned(), params);
        }

        // Component bodies

        let component_names: Vec<_> = doc.components().map(|n| n.name().to_owned()).collect();
        let mut component_bodies: HashMap<String, String> = Default::default();

        for component in doc.components() {
            bytes.truncate(0);
            js_writer.write_object(&mut bytes, &mut ctx, component)?;

            let name = component.name().to_owned();
            let body = str::from_utf8(bytes.as_slice())?.to_owned();

            component_bodies.insert(name, body);
        }

        // Render root block body
        bytes.truncate(0);

        js_writer.write_object(&mut bytes, &mut ctx, doc.root_block())?;
        let page_render_func_body = str::from_utf8(bytes.as_slice())?.to_owned();

        // Render HTML body
        bytes.truncate(0);

        // Write root_block
        let root_block = doc.root_block();
        html_writer.write_object(&mut bytes, &mut ctx, root_block)?;

        let page_body_html = str::from_utf8(bytes.as_slice())?.to_owned();

        // eprintln!("InternalTemplateRenderer page_body_html: {}", page_body_html);

        Ok(InternalTemplateRenderer {
            template_src: template_src,
            event_keys: event_keys,
            event_enterkeyflags: event_enterkeyflags,
            event_action_keys: event_action_keys,
            event_action_bodies: event_action_bodies,
            route_keys: route_keys,
            route_func_keys: route_func_keys,
            route_bodies: route_bodies,
            reducer_keys: reducer_keys,
            reducer_action_keys: reducer_action_keys,
            reducer_bodies: reducer_bodies,
            reducer_defaults: reducer_defaults,
            extern_reducer_keys: extern_reducer_keys,
            component_names: component_names,
            component_bodies: component_bodies,
            query_names: query_names,
            query_params: query_params,
            query_bodies: query_bodies,
            page_render_func_body: page_render_func_body,
            page_body_html: page_body_html,
        })
    }

    pub fn render(&self) -> DocumentProcessingResult<String> {
        let mut tctx = TrimmerContext::new();
        let src = &self.template_src;

        tctx.set("JS_PRELOAD_SRC", &src.preload_src);

        // Event functions
        tctx.set("PAGE_EVENT_KEYS", &self.event_keys);
        tctx.set("PAGE_EVENT_ENTERKEYFLAGS", &self.event_enterkeyflags);
        tctx.set("PAGE_EVENT_ACTION_KEYS", &self.event_action_keys);
        tctx.set("PAGE_EVENT_ACTION_BODIES", &self.event_action_bodies);

        // Routes
        tctx.set("PAGE_ROUTE_KEYS", &self.route_keys);
        tctx.set("PAGE_ROUTE_FUNC_KEYS", &self.route_func_keys);
        tctx.set("PAGE_ROUTE_BODIES", &self.route_bodies);

        // Reducers
        tctx.set("PAGE_REDUCER_KEYS", &self.reducer_keys);
        tctx.set("PAGE_REDUCER_ACTION_KEYS", &self.reducer_action_keys);
        tctx.set("PAGE_REDUCER_DEFAULTS", &self.reducer_defaults);
        tctx.set("PAGE_REDUCER_BODIES", &self.reducer_bodies);
        tctx.set("PAGE_EXTERN_REDUCER_KEYS", &self.extern_reducer_keys);

        // Components
        tctx.set("PAGE_COMPONENT_KEYS", &self.component_names);
        tctx.set("PAGE_COMPONENT_BODIES", &self.component_bodies);

        // Queries
        tctx.set("PAGE_QUERY_KEYS", &self.query_names);
        tctx.set("PAGE_QUERY_PARAMS", &self.query_params);
        tctx.set("PAGE_QUERY_BODIES", &self.query_bodies);

        // Body
        tctx.set("PAGE_RENDER_FUNC_BODY", &self.page_render_func_body);
        tctx.set("PAGE_BODY_HTML", &self.page_body_html);

        let out_buf = self::render_template(&src.template, &tctx)?;
        Ok(out_buf)
    }
}
