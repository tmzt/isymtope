#![allow(dead_code)]

use linked_hash_map::LinkedHashMap;

use parser::ast::*;
use scope::scope::*;
use scope::symbols::*;


#[derive(Debug, Clone)]
pub struct Context {
    // base_scope: Scope,
    scopes: LinkedHashMap<String, Scope>,
    symbol_maps: LinkedHashMap<String, Symbols>
}

impl Default for Context {
    fn default() -> Context {
        let symbols = Symbols::default();
        let base_scope = Scope::with_map_id(symbols.map_id());
        Context::new(base_scope, symbols)

    }
}

impl Context {
    pub fn new(base_scope: Scope, symbols: Symbols) -> Self {
        let mut ctx = Context {
            // base_scope: base_scope,
            scopes: Default::default(),
            symbol_maps: Default::default()
        };

        ctx.push_scope(base_scope);
        ctx.add_symbol_map(symbols);
        ctx
    }

    fn scope_ref_mut(&mut self) -> Option<&mut Scope> {
        let scope_id = self.scopes.back().map(|s| s.1.id().to_owned());
        if let Some(scope_id) = scope_id {
            return self.scopes.get_mut(&scope_id);
        }
        None
    }

    fn scope_ref(&mut self) -> Option<&Scope> {
        let scope_id = self.scopes.back().map(|s| s.1.id().to_owned());
        if let Some(scope_id) = scope_id {
            return self.scopes.get(&scope_id);
        }
        None
    }

    pub fn scope(&mut self) -> Scope {
        self.scope_ref().unwrap().clone()
    }

    pub fn create_child_scope(&mut self) -> Scope {
        let parent_scope = self.scope();
        let parent_map_id = parent_scope.map_id().to_owned();
        let symbol_path = parent_scope.symbol_path().clone();

        let symbols = Symbols::new(Some(&parent_map_id));
        let map_id = symbols.map_id().to_owned();
        self.add_symbol_map(symbols);

        Scope::new(&map_id, Some(symbol_path))
    }

    pub fn push_scope(&mut self, scope: Scope) {
        self.scopes.insert(scope.id().to_owned(), scope);
    }

    pub fn pop_scope(&mut self) {
        self.scopes.pop_back();
    }

    pub fn push_child_scope(&mut self) {
        let scope = self.create_child_scope();
        self.push_scope(scope);
    }

    pub fn resolve_sym(&mut self, key: &str) -> Option<Symbol> {
        let scope = self.scope();
        let map_id = scope.map_id();

        let mut cur_map = self.symbol_maps.get(map_id);
        while let Some(map) = cur_map {
            if let Some(sym) = map.get_sym(key) {
                return Some(sym.to_owned());
            };

            cur_map = map.parent_map_id().and_then(|id| self.symbol_maps.get(id));
        };

        None
    }

    pub fn add_symbol_map(&mut self, map: Symbols) {
        self.symbol_maps.insert(map.map_id().to_owned(), map);
    }

    pub fn add_sym(&mut self, key: &str, sym: Symbol) {
        let map_id = self.scope().map_id().to_owned();
        if let Some(map) = self.symbol_maps.get_mut(&map_id) {
            map.add_sym(key, sym);
        };
    }

    pub fn append_path_expr(&mut self, expr: &ExprValue) {
        if let Some(scope) = self.scope_ref_mut() {
            scope.append_path_expr(expr);
        };
    }

    pub fn append_path_str(&mut self, s: &str) {
        if let Some(scope) = self.scope_ref_mut() {
            scope.append_path_str(s);
        };
    }

}

#[cfg(test)]
mod tests {
    use parser::ast::*;
    use scope::context::*;

    fn create_symbols(key: &str, sym: Symbol, parent_map_id: Option<&str>) -> Symbols {
        let mut symbols = Symbols::new(parent_map_id);
        symbols.add_sym(key, sym);
        symbols
    }

    fn add_scope_with_symbols(ctx: &mut Context, key: &str, sym: Symbol, parent_map_id: Option<&str>) -> String {
        let symbols = create_symbols(key, sym, parent_map_id);
        let map_id =  format!("{}", symbols.map_id());
        ctx.add_symbol_map(symbols);
        let scope = Scope::with_map_id(&map_id);
        ctx.push_scope(scope);
        map_id
    }

    #[test]
    pub fn test_context_symbol_maps_super1() {
        let mut context = Context::default();

        let map_id = add_scope_with_symbols(&mut context, "abc", Symbol::prop("xyz1"), None);
        let map_id = add_scope_with_symbols(&mut context, "abc", Symbol::prop("xyz2"), Some(&map_id));
        add_scope_with_symbols(&mut context, "abc", Symbol::prop("xyz3"), Some(&map_id));

        // We should resolve the symbol from the nearest scope where it is defined
        assert_eq!(context.resolve_sym("abc"), Some(Symbol::prop("xyz3")));
    }

