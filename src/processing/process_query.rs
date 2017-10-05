use model::*;
use scope::*;
use processing::*;


// #[derive(Debug, Clone, PartialEq, Eq, Hash)]
// pub enum QueryDefinitionProcessorStateType {
//     CaseWhere
// }

#[derive(Debug, Default, Clone, PartialEq, Eq, Hash)]
pub struct QueryDefinitionProcessorOutput {
    name: Option<String>,
    formal_props: Option<FormalPropVec>,
    components: Option<Vec<QueryComponent>>,
    // state: Option<QueryDefinitionProcessorStateType>
}

#[derive(Debug, Default, Clone, PartialEq, Eq, Hash)]
pub struct QueryDefinitionProcessor {}

impl ProcessSourceNode<QueryDefinition> for QueryDefinitionProcessor {
    type Output = QueryDefinitionProcessorOutput;

    fn process_source_node(&mut self, _processing: &mut DocumentProcessingState, ctx: &mut Context, _bindings: &mut BindingContext, output: &mut Self::Output, source: &QueryDefinition) -> DocumentProcessingResult<()> {
        output.name = Some(source.name().to_owned());
        output.formal_props = source.params_iter().map(|iter| iter.map(|s| s.to_owned()).collect());
        // output.formal_props = source.params_iter().map(|iter| iter.map(|f| f.0.to_owned()).collect());
        // output.formal_props = source.params_iter().map(|iter| iter.map(|key| (key.to_owned(), None)).collect());

        output.components = source.components_iter().map(|iter| iter.map(|source_component| {
            match *source_component {
                QueryDefinitionComponent::CaseWhere(box ref val, box ref cond) => {
                    let val = ctx.reduce_expr_or_return_same(val);
                    let cond = ctx.reduce_expr_or_return_same(cond);
                    QueryComponent::case_where(val, cond)
                }
            }
        }).collect());

        Ok(())
    }
}

impl Into<Query> for QueryDefinitionProcessorOutput {
    fn into(self) -> Query {
        let name = self.name.expect("Name is required");
        Query::new(name, self.formal_props, self.components)
    }
}