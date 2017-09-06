
use parser::*;
use processing::*;
use scope::*;


#[derive(Debug, Clone)]
#[allow(dead_code)]
pub enum BareSymbolResolutionMode {
    ReducerKeyThenProp,
    PropThenReducerKey,
}

#[inline]
pub fn map_lens_using_scope<'input>(ctx: &mut Context,
                                    _bindings: &BindingContext,
                                    lens: &LensExprType)
                                    -> LensExprType {
    match lens {
        &LensExprType::ForLens(ref ele_key, ref coll_expr) => {
            let ele_key = ele_key.as_ref().map(|s| s.to_owned());
            if let Some(coll_expr) = ctx.reduce_expr(coll_expr) {
                return LensExprType::ForLens(ele_key, coll_expr);
            };
        }
        &LensExprType::GetLens(ref prop_expr) => {
            if let Some(prop_expr) = ctx.reduce_expr(prop_expr) {
                return LensExprType::GetLens(prop_expr);
            };
        }
        // _ => {}
    };

    lens.to_owned()
}
