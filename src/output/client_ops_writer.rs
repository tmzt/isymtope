
use std::io;
use std::clone::Clone;
use std::slice::Iter;

use linked_hash_map::LinkedHashMap;

use parser::ast::*;
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
        let mut loop_scope = scope.clone();

        if let Some(ele_key) = ele {
            loop_scope.1.symbol_map.insert(ele_key.to_owned(), (Some(SymbolReferenceType::LoopVarReference(ele_key.to_owned())), None));
            loop_scope = loop_scope
                .with_var(ele_key,
                    SymbolReferenceType::LoopVarReference(ele_key.to_owned()),
                    element_ty.as_ref().map(Clone::clone),
                    Some(SymbolValueType::ConstantValue(item_expr.clone()))
                );
        };

        let block_ops = self.blocks.get(block_id)
            .map(|block| block.ops.clone());

        if let Some(ref block_ops) = block_ops {
            // Output ops
            self.write_ops_content(w, block_ops.iter(), doc, &loop_scope, output_component_contents)?;

        };
        Ok(())
    }

    #[inline]
    fn invoke_component_with_props(&mut self, w: &mut io::Write, doc: &'input DocumentState, scope: &ElementOpScope, comp: &Component, props: Option<Iter<Prop>>, output_component_contents: bool) -> Result {
        let mut prop_scope = scope.clone();

        if let Some(props) = props {
            for &(ref key, ref expr) in props {
                let expr = expr.as_ref().and_then(|expr| reduce_expr(expr, doc, scope));
                let expr_sym = expr.and_then(|s| Some(SymbolValueType::ConstantValue(s.clone())));

                prop_scope = prop_scope
                    .with_var(key,
                        SymbolReferenceType::PropReference(key.to_owned()),
                        None,
                        expr_sym
                    );
            }
        };

        if let Some(ref ops) = comp.ops {
            self.write_ops_content(w, ops.iter(), doc, &prop_scope, output_component_contents)?;
        };
        Ok(())
    }

    #[inline]
    #[allow(unused_variables)]
    pub fn write_ops_content<'op>(&mut self, w: &mut io::Write, ops: Iter<'op, ElementOp>, doc: &'input DocumentState, scope: &ElementOpScope, output_component_contents: bool) -> Result {
        for op in ops {
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
                    let scope = self.scopes.back().map_or(scope.clone(), |s| s.1.clone());

                    let element_key = element_key.as_ref().map_or("null", |s| s);

                    let attrs = attrs.as_ref().map(|attrs| attrs.iter());
                    let events = events.as_ref().map(|events| events.iter());
                    
                    let element_key = format!("{}", scope.0.key_prefix(element_key));
                    self.stream_writer.write_op_element(w, op, doc, &scope, &element_key, element_tag, is_void, attrs, events)?;
                }
                &ElementOp::ElementClose(ref element_tag) => {
                    let scope = self.scopes.back().map_or(scope.clone(), |s| s.1.clone());

                    self.stream_writer.write_op_element_close(w, op, doc, &scope, element_tag)?;
                }
                &ElementOp::WriteValue(ref expr, ref value_key) => {
                    let scope = self.scopes.back().map_or(scope.clone(), |s| s.1.clone());

                    let value_key = value_key.as_ref().map_or("null", |s| s);
                    self.stream_writer.write_op_element_value(w, op, doc, &scope, expr, value_key)?;
                }
                &ElementOp::InstanceComponent(ref component_ty,
                                            ref component_key,
                                            ref props,
                                            ref lens) => {
                    let scope = self.scopes.back().map_or(scope.clone(), |s| s.1.clone());

                    let comp = doc.comp_map.get(component_ty.as_str());
                    if let Some(ref comp) = comp {
                        let props = props.as_ref().map(|s| s.clone());
                        let lens = lens.as_ref().map(|s| s.clone());

                        let component_key = component_key.as_ref().map_or("null", |s| s);
                        let component_id = format!("{}_1", component_key);

                        let props_iter = props.as_ref().map(|s| s.iter());

                        // OpenS
                        self.stream_writer.write_op_element_instance_component_open(w, op, doc, &scope, &comp, component_key, component_id.as_str(), props_iter.as_ref().map(Clone::clone), lens.as_ref())?;

                        if output_component_contents {
                            self.invoke_component_with_props(w, doc, &scope, comp, props_iter.as_ref().map(Clone::clone), true)?;
                            // if let Some(ref ops) = comp.ops {
                            //     self.write_ops_content(w, ops.iter(), doc, &scope, output_component_contents)?;
                            // };
                        };

                        // Close
                        self.stream_writer.write_op_element_instance_component_close(w, op, doc, &scope, &comp, component_key, component_id.as_str())?;
                    }
                }

                &ElementOp::StartBlock(ref block_id) => {
                    if output_component_contents {
                        // Collect blocks to render
                        self.cur_block_id = Some(block_id.to_owned());
                    } else {
                        // Write function header
                        let mut scope = self.scopes.back().map_or(scope.clone(), |s| s.1.clone());

                        let foridx = &format!("__foridx_{}", block_id);
                        let scope_prefixes = with_key_expr_prefix(&scope.0, ExprValue::VariableReference(foridx.clone()));
                        let scope_id = scope_prefixes.key_prefix(block_id);
                        scope.0 = scope_prefixes;
                        self.scopes.insert(scope_id, scope.clone());

                        self.stream_writer.write_op_element_start_block(w, op, doc, &scope, block_id)?;
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
                        let has_block = self.blocks.contains_key(block_id);
                        if has_block {
                            let ele = ele.as_ref().map(|e| e.as_str());

                            if let &ExprValue::SymbolReference((Some(SymbolReferenceType::ReducerKeyReference(ref reducer_key)), _)) = coll_expr {
                                if let Some(ref reducer_data) = doc.reducer_key_data.get(reducer_key) {
                                    if let Some(VarType::ArrayVar(Some(box ref element_ty))) = reducer_data.ty {
                                        if let Some(ExprValue::LiteralArray(Some(ref items))) = reducer_data.default_expr {
                                            for item_expr in items {
                                                self.write_loop_item(w, doc, item_expr, scope, ele, Some(element_ty), block_id, output_component_contents)?;
                                            };
                                            continue;
                                        };
                                    };
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