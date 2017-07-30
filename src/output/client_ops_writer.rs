
use std::io;
use std::clone::Clone;
use std::borrow::Borrow;
use std::slice::Iter;
use std::marker::PhantomData;
use std::collections::hash_map::HashMap;

use linked_hash_map::LinkedHashMap;

use parser::ast::*;
use parser::util::allocate_element_key;
use parser::store::*;
use processing::structs::*;
use output::scope::*;
use output::client_misc::*;
use output::client_output::*;
use output::client_ops_stream_writer::*;


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
    // pub scopes: LinkedHashMap<String, ScopePrefixes>,
    // pub expr_scopes: LinkedHashMap<String, ExprScopeProcessingState>,
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

                    // let attrs = attrs.as_ref().map(|attrs| attrs.clone().iter());
                    // let events = events.as_ref().map(|events| events.clone().iter());

                    let element_key = element_key.as_ref().map_or("null", |s| s);
                    // let element_key = self.scope_prefix(scope_prefix, element_key);

                    let attrs = attrs.as_ref().map(|attrs| attrs.iter());
                    let events = events.as_ref().map(|events| events.iter());
                    
                    let element_key = format!("{}", scope.0.key_prefix(element_key));
                    self.stream_writer.write_op_element(w, op, doc, &scope, &element_key, element_tag, is_void, attrs, events)?;
                }
                &ElementOp::ElementClose(ref element_tag) => {
                    let scope = self.scopes.back().map_or(scope.clone(), |s| s.1.clone());

                    // let scope = self.scopes.scope().unwrap_or(containing_scope);
                    self.stream_writer.write_op_element_close(w, op, doc, &scope, element_tag)?;
                }
                &ElementOp::WriteValue(ref expr, ref value_key) => {
                    let scope = self.scopes.back().map_or(scope.clone(), |s| s.1.clone());

                    // let scope = self.scopes.scope().unwrap_or(containing_scope);
                    let value_key = value_key.as_ref().map_or("null", |s| s);
                    self.stream_writer.write_op_element_value(w, op, doc, &scope, expr, value_key)?;
                }
                &ElementOp::InstanceComponent(ref component_ty,
                                            ref component_key,
                                            ref props,
                                            ref lens) => {
                    let scope = self.scopes.back().map_or(scope.clone(), |s| s.1.clone());
                    // let expr_scope = self.expr_scopes.back().map_or(expr_scope.clone(), |s| s.1.clone());

                    let comp = doc.comp_map.get(component_ty.as_str());
                    if let Some(ref comp) = comp {
//                        let props: Option<Vec<Prop>> = props.as_ref().map(|c| c.iter().map(Clone::clone).collect());
                        let lens = lens.as_ref().map(|s| s.as_str());

                        // let component_id = comp.as_ref().map(|c| c.component_id.clone());
                        let component_key = component_key.as_ref().map_or("null", |s| s);
                        let component_id = format!("{}_1", component_key);

                        let lens_props = props.as_ref().map(|p| p.iter());

                        // OpenS
                        self.stream_writer.write_op_element_instance_component_open(w, op, doc, &scope, &comp, component_key, component_id.as_str(), lens_props, lens)?;

                        if output_component_contents {
                            if let Some(ref ops) = comp.ops {
                                self.write_ops_content(w, ops.iter(), doc, &scope, output_component_contents)?;
                            };
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
                            // let symbol_map = scope.1.symbol_map.clone();
                            // let mut loop_expr_scope = ExprScopeProcessingState { symbol_map: symbol_map };

                            // let mut loop_scope = scope.clone();
                            // if !loop_scope.1.is_some() {
                            //     loop_scope.1 = Some(Default::default());
                            // }

                            // let expr_scope = loop_scope.1.map_or_else(|| Default::default(), |s| s.clone());

                            // // Insert forvar into symbol map
                            // if let &Some(ref ele_key) = ele {
                            //     expr_scope.symbol_map.insert(ele_key.clone(), (Some(SymbolReferenceType::LoopVarReference(ele_key.clone())), None));
                            // };

                            // loop_scope.1 = Some(expr_scope);

                            let mut loop_scope = scope.clone();

                            if let &ExprValue::SymbolReference((Some(SymbolReferenceType::ReducerKeyReference(ref reducer_key)), _)) = coll_expr {
                                if let Some(ref reducer_data) = doc.reducer_key_data.get(reducer_key) {
                                    if let Some(VarType::ArrayVar(Some(ref element_ty))) = reducer_data.ty {
                                        if let Some(ExprValue::LiteralArray(Some(ref items))) = reducer_data.default_expr {
                                            for item_expr in items {
                                                let mut loop_scope = scope.clone();

                                                if let &Some(ref ele_key) = ele {
                                                    loop_scope.1.symbol_map.insert(ele_key.clone(), (Some(SymbolReferenceType::LoopVarReference(ele_key.clone())), None));
                                                    loop_scope = loop_scope
                                                        .with_var(ele_key,
                                                            SymbolReferenceType::LoopVarReference(ele_key.to_owned()),
                                                            Some(*element_ty.clone()),
                                                            Some(SymbolValueType::ConstantValue(item_expr.clone()))
                                                        );
                                                };

                                                let block_ops = self.blocks.get(block_id)
                                                    .map(|block| block.ops.clone());

                                                if let Some(ref block_ops) = block_ops {
                                                    // Output ops
                                                    self.write_ops_content(w, block_ops.iter(), doc, &loop_scope, output_component_contents)?;

                                                };
                                            };
                                            continue;
                                        };
                                        
                                        if let &Some(ref ele_key) = ele {
                                            loop_scope.1.symbol_map.insert(ele_key.clone(), (Some(SymbolReferenceType::LoopVarReference(ele_key.clone())), None));
                                            loop_scope = loop_scope
                                                .with_var(ele_key,
                                                    SymbolReferenceType::LoopVarReference(ele_key.to_owned()),
                                                    Some(VarType::Primitive(PrimitiveVarType::StringVar)),
                                                    Some(SymbolValueType::ConstantValue(ExprValue::LiteralString("test".to_owned())))
                                                );
                                        };



                                    };
                                };
                            };

                            if let &Some(ref ele_key) = ele {
                                loop_scope.1.symbol_map.insert(ele_key.clone(), (Some(SymbolReferenceType::LoopVarReference(ele_key.clone())), None));
                                loop_scope = loop_scope
                                    .with_var(ele_key,
                                        SymbolReferenceType::LoopVarReference(ele_key.to_owned()),
                                        Some(VarType::Primitive(PrimitiveVarType::StringVar)),
                                        Some(SymbolValueType::ConstantValue(ExprValue::LiteralString("test".to_owned())))
                                    );
                            };

                            let block_ops = self.blocks.get(block_id)
                                .map(|block| block.ops.clone());

                            if let Some(ref block_ops) = block_ops {
                                // Output ops
                                self.write_ops_content(w, block_ops.iter(), doc, &loop_scope, output_component_contents)?;
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

// pub trait ComponentStreamWriter<'input> {
//     fn write_js_event_bindings(&self, w: &mut io::Write, events_iter: Iter<EventsItem>, scope_prefix: Option<&ScopePrefixType>) -> Result;
//     fn write_store_definition(&mut self, w: &mut io::Write, doc: &DocumentState, scope_prefix: Option<&ScopePrefixType>) -> Result;
//     fn write_component_definitions(&mut self, w: &mut io::Write, comp: &'input Iter<Component>, doc: &DocumentState, scope_prefix: Option<&ScopePrefixType>) -> Result;
//     fn write_component_definition(&mut self, w: &mut io::Write, comp: &Component, doc: &'input DocumentState, scope_prefix: Option<&ScopePrefixType>) -> Result;
// }

// pub trait ReducerActionStreamWriter<'input> {
//     fn write_reducer_action(&mut self, w: &mut io::Write, reducer_key: &'input str, reducer_data: &'input ReducerKeyData, action_data: &'input ReducerActionData, action_ty: Option<&str>, doc: &DocumentState, scope_prefix: Option<&ScopePrefixType>) -> Result;
// }

// pub trait JsValueStreamWriter<'input> {
//     fn write_js_var_reference(&mut self, w: &mut io::Write, var_name: Option<&str>, doc: &DocumentState, scope_prefix: Option<&ScopePrefixType>) -> Result;
//     fn write_js_expr_value(&mut self, w: &mut io::Write, node: &ExprValue, doc: &DocumentState, scope_prefix: Option<&ScopePrefixType>) -> Result;
//     fn write_js_props_object(&mut self, w: &mut io::Write, props: Option<Iter<'input, Prop>>, doc: &DocumentState, scope_prefix: Option<&ScopePrefixType>)-> Result;
//     fn write_js_incdom_attr_array(&mut self, w: &mut io::Write, attrs: Option<Iter<'input, Prop>>, doc: &DocumentState, scope_prefix: Option<&ScopePrefixType>, base_key: Option<&str>) -> Result;
// }

// impl<'input: 'scope, 'scope, T, S> WriteOpsContent for S where S: ElementOpsWriter<'input, Output = T> where T: ContentWriter {
