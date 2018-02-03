use std::fmt::Debug;
use std::rc::Rc;
use std::collections::HashMap;
use std::collections::HashSet;

use linked_hash_map::LinkedHashMap;

use util::*;
use error::*;
use traits::*;
use expressions::*;
use objects::*;
use ast::*;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Document {
    root_block: Block<ProcessedExpression>,
    reducers: LinkedHashMap<String, Reducer<ProcessedExpression>>,
    extern_reducers: Vec<ExternReducerNode>,
    default_reducer_key: Option<String>,

    components: LinkedHashMap<String, Component<ProcessedExpression>>,
    queries: LinkedHashMap<String, Query<ProcessedExpression>>,
    routes: LinkedHashMap<String, Route<ProcessedExpression>>,

    event_bindings: Vec<ElementEventBindingName<ProcessedExpression>>,
}

impl Document {
    pub fn new(
        root_block: Block<ProcessedExpression>,
        reducers: LinkedHashMap<String, Reducer<ProcessedExpression>>,
        extern_reducers: Vec<ExternReducerNode>,
        default_reducer_key: Option<String>,
        components: LinkedHashMap<String, Component<ProcessedExpression>>,
        queries: LinkedHashMap<String, Query<ProcessedExpression>>,
        routes: LinkedHashMap<String, Route<ProcessedExpression>>,
        event_bindings: Vec<ElementEventBindingName<ProcessedExpression>>,
    ) -> Self {
        Document {
            root_block: root_block,
            reducers: reducers,
            extern_reducers: extern_reducers,
            default_reducer_key: default_reducer_key,
            components: components,
            queries: queries,
            routes: routes,
            event_bindings: event_bindings,
        }
    }

    pub fn root_block<'a>(&'a self) -> &'a Block<ProcessedExpression> {
        &self.root_block
    }

    pub fn reducers<'doc>(
        &'doc self,
    ) -> Option<impl Iterator<Item = (&'doc str, &'doc Reducer<ProcessedExpression>)>> {
        Some(self.reducers.iter().map(|(a, b)| (a.as_str(), b)))
    }

    pub fn reducer<'doc>(&'doc self, key: &str) -> Option<&'doc Reducer<ProcessedExpression>> {
        self.reducers.get(key)
    }

    pub fn extern_reducers<'doc>(&'doc self) -> impl Iterator<Item = &'doc ExternReducerNode> {
        self.extern_reducers.iter()
    }

    pub fn query<'a>(&'a self, name: &str) -> Option<&'a Query<ProcessedExpression>> {
        self.queries.get(name)
    }

    pub fn queries<'a>(
        &'a self,
    ) -> impl Iterator<Item = (&'a str, &'a Query<ProcessedExpression>)> {
        self.queries
            .iter()
            .map(|(name, query)| (name.as_str(), query))
    }

    pub fn component<'a>(&'a self, name: &str) -> Option<&'a Component<ProcessedExpression>> {
        self.components.get(name)
    }

    pub fn components<'a>(&'a self) -> impl Iterator<Item = &'a Component<ProcessedExpression>> {
        self.components.iter().map(|(_, comp)| comp)
    }

    pub fn routes<'a>(&'a self) -> impl Iterator<Item = &'a Route<ProcessedExpression>> {
        self.routes.values()
    }

    pub fn event_bindings<'a>(
        &'a self,
    ) -> impl Iterator<Item = &'a ElementEventBindingName<ProcessedExpression>> {
        self.event_bindings.iter()
    }
}

pub trait ContentProcessingContext<T>: Debug {
    fn add_event_binding(
        &mut self,
        event_binding: ElementEventBindingName<T>,
    ) -> DocumentProcessingResult<()>;
}

#[derive(Default, Debug, Clone, PartialEq, Eq)]
pub struct DefaultContentProcessingContext {
    event_bindings: Vec<ElementEventBindingName<ProcessedExpression>>,
}

impl ContentProcessingContext<ProcessedExpression> for DefaultContentProcessingContext {
    fn add_event_binding(
        &mut self,
        event_binding: ElementEventBindingName<ProcessedExpression>,
    ) -> DocumentProcessingResult<()> {
        self.event_bindings.push(event_binding);
        Ok(())
    }
}

