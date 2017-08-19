#![allow(dead_code)]

use linked_hash_map::LinkedHashMap;

use parser::ast::*;
use scope::scope::*;
use scope::symbols::*;


#[derive(Debug, Clone, Default)]
pub struct Context {
    base_scope: Scope,
    scopes: LinkedHashMap<String, Scope>,
    symbol_maps: LinkedHashMap<String, Symbols>
}

impl Context {
    pub fn with_base(base_scope: Scope) -> Self {
        Context {
            base_scope: base_scope,
            scopes: Default::default(),
            symbol_maps: Default::default()
        }
    }

    pub fn scope(&mut self) -> Scope {
        if let Some(scope) = self.scopes.back().map(|s| s.1.clone()) {
            return scope;
        }

        self.base_scope.clone()
    }

    pub fn push_scope(&mut self, scope: Scope) {
        self.scopes.insert(scope.id().to_owned(), scope);
    }

    pub fn pop_scope(&mut self) {
        self.scopes.pop_back();
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
    struct ContextTestOutput {
    }

    impl ContextTestOutput {
        pub fn write_1(&self, _: &str) {

        }
    }

    #[derive(Debug, Clone, Default)]
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
        let mut context_test = ContextTest::default();
        context_test.example_1();
    }
}