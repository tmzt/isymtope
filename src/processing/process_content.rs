
use std::iter;

use model::*;
use parser::util::allocate_element_key;
use processing::*;
use scope::*;


#[derive(Debug, Default)]
pub struct ProcessContent {}

impl ProcessContent {
    #[inline]
    #[allow(dead_code)]
    pub fn process_block_contents<'a, I: IntoIterator<Item = &'a ContentNodeType>>(&mut self, processing: &mut DocumentProcessingState, ctx: &mut Context, bindings: &mut BindingContext, nodes: I) -> Result { 
        let mut root_block = BlockProcessingState::default();

        for node in nodes {
            self.process_block_content_node(
                processing,
                ctx,
                bindings,
                node,
                &mut root_block,
                None)?;
        }

        processing.root_block = root_block;
        Ok(())
    }

    #[inline]
    pub fn process_block_content_node(&mut self,
                                processing: &mut DocumentProcessingState,
                                ctx: &mut Context,
                                bindings: &mut BindingContext,
                                node: &ContentNodeType,
                                block: &mut BlockProcessingState,
                                _: Option<&str>)
                                -> DocumentProcessingResult<()> {
        match node {
            &ContentNodeType::ElementNode(ref element_data) => {
                ctx.push_child_scope();
                ctx.append_path_str(&element_data.element_key);

                let element_tag = element_data.element_ty.to_lowercase();
                // let lens = element_data.lens.as_ref().map(Clone::clone);

                // TODO: figure out when we want to pass along symbols or values

                // Try to locate a matching component
                let has_comp = processing.comp_map.contains_key(&element_tag);

                if has_comp {
                    return self.process_element_as_component(processing, ctx, bindings, element_data, block, Some(&element_tag));
                };

                // Treat this as an HTML element
                // TODO: Support imported elements
                self.process_element_as_element(processing, ctx, bindings, element_data, block, Some(&element_tag))?;

                ctx.pop_scope();
            }
            &ContentNodeType::ExpressionValueNode(ref expr, ref value_key) => {
                let expr = ctx.reduce_expr_or_return_same(expr);

                let complete_key = ctx.join_path(Some("."));
                let complete_key = format!("{}.{}", complete_key, value_key);
                block.ops_vec.push(ElementOp::WriteValue(expr, complete_key.to_owned()));
            }
            &ContentNodeType::ForNode(ref ele, ref coll_expr, ref nodes) => {
                ctx.push_child_scope();

                let block_id = allocate_element_key().replace("-", "_");
                block.ops_vec.push(ElementOp::StartBlock(block_id.clone()));

                let coll_expr = ctx.reduce_expr_or_return_same(coll_expr);

                if let &Some(ref nodes) = nodes {
                    for ref node in nodes {
                        // FIXME: forvar resolve
                        self.process_block_content_node(processing, ctx, bindings, node, block, None)?;
                    }
                };

                block.ops_vec.push(ElementOp::EndBlock(block_id.clone()));
                block.ops_vec.push(ElementOp::MapCollection(block_id.clone(),
                                                            ele.as_ref().map(Clone::clone),
                                                            coll_expr.clone()));

                ctx.pop_scope();
            }
        }
        (Ok(()))
    }

