
use std::io;
use std::clone::Clone;
use std::slice::Iter;

use linked_hash_map::LinkedHashMap;

use parser::ast::*;
use parser::util::*;
use processing::structs::*;
use processing::scope::*;
use output::client_ops_stream_writer::*;
use output::client_misc::*;


#[derive(Debug)]
pub struct BlockDefinition {
    pub block_id: String,
    pub ops: Vec<ElementOp>
}
pub type BlockMap = LinkedHashMap<String, BlockDefinition>;

pub struct ElementOpsWriter<'input: 'scope, 'scope> {
    pub doc: &'input DocumentState<'input>,
    pub stream_writer: &'scope mut ElementOpsStreamWriter,
    base_scope: ElementOpScope,
    pub scopes: LinkedHashMap<String, ElementOpScope>,
    pub blocks: BlockMap,
    pub events_vec: EventsVec,
    component_instances: Vec<(String, String)>,
    pub cur_block_id: Option<String>
}

impl<'input: 'scope, 'scope> ElementOpsWriter<'input, 'scope> {

    fn scope(&mut self) -> ElementOpScope {
        self.scopes.back().map_or(self.base_scope.clone(), |s| s.1.clone())
    }

    fn push_scope(&mut self, scope: ElementOpScope) {
        let scope_id = scope.0.complete_element_key();
        self.scopes.insert(scope_id, scope);
    }

    fn push_scope_as(&mut self, scope: ElementOpScope, scope_id: &str) {
        self.scopes.insert(scope_id.to_owned(), scope);
    }

    fn pop_scope(&mut self) { self.scopes.pop_back(); }

    pub fn with_doc(doc: &'input DocumentState<'input>, stream_writer: &'scope mut ElementOpsStreamWriter, base_scope: ElementOpScope) -> Self {
        ElementOpsWriter {
            doc: doc,
            stream_writer: stream_writer,
            base_scope: base_scope,
            scopes: Default::default(),
            blocks: Default::default(),
            events_vec: Default::default(),
            component_instances: Default::default(),
            cur_block_id: None
        }
    }

    pub fn events_iter(&self) -> Iter<EventsItem> {
        self.events_vec.iter()
    }

    pub fn component_instances_iter(&self) -> Iter<(String, String)> {
        self.component_instances.iter()
    }

    #[inline]
    fn write_loop_item(&mut self, w: &mut io::Write, doc: &'input DocumentState, item_expr: &ExprValue, ele: Option<&str>, element_ty: Option<&VarType>, block_id: &str, output_component_contents: bool) -> Result {
        let mut scope = self.scope();
        scope.0.append_key(block_id);

        let complete_key = scope.0.complete_element_key();

        if let Some(ele_key) = ele {
            scope.add_loop_var_with_value(ele_key, item_expr);

            if output_component_contents {
                scope.0.set_index(0);
            } else {
                let sym_expr = ExprValue::SymbolReference(Symbol::loop_idx("foridx", block_id));
                scope.0.set_prefix_expr(&sym_expr);
            };
        };

        let block_ops = self.blocks.get(block_id)
            .map(|block| block.ops.clone());

        if let Some(ref block_ops) = block_ops {
            // Push scope
            self.push_scope_as(scope, &complete_key);

            // Output ops
            self.write_ops_content(w, block_ops.iter(), doc, output_component_contents)?;

            // Pop scope
            self.pop_scope();
        };
        Ok(())
    }

    #[inline]
    fn invoke_component_with_props(&mut self, w: &mut io::Write, doc: &'input DocumentState, comp: &Component, props: Option<Iter<Prop>>, output_component_contents: bool) -> Result {
        let mut scope = self.scope();

        if let Some(props) = props {
            for prop in props {
                if let Some(ref expr) = prop.1 {
                    scope.2.add_prop_with_value(&prop.0, expr);
                };
            }
        };
        // TODO: Merge default props from Component object
        self.push_scope(scope);

        if let Some(ref ops) = comp.ops {
            self.write_ops_content(w, ops.iter(), doc, output_component_contents)?;
        };
        Ok(())
    }

    #[inline]
    #[allow(dead_code)]
    pub fn write_single_component_instance(&mut self, w: &mut io::Write, op: &ElementOp, doc: &'input DocumentState, comp: &Component, component_key: &str, props: Option<Iter<Prop>>, lens: Option<&LensExprType>, loop_iteration: Option<(&Symbol, i32)>, output_component_contents: bool) -> Result {
        let mut scope = self.scope();

        if output_component_contents {
            // Add iteration
            if let Some(ref li) = loop_iteration {
                scope.0.set_index(li.1);
            };

            // Push scope
            self.push_scope(scope.clone());

            let complete_key = scope.0.complete_element_key();

            // OpenS
            // let props_iter = props.as_ref().map(|s| s.clone());
            self.stream_writer.write_op_element_instance_component_open(w, op, doc, &scope, &comp, props.clone(), lens)?;

            // let props_iter = props.as_ref().map(|s| s.clone());
            self.invoke_component_with_props(w, doc, comp, props.clone(), true)?;

            // Close
            self.stream_writer.write_op_element_instance_component_close(w, op, doc, &scope, &comp)?;

            // DISABLE attempt to look up event bindings and write out
            // using the component_instances vector
            // // if let Some(ref events_vec) = events_vec {
            //     if let Some(ref events) = comp.events {
            //         for event in events {
            //             let mut event = event.clone();
            //             event.0 = format!("{}.{}", complete_key, &event.0);
            //             self.events_vec.push(event);
            //         }
            //     };
            // // };

            self.component_instances.push((complete_key.to_owned(), comp.name.to_owned()));

            // Pop scope
            self.pop_scope();
        } else {
            // let scope_key = {
            //     let mut scope = scope.clone();
            //     scope.0.append_key(component_key);
            //     scope.0.complete_element_key()
            // };

            let mut scope = scope.clone();
            // let props = props.as_ref().map(|p| map_prop_references(p.iter(), &scope));

            scope.0.clear_key();
            // scope.0.append_key(component_key);
            if let Some(ref li) = loop_iteration {
                let sym_expr = ExprValue::SymbolReference(li.0.clone());
                scope.0.set_prefix_expr(&sym_expr);
            };

            // let key_expr = ExprValue::LiteralString(format!("{}", component_key));
            // let prefix_expr = scope.0.make_prefix_expr(&key_expr, None);

            // self.scopes.insert(scope_key.to_owned(), scope.clone());

            // OpenS
            // let props_iter = props.as_ref().map(|s| s.clone());
            self.stream_writer.write_op_element_instance_component_open(w, op, doc, &scope, &comp, props.clone(), lens)?;

            // Close
            self.stream_writer.write_op_element_instance_component_close(w, op, doc, &scope, &comp)?;

            // self.scopes.pop_back();
        };

        Ok(())
    }

    #[inline]
    #[allow(unused_variables)]
    pub fn write_ops_content<'op>(&mut self, w: &mut io::Write, ops: Iter<'op, ElementOp>, doc: &'input DocumentState, output_component_contents: bool) -> Result {
        for op in ops {
            let mut scope = self.scope();

            if output_component_contents {
                if let &ElementOp::EndBlock(..) = op {
                    self.pop_scope();
                    self.cur_block_id = None;
                    continue;
                };

                if let Some(ref cur_block_id) = self.cur_block_id {
                    let block = self.blocks.entry(cur_block_id.to_owned())
                        .or_insert_with(|| BlockDefinition { block_id: cur_block_id.clone(), ops: Default::default() });

                    block.ops.push(op.clone());
                    continue;
                };
            };

            let is_void = if let &ElementOp::ElementVoid(..) = op { true } else { false };

            match op {
                &ElementOp::ElementOpen(ref element_tag, ref element_key, ref props, ref events, ref value_binding) |
                &ElementOp::ElementVoid(ref element_tag, ref element_key, ref props, ref events, ref value_binding) => {
                    let mut scope = scope.clone();

                    let props = if output_component_contents {
                        props.as_ref().map(|p| map_props_using_scope(p.iter(), &scope))
                    } else {
                        props.as_ref().map(|p| map_prop_references(p.iter(), &scope))
                    };

                    // let prop_list = prop_list.as_ref().map(|s| s.iter().map(|s| &s));
                    
                    let element_key = element_key.as_ref().map_or("null", |s| s);
                    let complete_key = scope.0.make_complete_element_key_with(element_key);
                    self.push_scope_as(scope.clone(), &complete_key);

                    let events = events.as_ref().map(|events| events.iter());
                    let value_binding = value_binding.as_ref().map(|s| s.clone());
                    
                    self.stream_writer.write_op_element(w, op, doc, &scope, &complete_key, element_tag, is_void, props.as_ref().map(|s| s.iter()), events, value_binding)?;
                    if is_void {
                        // Pop scope for self closing, this fixes issue with ElementVoid which
                        // was not being emitted previously by the parser/processor code.
                        self.pop_scope();
                    };
                }
                &ElementOp::ElementClose(ref element_tag) => {
                    // let scope = self.scopes.back().map_or(scope.clone(), |s| s.1.clone());

                    self.stream_writer.write_op_element_close(w, op, doc, &scope, element_tag)?;
                    self.pop_scope();
                }
                &ElementOp::WriteValue(ref expr, ref value_key) => {
                    // let scope = self.scopes.back().map_or(scope.clone(), |s| s.1.clone());

                    let value_key = value_key.as_ref().map_or("null", |s| s);
                    let complete_key = scope.0.make_complete_element_key_with(value_key);
                    self.stream_writer.write_op_element_value(w, op, doc, &scope, expr, &complete_key)?;
                }
                &ElementOp::InstanceComponent(ref component_ty,
                                            ref component_key,
                                            ref prop_list,
                                            ref lens) => {
                    let mut scope = scope.clone();
                    // let mut prop_list = prop_list.clone();

                    let mut props = if output_component_contents {
                        prop_list.as_ref().map(|p| map_prop_list_using_scope(p.iter(), &scope))
                    } else {
                        prop_list.as_ref().map(|p| map_prop_list_references(p.iter(), &scope))
                    };

                    let component_key = component_key.as_ref().map_or("null", |s| s);

                    let mut write_single = true;

                    if output_component_contents {
                        // Add component key to base key
                        scope.0.append_base_key(&format!("c_{}", component_key));
                        // scope.0.append_key(&s);
                        self.push_scope(scope.clone());

                    }

                    let comp = doc.comp_map.get(component_ty.as_str());
                    if let Some(ref comp) = comp {
                        match lens {
                            &Some(LensExprType::ForLens(ref ele_key, ref coll_sym)) => {
                                let coll_expr = ExprValue::SymbolReference(coll_sym.clone());
                                let coll_expr = reduce_expr(&coll_expr, doc, &scope);

                                if let &Some(ref ele_key) = ele_key {
                                    let mut scope = scope.clone();
                                    scope.1.add_param(ele_key);

                                    if output_component_contents {
                                        if let Some(ExprValue::LiteralArray(Some(ref items))) = coll_expr {
                                            write_single = false;

                                            for (item_idx, item_expr) in items.iter().enumerate() {
                                                let prefix_sym = Symbol::param("key_prefix");
                                                if let Some(ref mut props) = props {
                                                    props.push((ele_key.to_owned(), Some(item_expr.to_owned())));
                                                };
                                                self.write_single_component_instance(w, op, doc, comp, &component_key, props.as_ref().map(|p| p.iter()), lens.as_ref(), Some((&prefix_sym, item_idx as i32)), output_component_contents)?;
                                            };
                                        };
                                    } else {
                                        self.write_single_component_instance(w, op, doc, comp, &component_key, props.as_ref().map(|p| p.iter()), lens.as_ref(), None, output_component_contents)?;
                                   };
                                };
                            }

                            &Some(LensExprType::GetLens(ref sym)) => {
                            }

                            _ => {}
                        };

                        if write_single {
                            self.write_single_component_instance(w, op, doc, comp, &component_key, props.as_ref().map(|p| p.iter()), None, None, output_component_contents)?;
                        };

                        if output_component_contents {
                            self.pop_scope();
                        }
                    }
                }

                &ElementOp::StartBlock(ref block_id) => {
                    let mut scope = self.scope();

                    let complete_key = scope.0.complete_element_key();

                    if output_component_contents {
                        // Collect blocks to render
                        self.cur_block_id = Some(block_id.to_owned());
                    } else {
                        // Write function header
                        let loopidx_expr = ExprValue::SymbolReference(Symbol::loop_idx("foridx", block_id));
                        scope.0.set_prefix_expr(&loopidx_expr);
                        self.stream_writer.write_op_element_start_block(w, op, doc, &scope, block_id)?;
                    };

                    self.push_scope(scope);
                }

                &ElementOp::EndBlock(ref block_id) => {
                    if output_component_contents {
                        // Finish current block
                        self.cur_block_id = None;
                    } else {
                        self.scopes.pop_back();
                        let scope = self.scopes.back().map_or(scope.clone(), |s| s.1.clone());

                        self.stream_writer.write_op_element_end_block(w, op, doc, &scope, block_id)?;
                    };
                }

                &ElementOp::MapCollection(ref block_id, ref ele, ref coll_expr) => {
                    if output_component_contents {
                        let mut scope = self.scope();

                        let has_block = self.blocks.contains_key(block_id);
                        if has_block {
                            let ele = ele.as_ref().map(|e| e.as_str());

                            let coll_expr = reduce_expr(coll_expr, doc, &scope);

                            if let Some(ExprValue::LiteralArray(Some(ref items))) = coll_expr {
                                for item_expr in items {
                                    self.write_loop_item(w, doc, item_expr, ele, None, block_id, output_component_contents)?;
                                };
                            };
                        };
                    } else {
                        let forvar_default = &format!("__forvar_{}", block_id);
                        let scope_id = format!("{}_map", block_id);

                        // Map to block
                        self.stream_writer.write_op_element_map_collection_to_block(w, op, doc, &scope, coll_expr, block_id)?;
                    };
                }
            }
        }

        Ok(())
    }
}