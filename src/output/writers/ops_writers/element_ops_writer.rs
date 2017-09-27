
use std::io;
use std::iter;

use parser::*;
use processing::*;
use scope::*;
use output::*;


impl<O: OutputWriter + ElementOpsStreamWriter + ElementOpsUtilWriter + EventCollector> ElementOpsWriter for O {

    fn write_element_op(&mut self, w: &mut io::Write, doc: &Document, ctx: &mut Context, bindings: &BindingContext, op: &ElementOp) -> Result {
        let is_void = match *op { ElementOp::ElementVoid(..) => true, _ => false };

        match *op {
                ElementOp::ElementOpen(ref element_tag,
                                        ref element_key,
                                        ref props,
                                        ref events,
                                        ref value_binding) |
                ElementOp::ElementVoid(ref element_tag,
                                        ref element_key,
                                        ref props,
                                        ref events,
                                        ref value_binding) => {

                    let mut props: Vec<Prop> = props.as_ref().map_or_else(Default::default, |v| v.clone());

                    if element_tag == "a" {
                        let has_events = events.as_ref().map_or(false, |v| !v.is_empty());
                        if has_events {
                            props.insert(0, ("href".into(), Some(ExprValue::LiteralString("#".into()))));
                        }
                    };

                    if element_tag == "input" {
                        let is_checkbox = props.iter().any(|prop| prop.0 == "type" && prop.1.iter().any(|e| e.string_value() == Some("checkbox")));

                        if let Some(ref value_binding) = *value_binding {
                            if is_checkbox {
                                let sym = value_binding.1.replace_type(&VarType::boolean());
                                props.push(("checked".into(), Some(ExprValue::SymbolReference(sym))));
                            } else {
                                let sym = value_binding.1.to_owned();
                                props.push(("value".into(), Some(ExprValue::SymbolReference(sym))));
                            }
                        };
                    };

                    let has_props = !props.is_empty();

                    if has_props {
                        self.write_op_element_open(
                            w,
                            doc,
                            ctx,
                            bindings,
                            element_tag,
                            Some(element_key),
                            is_void,
                            props.iter().map(|p| (p.0.as_ref(), p.1.as_ref())),
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
                            Some(element_key),
                            is_void,
                            iter::empty(),
                            iter::empty(),
                            iter::once(value_binding),
                        )?;
                    };
                }

                ElementOp::ElementClose(ref element_tag) => {
                    self.write_op_element_close(
                        w,
                        doc,
                        ctx,
                        bindings,
                        element_tag,
                    )?;
                }

                ElementOp::WriteValue(ref expr, ref element_key) => {
                    self.write_op_element_value(
                        w,
                        doc,
                        ctx,
                        bindings,
                        expr,
                        element_key
                    )?;
                }

                ElementOp::InstanceComponent(ref component_ty, ref component_key, _, _, ref lens) => {
                    match *lens {
                        Some(LensExprType::ForLens(Some(ref coll_key), ref coll_expr)) => {
                            let binding = ExprValue::Binding(BindingType::MapItemBinding);
                            let props_iter = vec![(coll_key.as_str(), Some(&binding))].into_iter();
                            self.write_map_collection_to_component(w, doc, ctx, bindings, coll_key, coll_expr, Some("div"), component_ty, InstanceKey::Static(component_key), props_iter, iter::empty(), iter::empty())?;
                        }

                        Some(LensExprType::GetLens(ref key, ref expr)) => {
                            let props = vec![(key.as_str(), Some(expr))].into_iter();
                            self.render_component(w, doc, ctx, bindings, Some("div"), component_ty, InstanceKey::Static(component_key), false, props, iter::empty(), iter::empty(), Some(LensItemType::GetLens(key, expr)))?;
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

