
use std::rc::Rc;
use std::collections::HashSet;
use linked_hash_map::LinkedHashMap;

use error::*;
use traits::*;
use expressions::*;
use scope::*;
use ast::*;
use objects::*;
use input::*;
use super::*;


impl TryProcessFrom<Rc<Template>> for Document {

    fn try_process_from(template: &Rc<Template>, ctx: &mut ProcessingContext) -> DocumentProcessingResult<Document> {

        eprintln!("DocumentProcessingState: Template: {:?}", template);
        let mut processor = DocumentProcessor::new(template.clone());
        // let mut ctx = ProcessingContext::default();

        // Process store definitions first
        processor.process_store_definitions(ctx)?;

        // Process content and nested templates.
        processor.process_content(ctx)?;

        let root_block: Block<ProcessedExpression> = TryProcessFrom::try_process_from(&state.root_block)?;
        let reducers = state.reducers;
        let default_reducer_key = state.default_reducer_key;

        eprintln!("DocumentProcessingState: root_block: {:?}", &root_block);

        Ok(Document::new(
            root_block,
            reducers,
            default_reducer_key
        ))
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct DocumentProcessor {
    ast: Rc<Template>,

    reducers: LinkedHashMap<String, Reducer<ProcessedExpression>>,
    default_reducer_key: Option<String>,
}

impl DocumentProcessor {
    pub fn new(ast: Rc<Template>) -> Self { DocumentProcessor { ast: ast } }

    pub fn process_store_definitions(&mut self, ctx: &mut ProcessingContext) -> DocumentProcessingResult<()> {
        let ast = self.ast.as_ref();

        let store_definitions: Vec<_> = ast.children()
            .filter_map(|n| match *n { TemplateNode::StoreDefinition(ref n, _) => Some(n), _ => None })
            .collect();

        for store_definition in store_definitions {
            if let Some(v) = store_definition.children() {
                for child in v {
                    // if let StoreRootScopeNode::Common(StoreCommonNode::ChildScopeNode(ref scope, ref children)) = child {
                    match *child {
                        StoreRootScopeNode::Common(StoreCommonNode::ChildScopeNode(ref scope, ref children), _) => {
                            let actions: Vec<_> = children.as_ref().map(|v| v.iter()
                                    .filter_map(|n| match *n { StoreChildScopeNode::Action(ref n, _) => Some(n.to_owned()), _ => None })
                                    .map(|action| action.map_idents())
                                    .collect()
                                ).unwrap_or_default();

                            eprintln!("Actions: {:?}", actions);

                            let reducer = Reducer::new(scope.to_owned(), Some(actions), None);
                            eprintln!("Reducer<SourceExpression>: {:?}", reducer);

                            let reducer: Reducer<ProcessedExpression> = TryProcessFrom::try_process_from(&reducer, ctx)?;
                            eprintln!("Reducer<ProcessedExpression>: {:?}", reducer);

                            self.add_reducer(reducer);
                        }

                        _ => {}
                    };
                }
            };
        }

        Ok(())
    }

    pub fn process_content(&mut self, ctx: &mut ProcessingContext) -> DocumentProcessingResult<()> {
        let ast = self.ast.as_ref();

        ///
        /// Content
        ///

        let content_nodes: Vec<_> = ast.children()
            .filter_map(|n| match *n { TemplateNode::Content(ref n, _) => Some(n), _ => None })
            .collect();

        let root_block_id = allocate_element_key();
        let root_block = Block::new(root_block_id, None, Some(content_nodes));

        ctx.push_child_scope();

        // Add default action path for the default reducer key
        if let Some(ref default_reducer_key) = processing.default_reducer_key {
            // ctx.append_action_path_str(default_reducer_key);
        };

        ctx.pop_scope();

        Ok(())
    }
}