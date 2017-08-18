
use std::clone::Clone;

use parser::ast::*;
use parser::store::*;
use parser::api::*;
use parser::util::allocate_element_key;
use processing::structs::*;
use processing::scope::*;


#[derive(Debug, Clone)]
#[allow(dead_code)]
pub enum BareSymbolResolutionMode {
    ReducerKeyThenProp,
    PropThenReducerKey
}

#[inline]
#[allow(dead_code)]
pub fn resolve_reducer_key(processing: &DocumentProcessingState, scope: &mut ElementOpScope, reducer_key: &str) -> Option<Symbol> {
    // Try to resolve and cache the symbol as a reducer key reference
    if let Some(reducer_data) = processing.reducer_key_data.get(reducer_key) {
        if let Some(ref default_expr) = reducer_data.default_expr {
            scope.1.add_cached_reducer_key_with_value(reducer_key, default_expr);
            scope.2.add_cached_reducer(reducer_key, default_expr);
            // return Some(Symbol::reducer_key_with_value(reducer_key, default_expr));
        };
        
        return Some(Symbol::reducer_key(reducer_key));
    };

    None
}

#[inline]
#[allow(dead_code)]
pub fn map_expr<'input, F: Fn(&ExprValue) -> ExprValue>(expr: &'input ExprValue, f: &F) -> ExprValue {
    match expr {
        &ExprValue::Expr(ref op, ref l, ref r) => {
            let l_val = map_expr(l, f);
            let r_val = map_expr(r, f);
            ExprValue::Expr(op.clone(), Box::new(l_val), Box::new(r_val))
        }

        _ => {
            f(expr)
        }
    }
}

#[inline]
#[allow(dead_code)]
pub fn resolve_sym(sym: &Symbol, processing: &DocumentProcessingState, scope: &mut ElementOpScope) -> Option<Symbol> {
    if let &SymbolReferenceType::UnresolvedReference(ref key) = sym.sym_ref() {
        if let Some(_) = scope.1.params.get(key) {
            return Some(Symbol::param(key));
        };

        if let Some(_) = scope.1.props.get(key) {
            return Some(Symbol::prop(key));
        };

        if let Some(_) = scope.1.action_params.get(key) {
            return Some(Symbol::action_param(key));
        };

        if let Some(_) = scope.1.lens_params.get(key) {
            return Some(Symbol::for_lens_element_key(key));
        };

        if let Some(_) = scope.1.block_params.get(key) {
            return Some(Symbol::block_param(key));
        };

        if let Some(value_binding) = scope.1.element_value_bindings.get(key) {
            return Some(value_binding.to_owned());
        };

        // Last
        if let Some(_) = resolve_reducer_key(processing, scope, key) {
            return Some(Symbol::reducer_key(key));
        };
    }

    return None;
}

#[inline]
#[allow(dead_code)]
pub fn map_expr_using_scope<'input>(expr: &'input ExprValue,
                processing: &DocumentProcessingState,
                scope: &mut ElementOpScope,
                resolution_mode: &BareSymbolResolutionMode)
                -> ExprValue {
    match expr {
        &ExprValue::Expr(ref op, ref l, ref r) => {
            let l_vars = map_expr_using_scope(l, processing, scope, resolution_mode);
            let r_vars = map_expr_using_scope(r, processing, scope, resolution_mode);

            let left_expr = Box::new(l_vars);
            let right_expr = Box::new(r_vars);

            ExprValue::Expr(op.clone(), left_expr, right_expr)
        }

        &ExprValue::SymbolReference(ref sym) => {
            if let Some(sym) = resolve_sym(sym, processing, scope) {
                return ExprValue::SymbolReference(sym);
            };

            expr.clone()
        }

        &ExprValue::DefaultVariableReference => {
            // NOTE: This is currently used primarily for action expressions

            // If we have a valid default var in the scope, expand the DefaultVariableReference into a symbol reference
            // if let Some(ref sym) = (scope.0).2 {
            //     return ExprValue::SymbolReference(sym.clone());
            // };

            ExprValue::DefaultVariableReference
        }

        _ => expr.clone()
    }
}


#[inline]
pub fn map_lens_using_scope<'input>(lens: Option<&LensExprType>,
                processing: &DocumentProcessingState,
                scope: &mut ElementOpScope)
                -> Option<LensExprType> {
    match lens {
        Some(&LensExprType::ForLens(ref ele_key, ref coll_sym)) => {
            let ele_key = ele_key.as_ref().map(|s| s.clone());
            if let Some(resolved) = resolve_sym(coll_sym, processing, scope) {
                return Some(LensExprType::ForLens(ele_key, resolved))
            };
        }
        Some(&LensExprType::GetLens(ref sym)) => {
            if let Some(resolved) = resolve_sym(sym, processing, scope) {
                let resolved = match resolved.sym_ref() {
                    &SymbolReferenceType::ResolvedReference(_, ResolvedSymbolType::ReducerKeyReference(ref key)) =>
                        Symbol::prop(key),
                    _ => resolved.clone()
                };
                return Some(LensExprType::GetLens(resolved));
            };

            // None
            // let resolution_mode = BareSymbolResolutionMode::PropThenReducerKey;

            // // Resolve variable as reducer key reference first
            // // let sym = resolve_existing_symbol()

            // let mut lens_scope = scope.clone();
            // if let &ExprValue::VariableReference(ref prop_key) = lens_expr {
            //     lens_scope.with_prop(prop_key, None, None);
            // };

            // let expr = map_expr_using_scope(lens_expr, processing, &mut lens_scope, &resolution_mode);
            // Some(LensExprType::GetLens(expr))
        }
        _ => {}
    };

    None
}