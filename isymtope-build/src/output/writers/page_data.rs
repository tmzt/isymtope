use std::str;
use std::collections::HashMap;
use std::rc::Rc;

use isymtope_ast_common::*;
use output::*;
use input::*;

#[derive(Debug)]
pub struct InternalTemplateData {
    pub base_url: String,
    pub route_keys: Vec<String>,
    pub route_func_keys: HashMap<String, String>,
    pub route_bodies: HashMap<String, String>,
    pub event_keys: Vec<String>,
    pub event_enterkeyflags: HashMap<String, bool>,
    pub event_action_keys: HashMap<String, Vec<String>>,
    pub event_action_bodies: HashMap<String, HashMap<String, String>>,
    pub reducer_keys: Vec<String>,
    pub reducer_action_keys: HashMap<String, Vec<String>>,
    pub reducer_bodies: HashMap<String, HashMap<String, String>>,
    pub reducer_defaults: HashMap<String, String>,
    pub extern_reducer_keys: Vec<String>,
    pub query_names: Vec<String>,
    pub query_params: HashMap<String, Vec<String>>,
    pub query_bodies: HashMap<String, String>,
    pub component_names: Vec<String>,
    pub component_bodies: HashMap<String, String>,
    pub page_render_func_body: String,
    pub page_body_html: String,
}

#[derive(Debug)]
pub struct InternalTemplateDataBuilder {
    document_provider: Rc<DocumentProvider>,
    state_provider: Option<Rc<ReducerStateProvider>>,
    base_url: String
}

impl InternalTemplateDataBuilder {
    pub fn new(document_provider: Rc<DocumentProvider>, state_provider: Option<Rc<ReducerStateProvider>>, base_url: &str) -> Self {
        InternalTemplateDataBuilder {
            document_provider: document_provider,
            state_provider: state_provider,
            base_url: base_url.to_owned()
        }
    }

    pub fn build(&self) -> DocumentProcessingResult<InternalTemplateData> {
        let ref document_provider = self.document_provider;
        let base_url = self.base_url.clone();

        // Initialize output context
        let mut ctx: DefaultOutputContext =
            DefaultOutputContext::create(document_provider.clone(), self.state_provider.as_ref().map(|s| s.clone()));
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

        Ok(InternalTemplateData {
            base_url: base_url,
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
}