// #![allow(dead_code)]

use parser::ast::*;
use parser::util::allocate_element_key;
use scope::context::*;
use scope::symbol_paths::*;


#[derive(Debug, Clone)]
pub struct Scope {
    scope_id: String,
    map_id: String,

    parent_id: Option<String>,

    symbol_path: SymbolPathScope,
    action_path: SymbolPathScope
}

impl Scope {
    pub fn new(map_id: &str, parent_id: Option<&str>, symbol_path: Option<SymbolPathScope>, action_path: Option<SymbolPathScope>) -> Scope {
        let scope_id = allocate_element_key();
        let symbol_path = symbol_path.map_or_else(|| Default::default(), |symbol_path| symbol_path);
        let action_path = action_path.map_or_else(|| Default::default(), |action_path| action_path);
        let parent_id = parent_id.map(|parent_id| parent_id.to_owned());
        Scope {
            scope_id: scope_id,
            map_id: map_id.to_owned(),
            parent_id: parent_id,
            symbol_path: symbol_path,
            action_path: action_path
        }
    }

    pub fn new_from_parent(map_id: &str, parent_scope: &Scope) -> Scope {
        let scope_id = allocate_element_key();
        let symbol_path = parent_scope.symbol_path().clone();
        let action_path = parent_scope.action_path().clone();
        let parent_id = Some(parent_scope.id().to_owned()); 
        Scope {
            scope_id: scope_id,
            map_id: map_id.to_owned(),
            parent_id: parent_id,
            symbol_path: symbol_path,
            action_path: action_path
        }
    }

    pub fn with_map_id(map_id: &str) -> Scope {
        Scope::new(map_id, None, None, None)
    }

    pub fn id(&self) -> &str { &self.scope_id }
    pub fn map_id(&self) -> &str { &self.map_id }
    pub fn parent_id(&self) -> Option<&str> { self.parent_id.as_ref().map(|s| s.as_str()) }
    pub fn symbol_path(&self) -> &SymbolPathScope { &self.symbol_path }
    pub fn action_path(&self) -> &SymbolPathScope { &self.action_path }

    // pub fn get_sym(&self, key: &str) -> Option<&Symbol> {
    //     let map_id = self.map_id();
    //     if let Some(map) = self.symbol_maps.get(map_id) {
    //         return map.get_sym(key)
    //     };
    //     None
    // }

    pub fn append_path_expr(&mut self, expr: &ExprValue) {
        self.symbol_path.append_expr(expr);
    }

    pub fn append_path_str(&mut self, s: &str) {
        self.symbol_path.append_str(s);
    }

    pub fn append_action_path_expr(&mut self, expr: &ExprValue) {
        self.action_path.append_expr(expr);
    }

    pub fn append_action_path_str(&mut self, s: &str) {
        self.action_path.append_str(s);
    }

    pub fn join_path(&self, ctx: &mut Context, s: Option<&str>) -> String {
        self.symbol_path.join_as_str(ctx, s)
    }

    #[allow(dead_code)]
    pub fn join_path_as_expr(&self, s: Option<&str>) -> ExprValue {
        self.symbol_path.join_as_expr(s)
    }

    pub fn join_path_as_expr_with_expr(&self, sep: Option<&str>, last: &ExprValue) -> ExprValue {
        self.symbol_path.join_as_expr_with_expr(sep, last)
    }

    pub fn join_path_as_expr_with(&self, sep: Option<&str>, last: &str) -> ExprValue {
        self.symbol_path.join_as_expr_with(sep, last)
    }

    pub fn join_action_path(&self, ctx: &mut Context, s: Option<&str>) -> String {
        self.action_path.join_as_str(ctx, s)
    }

    pub fn join_action_path_as_expr(&self, s: Option<&str>) -> ExprValue {
        self.action_path.join_as_expr(s)
    }
}


#[cfg(test)]
mod tests {
    use super::*;
    use parser::ast::*;
    use scope::symbol_paths::*;

    #[test]
    pub fn test_scope_symbol_path_joined1() {
        let mut scope = Scope::with_map_id("m1");

        let expr1 = ExprValue::Expr(ExprOp::Add, Box::new(ExprValue::LiteralNumber(1)), Box::new(ExprValue::LiteralNumber(2)));

        scope.append_path_expr(&expr1);
        scope.append_path_str("test");

        let expr = scope.join_path_as_expr(Some("."));
        assert_eq!(expr, ExprValue::Apply(ExprApplyOp::JoinString(Some(".".to_owned())), Some(vec![
            Box::new(ExprValue::Expr(ExprOp::Add, Box::new(ExprValue::LiteralNumber(1)), Box::new(ExprValue::LiteralNumber(2)))),
            Box::new(ExprValue::LiteralString("test".to_owned()))
        ])));
    }
}