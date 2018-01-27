
pub mod context;
pub mod state;

pub use self::context::*;
pub use self::state::*;

use error::*;
use scope::*;
use objects::*;


// pub trait ProcessSourceNode<S> {
//     type Output;

//     fn process_source_node(&mut self, processing: &mut DocumentProcessingState, ctx: &mut Context, output: &mut Self::Output, source: &S) -> DocumentProcessingResult<()>;
// }

// pub trait PushElementOp<T> {
//     fn push_element_op(&mut self, ctx: &mut Context, op: ElementOp<T>) -> Result;
// }