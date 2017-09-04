
use std::io;
use std::iter;

use processing::structs::*;
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

impl<O: OutputWriter + ElementOpsStreamWriter> ElementOpsWriter for O {
    // type O = O;

    fn write_element_op(&mut self, w: &mut io::Write, doc: &DocumentState, ctx: &mut Context, bindings: &BindingContext, op: &ElementOp) -> Result {
        // write_element_op(w, self, ctx, bindings, op)
        let is_void = if let &ElementOp::ElementVoid(..) = op { true } else { false };

        match op {
                &ElementOp::ElementOpen(ref element_tag,
                                        ref element_key,
                                        ref _props,
                                        ref _events,
                                        ref _value_binding) |
                &ElementOp::ElementVoid(ref element_tag,
                                        ref element_key,
                                        ref _props,
                                        ref _events,
                                        ref _value_binding) => {

                    self.write_op_element_open(
                        w,
                        ctx,
                        bindings,
                        element_tag,
                        element_key,
                        is_void,
                        iter::empty(),
                        iter::empty(),
                        iter::empty(),
                    )?;

                }

                &ElementOp::ElementClose(ref element_tag) => {
                    self.write_op_element_close(
                        w,
                        ctx,
                        bindings,
                        element_tag,
                    )?;
                }

                &ElementOp::WriteValue(ref expr, ref element_key) => {
                    self.write_op_element_value(
                        w,
                        ctx,
                        bindings,
                        expr,
                        element_key
                    )?;
                }

                &ElementOp::InstanceComponent(ref component_ty, ref component_key, _, _, _) => {
                    self.write_op_element_instance_component(
                        w,
                        doc,
                        ctx,
                        bindings,
                        component_ty,
                        component_key,
                        true,
                        None,
                        None,
                        None,
                        iter::empty()
                    )?;
                }

                _ => {}
        };

        Ok(())
    }

    fn write_element_ops<'a, I: IntoIterator<Item = &'a ElementOp>>(&mut self, w: &mut io::Write, doc: &DocumentState, ctx: &mut Context, bindings: &BindingContext, ops: I) -> Result {
        for op in ops {
            self.write_element_op(w, doc, ctx, bindings, op)?;
        }
        Ok(())
    }
}

