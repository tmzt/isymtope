use model::*;
use parser::util::allocate_element_key;
use scope::*;


#[derive(Debug, Clone)]
pub struct Scope {
    scope_id: String,
    map_id: String,

    parent_id: Option<String>,

    symbol_path: SymbolPathScope,
    action_path: SymbolPathScope
}

impl Scope {

    #[allow(dead_code)]
    pub fn new(map_id: &str) -> Self {
    	Scope::create(map_id, None, None, None)
    }

    pub fn new_from_parent(map_id: &str, parent_scope: &mut Scope) -> Scope {
        let symbol_path = parent_scope.symbol_path().clone();
        let action_path = parent_scope.action_path().clone();
    	Scope::create(map_id, Some(parent_scope.id()), Some(symbol_path), Some(action_path)) 
    }

    pub fn with_map_id(map_id: &str) -> Scope {
        Scope::create(map_id, None, None, None)
    }

    fn create(map_id: &str, parent_id: Option<&str>, symbol_path: Option<SymbolPathScope>, action_path: Option<SymbolPathScope>) -> Scope {
        let scope_id = allocate_element_key();
        let symbol_path = symbol_path.map_or_else(|| SymbolPathScope::with_sep(".") , |symbol_path| symbol_path);
        let action_path = action_path.map_or_else(|| SymbolPathScope::with_sep("_"), |action_path| action_path);
        let parent_id = parent_id.map(|parent_id| parent_id.to_owned());
        Scope {
            scope_id: scope_id,
            map_id: map_id.to_owned(),
            parent_id: parent_id,
            symbol_path: symbol_path,
            action_path: action_path
        }
    }

    pub fn id(&self) -> &str { &self.scope_id }
    pub fn map_id(&self) -> &str { &self.map_id }
    pub fn parent_id(&self) -> Option<&str> { self.parent_id.as_ref().map(|s| s.as_str()) }

    pub fn symbol_path(&self) -> &SymbolPathScope { &self.symbol_path }
    pub fn action_path(&self) -> &SymbolPathScope { &self.action_path }

    pub fn symbol_path_mut(&mut self) -> &mut SymbolPathScope { &mut self.symbol_path }
    pub fn action_path_mut(&mut self) -> &mut SymbolPathScope { &mut self.action_path }
}


#[cfg(test)]
mod tests {
    use super::*;


    #[test]
    pub fn test_scope_symbol_path_joined1() {
        let mut scope = Scope::with_map_id("m1");

        let expr1 = ExprValue::Expr(ExprOp::Add, Box::new(ExprValue::LiteralNumber(1)), Box::new(ExprValue::LiteralNumber(2)));

        scope.symbol_path_mut().append_expr(&expr1);
        scope.symbol_path_mut().append_str("test");

        //let expr = scope.join_path_as_expr(Some("."));
    	let expr = scope.symbol_path().path_expr();
        assert_eq!(expr, ExprValue::Apply(ExprApplyOp::JoinString(Some(".".to_owned())), Some(vec![
            Box::new(ExprValue::Expr(ExprOp::Add, Box::new(ExprValue::LiteralNumber(1)), Box::new(ExprValue::LiteralNumber(2)))),
            Box::new(ExprValue::LiteralString("test".to_owned()))
        ])));
    }
}
