use model::*;
use scope::*;
use processing::*;


#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum RouteProcessorStateType {
    Content(BlockProcessingState),
    Actions(Option<Vec<ActionOpNode>>)
}

#[derive(Debug, Default, Clone, PartialEq, Eq, Hash)]
pub struct RouteProcessorOutput {
    pattern: Option<String>,
    formal_props: Option<FormalPropVec>,
    state: Option<RouteProcessorStateType>
}

#[derive(Debug, Default, Clone, PartialEq, Eq, Hash)]
pub struct RouteProcessor {}

impl ProcessSourceNode<RouteDefinitionType> for RouteProcessor {
    type Output = RouteProcessorOutput;

    fn process_source_node(&mut self, processing: &mut DocumentProcessingState, ctx: &mut Context, bindings: &mut BindingContext, output: &mut Self::Output, source: &RouteDefinitionType) -> DocumentProcessingResult<()> {
        output.pattern = Some(source.pattern().to_owned());
        output.formal_props = source.formals_iter().map(|iter| iter.map(|f| f.0.to_owned()).collect());

        match source.action_ref() {
            &RouteDefinitionActionType::Content(ref content_nodes) => {
                let mut block = BlockProcessingState::default();
                // output.mode = Some(RouteProcessorOutputMode::Content);

                let mut content_processor = ProcessContent::default();
                if let &Some(ref content_nodes) = content_nodes {
                    for node in content_nodes {
                        if let &NodeType::ContentNode(ref content_node) = node {
                            content_processor.process_block_content_node(processing, ctx, bindings, content_node, &mut block, None)?;
                        };
                    };
                };

                output.state = Some(RouteProcessorStateType::Content(block));
            }

            &RouteDefinitionActionType::Actions(ref action_ops) => {
                output.state = Some(RouteProcessorStateType::Actions(action_ops.clone()));
            }
        }

        Ok(())
    }
}

impl Into<Route> for RouteProcessorOutput {
    fn into(self) -> Route {
        let pattern = self.pattern.unwrap_or("*".into());

        match self.state.expect("Must processed source node before calling Into::into.") {
            RouteProcessorStateType::Actions(action_ops) => {
                Route::actions(pattern, self.formal_props, action_ops)
            }

            RouteProcessorStateType::Content(block) => {
                Route::content(pattern, self.formal_props, block.into())
            }
        }
    }
}