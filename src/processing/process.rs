
use std::iter;

use parser::ast::*;

use processing::structs::*;
use processing::process_content::*;
use processing::process_store::*;
use processing::process_comp_def::*;
use scope::*;


pub struct ProcessDocument<'input> {
    ast: &'input Template,
    root_block: BlockProcessingState,
    processing: DocumentProcessingState,
    // store_processing: StoreOutputProcessing,
    compdef_processing: CompDefProcessorOutput,
    // scope: ElementOpScope,
}

impl<'inp> Into<DocumentState<'inp>> for ProcessDocument<'inp> {
    fn into(self) -> DocumentState<'inp> {
        DocumentState {
            ast: self.ast,
            root_block: self.root_block,
            comp_map: self.processing.comp_map,
            block_map: self.processing.block_map,
            reducer_key_data: self.processing.reducer_key_data,
            default_state_map: self.processing.default_state_map,
            default_state_symbol: self.processing.default_state_symbol,
            default_reducer_key: self.processing.default_reducer_key,
        }
    }
}

impl<'input> ProcessDocument<'input> {
    #[allow(dead_code)]
    pub fn from_template<'inp>(ast: &'inp Template) -> ProcessDocument<'inp> {

        ProcessDocument {
            ast: ast,
            root_block: Default::default(),
            processing: Default::default(),
            // store_processing: Default::default(),
            compdef_processing: Default::default()
            // scope: scope,
        }
    }

    #[allow(dead_code)]
    pub fn process_component_definition(&mut self,
                                        ctx: &mut Context,
                                        _bindings: &mut BindingContext,
                                        component_data: &'input ComponentDefinitionType)
                                        -> DocumentProcessingResult<()> {
        // let mut comp_output = CompDefProcessorOutput::default();
        let mut comp_processor = CompDefProcessor::with_output(&mut self.compdef_processing);
        comp_processor.push_component_definition_scope(ctx, &component_data.name, iter::empty());


    //     if let Some(ref children) = component_data.children {
    //         for ref child in children {


        Ok(())
    }


    #[allow(dead_code)]
    #[allow(unused_variables)]
    pub fn process_document(&mut self, ctx: &mut Context, bindings: &mut BindingContext) -> DocumentProcessingResult<()> {
        let mut root_block = BlockProcessingState::default();
        // let base_scope: ScopePrefixes = Default::default();

        // let mut store_processing = StoreOutputProcessing::default();
        let mut process_store = ProcessStore::default();
        let mut content_processor = ProcessContent::default();

        // self.process_nodes(&base_scope, &mut root_block)?;
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

        self.root_block = root_block;
        Ok(())
    }
}