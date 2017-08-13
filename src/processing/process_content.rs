
use std::clone::Clone;

use linked_hash_map::LinkedHashMap;

use parser::ast::*;
use parser::store::*;
use parser::api::*;
use parser::util::allocate_element_key;
use processing::structs::*;
use processing::scope::*;
use processing::process_util::*;


#[derive(Debug, Default)]
pub struct ProcessContent {
    // root_node: &'input ContentNodeType,
    root_block: BlockProcessingState,
    base_scope: ElementOpScope,
    scopes: LinkedHashMap<String, ElementOpScope>
}

impl ProcessContent {
    // pub fn with_root_node(root_node: &'input ContentNodeType) -> Self {
    //     ProcessContent {
    //         // root_node: root_node,
    //         root_block: Default::default(),
    //         base_scope: Default::default(),
    //         scopes: Default::default()
    //     }
    // }

    #[inline]
    pub fn process_content_node(&mut self,
                            node: &ContentNodeType,
                            processing: &DocumentProcessingState,
                            block: &mut BlockProcessingState,
                            resolution_mode: &BareSymbolResolutionMode)
                            -> DocumentProcessingResult<()> {
        match node {
            &ContentNodeType::ElementNode(ref element_data) => {
                let element_tag = element_data.element_ty.to_lowercase();
                let element_key =
                    element_data.element_key.as_ref().map_or(String::from(""), Clone::clone);

                let attrs = element_data.attrs.as_ref().map(Clone::clone);
                let lens = element_data.lens.as_ref().map(Clone::clone);
                let bindings = element_data.bindings.as_ref().map(|s| s.clone());

                // Try to locate a matching component
                if let Some(..) = processing.comp_map.get(element_data.element_ty.as_str()) {

                    // Attempt to map lens values
                    // FIXME
                    let lens = map_lens_using_scope(lens.as_ref(), processing, &mut block.scope);

                    let attrs = match lens {
                        Some(LensExprType::GetLens(ref sym)) => {
                            let mut attrs = attrs.as_ref().map_or_else(|| Default::default(), |s| s.clone());

                            if let &SymbolReferenceType::UnresolvedReference(ref key) = sym.sym_ref() {
                                if let Some(ref sym) = resolve_reducer_key(processing, &mut block.scope, key) {
                                    let value = Some(ExprValue::SymbolReference(sym.clone()));
                                    attrs.push((key.clone(), value));
                                };
                            };

                            Some(attrs)
                        }

                        Some(LensExprType::ForLens(ref ele_key, ref coll_sym)) => {
                            let mut attrs = attrs.as_ref().map_or_else(|| Default::default(), |s| s.clone());

                            // let resolved = resolve_sym(coll_sym, processing, &mut scope);
                            // let coll_expr = ExprValue::SymbolReference(coll_sym.clone());
                            // let coll_expr = map_expr_using_scope(&coll_expr, processing, &mut block.scope, resolution_mode);
                            // let ele_sym = Symbol::unresolved(ele_key.to_owned());

                            // if let &SymbolReferenceType::UnresolvedReference(ref key) = coll_sym.sym_ref() {
                            //     if let Some(ref sym) = resolve_reducer_key(processing, &mut scope, key) {
                            //         let value = Some(ExprValue::SymbolReference(sym.clone()));
                            //         attrs.push((key.clone(), value));
                            //     };
                            // };

                            if let &Some(ref ele_key) = ele_key {
                                let sym = Symbol::prop(ele_key);
                                let value = Some(ExprValue::SymbolReference(sym.clone()));
                                attrs.push((ele_key.clone(), value));
                            };

                            Some(attrs)
                        }

                        _ => attrs
                    };

                    // Render a component during render
                    block.ops_vec.push(ElementOp::InstanceComponent(element_tag,
                                                                Some(element_key),
                                                                attrs,
                                                                lens));

                } else {
                    let mut scope = block.scope.clone();
                    scope.0.append_key(&element_key);

                    // Treat this as an HTML element
                    // TODO: Support imported elements

                    let mut events: Option<Vec<ElementEventBinding>> = Default::default();
                    let mut value_binding: ElementValueBinding = Default::default();

                    // Process bindings
                    if let Some(ref bindings) = element_data.bindings {
                        // Loop through value bindings first
                        for binding in bindings {
                            if let &ElementBindingNodeType::ElementValueBindingNode(ref key) = binding {
                                value_binding = Some(key.to_owned());
                                block.scope.1.add_element_value_binding(key, &element_key);
                            };
                        }

                    //                     param.1 = param.1.as_ref().map(|expr| map_expr_using_scope(expr, processing, &mut block.scope, resolution_mode));

                        // Loop through the event bindings
                        for binding in bindings {
                            if let &ElementBindingNodeType::ElementEventBindingNode(ref event) = binding {
                                let mut event = event.clone();

                                event.2 = event.2.as_ref().map(|action_ops| {
                                    action_ops.iter().map(|action_op| {
                                    // for mut action_op = action_ops {
                                        let mut action_op = action_op.to_owned();
                                        if let ActionOpNode::DispatchAction(_, Some(ref mut action_params)) = action_op {
                                            for param in action_params {
                                                param.1 = param.1.as_ref().map(|expr| {
                                                    if let &ExprValue::SymbolReference(ref sym) = expr {
                                                        if let &SymbolReferenceType::UnresolvedReference(ref key) = sym.sym_ref() {
                                                            if let Some(sym) = block.scope.1.element_value_bindings.get(key) {
                                                                return ExprValue::SymbolReference(sym.to_owned());
                                                            };
                                                        };
                                                    };
                                                    
                                                    map_expr_using_scope(expr, processing, &mut block.scope, resolution_mode)
                                                });
                                            }
                                        };
                                        action_op
                                    }).collect()
                                });

                                let complete_key = scope.0.complete_element_key();
                                let event_element_key = format!("{}.{}", complete_key, element_key);
                                block.events_vec.push((event_element_key,
                                                    event.0.as_ref().map(|s| s.to_owned()),
                                                    event.1.as_ref().map(|s| s.to_owned()),
                                                    event.2.as_ref().map(|s| s.to_owned()),
                                                    None,
                                                    Some(block.block_id.to_owned())));
                                if !events.is_some() { events = Some(Default::default()); }
                                if let Some(ref mut events) = events { events.push(event.clone()); }
                            };
                        }
                    }

                    // Process events
                    // let expr = map_expr_using_scope(expr, processing, &mut block.scope, resolution_mode);

                    // for ref mut event in events {
                    //     if let Some(ref mut action_ops) = event.action_ops {
                    //         for ref mut action_op in action_ops {
                    //             if let &ActionOpNode::DispatchAction(_, ref mut action_params) = action_op {
                    //                 for ref mut param in action_params {
                    //                     param.1 = param.1.as_ref().map(|expr| map_expr_using_scope(expr, processing, &mut block.scope, resolution_mode));
                    //                 }
                    //             }
                    //         }
                    //     };
                    // }

                    // let events = events.as_ref().map(|event| {
                    //     event.action_ops = event.action_ops.as_ref().map(|action_op| {
                    //         if let &ActionOpNode::DispatchAction(_, ref action_params) = action_op {
                    //             if let Some(ref mut action_params) = action_params {
                    //                 let action_params: PropVec = action_params.iter().map(|param| {
                    //                     if let Some(ExprValue::SymbolReference(ref sym)) = param.1 {
                    //                         if let &SymbolReferenceType::UnresolvedReference(ref key) = sym.sym_ref() {
                    //                             if let Some(sym) = scope.1.element_value_bindings.get(key) {
                    //                                 return (param.0.to_owned(), Some(ExprValue::SymbolReference(sym.to_owned())));
                    //                             };
                    //                         };

                    //                         if let Some(ref sym) = resolve_document_symbol(sym, self.doc, &mut scope) {
                    //                             return (param.0.to_owned(), Some(ExprValue::SymbolReference(sym.to_owned())));
                    //                         };
                    //                     };

                    //                     param.to_owned()
                    //                 }).collect();
                    //         }

                    //     })


                    // });

                    // This should only be Some if there are actually children
                    if let Some(ref children) = element_data.children {
                        // Push element open
                        block.ops_vec.push(ElementOp::ElementOpen(element_tag.clone(),
                                                            Some(element_key),
                                                            attrs,
                                                            events,
                                                            value_binding));

                        // Iterate over children
                        for ref child in children {
                            self.process_content_node(child, processing, block, resolution_mode)?;
                        }

                        // Push element close
                        block.ops_vec.push(ElementOp::ElementClose(element_tag.clone()));
                    } else {
                        block.ops_vec.push(ElementOp::ElementVoid(element_tag.clone(),
                                                            Some(element_key),
                                                            attrs,
                                                            events,
                                                            value_binding));
                    }
                }
            }
            &ContentNodeType::ExpressionValueNode(ref expr) => {
                let expr = map_expr_using_scope(expr, processing, &mut block.scope, resolution_mode);

                block.ops_vec.push(ElementOp::WriteValue(expr, Some(allocate_element_key())));
            }
            &ContentNodeType::ForNode(ref ele, ref coll_expr, ref nodes) => {
                let block_id = allocate_element_key().replace("-", "_");
                block.ops_vec.push(ElementOp::StartBlock(block_id.clone()));

                let coll_expr = map_expr_using_scope(coll_expr, processing, &mut block.scope, resolution_mode);

                // Add forvar as a parameter in the symbol map
                if let &Some(ref ele_key) = ele {
                    block.scope.1.add_loop_var(ele_key);
                }

                if let &Some(ref nodes) = nodes {
                    for ref node in nodes {
                        // FIXME: forvar resolve
                        self.process_content_node(node, processing, block, resolution_mode)?;
                    }
                };

                block.ops_vec.push(ElementOp::EndBlock(block_id.clone()));
                block.ops_vec.push(ElementOp::MapCollection(block_id.clone(),
                                                        ele.as_ref().map(Clone::clone),
                                                        coll_expr.clone()));
            }
        }
        (Ok(()))
    }
}