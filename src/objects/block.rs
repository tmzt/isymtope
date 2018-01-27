
use std::marker::PhantomData;
use std::hash::Hash;
use std::fmt::Debug;

use error::*;
use traits::*;
use expressions::*;
use objects::*;
use output::*;


#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Block<T> {
    block_id: String,
    children: Option<Box<Vec<Block<T>>>>,
    ops: Option<Vec<ElementOp<T>>>,

    // events: Option<Vec<ElementEvent>>,
    // compkey_mappings: Option<ComponentKeyMappingVec>

    _ty: PhantomData<T>
}

impl<T> Block<T> {
    pub fn new(id: String, children: Option<Vec<Block<T>>>, ops: Option<Vec<ElementOp<T>>>) -> Self {
        let children = children.map(Box::new);

        Block {
            block_id: id,
            children: children,
            ops: ops,
            _ty: Default::default()
        }
    }

    pub fn id(&self) -> &str { &self.block_id }

    #[allow(dead_code)]
    pub fn children<'a>(&'a self) -> Option<impl IntoIterator<Item = &'a Block<T>>> {
        self.children.as_ref().map(|&box ref v| v.into_iter())
    }

    #[allow(dead_code)]
    pub fn ops<'a>(&'a self) -> Option<impl IntoIterator<Item = &'a ElementOp<T>>> {
        self.ops.as_ref().map(|v| v.into_iter())
    }

    // #[allow(dead_code)]
    // pub fn events<'a>(&'a self) -> Option<impl IntoIterator<Item = &'a EventsItem>> {
    //     self.events.as_ref().map(|events| events.into())
    // }

    // #[allow(dead_code)]
    // pub fn componentkey_mappings<'a>(&'a self) -> Option<impl IntoIterator<Item = &'a ComponentKeyMapping>> {
    //     self.compkey_mappings.as_ref().map(|compkey_mappings| compkey_mappings.into())
    // }
}

impl TryProcessFrom<Block<SourceExpression>> for Block<ProcessedExpression> {
    fn try_process_from(src: &Block<SourceExpression>, ctx: &mut ProcessingContext) -> DocumentProcessingResult<Self> {
        let children: Option<Vec<Block<ProcessedExpression>>> = TryProcessFrom::try_process_from(&src.children, ctx)?;
        let ops: Option<Vec<ElementOp<ProcessedExpression>>> = TryProcessFrom::try_process_from(&src.ops, ctx)?;

        debug!("TryProcessFrom Block -> Block: ops: {:?}", ops);

        Ok(Block::new(src.block_id.clone(), children, ops))
    }
}

// fn eval_block<T>(src: &Block<T>, ctx: &mut OutputContext<ProcessedExpression>) -> DocumentProcessingResult<Block<OutputExpression>> where ExpressionValue<OutputExpression>: TryEvalFrom<ExpressionValue<T>>, T: Debug + Hash + Eq {
//     let children: Option<Vec<Block<OutputExpression>>> = TryEvalFrom::try_eval_from(&src.children, ctx)?;
//     let ops: Option<Vec<ElementOp<OutputExpression>>> = TryEvalFrom::try_eval_from(&src.ops, ctx)?;

//     debug!("TryEvalFrom Block -> Block: ops: {:?}", ops);
//     eprintln!("Done evaluating block {}", src.block_id);
//     eprintln!("Children: {:?}", children);
//     eprintln!("Ops: {:?}", ops);

//     Ok(Block::new(src.block_id.clone(), children, ops))
// }

// impl<T> TryEvalFrom<Block<T>> for Block<OutputExpression> where ExpressionValue<OutputExpression>: TryEvalFrom<ExpressionValue<T>>, T: Debug + Hash + Eq {
//     fn try_eval_from(src: &Block<T>, ctx: &mut OutputContext<ProcessedExpression>) -> DocumentProcessingResult<Self> {
//         eval_block(src, ctx)
//     }
// }

impl TryEvalFrom<Block<ProcessedExpression>> for Block<OutputExpression> {
    fn try_eval_from(src: &Block<ProcessedExpression>, ctx: &mut OutputContext<ProcessedExpression>) -> DocumentProcessingResult<Self> {
        let children: Option<Vec<Block<OutputExpression>>> = TryEvalFrom::try_eval_from(&src.children, ctx)?;
        let ops: Option<Vec<ElementOp<OutputExpression>>> = TryEvalFrom::try_eval_from(&src.ops, ctx)?;

        debug!("TryEvalFrom Block -> Block: ops: {:?}", ops);
        eprintln!("Done evaluating block {}", src.block_id);
        eprintln!("Children: {:?}", children);
        eprintln!("Ops: {:?}", ops);

        Ok(Block::new(src.block_id.clone(), children, ops))
    }
}