    #[inline]
    pub fn process_element_as_element(&mut self,
                                processing: &mut DocumentProcessingState,
                                ctx: &mut Context,
                                bindings: &mut BindingContext,
                                element_data: &ElementType,
                                block: &mut BlockProcessingState,
                                _: Option<&str>)
                                -> Result {
        let complete_key = ctx.join_path(Some("."));
        let element_tag = element_data.element_ty.to_lowercase();

        let props = element_data.attrs.as_ref().map(|attrs| ctx.map_props(attrs.into_iter()));

        let mut event_handlers: EventHandlersVec = Default::default();
        // let mut events: EventsVec = Default::default();
        let mut value_binding: ElementValueBinding = Default::default();

        // Process bindings
        if let Some(ref element_bindings) = element_data.bindings {
            // Loop through value bindings first
            for element_binding in element_bindings {
                if let &ElementBindingNodeType::ElementValueBindingAsNode(ref key, ref prop_side, ref read_side) = element_binding {
                    let prop_side = ctx.resolve_symbol_to_symbol(prop_side);
                    let read_side = ctx.resolve_symbol_to_symbol(read_side);
                    value_binding = Some((key.to_owned(), prop_side, Some(read_side)));
                };

                if let &ElementBindingNodeType::ElementValueBindingNode(_, ref sym) = element_binding {
                    let resolved_sym = ctx.resolve_symbol_to_symbol(sym);
                    let binding_key = sym.as_path_str().map_or_else(|| allocate_element_key(), |path| path.replace(".", "_"));
                    value_binding = Some((binding_key, resolved_sym, None));
                };
            }

            // Loop through the event bindings
            for element_binding in element_bindings {
                if let &ElementBindingNodeType::ElementEventBindingNode(ref event_handler) =
                        element_binding {

                    ctx.push_child_scope();

                    // Handle special case
                    if element_tag == "input" {
                        let is_checkbox = props.as_ref().map_or(false, |props| props.iter().any(|prop| prop.0 == "type" && prop.1.iter().any(|e| e.string_value() == Some("checkbox"))));
                        value_binding = value_binding.as_ref().map(|value_binding| {
                            let dom_binding: BindingType;
                            if is_checkbox {
                                let key = ReducedValue::Static(StaticValue::StaticString(complete_key.to_owned()));
                                dom_binding = BindingType::DOMInputCheckboxElementCheckedBinding(Box::new(key));
                            } else {
                                dom_binding = BindingType::DOMInputElementValueBinding(complete_key.to_owned());
                            } 

                            // let dom_binding: Symbol = BindingType::DOMInputElementValueBinding(complete_key.to_owned()).into();
                            let dom_binding: Symbol = dom_binding.into();

                            if let Some(ref read_sym) = value_binding.2 {
                                if let Some(ref read_varname) = read_sym.as_single_part_str() {
                                    ctx.add_sym(read_varname, dom_binding.clone());
                                };
                            };

                            let sym = Symbol::initial_value(&value_binding.1, &dom_binding);
                            ctx.add_sym(&value_binding.0, sym.clone());
                            (value_binding.0.to_owned(), sym, value_binding.2.to_owned())
                        });
                    }

                    let binding = BindingType::EventElementValueBinding;
                    ctx.add_sym("value", Symbol::binding(&binding));

                    let event_handler = ctx.map_event_handler_symbols(event_handler);

                    // Collect types for action params
                    if let Some(action_ops) = match &event_handler {
                        &EventHandler::Event(_, _, ref action_ops) => action_ops.as_ref(),
                        &EventHandler::DefaultEvent(_, ref action_ops) => action_ops.as_ref()
                    } {
                        self.process_event_handler_action_param_types(processing, ctx, action_ops.iter())?;
                    };

                    let event = event_handler.create_event(&complete_key, ctx.scope().id());

                    block.events_vec.push(event);
                    event_handlers.push(event_handler);

                    ctx.pop_scope();
                };
            }
        }

        // This should only be Some if there are actually children
        if let Some(ref children) = element_data.children {
            // Push element open
            let event_handlers = if !event_handlers.is_empty() { Some(event_handlers) } else { None };
            block.ops_vec.push(ElementOp::ElementOpen(element_tag.clone(),
                                                        complete_key.clone(),
                                                        props,
                                                        event_handlers,
                                                        value_binding));

            // Push scope
            ctx.push_child_scope();

            // Iterate over children
            for ref child in children {
                self.process_block_content_node(processing, ctx, bindings, child, block, Some(&element_tag))?;
            }

            // Pop scope
            ctx.pop_scope();

            // Push element close
            block.ops_vec.push(ElementOp::ElementClose(element_tag.clone()));
        } else {
            let event_handlers = if !event_handlers.is_empty() { Some(event_handlers) } else { None };
            block.ops_vec.push(ElementOp::ElementVoid(element_tag.clone(),
                                                        complete_key.clone(),
                                                        props,
                                                        event_handlers,
                                                        value_binding));
        }


        Ok(())
    }

