
use std::io;
use std::iter;

use parser::ast::*;
use processing::structs::*;
use scope::context::*;
use scope::bindings::*;
use output::writers::*;


fn write_element_op<S: ElementOpsStreamWriter>(w: &mut io::Write, stream_writer: &mut S, expression_writer: &mut S::E, value_writer: &mut <S::E as ExpressionWriter>::V, ctx: &mut Context, bindings: &BindingContext, op: &ElementOp) -> Result {

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
                // Only push prefixes for components and other cases where the prefix can vary when rendering.
                // ctx.push_child_scope();
                // ctx.append_path_str(element_key);

                // let props = if output_component_contents {
                //     props.as_ref().map(|props| props.iter().map(|p| ctx.reduce_expr()))
                //     // props.as_ref().map(|p| map_props_using_scope(p.iter(), &scope))
                // } else {
                //     props.as_ref().map(|props| props.iter().map(|p| ctx.reduce_expr()))
                //     // props.as_ref().map(|p| map_prop_references(p.iter(), &scope))
                // };

                // let props = props.as_ref().map(|props| props.iter().filter_map(|p| p.1.as_ref().map(|expr| ctx.reduce_expr(expr))));


                // let prop_list = prop_list.as_ref().map(|s| s.iter().map(|s| &s));

                // let complete_key = scope.0.make_complete_element_key_with(element_key);
                // self.push_scope_as(scope.clone(), &complete_key);

                // let events = events.as_ref().map(|events| events.iter());
                // let value_binding = value_binding.as_ref().map(|s| s.clone());

                // let events = events.as_ref().map_or_else(|| iter::empty(), |v| v.iter());
                // let value_bindings = value_bindings.as_ref().map_or_else(|| iter::empty(), |v| v.iter());

                stream_writer.write_op_element_open(
                    w,
                    expression_writer,
                    value_writer,
                    ctx,
                    bindings,
                    element_tag,
                    element_key,
                    is_void,
                    iter::empty(),
                    iter::empty(),
                    iter::empty(),
                )?;

                // self.stream_writer.write_op_element_open(
                //     w,
                //     self,
                //     ctx,
                //     bindings,
                //     element_key,
                //     element_tag,
                //     is_void,
                //     props,
                //     events,
                //     value_bindings
                // )?;



                // self.stream_writer
                //     .write_op_element(w,
                //                       op,
                //                       doc,
                //                       &scope,
                //                       &complete_key,
                //                       element_tag,
                //                       is_void,
                //                       props.as_ref().map(|s| s.iter()),
                //                       events,
                //                       value_bindings)?;

                // if is_void {
                //     // Pop scope for self closing, this fixes issue with ElementVoid which
                //     // was not being emitted previously by the parser/processor code.
                //     ctx.pop_scope();
                // };
            }

            &ElementOp::ElementClose(ref element_tag) => {
                stream_writer.write_op_element_close(
                    w,
                    expression_writer,
                    value_writer,
                    ctx,
                    bindings,
                    element_tag,
                )?;

                // ctx.pop_scope();
            }

            &ElementOp::WriteValue(ref expr, ref element_key) => {
                stream_writer.write_op_element_value(
                    w,
                    expression_writer,
                    value_writer,
                    ctx,
                    bindings,
                    expr,
                    element_key
                )?;
            }

            _ => {}
    };

    Ok(())
}

impl<V: ValueWriter, E: ExpressionWriter<V = V>, S: ElementOpsStreamWriter<E = E>> ElementOpsWriter for DefaultOutputWriter<V, E, S> {
    type E = E;
    type S = S;

    fn write_element_op(&mut self, w: &mut io::Write, ctx: &mut Context, bindings: &BindingContext, op: &ElementOp) -> Result {
        write_element_op(w, &mut self.stream_writer, &mut self.expression_writer, &mut self.value_writer, ctx, bindings, op)
    }

    fn write_element_ops<'a, I: IntoIterator<Item = &'a ElementOp>>(&mut self, w: &mut io::Write, ctx: &mut Context, bindings: &BindingContext, ops: I) -> Result {
        for op in ops {
            write_element_op(w, &mut self.stream_writer, &mut self.expression_writer, &mut self.value_writer, ctx, bindings, op)?;
        }
        Ok(())
    }
}