impl DefaultContentProcessingContext {
    fn event_bindings<'a>(
        &'a self,
    ) -> impl Iterator<Item = &'a ElementEventBindingName<ProcessedExpression>> {
        self.event_bindings.iter()
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ContentProcessor {
    component_names: Rc<HashSet<String>>,
    element_key: Vec<String>,
    blocks: LinkedHashMap<String, Block<ProcessedExpression>>,
    ops: Vec<ElementOp<ProcessedExpression>>,
}

impl ContentProcessor {
    pub fn new(component_names: Rc<HashSet<String>>) -> Self {
        ContentProcessor {
            component_names: component_names,
            element_key: Default::default(),
            blocks: Default::default(),
            ops: Default::default(),
        }
    }

    fn process_component_call(
        &mut self,
        ctx: &mut ProcessingContext,
        content_ctx: &mut ContentProcessingContext<ProcessedExpression>,
        attrs: Vec<ElementAttrValue<ProcessedExpression>>,
        el_desc: ElementDescriptor<ProcessedExpression>,
    ) -> DocumentProcessingResult<()> {
        eprintln!(
            ">>>>> Processing component call with tag: {}",
            el_desc.tag()
        );

        let named_props: Vec<_> = attrs
            .iter()
            .filter_map(|p| match *p {
                ElementAttrValue::Prop(ref p) => Some(p.to_owned()),
                _ => None,
            })
            .collect();
        eprintln!("ContentProcessor: element named_props: {:?}", named_props);

        let positionals: Vec<_> = attrs
            .iter()
            .filter_map(|p| match *p {
                ElementAttrValue::Positional(ref p) => Some(p.to_owned()),
                _ => None,
            })
            .collect();
        eprintln!(
            "ContentProcessor: element positional attrs: {:?}",
            positionals
        );

        let first_pos = positionals.get(0);
        eprintln!(
            "ContentProcessor: element first positional argument: {:?}",
            first_pos
        );

        let for_lens = match first_pos {
            Some(&ExpressionValue::Lens(LensValue::ForLens(ref key, box ref expr, _), _)) => {
                Some((key.to_owned(), expr.to_owned()))
            }
            _ => None,
        };
        eprintln!("ContentProcessor: for_lens: {:?}", for_lens);

        let alias_props: Vec<_> = positionals
            .iter()
            .filter_map(|p| match *p {
                ExpressionValue::Lens(ref l, _) => Some(l),
                _ => None,
            })
            .filter_map(|l| match *l {
                LensValue::GetLens(..) | LensValue::QueryLens(..) => Some(l),
                _ => None,
            })
            .map(|l| Ok(ElementPropValue::new(l.default_alias()?, l.expr()?)))
            .collect();
        let alias_props: Vec<_> = ok_or_error(alias_props)?.collect();
        eprintln!("ContentProcessor: alias_props: {:?}", alias_props);

        let for_item_props: Vec<_> = match for_lens {
            Some((Some(ref item_key), _)) => {
                let binding = CommonBindings::CurrentItem(Default::default());
                let prop = ElementPropValue::new(
                    item_key.to_owned(),
                    ExpressionValue::Binding(binding, Default::default()),
                );

                vec![prop]
            }

            _ => Default::default(),
        };

        let component_props: Vec<_> = named_props
            .into_iter()
            .chain(alias_props.into_iter())
            .chain(for_item_props.into_iter())
            .collect();
        eprintln!("ContentProcessor: component_props: {:?}", component_props);

        let comp_desc = ComponentInstanceDescriptor::new(el_desc, None, Some(component_props));

        let inst: ElementOp<ProcessedExpression> = match for_lens {
            Some((ref item_key, ref expr)) => ElementOp::MapInstanceComponent(
                comp_desc,
                item_key.to_owned(),
                expr.to_owned(),
                Default::default(),
            ),

            _ => ElementOp::InstanceComponent(comp_desc, Default::default()),
        };
        eprintln!("ContentProcessor: pushing inst: {:?}", inst);
        self.ops.push(inst);

        Ok(())
    }