    #[inline]
    pub fn process_element_as_component(&mut self,
                                processing: &mut DocumentProcessingState,
                                ctx: &mut Context,
                                bindings: &mut BindingContext,
                                element_data: &ElementType,
                                block: &mut BlockProcessingState,
                                parent_tag: Option<&str>)
                                -> Result {
        let complete_key = ctx.join_path(Some("."));
        let element_tag = element_data.element_ty.to_lowercase();
        let lens = element_data.lens.as_ref();

        let query_lens = {
            let query_expr = match lens { Some(&LensExprType::QueryLens(ref expr)) => Some(expr), _ => None };
            query_expr.and_then(|query_expr| {
                let unresolved_query = match *query_expr { ExprValue::Binding(BindingType::UnresolvedQueryBinding(ref query_ref)) => Some(query_ref), _ => None };
                unresolved_query.and_then(|unresolved_query| {
                    let query_name = unresolved_query.query_name();
                    processing.query_map.get(query_name)
                        .map(|_| {
                            // let query_props: Option<PropVec> = unresolved_query.props_iter().map(|iter| iter.map(|p| (p.0.to_owned(), p.1.map(|expr| expr.to_owned())) ).collect());
                            let query_props: Option<PropVec> = unresolved_query.props_iter().map(|iter| iter.map(|p| (p.to_owned(), None)).collect());
                            let resolved_query = LocalQueryInvocation::new(query_name, query_props, unresolved_query.ty().map(|ty| ty.to_owned()));

                            let binding = ExprValue::Binding(BindingType::LocalQueryBinding(resolved_query));
                            LensExprType::QueryLens(binding)
                        })
                })
            })
        };

        let lens = query_lens.as_ref().or(lens);
        let lens = lens.map(|lens| ctx.map_lens(bindings, lens));
        
        let lens_key = match lens {
            Some(LensExprType::GetLens(ref key, _)) |
            Some(LensExprType::ForLens(Some(ref key), _)) => Some(key.to_owned()),
            _ => None
        };

        // Create list of prop keys
        let prop_list: Option<Vec<_>> = element_data.attrs.as_ref()
                .map(|s| s.iter().map(|s| Some(s.0.to_owned()))
                    .chain(iter::once(lens_key))
                    .flat_map(|m| m)
                    .collect());

        // Render a component during render
        let component_ty = element_tag.to_owned();
        block.ops_vec
            .push(ElementOp::InstanceComponent(component_ty,
                                                complete_key.clone(),
                                                parent_tag.map(|s| s.to_owned()),
                                                prop_list,
                                                lens.clone()));

        // Add mapping from the instance_key to the component_ty
        block.compkey_mappings.push((complete_key.to_owned(), element_tag.to_owned(), None, lens));

        Ok(())
    }

    #[inline]
    pub fn process_event_handler_action_param_types<'a, I: IntoIterator<Item = &'a ActionOpNode>>(&mut self, processing: &mut DocumentProcessingState, ctx: &mut Context, action_ops: I) -> Result {
        for action_op in action_ops.into_iter() {
            match *action_op {
                ActionOpNode::DispatchAction(ref action_name, ref action_params) |
                ActionOpNode::DispatchActionTo(ref action_name, ref action_params, _) => {
                    if let Some(ref action_params) = *action_params {
                        let complete_key = ctx.join_action_path_with(Some("."), action_name);

                        for ref action_param in action_params {
                            if let Some(ty) = action_param.1.as_ref().and_then(|e| e.peek_ty()) {
                                processing.insert_prop_type(&complete_key, &action_param.0, &ty)?;
                            }
                        }
                    }
                }
            }
        }

        Ok(())
    }
}


#[cfg(test)]
mod tests {
    use super::*;
    use model::*;
    use parser::*;
    use scope::scope::*;
    use scope::context::*;
    use scope::bindings::*;


    #[test]
    pub fn tests_processing_process_content_1() {
        let mut ctx = Context::default();
        let mut bindings = BindingContext::default();

        let nodes: Vec<ContentNodeType> = vec![
            ContentNodeType::ElementNode(ElementType { element_ty: "div".into(), element_key: "Ab".into(), attrs: None, lens: None, bindings: None, children: Some(vec![
                ContentNodeType::ElementNode(ElementType { element_ty: "span".into(), element_key: "Cd".into(), attrs: None, lens: None, bindings: None, children: Some(vec![
                    ContentNodeType::ExpressionValueNode(ExprValue::LiteralString("hi".into()), "45daee35".into())
                ])})
            ])})
        ];

        let mut processing = ProcessContent::default();
        let mut state = DocumentProcessingState::default();

        // let mut output_processing = ProcessContent::default();
        let res = processing.process_block_contents(
            &mut state,
            &mut ctx,
            &mut bindings,
            &nodes);
        let doc: Document = state.into();
        // let output: ContentOutput = output_processing.into();

        assert!(res.is_ok());
        let ops: Option<OpsVec> = doc.root_block().ops_iter().map(|ops_iter| ops_iter.into_iter().map(|o| o.to_owned()).collect());
        assert_eq!(ops, Some(vec![
            ElementOp::ElementOpen("div".into(), "Ab".into(), None, None, None),
            ElementOp::ElementOpen("span".into(), "Ab.Cd".into(), None, None, None),
            ElementOp::WriteValue(ExprValue::LiteralString("hi".into()), "Ab.Cd.45daee35".into()),
            ElementOp::ElementClose("span".into()),
            ElementOp::ElementClose("div".into())
        ]));
    }
}