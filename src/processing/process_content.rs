
use parser::ast::*;
use parser::util::allocate_element_key;
use processing::structs::*;
use processing::process_util::*;
use scope::*;


#[derive(Debug, Default)]
pub struct ContentOutputProcessing {
    root_block: BlockProcessingState,
}

#[derive(Debug)]
pub struct ContentOutput {
    pub root_block: BlockProcessingState,
}

impl Into<ContentOutput> for ContentOutputProcessing {
    fn into(self) -> ContentOutput {
        ContentOutput { root_block: self.root_block }
    }
}

#[derive(Debug, Default)]
pub struct ProcessContent {}

impl ProcessContent {
    #[inline]
    #[allow(dead_code)]
    pub fn process_block_contents(&mut self,
                                output: &mut ContentOutputProcessing,

                                ctx: &mut Context,
                                bindings: &mut BindingContext,

                                nodes: &Vec<ContentNodeType>,
                                processing: &DocumentProcessingState)
                                // block: &mut BlockProcessingState,
                                -> DocumentProcessingResult<()> {

            for node in nodes {
                self.process_block_content_node(
                    ctx,
                    bindings,
                    node,
                    &mut output.root_block,
                    processing,
                    None)?;
            }
            Ok(())
    }

    #[inline]
    pub fn process_block_content_node(&mut self,
                                ctx: &mut Context,
                                bindings: &mut BindingContext,

                                node: &ContentNodeType,
                                block: &mut BlockProcessingState,
                                processing: &DocumentProcessingState,
                                parent_tag: Option<&str>)
                                -> DocumentProcessingResult<()> {
        match node {
            &ContentNodeType::ElementNode(ref element_data) => {
                ctx.push_child_scope();
                ctx.append_path_str(&element_data.element_key);
                let complete_key = ctx.join_path(Some("."));
                // let is_void = element_data.children.as_ref().map_or(false, |c| c.len() > 0);

                let element_tag = element_data.element_ty.to_lowercase();

                // let attrs = element_data.attrs.as_ref().map(Clone::clone);
                let lens = element_data.lens.as_ref().map(Clone::clone);
                // let element_bindings = element_data.bindings.as_ref().map(|s| s.clone());

                // TODO: figure out when we want to pass along symbols or values

                // Try to locate a matching component
                if let Some(comp) = processing.comp_map.get(&element_tag) {

                    // Attempt to map lens values
                    // FIXME
                    // let lens = map_lens_using_scope(ctx, bindings, lens.as_ref(), processing).or(lens);
                    let lens = lens.as_ref().map(|lens| map_lens_using_scope(ctx, bindings, lens));

                    // Create list of prop keys
                    let mut prop_list: Option<Vec<PropKey>> = element_data.attrs
                        .as_ref()
                        .map(|s| s.iter().map(|s| s.0.to_owned()).collect());

                    match lens {
                        Some(LensExprType::GetLens(_)) => {
                            if !prop_list.is_some() {
                                prop_list = Some(Default::default());
                            }
                        }
                        Some(LensExprType::ForLens(ref ele_key, _)) => {
                            if !prop_list.is_some() {
                                prop_list = Some(Default::default());
                            }

                            if let &Some(ref ele_key) = ele_key {
                                if let Some(ref mut prop_list) = prop_list {
                                    prop_list.push(ele_key.to_owned());
                                };
                            };
                        }
                        _ => {}
                    };

                    // Render a component during render
                    let component_ty = element_tag.to_owned();
                    block.ops_vec
                        .push(ElementOp::InstanceComponent(component_ty,
                                                           complete_key.clone(),
                                                           parent_tag.map(|s| s.to_owned()),
                                                           prop_list,
                                                           lens));

                    // Add mapping from the instance_key to the component_ty
                    block.compkey_mappings.push((complete_key.to_owned(), element_tag.to_owned()));

                } else {
                    // Treat this as an HTML element
                    // TODO: Support imported elements

                    let props = element_data.attrs.as_ref().map(|attrs| {
                        attrs.iter()
                            .map(|attr| {
                                let expr = attr.1.as_ref().map(|expr| {
                                    if expr.is_literal() {
                                        expr.clone()
                                    } else {
                                        ctx.reduce_expr_or_return_same(expr)
                                    }
                                });

                                (attr.0.clone(), expr)
                            })
                            .collect()
                    });

                    // let mut props = element_data.attrs.as_ref().map(|s| s.clone());
                    let mut event_handlers: EventHandlersVec = Default::default();
                    // let mut events: EventsVec = Default::default();
                    let mut value_binding: ElementValueBinding = Default::default();

                    // Process bindings
                    if let Some(ref element_bindings) = element_data.bindings {
                        // Loop through value bindings first
                        for element_binding in element_bindings {
                            if let &ElementBindingNodeType::ElementValueBindingNode(ref key) =
                                   element_binding {
                                value_binding = Some(key.to_owned());
                            };
                        }

                        //                     param.1 = param.1.as_ref().map(|expr| map_expr_using_scope(expr, processing, &mut block.scope, resolution_mode));

                        // Handle special case
                        if element_tag == "input" {
                            if let Some(ref key) = value_binding {
                                let binding = BindingType::DOMElementAttributeBinding(complete_key.to_owned(), "value".into());
                                ctx.add_sym(key, Symbol::binding(&binding));
                            };
                        }

                        // Loop through the event bindings
                        for element_binding in element_bindings {
                            if let &ElementBindingNodeType::ElementEventBindingNode(ref event_handler) =
                                   element_binding {
                                // let mut event = event.clone();

                                // event.2 = event.2.as_ref().map(|action_ops| {
                                //     self.process_content_action_ops(ctx,
                                //                                     bindings,
                                //                                     action_ops.iter(),
                                //                                     processing)
                                // });

                                // block.events_vec.push((complete_key.clone(),
                                //                        event.0.to_owned(),
                                //                        event.1.to_owned(),
                                //                        event.2.as_ref().map(|s| s.to_owned()),
                                //                        None,
                                //                        Some(block.block_id.to_owned())));

                                let event_handler = ctx.map_event_handler_symbols(event_handler);
                                let event = event_handler.create_event(&complete_key, ctx.scope().id());

                                block.events_vec.push(event);
                                event_handlers.push(event_handler);
                            };
                        }
                    }

                    // This should only be Some if there are actually children
                    if let Some(ref children) = element_data.children {
                        // Push element open
                        let has_len = event_handlers.len() > 0;
                        let event_handlers = if has_len { Some(event_handlers) } else { None };
                        block.ops_vec.push(ElementOp::ElementOpen(element_tag.clone(),
                                                                  complete_key.clone(),
                                                                  props,
                                                                  event_handlers,
                                                                  value_binding));

                        // Push scope
                        ctx.push_child_scope();

                        // Iterate over children
                        for ref child in children {
                            self.process_block_content_node(
                                                      ctx,
                                                      bindings,
                                                      child,
                                                      block,
                                                      processing,
                                                      Some(&element_tag))?;
                        }

                        // Pop scope
                        ctx.pop_scope();

                        // Push element close
                        block.ops_vec.push(ElementOp::ElementClose(element_tag.clone()));
                    } else {
                        let has_len = event_handlers.len() > 0;
                        let event_handlers = if has_len { Some(event_handlers) } else { None };
                        block.ops_vec.push(ElementOp::ElementVoid(element_tag.clone(),
                                                                  complete_key.clone(),
                                                                  props,
                                                                  event_handlers,
                                                                  value_binding));
                    }
                }
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

                // let coll_expr =
                //     map_expr_using_scope(coll_expr, processing, &mut block.scope, resolution_mode);
                let coll_expr = ctx.reduce_expr_or_return_same(coll_expr);

                // Add forvar as a parameter in the symbol map
                // if let &Some(ref ele_key) = ele {
                    // TODO
                    // scope.1.add_loop_var(ele_key);
                // }

                if let &Some(ref nodes) = nodes {
                    for ref node in nodes {
                        // FIXME: forvar resolve
                        self.process_block_content_node(ctx, bindings, node, block, processing, None)?;
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
    pub fn process_content_action_ops<'a, I: IntoIterator<Item = &'a ActionOpNode>>(&mut self,
                                      ctx: &mut Context,
                                      _bindings: &mut BindingContext,

                                      action_ops: I,
                                      _processing: &DocumentProcessingState)
                                      -> Vec<ActionOpNode> {
        action_ops.into_iter().map(|action_op| {
            match action_op {
                &ActionOpNode::DispatchAction(ref action_key, ref action_params) => {
                    // let action_ty = scope.0.make_action_type(&action_key);
                    let action_ty = format!("{}.{}", ctx.join_action_path(Some(".".into())), &action_key);

                    let action_params = action_params.as_ref().map(|action_params| {
                        let mut action_params = action_params.clone();
                        for param in &mut action_params {
                            param.1 = param.1.as_ref().and_then(|expr| {
                                // if let &ExprValue::SymbolReference(ref sym) = expr {
                                //     if let &SymbolReferenceType::UnresolvedReference(ref key) = sym.sym_ref() {
                                //         if let Some(sym) = ctx.resolve_sym(key) {
                                //             return 

                                //         };
                                //         // if let Some(sym) = block.scope.1.element_value_bindings.get(key) {
                                //         //     return Some(ExprValue::SymbolReference(sym.to_owned()));
                                //         // };
                                //     };
                                // };
                                
                                // map_expr_using_scope(expr, processing, &mut block.scope, resolution_mode)
                                ctx.reduce_expr(expr)
                            });
                        }
                        action_params
                    });

                    ActionOpNode::DispatchAction(action_ty, action_params)
                }
            }
        }).collect()
    }
}


#[cfg(test)]
mod tests {
    use super::*;
    use parser::ast::*;
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

        let mut output_processing = ContentOutputProcessing::default();
        let res = processing.process_block_contents(
            &mut output_processing,
            &mut ctx,
            &mut bindings,
            &nodes,
            &mut state);
        let output: ContentOutput = output_processing.into();

        assert!(res.is_ok());
        assert_eq!(output.root_block.ops_vec, vec![
            ElementOp::ElementOpen("div".into(), "Ab".into(), None, None, None),
            ElementOp::ElementOpen("span".into(), "Ab.Cd".into(), None, None, None),
            ElementOp::WriteValue(ExprValue::LiteralString("hi".into()), "Ab.Cd.45daee35".into()),
            ElementOp::ElementClose("span".into()),
            ElementOp::ElementClose("div".into())
        ]);

    }
}