#![allow(dead_code)]

use std::iter;

use model::*;
use scope::*;
use processing::*;


// type FormalProp<'a> = (&'a str);
// type FormalPropVec<'a> = Vec<FormalProp<'a>>;

type PropKeyRef = (String, String);
type PropKeyRefVec = Vec<PropKeyRef>;

type FormalPropRef<'a> = (&'a str);

#[derive(Debug, Clone, Default)]
pub struct CompDefProcessorOutput {
    ty: Option<String>,
    block: BlockProcessingState,
    formal_props: Option<FormalPropVec>
    // ops: Option<OpsVec>
}

impl Into<Component> for CompDefProcessorOutput {
    fn into(self) -> Component {
        // let formal_props = if self.formal_props.len() > 0 { Some(self.formal_props) } else { None };
        Component::new(self.ty.unwrap(), self.block.into(), self.formal_props)
    }
}

#[derive(Debug, Default)]
pub struct CompDefProcessor {}

impl CompDefProcessor {
    pub fn push_component_definition_scope<'a, I>(&mut self, output: &mut CompDefProcessorOutput, ctx: &mut Context,component_ty: &str, formals: I)
        where I: IntoIterator<Item = FormalPropRef<'a>>
    {
        if output.ty.is_some() {
            panic!("Already have component defintion in this output.");
        }

        let mut formals = formals.into_iter();
        ctx.push_formal_parameter_scope(formals.by_ref());

        output.ty = Some(component_ty.to_owned());
        output.formal_props = Some(formals.map(|p| p.to_owned()).collect());
    }

    pub fn push_element_scope(&mut self, ctx: &mut Context, element_id: &str, _element_ty: &str) {
        ctx.push_child_scope();
        ctx.append_path_str(element_id);
    }

    pub fn process_component_definition<'a, I: IntoIterator<Item = &'a NodeType>>(&mut self,
                                        output: &mut CompDefProcessorOutput,
                                        processing: &mut DocumentProcessingState,
                                        ctx: &mut Context,
                                        bindings: &mut BindingContext,
                                        component_data: &ComponentDefinitionType,
                                        nodes: I)
                                        -> Result
    {
        let iter = iter::empty()
            .chain(component_data.inputs.as_ref().map((|v| v.iter())))
            .flat_map(|m| m);

        // Prepare scope with formal parameters
        self.push_component_definition_scope(output, ctx, &component_data.name, iter.map(|s| s.as_str()));

        // Process the content
        let mut content_processor = ProcessContent::default();
        for node in nodes {
            if let &NodeType::ContentNode(ref content_node) = node {
                content_processor.process_block_content_node(processing, ctx, bindings, content_node, &mut output.block, None)?;
            };
        };

        ctx.pop_scope();
        Ok(())
    }
}


#[cfg(test)]
mod tests {
    use super::*;
    use std::iter::*;


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
        fn write_op_to<W>(ctx: &mut Context, bindings: &BindingContext, ops_writer: &mut W, comp_op: &ElementOp)
          where W: OpsWriter
        {
            match comp_op {
                &ElementOp::ElementVoid(ref element_ty, ref element_id, ref props, ref event_handlers, ref value_bindings) => {
                    if let &Some(ref props) = props {
                        let prop_iter = props.iter().map(|prop| (prop.0.as_str(), prop.1.as_ref()));
                        let bindings_iter = SymbolBindingPropResolver::new(ctx, bindings, prop_iter);
                        let resolved_props: Vec<Prop> = bindings_iter.collect();

                        let op = ElementOp::ElementVoid(element_ty.to_owned(), element_id.to_owned(), Some(resolved_props), event_handlers.to_owned(), value_bindings.to_owned());
                        ops_writer.write_op(&op);
                    };
                },
                _ => { ops_writer.write_op(comp_op); }
            };
        }

        pub fn write_component<W>(&mut self, ctx: &mut Context, bindings: &BindingContext, ops_writer: &mut W, comp: &Component)
          where W: OpsWriter
        {
            if let Some(ops_iter) = comp.root_block().ops_iter() {
                for op in ops_iter {
                    Self::write_op_to(ctx, bindings, ops_writer, op);
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
            let mut processor = CompDefProcessor::default();

            // component Component(todo)
            // Create new component context with unbound formal prop (todo)
            {
                let formals: Vec<String> = vec!["todo".into()];
                processor.push_component_definition_scope(&mut output, &mut ctx, "Component", formals.iter().map(|s| (s.as_str())));
            }
            comp_definition_scope_id = ctx.scope_ref().unwrap().id().to_owned();

            // within Component definition
            {
                // The local (todo) should be a component prop (todo)
                assert_eq!(ctx.resolve_sym("todo"),
                    // Some(Symbol::ref_prop_in_scope("todo", "todo", Some(&lm_element_scope_id)))
                    Some(Symbol::binding(&BindingType::ComponentPropBinding("todo".into())))
                );
            }
            // element_param_defs_scope_id = ctx.scope_ref().unwrap().id().to_owned();

            // element Pq invocation
            {
                // let props: PropKeyRefVec = vec![
                //     ("value".into(), "todo".into())
                // ];
                // processor.push_element_parameter_definition_scope(&mut output.block, &mut ctx, "Pq", "input", Some(props.into_iter()));
            }
            // let element_pq_invocation_scope_id = ctx.scope_ref().unwrap().id().to_owned();

            // element Pq scope
            {
                processor.push_element_scope(&mut ctx, "Pq", "input");

                // The local (todo) should still be a component prop (todo)
                assert_eq!(ctx.resolve_sym("todo"),
                    Some(Symbol::binding(&BindingType::ComponentPropBinding("todo".into())))
                );
            }
        }

        // Verify output
        let comp: Component = output.into();
        assert_eq!(comp.ty(), "Component");
        assert_eq!(
            comp.root_block().ops_iter().map(|v| v.into_iter().cloned().collect()),
            Some(vec![
                ElementOp::ElementVoid("input".into(), "Pq".into(), Some(vec![(
                    "value".into(),
                    Some(ExprValue::SymbolReference(Symbol::binding(&BindingType::ComponentPropBinding("todo".into()))))
                )]), None, None)
            ])
        );

        // Test writer
        let mut ops_writer = TestOpsWriter::default();

        {
            let mut ctx = Context::default();
            let mut bindings = BindingContext::default();
            bindings.add_reducer_key("todo", "todo");

            let mut comp_writer = ComponentWriter::default();

            comp_writer.write_component(&mut ctx, &bindings, &mut ops_writer, &comp);
        }

        assert_eq!(ops_writer.ops_iter().cloned().collect::<OpsVec>(), vec![
                ElementOp::ElementVoid("input".into(), "Pq".into(), Some(vec![(
                    "value".into(),
                    Some(ExprValue::Binding((BindingType::ReducerPathBinding("todo".into()))))
                )]), None, None)
        ]);
    }
}