    #[test]
    pub fn test_context_symbol_maps_super2() {
        let mut context = Context::default();

        let map_id = add_scope_with_symbols(&mut context, "abc", Symbol::prop("xyz1"), None);
        let map_id = add_scope_with_symbols(&mut context, "def", Symbol::prop("xyz2"), Some(&map_id));
        add_scope_with_symbols(&mut context, "ghi", Symbol::prop("xyz3"), Some(&map_id));

        // We should resolve the symbol from the nearest scope where it is defined
        assert_eq!(context.resolve_sym("abc"), Some(Symbol::prop("xyz1")));
    }

    #[derive(Debug, Clone, Default)]
    struct ContextTestDocument {
    }

    #[derive(Debug, Clone, Default)]
    struct ContextTestOutput {
        doc: ContextTestDocument
    }

    impl ContextTestOutput {
        pub fn write_1(&self, _: &str) {

        }
    }

    #[derive(Debug, Clone)]
    struct ContextTest {
        ctx: Context,
        output: ContextTestOutput
    }

    impl ContextTest {
        pub fn example_1(&mut self) {
            let map_id = add_scope_with_symbols(&mut self.ctx, "abc", Symbol::prop("xyz1"), None);
            let map_id = add_scope_with_symbols(&mut self.ctx, "def", Symbol::prop("xyz2"), Some(&map_id));
            add_scope_with_symbols(&mut self.ctx, "ghi", Symbol::prop("xyz3"), Some(&map_id));

            self.output.write_1("zyx");

            // We should resolve the symbol from the nearest scope where it is defined
            assert_eq!(self.ctx.resolve_sym("abc"), Some(Symbol::prop("xyz1")));
        }
    }

    #[test]
    pub fn test_context_1() {
        // let map_id = "m1";
        // let scope = Scope::with_map_id(map_id);
        // let context = Context::with_base_scope(scope);
        let ctx = Context::default();
        let mut context_test = ContextTest { ctx: ctx, output: Default::default() };
        context_test.example_1();
    }

    #[test]
    pub fn test_context_symbol_path_mixed1() {
        let mut ctx = Context::default();
        let mut scope = ctx.scope();

        let expr1 = ExprValue::Expr(ExprOp::Add, Box::new(ExprValue::LiteralNumber(1)), Box::new(ExprValue::LiteralNumber(2)));
        scope.append_path_expr(&expr1);
        scope.append_path_str("test");

        ctx.push_scope(scope);

        let expr = ctx.scope().join_path_as_expr(None);
        assert_eq!(expr, Some(ExprValue::Apply(ExprApplyOp::JoinString(None), Some(vec![
            Box::new(ExprValue::Expr(ExprOp::Add, Box::new(ExprValue::LiteralNumber(1)), Box::new(ExprValue::LiteralNumber(2)))),
            Box::new(ExprValue::LiteralString("test".to_owned()))
        ]))));
    }

    fn create_child_scope_with_symbols(ctx: &mut Context, key: &str, sym: Symbol) -> Scope {
        let parent_map_id = ctx.scope().map_id().to_owned();
        let symbol_path = ctx.scope().symbol_path().clone();
        let symbols = create_symbols(key, sym, Some(&parent_map_id));
        let map_id = symbols.map_id().to_owned();
        ctx.add_symbol_map(symbols);
        Scope::new(&map_id, Some(symbol_path))
    }

    #[test]
    pub fn test_context_scope_nesting1() {
        let mut ctx = Context::default();

        // Lm
        {
            ctx.push_child_scope();
            ctx.append_path_str("Lm");
            ctx.add_sym("abc", Symbol::prop("xyz3"));
        }

        // Lm.No
        {
            ctx.push_child_scope();
            ctx.append_path_str("No");
            ctx.add_sym("abc", Symbol::prop("xyz2"));
            ctx.add_sym("def", Symbol::prop("def2"));
        }

        // Lm.No.Pq
        {
            ctx.push_child_scope();
            ctx.append_path_str("Pq");
            ctx.add_sym("abc", Symbol::prop("xyz3"));
        }

        // The joined path (dynamic) should be a string join operation
        let expr = ctx.scope().join_path_as_expr(Some("."));
        assert_eq!(expr, Some(ExprValue::Apply(ExprApplyOp::JoinString(Some(".".to_owned())), Some(vec![
            Box::new(ExprValue::LiteralString("Lm".to_owned())),
            Box::new(ExprValue::LiteralString("No".to_owned())),
            Box::new(ExprValue::LiteralString("Pq".to_owned()))
        ]))));

        // We should resolve the symbol from the nearest scope where it is defined
        assert_eq!(ctx.resolve_sym("abc"), Some(Symbol::prop("xyz3")));

        // We should resolve the symbol from the nearest scope where it is defined
        assert_eq!(ctx.resolve_sym("def"), Some(Symbol::prop("def2")));
    }
}