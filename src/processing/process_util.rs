
use std::clone::Clone;

use parser::ast::*;
use processing::structs::*;
use scope::context::*;
use scope::bindings::*;


#[derive(Debug, Clone)]
#[allow(dead_code)]
pub enum BareSymbolResolutionMode {
    ReducerKeyThenProp,
    PropThenReducerKey,
}

#[inline]
pub fn map_lens_using_scope<'input>(ctx: &mut Context,
                                    bindings: &BindingContext,
                                    lens: Option<&LensExprType>,
                                    processing: &DocumentProcessingState)
                                    -> Option<LensExprType> {
    match lens {
        Some(&LensExprType::ForLens(ref ele_key, ref coll_sym)) => {
            let ele_key = ele_key.as_ref().map(|s| s.clone());
            if let Some(resolved) = ctx.resolve_unresolved_sym_object(coll_sym) {
                return Some(LensExprType::ForLens(ele_key, resolved));
            };
        }
        Some(&LensExprType::GetLens(ref sym)) => {
            if let Some(resolved) = ctx.resolve_unresolved_sym_object(sym) {
                return Some(LensExprType::GetLens(resolved));
            };
        }
        _ => {}
    };

    None
}
