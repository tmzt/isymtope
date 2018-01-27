
use linked_hash_map::LinkedHashMap;

use util::*;
use error::*;
use traits::*;
use expressions::*;
use scope::*;
use input::*;
use objects::*;


#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct BlockProcessingState {
    block_id: String,
    children: LinkedHashMap<String, BlockProcessingState>,

    // children: Vec<Box<BlockProcessingState>>,
    ops: Vec<ElementOp<SourceExpression>>,

    // events: Vec<ElementEvent>,
    // compkey_mappings: ComponentKeyMappingVec
}

impl Default for BlockProcessingState {
    fn default() -> Self {
        let block_id = allocate_element_key();
        BlockProcessingState {
            block_id: block_id,
            children: Default::default(),
            ops: Default::default(),

            // events: Default::default(),
            // compkey_mappings: Default::default()
        }
    }
}

impl PushElementOp<SourceExpression> for BlockProcessingState {
    fn push_element_op(&mut self, ctx: &mut Context, op: ElementOp<SourceExpression>) -> Result {
        self.ops.push(op);
        Ok(())
    }
}

impl TryProcessFrom<BlockProcessingState> for Block<ProcessedExpression> {
    fn try_process_from(src: &BlockProcessingState) -> DocumentProcessingResult<Self> {
        let id = src.block_id.clone();

        let children: Vec<BlockProcessingState> = src.children.iter().map(|(_, b)| b.to_owned()).collect();
        let children: Vec<Block<ProcessedExpression>> = TryProcessFrom::try_process_from(&children)?;

        let ops: Vec<ElementOp<ProcessedExpression>> = TryProcessFrom::try_process_from(&src.ops)?;

        Ok(Block::new(id, Some(children), Some(ops)))
    }
}

