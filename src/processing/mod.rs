pub mod structs;
pub mod structs_store;
pub mod structs_route;
pub mod process;
pub mod process_content;
pub mod process_comp_def;
pub mod process_route;
pub mod process_store;
pub mod events;

pub use self::structs::*;
pub use self::structs_store::*;
pub use self::structs_route::*;
pub use self::process::ProcessDocument;
pub use self::process_content::ProcessContent;
pub use self::events::*;

use model::*;
use scope::*;


pub trait ProcessSourceNode<S> {
    type Output;

    fn process_source_node(&mut self, processing: &mut DocumentProcessingState, ctx: &mut Context, bindings: &mut BindingContext, output: &mut Self::Output, source: &S) -> DocumentProcessingResult<()>;
}