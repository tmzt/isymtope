#![allow(dead_code)]

use parser::ast::*;
use scope::context::*;
use processing::structs::*;


type FormalProp<'a> = (&'a str);
type FormalPropVec<'a> = Vec<FormalProp<'a>>;

type PropKeyRef = (String, String);
type PropKeyRefVec = Vec<PropKeyRef>;

// type PropValue<'a> = (&'a str, Option<&'a ExprValue>);
// type PropValueVec<'a> = Vec<PropValue<'a>>;


#[derive(Debug, Clone, Default)]
pub struct CompDefProcessorOutput {
    ty: Option<String>,
    ops: Option<OpsVec>
}

impl Into<Component> for CompDefProcessorOutput {
    fn into(self) -> Component {
        Component::new_with_vec(self.ty.unwrap(), self.ops)
    }
}

#[derive(Debug)]
pub struct CompDefProcessor<'out> {
    output: &'out mut CompDefProcessorOutput
}

impl<'out> CompDefProcessor<'out> {
    pub fn with_output(output: &'out mut CompDefProcessorOutput) -> Self {
        CompDefProcessor { output: output }
    }

    fn add_op(&mut self, op: ElementOp) {
        if let Some(ref mut ops) = self.output.ops {
            ops.push(op);
        } else {
            self.output.ops = Some(vec![op]);
        }
    }

    pub fn push_component_definition_scope<'a, I>(&mut self, ctx: &mut Context,component_ty: &str, formals: I)
        where I: IntoIterator<Item = &'a FormalProp<'a>>
    {
        if self.output.ty.is_some() {
            panic!("Already have component defintion in this output.");
        }
        self.output.ty = Some(component_ty.to_owned());

        ctx.push_child_scope();
        for formal in formals {
            ctx.add_unbound_formal_param(formal);
        }
    }

    pub fn push_element_parameter_definition_scope<'a, I>(&mut self, ctx: &mut Context, element_id: &str, element_ty: &str, props: Option<I>)
        where I: IntoIterator<Item = &'a PropKeyRef> + Clone
    {
        // let props = props.map(|props| props.clone().into_iter());
        let element_props = props.as_ref().map(|props| props.clone().into_iter().map(|prop| {
            let key = &prop.0;
            let ref_key = &prop.1;
            let value_sym = ctx.resolve_sym(ref_key);
            if let Some(value_sym) = value_sym {
                return (key.to_owned(), Some(ExprValue::SymbolReference(value_sym)));
            };
            (key.to_owned(), None)
        }).collect());

        let op = ElementOp::ElementVoid(element_ty.to_owned(), Some(element_id.to_owned()), element_props, None, None);
        self.add_op(op);

        let parent_scope_id = ctx.scope().id().to_owned();
        ctx.push_child_scope();
        if let Some(props) = props {
            for prop in props {
                ctx.add_element_prop_ref(&prop.0, &prop.1, Some(&parent_scope_id));
            }
        }
    }

    pub fn push_element_scope(&mut self, ctx: &mut Context, element_id: &str, _element_ty: &str) {
        ctx.push_child_scope();
        ctx.append_path_str(element_id);
    }
}


#[cfg(test)]
mod tests {
    use super::*;
    use std::iter::*;
    use parser::ast::*;
    use scope::context::*;


    trait OpsWriter {
        fn write_op(&mut self, op: &ElementOp);
    }

    #[derive(Debug, Default)]
    struct TestOpsWriter {
        output_ops: Vec<ElementOp>
    }
    
    impl<'a> TestOpsWriter {
        pub fn ops_iter(&'a self) -> impl Iterator<Item = &'a ElementOp> {
            self.output_ops.iter()
        }
    }

    impl OpsWriter for TestOpsWriter {
        fn write_op(&mut self, op: &ElementOp) {
            self.output_ops.push(op.to_owned());
        }
    }

    #[derive(Debug, Default)]
    struct ComponentWriter {}