    fn process_element(
        &mut self,
        ctx: &mut ProcessingContext,
        content_ctx: &mut ContentProcessingContext<ProcessedExpression>,
        n: &ElementNode<SourceExpression>,
    ) -> DocumentProcessingResult<()> {
        eprintln!("ContentProcessor process_element: n: {:?}", n);

        let tag = n.tag();
        let key = n.key();

        eprintln!(
            ">>>>> Processing element or component call with tag: {}",
            tag
        );

        // FIXME: Ignore values other than props for now, we will want to flatten
        // or wrap the element ops when lens attrs are given

        let attrs: Vec<ElementAttrValue<SourceExpression>> = n.attrs()
            .map(|v| v.map(|e| e.to_owned()).collect())
            .unwrap_or_default();
        let attrs: Vec<ElementAttrValue<ProcessedExpression>> =
            TryProcessFrom::try_process_from(&attrs, ctx)?;

        let props: Vec<_> = n.attrs().map(|v| v.collect()).unwrap_or_default();

        let bindings: Vec<_> = n.bindings().map(|v| v.collect()).unwrap_or_default();

        let props: Vec<ElementPropValue<ProcessedExpression>> =
            ok_or_error(props.into_iter().filter_map(|p| match *p {
                ElementAttrValue::Prop(ref p) => Some(TryProcessFrom::try_process_from(p, ctx)),
                _ => None,
            }))?.collect();

        let value_binding: Option<_> = bindings
            .iter()
            .filter_map(|b| match **b {
                ElementBinding::Value(ref b, _) => Some(b.to_owned()),
                _ => None,
            })
            .nth(0);

        ctx.push_child_scope();
        if let Some(value_binding) = value_binding {
            if let Some(ident) = value_binding.ident() {
                let ident = ident.to_owned();
                let binding = CommonBindings::CurrentElementValue(Default::default());
                let expr = value_binding.expr().to_owned();
                eprintln!(
                    "Binding [{}] as [{:?}] to value [{:?}]",
                    ident, binding, expr
                );
                ctx.bind_ident(ident, binding)?;
            }
        }

        let event_bindings: Vec<_> = ok_or_error(
            bindings
                .iter()
                .filter_map(|e| match *e {
                    &ElementBinding::Event(ref e, _) => Some(e.to_owned()),
                    _ => None,
                })
                .map(|e| {
                    let expr: DocumentProcessingResult<
                        ElementEventBinding<ProcessedExpression>,
                    > = TryProcessFrom::try_process_from(&e, ctx);
                    expr.and_then(|e| Ok(ElementEventBindingName::create(e)))
                }),
        )?.collect();
        ctx.pop_scope();

        let desc = ElementDescriptor::new(
            tag.to_owned(),
            key.to_owned(),
            props,
            Some(event_bindings),
            None,
            false,
        );

        // Invoke component
        if self.component_names.contains(tag) {
            return self.process_component_call(ctx, content_ctx, attrs, desc);
        }

        if let Some(events) = desc.events() {
            for event_binding in events {
                content_ctx.add_event_binding(event_binding.to_owned())?;
            }
        };

        let children: Vec<_> = n.children().map(|v| v.collect()).unwrap_or_default();

        let has_children = children.len() > 0;
        let is_script = tag == "script" || tag == "SCRIPT";
        let is_iframe = tag == "iframe" || tag == "IFRAME";
        let is_div = tag == "div" || tag == "DIV";
        let is_span = tag == "span" || tag == "SPAN";
        let is_link = tag == "link" || tag == "LINK";

        let needs_close = is_script || is_iframe || is_div || is_span;
        let has_content = (has_children && !is_link) || needs_close;

        if has_content {
            let open = ElementOp::ElementOpen(desc, Default::default());
            let close = ElementOp::ElementClose(tag.to_owned());

            self.ops.push(open);

            self.element_key.push(key.to_owned());

            for child in children {
                self.process_content_node(ctx, content_ctx, child)?;
            }

            self.element_key.pop();

            self.ops.push(close);
        } else {
            let void_el = ElementOp::ElementVoid(desc, Default::default());

            self.ops.push(void_el);
        }

        Ok(())
    }

    pub fn process_content_node(
        &mut self,
        ctx: &mut ProcessingContext,
        content_ctx: &mut ContentProcessingContext<ProcessedExpression>,
        n: &ContentNode<SourceExpression>,
    ) -> DocumentProcessingResult<()> {
        eprintln!("ContentProcessor process_content_node: n: {:?}", n);

        match *n {
            ContentNode::Element(ref e, _) => self.process_element(ctx, content_ctx, e),

            ContentNode::ExpressionValue(box ref expr, ref key, _) => {
                let expr: ExpressionValue<ProcessedExpression> =
                    TryProcessFrom::try_process_from(expr, ctx)?;
                self.ops.push(ElementOp::WriteValue(expr, key.to_owned()));
                Ok(())
            }

            ContentNode::Extern(ref ext, _) => {
                if let Some(children) = ext.children() {
                    for child in children {
                        match *child {
                            ContentNode::Element(..)
                            | ContentNode::ExpressionValue(..)
                            | ContentNode::Primitive(..) => {
                                self.ops.push(ElementOp::SkipNode);
                            }

                            _ => {}
                        };
                    }
                };
                Ok(())
            }

            _ => Ok(()),
        }
    }

