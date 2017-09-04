
use std::iter;

use parser::ast::*;

use processing::structs::*;
use processing::process_content::*;
use processing::process_store::*;
use processing::process_comp_def::*;
use scope::*;


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
                                        component_data: &'input ComponentDefinitionType)
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
            if let NodeType::ContentNode(ref content_node) = loc.inner {
                content_processor.process_block_content_node(ctx, bindings, content_node, &mut root_block, &mut self.processing, None)?;
            };
        }

        self.processing.root_block = root_block;
        Ok(())
    }
}