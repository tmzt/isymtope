#![allow(dead_code)]

use std::iter;

use parser::ast::*;
use scope::context::*;
use scope::bindings::*;
use processing::structs::*;
use processing::process_content::*;


type FormalProp<'a> = (&'a str);
type FormalPropVec<'a> = Vec<FormalProp<'a>>;

type PropKeyRef = (String, String);
type PropKeyRefVec = Vec<PropKeyRef>;

// type PropValue<'a> = (&'a str, Option<&'a ExprValue>);
// type PropValueVec<'a> = Vec<PropValue<'a>>;


#[derive(Debug, Clone, Default)]
pub struct CompDefProcessorOutput {
    ty: Option<String>,
    block: BlockProcessingState
    // ops: Option<OpsVec>
}

impl Into<Component> for CompDefProcessorOutput {
    fn into(self) -> Component {
        Component::new(self.ty.unwrap(), self.block.into())
    }
}

#[derive(Debug, Default)]
pub struct CompDefProcessor {
    // output: &'out mut CompDefProcessorOutput
}

impl CompDefProcessor {
    // pub fn with_output(output: &'out mut CompDefProcessorOutput) -> Self {
    //     CompDefProcessor { output: output }
    // }

    // fn add_op(&mut self, op: ElementOp) {
    //     if let Some(ref mut ops) = self.output.ops {
    //         ops.push(op);
    //     } else {
    //         self.output.ops = Some(vec![op]);
    //     }
    // }

    pub fn push_component_definition_scope<'a, I>(&mut self, output: &mut CompDefProcessorOutput, ctx: &mut Context,component_ty: &str, formals: I)
        where I: IntoIterator<Item = FormalProp<'a>>
    {
        if output.ty.is_some() {
            panic!("Already have component defintion in this output.");
        }
        output.ty = Some(component_ty.to_owned());

        ctx.push_child_scope();
        for formal in formals {
            let binding = BindingType::ComponentPropBinding(formal.to_owned());
            ctx.add_sym(formal, Symbol::binding(&binding));
            // ctx.add_unbound_formal_param(formal);
        }
    }

    pub fn push_element_parameter_definition_scope<'a, I>(&mut self, block: &mut BlockProcessingState, ctx: &mut Context, element_id: &str, element_ty: &str, props: Option<I>)
        where I: IntoIterator<Item = PropKeyRef> + Clone
    {
        let props: Option<Vec<PropKeyRef>> = props.map(|props| props.into_iter().collect());

        // let props = props.map(|props| props.clone().into_iter());
        let element_props = props.as_ref().map(|props| props.iter().map(|prop| {
            let key = &prop.0;
            let ref_key = &prop.1;
            let value_sym = ctx.resolve_sym(ref_key);
            if let Some(value_sym) = value_sym {
                return (key.to_owned(), Some(ExprValue::SymbolReference(value_sym)));
            };
            (key.to_owned(), None)
        }).collect());

        let op = ElementOp::ElementVoid(element_ty.to_owned(), element_id.to_owned(), element_props, None, None);
        block.ops_vec.push(op);
        // self.add_op(op);

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

    pub fn process_component_definition<'a, I: IntoIterator<Item = &'a NodeType>>(&mut self,
                                        output: &mut CompDefProcessorOutput,
                                        processing: &mut DocumentProcessingState,
                                        ctx: &mut Context,
                                        bindings: &mut BindingContext,
                                        component_data: &ComponentDefinitionType,
                                        nodes: I)
                                        -> Result
    {
        // Prepare scope with formal parameters
        // TODO: Actually implement the formal parameters
        if let Some(ref props) = component_data.inputs {
            self.push_component_definition_scope(output, ctx, &component_data.name, props.iter().map(|s| (s.as_str()) ));
            // self.push_component_definition_scope(output, ctx, &component_data.name, Some(props.iter().map(|s| (s.to_owned(), s.to_owned()) )));
        } else {
            self.push_component_definition_scope(output, ctx, &component_data.name, iter::empty());
        };
        // self.push_component_definition_scope(output, ctx, &component_data.name, iter::empty());

        let mut content_processor = ProcessContent::default();
        for node in nodes {
            if let &NodeType::ContentNode(ref content_node) = node {
                content_processor.process_block_content_node(ctx, bindings, content_node, &mut output.block, processing, None)?;
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
                processor.push_element_parameter_definition_scope(&mut output.block, &mut ctx, "Pq", "input", Some(props.into_iter()));
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
            comp.root_block().ops_iter().map(|v| v.into_iter().cloned().collect()),
            Some(vec![
                ElementOp::ElementVoid("input".into(), "Pq".into(), Some(vec![(
                    "value".into(),
                    Some(ExprValue::SymbolReference(Symbol::unbound_formal_param("todo", Some(&comp_definition_scope_id)) ))
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