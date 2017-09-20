
use std::io;
use std::iter;

use parser::*;
use processing::*;
use scope::*;
use output::*;


// impl<V: ValueWriter, E: ExpressionWriter<V = V>, S: ElementOpsStreamWriter<E = E>> ElementOpsWriter for DefaultOutputWriter<V, E, S> {
// impl<V: ValueWriter, E: ExpressionWriter<V = V>, S: ElementOpsStreamWriter<E = E>> ElementOpsWriter for DefaultOutputWriter<V, E, S> {
// impl<O: OutputWriter> ElementOpsWriter for DefaultOutputWriter<<O as OutputWriter>::V, <<O as OutputWriter>::V as ValueWriter>::E, <O as OutputWriter>::S> {
// impl<V: ValueWriter, E: ExpressionWriter<V = V>, S: ElementOpsStreamWriter<O = Self>> ElementOpsWriter for DefaultOutputWriter<E, S> {
//     type O = O;

//     fn write_element_op(&mut self, w: &mut io::Write, ctx: &mut Context, bindings: &BindingContext, op: &ElementOp) -> Result {
//         write_element_op(w, self, &mut self.stream_writer, ctx, bindings, op)
//     }

//     fn write_element_ops<'a, I: IntoIterator<Item = &'a ElementOp>>(&mut self, w: &mut io::Write, ctx: &mut Context, bindings: &BindingContext, ops: I) -> Result {
//         for op in ops {
//             write_element_op(w, self, &mut self.stream_writer, ctx, bindings, op)?;
//         }
//         Ok(())
//     }
// }

impl<O: OutputWriter + ElementOpsStreamWriter + ElementOpsUtilWriter + EventCollector> ElementOpsWriter for O {
    // type O = O;

    fn write_element_op(&mut self, w: &mut io::Write, doc: &Document, ctx: &mut Context, bindings: &BindingContext, op: &ElementOp) -> Result {
        // write_element_op(w, self, ctx, bindings, op)
        let is_void = if let &ElementOp::ElementVoid(..) = op { true } else { false };

        match op {
                &ElementOp::ElementOpen(ref element_tag,
                                        ref element_key,
                                        ref props,
                                        ref events,
                                        ref value_binding) |
                &ElementOp::ElementVoid(ref element_tag,
                                        ref element_key,
                                        ref props,
                                        ref events,
                                        ref value_binding) => {

                    let mut props: Vec<Prop> = props.as_ref().map_or_else(|| Default::default(), |v| v.clone());

                    if element_tag == "a" {
                        let has_events = events.as_ref().map_or(false, |v| v.len() > 0);
                        if has_events {
                            props.insert(0, ("href".into(), Some(ExprValue::LiteralString("#".into()))));
                        }
                    };

                    if element_tag == "input" {
                        let is_checkbox = props.iter().any(|prop| prop.0 == "type" && prop.1.iter().any(|e| e.string_value() == Some("checkbox")));

                        if let &Some(ref value_binding) = value_binding {
                            if is_checkbox {
                                let sym = value_binding.1.replace_type(&VarType::boolean());
                                props.push(("checked".into(), Some(ExprValue::SymbolReference(sym))));
                            } else {
                                let sym = value_binding.1.to_owned();
                                props.push(("value".into(), Some(ExprValue::SymbolReference(sym))));
                            }
                        };
                    };

                    // let has_props = props.as_ref().map_or(false, |v| v.len() > 0);
                    let has_props = !props.is_empty();

                    if has_props {
                        self.write_op_element_open(
                            w,
                            doc,
                            ctx,
                            bindings,
                            element_tag,
                            element_key,
                            is_void,
                            props.iter().map(|p| (p.0.as_ref(), p.1.as_ref().map(|s| s))),
                            iter::empty(),
                            iter::once(value_binding),
                        )?;
                    } else {
                        self.write_op_element_open(
                            w,
                            doc,
                            ctx,
                            bindings,
                            element_tag,
                            element_key,
                            is_void,
                            iter::empty(),
                            iter::empty(),
                            iter::once(value_binding),
                        )?;
                    };
                }

                &ElementOp::ElementClose(ref element_tag) => {
                    self.write_op_element_close(
                        w,
                        doc,
                        ctx,
                        bindings,
                        element_tag,
                    )?;
                }

                &ElementOp::WriteValue(ref expr, ref element_key) => {
                    self.write_op_element_value(
                        w,
                        doc,
                        ctx,
                        bindings,
                        expr,
                        element_key
                    )?;
                }

                &ElementOp::InstanceComponent(ref component_ty, ref component_key, _, ref props, ref lens) => {
                    match lens {
                        &Some(LensExprType::ForLens(Some(ref coll_key), ref coll_expr)) => {
                            let props = vec![(coll_key.to_owned(), Some(ExprValue::Binding(BindingType::MapItemBinding)))];
                            self.write_map_collection_to_component(w, doc, ctx, bindings, coll_key, coll_expr, Some("div"), component_ty, InstanceKey::Static(component_key), props.iter().map(|p| (p.0.as_ref(), p.1.as_ref().map(|s| s))), iter::empty(), iter::empty())?;
                        }

                        &Some(LensExprType::GetLens(ref key, ref expr)) => {
                            // let expr = if let Some(sym) = ctx.resolve_sym(key) { ExprValue::SymbolReference(sym) } else { ExprValue::Binding(BindingType::ComponentPropBinding(key.to_owned())) };
                            // let props = vec![(key.to_owned(), Some(expr))];
                            // let props = vec![(key.to_owned(), Some(ExprValue::Binding(BindingType::MapItemBinding)))];
                            // self.render_component(w, doc, ctx, bindings, Some("div"), component_ty, InstanceKey::Static(component_key), false, props.iter().map(|p| (p.0.as_ref(), p.1.as_ref().map(|s| s))), iter::empty(), iter::empty(), Some(LensItemType::GetLens(key, expr)))?;

                            let props = vec![(key.as_str(), Some(expr))].into_iter();
                            self.render_component(w, doc, ctx, bindings, Some("div"), component_ty, InstanceKey::Static(component_key), false, props, iter::empty(), iter::empty(), Some(LensItemType::GetLens(key, expr)))?;

                            // self.render_component(w, doc, ctx, bindings, Some("div"), component_ty, InstanceKey::Static(component_key), false, props.as_ref().map(|p| p.into_iter().map(|s| s.as_str())), iter::empty(), iter::empty(), Some(LensItemType::GetLens(&expr)))?;
                        }

                        _ => {
                            self.write_op_element_instance_component(w, doc, ctx, bindings, component_ty, component_key, true, None, None, None)?;
                        }
                    }
                }

                _ => {}
        };

        Ok(())
    }

    fn write_element_ops<'a, I: IntoIterator<Item = &'a ElementOp>>(&mut self, w: &mut io::Write, doc: &Document, ctx: &mut Context, bindings: &BindingContext, ops: I) -> Result {
        for op in ops {
            self.write_element_op(w, doc, ctx, bindings, op)?;
        }
        Ok(())
    }
}