    pub fn into_block(self) -> Block<ProcessedExpression> {
        eprintln!("ContentProcessor into_block");
        let block_id = allocate_element_key();
        eprintln!("ContentProcessor into_block: block_id: {}", block_id);
        let ops = self.ops;
        // let event_bindings = self.event_bindings;

        eprintln!("ContentProcessor into_block: ops: {:?}", ops);
        Block::new(block_id, None, Some(ops))
    }
}

impl TryProcessFrom<Template> for Document {
    fn try_process_from(
        ast: &Template,
        ctx: &mut ProcessingContext,
    ) -> DocumentProcessingResult<Document> {
        // Collect values for entire document
        let mut content_ctx: DefaultContentProcessingContext = Default::default();

        //
        // Store and reducers
        //

        let default_reducer_key: Option<String> = None;

        let store_definition = ast.children()
            .filter_map(|n| match *n {
                TemplateNode::StoreDefinition(ref n, _) => Some(n),
                _ => None,
            })
            .nth(0);

        let root_children = store_definition.and_then(|n| n.children());
        let root_children: Vec<_> = root_children.map(|v| v.collect()).unwrap_or_default();

        let root_default_nodes: Vec<_> = root_children
            .iter()
            .filter_map(|n| match **n {
                StoreRootScopeNode::Common(ref c, _) => Some(c),
                _ => None,
            })
            .filter_map(|n| match *n {
                StoreCommonNode::LetNode(ref s, ref e, _) => Some((s.to_owned(), e.to_owned())),
                _ => None,
            })
            .collect();

        let reducer_defaults: HashMap<_, _> = root_default_nodes.into_iter().collect();

        let child_scopes: Vec<_> = root_children
            .iter()
            .filter_map(|n| match *n {
                &StoreRootScopeNode::Common(
                    StoreCommonNode::ChildScopeNode(ref scope, ref children),
                    _,
                ) => Some((scope, children)),
                _ => None,
            })
            .collect();

        let extern_reducers: Vec<_> = root_children
            .iter()
            .filter_map(|n| match *n {
                &StoreRootScopeNode::Common(StoreCommonNode::ExternReducerNode(ref node, _), _) => {
                    Some(node.to_owned())
                }
                _ => None,
            })
            .collect();

        let root_reducers: Vec<_> = child_scopes
            .into_iter()
            .map(|(scope, children)| {
                let children: Vec<_> = children
                    .as_ref()
                    .map(|v| v.into_iter().collect())
                    .unwrap_or_default();

                let actions: Vec<
                    DocumentProcessingResult<ReducerAction<SourceExpression>>,
                > = children
                    .iter()
                    .filter_map(|n| match *n {
                        &StoreChildScopeNode::Action(ref n, _) => Some(n.to_owned()),
                        _ => None,
                    })
                    .map(|action| action.map_idents(ctx))
                    .collect();
                let actions: Vec<_> = ok_or_error(actions)?.collect();

                let default_value = reducer_defaults
                    .get(scope.as_str())
                    .and_then(|e| e.to_owned());

                let shape = default_value.as_ref().map(|e| e.shape());
                let reducer = Reducer::new(scope.to_owned(), Some(actions), default_value, shape);

                Ok(reducer)
            })
            .collect();

        let root_reducers: Vec<_> = ok_or_error(root_reducers)?.collect();

        let root_reducers: Vec<Reducer<ProcessedExpression>> = ok_or_error(
            root_reducers
                .into_iter()
                .map(|r| TryProcessFrom::try_process_from(&r, ctx)),
        )?.collect();

        let reducers: LinkedHashMap<String, Reducer<ProcessedExpression>> = root_reducers
            .into_iter()
            .map(|r| (r.key().to_owned(), r))
            .collect();

        eprintln!("Document: reducers: {:?}", reducers);

        //
        // Add reducer keys to context so they are available to lenses
        //

        for reducer_key in reducers.keys() {
            ctx.add_reducer_key(reducer_key.to_owned())?;
        }

        // Queries
        let queries: Vec<_> = ast.children()
            .filter_map(|n| match *n {
                TemplateNode::QueryDefinition(ref q, _) => Some(q),
                _ => None,
            })
            .collect();

        let queries: Vec<Query<ProcessedExpression>> = ok_or_error(
            queries
                .into_iter()
                .map(|r| TryProcessFrom::try_process_from(r, ctx)),
        )?.collect();

        let queries: LinkedHashMap<String, Query<ProcessedExpression>> = queries
            .into_iter()
            .map(|r| (r.name().to_owned(), r))
            .collect();

        //
        // Component definitions
        //
        let mut components: LinkedHashMap<String, Component<ProcessedExpression>> =
            Default::default();

        let component_nodes: Vec<_> = ast.children()
            .filter_map(|n| match *n {
                TemplateNode::ComponentDefinition(ref n, _) => Some(n),
                _ => None,
            })
            .collect();

        // Extract names first
        let component_names: Rc<HashSet<_>> = Rc::new(
            component_nodes
                .iter()
                .map(|n| n.name().to_owned())
                .collect(),
        );

        for component_node in component_nodes {
            let children: Vec<_> = component_node
                .children()
                .map(|v| v.collect())
                .unwrap_or_default();
            let mut content_processor = ContentProcessor::new(component_names.clone());

            ctx.push_child_scope();

            // Define props
            if let Some(params) = component_node.params() {
                for ident in params {
                    let ident = ident.to_owned();
                    let binding =
                        CommonBindings::NamedComponentProp(ident.clone(), Default::default());
                    eprintln!(
                        "Binding component definition prop [{}] as [{:?}]",
                        &ident, &binding
                    );
                    ctx.bind_ident(ident, binding)?;
                }
            }

            for ref child in children {
                content_processor.process_content_node(ctx, &mut content_ctx, child)?;
            }
            let block = content_processor.into_block();

            let name = component_node.name().to_owned();
            let params: Option<Vec<_>> = component_node
                .params()
                .map(|v| v.map(|s| s.to_owned()).collect());

            let component = Component::new(name.to_owned(), FormalParams::new(params), block);

            components.insert(name, component);

            ctx.pop_scope();
        }

        //
        // Content and elements
        //

        let content_nodes: Vec<_> = ast.children()
            .filter_map(|n| match *n {
                TemplateNode::Content(ref n, _) => Some(n.to_owned()),
                _ => None,
            })
            .collect();
        eprintln!("Document: content_nodes: {:?}", content_nodes);

        let mut content_processor: ContentProcessor = ContentProcessor::new(component_names);
        for ref n in content_nodes {
            eprintln!("Document: process content node: {:?}", n);
            content_processor.process_content_node(ctx, &mut content_ctx, n)?;
        }

        let root_block = content_processor.into_block();

        eprintln!("Document: Template: {:?}", ast);
        eprintln!("Document: root_block: {:?}", &root_block);

        //
        // Routing
        //

        let routing_nodes: Vec<_> = ast.children()
            .filter_map(|n| match *n {
                TemplateNode::RouteDefinition(ref n, _) => Some(n),
                _ => None,
            })
            .collect();

        eprintln!("Document: routing_nodes: {:?}", routing_nodes);

        let mut routes: LinkedHashMap<String, Route<ProcessedExpression>> = Default::default();

        for routing_node in routing_nodes {
            let pattern = routing_node.pattern().to_owned();
            let route: Route<ProcessedExpression> =
                TryProcessFrom::try_process_from(routing_node, ctx)?;

            eprintln!("Document: inserting route for {}: {:?}", pattern, route);
            routes.insert(pattern, route);
        }

        //
        // Events
        //

        let event_bindings: Vec<ElementEventBindingName<ProcessedExpression>> =
            content_ctx.event_bindings().map(|e| e.to_owned()).collect();

        //
        // Construct Document
        //

        eprintln!("Constructing Document");

        let doc = Document::new(
            root_block,
            reducers,
            extern_reducers,
            default_reducer_key,
            components,
            queries,
            routes,
            event_bindings,
        );

        eprintln!("Completed Constructing Document");
        Ok(doc)
    }
}
