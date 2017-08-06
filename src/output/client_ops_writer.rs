
use std::io;
use std::clone::Clone;
use std::slice::Iter;

use linked_hash_map::LinkedHashMap;

use parser::ast::*;
use parser::util::*;
use processing::structs::*;
use output::client_ops_stream_writer::*;
use output::client_misc::*;
use output::scope::*;


#[derive(Debug)]
pub struct BlockDefinition {
    pub block_id: String,
    pub ops: Vec<ElementOp>
}
pub type BlockMap = LinkedHashMap<String, BlockDefinition>;

pub struct ElementOpsWriter<'input: 'scope, 'scope> {
    pub doc: &'input DocumentState<'input>,
    pub stream_writer: &'scope mut ElementOpsStreamWriter,
    pub scopes: LinkedHashMap<String, ElementOpScope>,
    pub blocks: BlockMap,
    pub cur_block_id: Option<String>
}

impl<'input: 'scope, 'scope> ElementOpsWriter<'input, 'scope> {

    pub fn with_doc(doc: &'input DocumentState<'input>, stream_writer: &'scope mut ElementOpsStreamWriter) -> Self {
        ElementOpsWriter {
            doc: doc,
            stream_writer: stream_writer,
            scopes: Default::default(),
            blocks: Default::default(),
            cur_block_id: None
        }
    }

    #[inline]
    fn write_loop_item(&mut self, w: &mut io::Write, doc: &'input DocumentState, item_expr: &ExprValue, scope: &ElementOpScope, ele: Option<&str>, element_ty: Option<&VarType>, block_id: &str, output_component_contents: bool) -> Result {
        let mut scope = scope.clone();

        if let Some(ele_key) = ele {
            scope.add_loop_var_with_value(ele_key, item_expr);

            // loop_scope.1.symbol_map.insert(ele_key.to_owned(), (Some(SymbolReferenceType::LoopVarReference(ele_key.to_owned())), None));
            // loop_scope = loop_scope
            //     .with_var(ele_key,
            //         SymbolReferenceType::LoopVarReference(ele_key.to_owned()),
            //         element_ty.as_ref().map(Clone::clone),
            //         Some(SymbolValueType::ConstantValue(item_expr.clone()))
            //     );
        };

        let block_ops = self.blocks.get(block_id)
            .map(|block| block.ops.clone());

        if let Some(ref block_ops) = block_ops {
            // Push scope
            let scope_id = format!("{}_{}", scope.0.key_prefix(block_id), 1);
            self.scopes.insert(scope_id, scope.clone());

            // Output ops
            self.write_ops_content(w, block_ops.iter(), doc, &scope, output_component_contents)?;

            // Pop scope
            self.scopes.pop_back();
        };
        Ok(())
    }

    #[inline]
    fn invoke_component_with_props(&mut self, w: &mut io::Write, doc: &'input DocumentState, scope: &ElementOpScope, comp: &Component, props: Option<Iter<Prop>>, output_component_contents: bool) -> Result {
        let mut scope = scope.clone();

        if let Some(props) = props {
            for &(ref key, ref expr) in props {
                if let Some(ref expr) = expr.as_ref().and_then(|expr| reduce_expr(&expr, doc, &scope)) {
                    scope.add_prop_with_value(key, &expr);
                };
            }
        };

        // TODO: Restore props from Component object

        let scope_key = allocate_element_key();
        let scope_id = scope.0.key_prefix(&scope_key);
        self.scopes.insert(scope_id, scope.clone());

        if let Some(ref ops) = comp.ops {
            self.write_ops_content(w, ops.iter(), doc, &scope, output_component_contents)?;
        };
        Ok(())
    }

    #[inline]
    #[allow(dead_code)]
    pub fn write_single_component_instance(&mut self, w: &mut io::Write, op: &ElementOp, doc: &'input DocumentState, scope: &ElementOpScope, comp: &Component, component_key: &str, props: Option<Iter<Prop>>, lens: Option<&LensExprType>, loop_iteration: Option<(&Symbol, i32)>, output_component_contents: bool) -> Result {
        let mut component_id =  format!("{}", component_key);
        // let mut props: Vec<Prop> = props.collect();

        let mut scope = scope.clone();

        let props_iter = props.as_ref().map(|s| s.clone());
        if let Some(props_iter) = props_iter {
            for &(ref key, ref expr) in props_iter {
                if let &Some(ref expr) = expr {
                    if let Some(expr) = reduce_expr(&expr, doc, &scope) {
                        scope.add_prop_with_value(&key, &expr);
                    };
                }
            }
        };

        if let Some(ref loop_iteration) = loop_iteration {
            let loop_idx = loop_iteration.1;
            let loopidx_ref = Symbol::loop_idx("foridx", &component_id);

            component_id = format!("{}_{}", component_id, loop_idx);
            let key_suffix = format!("{}", loop_idx);
            scope.0 = add_key_prefix(&scope.0, &key_suffix);
            scope.0 = with_key_expr_prefix(&scope.0, ExprValue::SymbolReference(loopidx_ref));
        };

        // let props_iter = props.as_ref().map(|s| s.iter());

        // OpenS
        let props_iter = props.as_ref().map(|s| s.clone());
        self.stream_writer.write_op_element_instance_component_open(w, op, doc, &scope, &comp, component_key, component_id.as_str(), props_iter, lens)?;

        if output_component_contents {
            let props_iter = props.as_ref().map(|s| s.clone());
            self.invoke_component_with_props(w, doc, &scope, comp, props_iter, true)?;
            // if let Some(ref ops) = comp.ops {
            //     self.write_ops_content(w, ops.iter(), doc, &scope, output_component_contents)?;
            // };
        };

        // Close
        self.stream_writer.write_op_element_instance_component_close(w, op, doc, &scope, &comp, component_key, component_id.as_str())?;

        Ok(())
    }

    #[inline]
    #[allow(unused_variables)]
    pub fn write_ops_content<'op>(&mut self, w: &mut io::Write, ops: Iter<'op, ElementOp>, doc: &'input DocumentState, scope: &ElementOpScope, output_component_contents: bool) -> Result {
        for op in ops {
            let mut scope = self.scopes.back().map_or(scope.clone(), |s| s.1.clone());

            if output_component_contents {
                if let &ElementOp::EndBlock(..) = op {
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
                &ElementOp::ElementOpen(ref element_tag, ref element_key, ref attrs, ref events) |
                &ElementOp::ElementVoid(ref element_tag, ref element_key, ref attrs, ref events) => {
                    let element_key = element_key.as_ref().map_or("null", |s| s);

                    let attrs = attrs.as_ref().map(|attrs| attrs.iter());
                    let events = events.as_ref().map(|events| events.iter());
                    
                    let element_key = format!("{}", scope.0.key_prefix(element_key));
                    self.stream_writer.write_op_element(w, op, doc, &scope, &element_key, element_tag, is_void, attrs, events)?;
                }
                &ElementOp::ElementClose(ref element_tag) => {
                    // let scope = self.scopes.back().map_or(scope.clone(), |s| s.1.clone());

                    self.stream_writer.write_op_element_close(w, op, doc, &scope, element_tag)?;
                }
                &ElementOp::WriteValue(ref expr, ref value_key) => {
                    // let scope = self.scopes.back().map_or(scope.clone(), |s| s.1.clone());

                    let value_key = value_key.as_ref().map_or("null", |s| s);
                    self.stream_writer.write_op_element_value(w, op, doc, &scope, expr, value_key)?;
                }
                &ElementOp::InstanceComponent(ref component_ty,
                                            ref component_key,
                                            ref props,
                                            ref lens) => {
                    let comp = doc.comp_map.get(component_ty.as_str());
                    if let Some(ref comp) = comp {
                        let component_key = component_key.as_ref().map_or("null", |s| s);
                        let component_id = format!("{}_1", component_key);

                        match lens {
                            &Some(LensExprType::ForLens(ref ele_key, ref coll_sym)) => {
                                let coll_expr = ExprValue::SymbolReference(coll_sym.clone());
                                let coll_expr = reduce_expr(&coll_expr, doc, &scope);

                                let mut props = props.as_ref().map_or(vec![], |p| p.clone());

                                if let &Some(ref ele_key) = ele_key {

                                    // Prepare scope and props
                                    let expr = ExprValue::SymbolReference(Symbol::prop(&ele_key));
                                    props.push((ele_key.to_owned(), Some(expr)));

                                    let mut scope = scope.clone();
                                    scope.1.add_param(ele_key);

                                    if output_component_contents {
                                        if let Some(ExprValue::LiteralArray(Some(ref items))) = coll_expr {
                                            for item_expr in items {
                                                let ele_sym = Symbol::prop(ele_key);
                                                scope.1.add_prop_with_value(ele_key, item_expr);
                                                self.write_single_component_instance(w, op, doc, &scope, comp, component_key, Some(props.iter()), lens.as_ref(), Some((&ele_sym, 1)), output_component_contents)?;
                                            };
                                        };
                                    } else {
                                        self.write_single_component_instance(w, op, doc, &scope, comp, component_key, Some(props.iter()), lens.as_ref(), None, output_component_contents)?;
                                    };
                                };
                            }

                            &Some(LensExprType::GetLens(ref sym)) => {
                                let props_iter = props.as_ref().map(|p| p.iter());
                                self.write_single_component_instance(w, op, doc, &scope, comp, component_key, props_iter, None, None, output_component_contents)?;
                            }

                            _ => {
                                let props_iter = props.as_ref().map(|p| p.iter());
                                self.write_single_component_instance(w, op, doc, &scope, comp, component_key, props_iter, None, None, output_component_contents)?;
                            }
                        };

                        // let props = props.as_ref().map(|s| s.clone());
                        // let lens = lens.as_ref().map(|s| s.clone());

                        // let props_iter = props.as_ref().map(|s| s.iter());

                        // // OpenS
                        // self.stream_writer.write_op_element_instance_component_open(w, op, doc, &scope, &comp, component_key, component_id.as_str(), props_iter.as_ref().map(Clone::clone), lens.as_ref())?;

                        // if output_component_contents {
                        //     self.invoke_component_with_props(w, doc, &scope, comp, props_iter.as_ref().map(Clone::clone), true)?;
                        //     // if let Some(ref ops) = comp.ops {
                        //     //     self.write_ops_content(w, ops.iter(), doc, &scope, output_component_contents)?;
                        //     // };
                        // };

                        // // Close
                        // self.stream_writer.write_op_element_instance_component_close(w, op, doc, &scope, &comp, component_key, component_id.as_str())?;
                    }
                }

                &ElementOp::StartBlock(ref block_id) => {
                    if output_component_contents {
                        // Collect blocks to render
                        self.cur_block_id = Some(block_id.to_owned());
                    } else {
                        // Write function header
                        let loopidx_ref = Symbol::loop_idx("foridx", block_id);
                        scope.0 = with_key_expr_prefix(&scope.0, ExprValue::SymbolReference(loopidx_ref));
                        self.stream_writer.write_op_element_start_block(w, op, doc, &scope, block_id)?;

                        let scope_id = scope.0.key_prefix(block_id);
                        self.scopes.insert(scope_id, scope.clone());
                    };
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
                        let scope = self.scopes.back().unwrap().1.clone();
                        // let mut scope = self.scopes.back().map_or(scope.clone(), |s| s.1.clone());

                        let has_block = self.blocks.contains_key(block_id);
                        if has_block {
                            let ele = ele.as_ref().map(|e| e.as_str());

                            let coll_expr = reduce_expr(coll_expr, doc, &scope);

                            if let Some(ExprValue::LiteralArray(Some(ref items))) = coll_expr {
                                for item_expr in items {
                                    self.write_loop_item(w, doc, item_expr, &scope, ele, None, block_id, output_component_contents)?;
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