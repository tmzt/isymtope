use std::fmt::Debug;

use error::*;
use traits::*;

pub trait MapIdents<T: Debug> {
    fn map_idents(self, ctx: &mut ProcessingContext) -> DocumentProcessingResult<Self>
    where
        Self: Sized;
}
