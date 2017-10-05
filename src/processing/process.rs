
use std::iter;

use model::*;
use parser::*;
use parser::loc::*;
use scope::*;

use processing::*;
use processing::process_content::*;
use processing::process_store::*;
use processing::process_comp_def::*;
use processing::process_route::*;
use processing::process_query::*;


pub struct ProcessDocument<'input> {
    ast: &'input Template,
    processing: DocumentProcessingState
}

impl<'inp> Into<Document> for ProcessDocument<'inp> {
    fn into(self) -> Document {
        self.processing.into()
    }
}

impl<'input> ProcessDocument<'input> {
    #[allow(dead_code)]
    pub fn from_template<'inp>(ast: &'inp Template) -> ProcessDocument {

        ProcessDocument {
            ast: ast,
            processing: Default::default()
        }
    }

    pub fn process_component_definition(&mut self,
                                        ctx: &mut Context,
                                        bindings: &mut BindingContext,
                                        component_data: &ComponentDefinitionType)
                                        -> DocumentProcessingResult<()> {
        let mut comp_output = CompDefProcessorOutput::default();
        let mut comp_processor = CompDefProcessor::default();

        if let Some(ref children) = component_data.children {
            comp_processor.process_component_definition(&mut comp_output, &mut self.processing, ctx, bindings, component_data, children.iter())?;
        } else {
            comp_processor.process_component_definition(&mut comp_output, &mut self.processing, ctx, bindings, component_data, iter::empty())?;
        }

        let comp: Component = comp_output.into();
        self.processing.comp_map.insert(component_data.name.to_owned(), comp);

        Ok(())
    }

    pub fn process_route_definition(&mut self, ctx: &mut Context, bindings: &mut BindingContext, route: &RouteDefinitionType) -> DocumentProcessingResult<()> {
        let mut output = RouteProcessorOutput::default();
        let mut processor = RouteProcessor::default();
        processor.process_source_node(&mut self.processing, ctx, bindings, &mut output, route)?;
        let route: Route = output.into();
        self.processing.route_map.insert(route.pattern().to_owned(), route);
        Ok(())
    }

    pub fn process_query_definition(&mut self, ctx: &mut Context, bindings: &mut BindingContext, query_def: &QueryDefinition) -> DocumentProcessingResult<()> {
        let mut output = QueryDefinitionProcessorOutput::default();
        let mut processor = QueryDefinitionProcessor::default();
        processor.process_source_node(&mut self.processing, ctx, bindings, &mut output, query_def)?;
        let query: Query = output.into();
        self.processing.query_map.insert(query.name().to_owned(), query);
        Ok(())
    }

    #[allow(dead_code)]
    #[allow(unused_variables)]
    pub fn process_document(&mut self, ctx: &mut Context, bindings: &mut BindingContext) -> DocumentProcessingResult<()> {
        let mut root_block = BlockProcessingState::default();
        let mut process_store = ProcessStore::default();
        let mut content_processor = ProcessContent::default();

        for ref loc in self.ast.children.iter() {
            if let NodeType::ComponentDefinitionNode(ref component_data) = loc.inner {
                self.process_component_definition(ctx, bindings, component_data)?;
            }
        }

        for ref loc in self.ast.children.iter() {
            if let NodeType::StoreNode(ref scope_nodes) = loc.inner {
                for scope_node in scope_nodes {
                    process_store.process_store_default_scope_node(
                        &mut self.processing,
                        ctx,
                        bindings,
                        scope_node)?;
                }
            };
        }

        for ref loc in self.ast.children.iter() {
            if let NodeType::RouteDefinitionNode(ref route) = loc.inner {
                self.process_route_definition(ctx, bindings, route)?;
            }
        }

        for ref loc in self.ast.children.iter() {
            if let NodeType::QueryNode(ref query) = loc.inner {
                self.process_query_definition(ctx, bindings, query)?;
            }
        }

        // if let Some(ref default_reducer_key) = self.processing.default_reducer_key {
        //     ctx.append_action_path_str(default_reducer_key);
        //     let binding = BindingType::ReducerPathBinding(default_reducer_key.to_owned());
        //     ctx.add_sym(default_reducer_key, Symbol::binding(&binding));
        // };

        for (_, reducer_data) in self.processing.reducer_key_data.iter() {
            let reducer_key = reducer_data.reducer_key.to_owned();
            // let ty = reducer_data.ty
            let binding = BindingType::ReducerPathBinding(reducer_key.clone());
            if let Some(ref ty) = reducer_data.ty {
                ctx.add_sym(&reducer_key, Symbol::typed_binding(&binding, ty));
            } else {
                ctx.add_sym(&reducer_key, Symbol::binding(&binding));
            }
        }

        for ref loc in self.ast.children.iter() {
            if let NodeType::ContentNode(ref content_node) = loc.inner {
                content_processor.process_block_content_node(&mut self.processing, ctx, bindings, content_node, &mut root_block, None)?;
            };
        }

        self.processing.root_block = root_block;
        Ok(())
    }
}