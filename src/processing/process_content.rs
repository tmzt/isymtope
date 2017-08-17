
use std::clone::Clone;

use linked_hash_map::LinkedHashMap;

use parser::ast::*;
use parser::store::*;
use parser::api::*;
use parser::util::allocate_element_key;
use processing::structs::*;
use processing::scope::*;
use processing::process_util::*;


#[derive(Debug)]
pub struct ProcessContent {
    // root_node: &'input ContentNodeType,
    root_block: BlockProcessingState,
    base_scope: ElementOpScope,
    scopes: LinkedHashMap<String, ElementOpScope>
}

impl ProcessContent {

    pub fn new(base_scope: ElementOpScope) -> Self {
        ProcessContent {
            root_block: Default::default(),
            base_scope: base_scope,
            scopes: Default::default()
        }
    }

    fn scope(&mut self) -> ElementOpScope {
        self.scopes.back().map_or(self.base_scope.clone(), |s| s.1.clone())
    }

    fn push_scope(&mut self, scope: ElementOpScope) {
        let scope_id = scope.0.complete_element_key();
        self.scopes.insert(scope_id, scope);
    }

    fn pop_scope(&mut self) { self.scopes.pop_back(); }

    #[inline]
    pub fn process_content_node(&mut self,
                            node: &ContentNodeType,
                            processing: &DocumentProcessingState,
                            block: &mut BlockProcessingState,
                            parent_tag: Option<&str>,
                            resolution_mode: &BareSymbolResolutionMode)
                            -> DocumentProcessingResult<()> {
        let mut scope = self.scope();

        match node {
            &ContentNodeType::ElementNode(ref element_data) => {
                let is_void = element_data.children.as_ref().map_or(false, |c| c.len() > 0);

                let element_tag = element_data.element_ty.to_lowercase();
                let element_key =
                    element_data.element_key.as_ref().map_or(String::from(""), Clone::clone);

                // let attrs = element_data.attrs.as_ref().map(Clone::clone);
                let lens = element_data.lens.as_ref().map(Clone::clone);
                let bindings = element_data.bindings.as_ref().map(|s| s.clone());

                // TODO: figure out when we want to pass along symbols or values

                // Try to locate a matching component
                if let Some(..) = processing.comp_map.get(element_data.element_ty.as_str()) {

                    // Attempt to map lens values
                    // FIXME
                    let lens = map_lens_using_scope(lens.as_ref(), processing, &mut block.scope);

                    // Create list of prop keys
                    let mut prop_list: Option<Vec<PropKey>> = element_data.attrs.as_ref().map(|s| s.iter().map(|s| s.0.to_owned()).collect());

                    match lens {
                        Some(LensExprType::GetLens(ref sym)) => {
                            if !prop_list.is_some() { prop_list = Some(Default::default()); }
                            // let mut attrs = attrs.as_ref().map_or_else(|| Default::default(), |s| s.clone());

                            // if let &SymbolReferenceType::UnresolvedReference(ref key) = sym.sym_ref() {
                            //     if let Some(ref sym) = resolve_reducer_key(processing, &mut block.scope, key) {
                            //         let value = Some(ExprValue::SymbolReference(sym.clone()));
                            //         // attrs.push((key.clone(), value));
                            //         prop_list.push(key.to_owned());
                            //     };
                            // };
                        }
                        Some(LensExprType::ForLens(ref ele_key, ref coll_sym)) => {
                            if !prop_list.is_some() { prop_list = Some(Default::default()); }

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
                    block.ops_vec.push(ElementOp::InstanceComponent(component_ty,
                                                                Some(element_key),
                                                                parent_tag.map(|s| s.to_owned()),
                                                                prop_list,
                                                                lens));

                } else {
                    let mut scope = scope.clone();
                    scope.0.append_key(&element_key);
                    let complete_key = scope.0.complete_element_key();

                    // Treat this as an HTML element
                    // TODO: Support imported elements

                    let props = element_data.attrs.as_ref().map(|attrs| {
                        attrs.iter().map(|attr| {
                            let expr = attr.1.as_ref().map(|expr| {
                                if expr.is_literal() {
                                    expr.clone()
                                } else {
                                    map_expr_using_scope(expr, processing, &mut block.scope, resolution_mode)
                                }
                            });

                            (attr.0.clone(), expr)
                        }).collect()
                    });

                    // let mut props = element_data.attrs.as_ref().map(|s| s.clone());
                    let mut events: Option<Vec<ElementEventBinding>> = Default::default();
                    let mut value_binding: ElementValueBinding = Default::default();

                    // Process bindings
                    if let Some(ref bindings) = element_data.bindings {
                        // Loop through value bindings first
                        for binding in bindings {
                            if let &ElementBindingNodeType::ElementValueBindingNode(ref key) = binding {
                                value_binding = Some(key.to_owned());
                                block.scope.1.add_element_value_binding(key, &complete_key);
                            };
                        }

                    //                     param.1 = param.1.as_ref().map(|expr| map_expr_using_scope(expr, processing, &mut block.scope, resolution_mode));

                        // Loop through the event bindings
                        for binding in bindings {
                            if let &ElementBindingNodeType::ElementEventBindingNode(ref event) = binding {
                                let mut event = event.clone();

                                event.2 = event.2.as_ref().map(|action_ops| {
                                    action_ops.iter().map(|action_op| {
                                        match action_op {
                                            &ActionOpNode::DispatchAction(ref action_key, ref action_params) => {
                                                let action_ty = scope.0.make_action_type(&action_key);

                                                let action_params = action_params.as_ref().map(|action_params| {
                                                    let mut action_params = action_params.clone();
                                                    for mut param in &mut action_params {
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
                                                    action_params
                                                });

                                                ActionOpNode::DispatchAction(action_ty, action_params)
                                            },
                                            _ => action_op.to_owned()
                                        }
                                    }).collect()
                                });

                                block.events_vec.push((scope.0.complete_element_key(),
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

                    // This should only be Some if there are actually children
                    if let Some(ref children) = element_data.children {
                        // Push element open
                        block.ops_vec.push(ElementOp::ElementOpen(element_tag.clone(),
                                                            Some(complete_key),
                                                            props,
                                                            events,
                                                            value_binding));

                        // Push scope
                        self.push_scope(scope.clone());

                        // Iterate over children
                        for ref child in children {
                            self.process_content_node(child, processing, block, Some(&element_tag), resolution_mode)?;
                        }

                        // Pop scope
                        self.pop_scope();

                        // Push element close
                        block.ops_vec.push(ElementOp::ElementClose(element_tag.clone()));
                    } else {
                        block.ops_vec.push(ElementOp::ElementVoid(element_tag.clone(),
                                                            Some(complete_key),
                                                            props,
                                                            events,
                                                            value_binding));
                    }
                }
            }
            &ContentNodeType::ExpressionValueNode(ref expr) => {
                let mut scope = self.scope();
                let expr = map_expr_using_scope(expr, processing, &mut scope, resolution_mode);

                let value_key = allocate_element_key();
                let complete_key = scope.0.make_complete_element_key_with(&value_key);
                block.ops_vec.push(ElementOp::WriteValue(expr, Some(complete_key.to_owned())));
            }
            &ContentNodeType::ForNode(ref ele, ref coll_expr, ref nodes) => {
                let mut scope = scope.clone();

                let block_id = allocate_element_key().replace("-", "_");
                block.ops_vec.push(ElementOp::StartBlock(block_id.clone()));

                let coll_expr = map_expr_using_scope(coll_expr, processing, &mut block.scope, resolution_mode);

                // Add forvar as a parameter in the symbol map
                if let &Some(ref ele_key) = ele {
                    scope.1.add_loop_var(ele_key);
                }

                if let &Some(ref nodes) = nodes {
                    for ref node in nodes {
                        // FIXME: forvar resolve
                        self.process_content_node(node, processing, block, None, resolution_mode)?;
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