    impl ComponentWriter {
        fn write_op_to<W>(ctx: &mut Context, ops_writer: &mut W, comp_op: &ElementOp)
          where W: OpsWriter
        {
            match comp_op {
                &ElementOp::ElementVoid(ref element_ty, ref element_id, ref props, ref event_handlers, ref value_bindings) => {
                    if let &Some(ref props) = props {
                        let prop_iter = props.iter().map(|prop| (prop.0.as_str(), prop.1.as_ref()));
                        // let iter = ctx.resolve_props(prop_iter);
                        let iter = SymbolResolver::new(ctx, prop_iter);
                        let resolved_props: Vec<Prop> = iter.collect();

                        let op = ElementOp::ElementVoid(element_ty.to_owned(), element_id.to_owned(), Some(resolved_props), event_handlers.to_owned(), value_bindings.to_owned());
                        ops_writer.write_op(&op);
                    };
                },
                _ => { ops_writer.write_op(comp_op); }
            };
        }

        pub fn write_component<W>(&mut self, ctx: &mut Context, ops_writer: &mut W, comp: &Component)
          where W: OpsWriter
        {
            if let Some(ops_iter) = comp.ops_iter() {
                for op in ops_iter {
                    Self::write_op_to(ctx, ops_writer, op);
                    // ops_writer.write_op(op);
                }
            };
        }
    }

    #[derive(Debug)]
    pub struct ComponentOutputProcessor {
    }

    impl ComponentOutputProcessor {
        pub fn push_component_invocation_scope<'a, I>(&mut self, ctx: &mut Context, comp: &Component)
          where I: IntoIterator<Item = PropValue<'a>>
        {
            ctx.push_child_scope();
        }
    }

    #[test]
    pub fn test_process_comp_def1() {
        let mut output = CompDefProcessorOutput::default();

        let comp_definition_scope_id: String;
        // let element_param_defs_scope_id: String;
        {
            let mut ctx = Context::default();
            let mut processor = CompDefProcessor::with_output(&mut output);

            // component Component(todo)
            // Create new component context with unbound formal prop (todo)
            {
                let formals: FormalPropVec = vec![("todo")];
                processor.push_component_definition_scope(&mut ctx, "Component", formals.iter());
            }
            comp_definition_scope_id = ctx.scope().id().to_owned();

            // within Component definition
            {
                // The local (todo) should be an unbound formal prop (todo)
                assert_eq!(ctx.resolve_sym("todo"),
                    // Some(Symbol::ref_prop_in_scope("todo", "todo", Some(&lm_element_scope_id)))
                    Some(Symbol::unbound_formal_param("todo", Some(&comp_definition_scope_id)))
                );
            }
            // element_param_defs_scope_id = ctx.scope().id().to_owned();

            // element Pq invocation
            {
                let props: PropKeyRefVec = vec![
                    ("value".into(), "todo".into())
                ];
                processor.push_element_parameter_definition_scope(&mut ctx, "Pq", "input", Some(props.iter()));
            }
            // let element_pq_invocation_scope_id = ctx.scope().id().to_owned();

            // element Pq scope
            {
                processor.push_element_scope(&mut ctx, "Pq", "input");

                // The local (todo) should still be an unbound formal param (todo)
                assert_eq!(ctx.resolve_sym("todo"),
                    Some(Symbol::unbound_formal_param("todo", Some(&comp_definition_scope_id)))
                );
            }
        }

        // Verify output
        let comp: Component = output.into();
        assert_eq!(comp.ty(), "Component");
        assert_eq!(
            comp.ops_iter().map(|v| v.into_iter().cloned().collect()),
            Some(vec![
                ElementOp::ElementVoid("input".into(), Some("Pq".into()), Some(vec![(
                    "value".into(),
                    Some(ExprValue::SymbolReference(Symbol::unbound_formal_param("todo", Some(&comp_definition_scope_id)) ))
                )]), None, None)
            ])
        );

        // Test writer
        let mut ops_writer = TestOpsWriter::default();

        {
            let mut ctx = Context::default();
            let mut comp_writer = ComponentWriter::default();

            comp_writer.write_component(&mut ctx, &mut ops_writer, &comp);
        }

        assert_eq!(ops_writer.ops_iter().cloned().collect::<OpsVec>(), vec![
                ElementOp::ElementVoid("input".into(), Some("Pq".into()), Some(vec![(
                    "value".into(),
                    Some(ExprValue::SymbolReference(Symbol::unbound_formal_param("todo", Some(&comp_definition_scope_id)) ))
                )]), None, None)
        ]);
    }
}