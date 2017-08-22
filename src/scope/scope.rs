#![allow(dead_code)]

use parser::ast::*;
use parser::util::allocate_element_key;
use scope::symbol_paths::*;


#[derive(Debug, Clone)]
pub struct Scope {
    scope_id: String,
    map_id: String,

    symbol_path: SymbolPathScope
}

impl Scope {
    pub fn new(map_id: &str, symbol_path: Option<SymbolPathScope>) -> Scope {
        let scope_id = allocate_element_key();
        let symbol_path = symbol_path.map_or_else(|| Default::default(), |symbol_path| symbol_path);
        Scope {
            scope_id: scope_id,
            map_id: map_id.to_owned(),
            symbol_path: symbol_path
        }
    }

    pub fn with_map_id(map_id: &str) -> Scope {
        Scope::new(map_id, None)
    }

    pub fn id(&self) -> &str { &self.scope_id }
    pub fn map_id(&self) -> &str { &self.map_id }
    pub fn symbol_path(&self) -> &SymbolPathScope { &self.symbol_path }

    pub fn append_path_expr(&mut self, expr: &ExprValue) {
        self.symbol_path.append_expr(expr);
    }

    pub fn append_path_str(&mut self, s: &str) {
        self.symbol_path.append_str(s);
    }

    pub fn join_path_as_expr(&self, s: Option<&str>) -> ExprValue {
        self.symbol_path.join_as_expr(s)
    }

    pub fn prop(&self, key: &str) -> Symbol {
        Symbol::prop(key, self.id())
    }

    pub fn param(&self, key: &str) -> Symbol {
        Symbol::param(key, self.id())
    }

    pub fn unbound_formal_param(&self, key: &str) -> Symbol {
        Symbol::unbound_formal_param(key, Some(self.id()))
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