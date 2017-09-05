
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

#[inline]
pub fn peek_var_ty(expr: &ExprValue) -> Option<VarType> {
    match *expr {
        ExprValue::LiteralNumber(..) => {
            return Some(VarType::Primitive(PrimitiveVarType::Number));
        }
        ExprValue::LiteralString(..) => {
            return Some(VarType::Primitive(PrimitiveVarType::StringVar));
        }
        ExprValue::LiteralArray(Some(ref items)) => {
            if !items.is_empty() {
                if let Some(ref first_item) = items.get(0) {
                    if let Some(var_ty) = peek_var_ty(first_item) {
                        return Some(VarType::ArrayVar(Some(Box::new(var_ty))));
                    }
                    return Some(VarType::ArrayVar(None));
                };
            };
            return Some(VarType::ArrayVar(None));
        }
        _ => {}
    };
    None
}