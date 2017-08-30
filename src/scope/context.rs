#![allow(dead_code)]

use linked_hash_map::LinkedHashMap;

use parser::ast::*;
use scope::scope::*;
use scope::symbols::*;



pub type PropValue<'a> = (&'a str, Option<&'a ExprValue>);

#[derive(Debug)]
pub struct SymbolResolver<'a, I: Iterator<Item = PropValue<'a>>> {
    ctx: &'a mut Context,
    iter: I
}

impl<'a, I: Iterator<Item = PropValue<'a>>> SymbolResolver<'a, I>
{
    pub fn new(ctx: &'a mut Context, iter: I) -> Self {
        SymbolResolver { ctx: ctx, iter: iter }
    }
}

impl<'a, I: Iterator<Item = PropValue<'a>>> Iterator for SymbolResolver<'a, I>
{
    type Item = Prop;

    fn next(&mut self) -> Option<Self::Item> {
        if let Some(prop) = self.iter.next() {
            let key = prop.0.to_owned();
            let expr = &prop.1;
            let resolved_sym = self.ctx.resolve_sym(&key);
            if let Some(resolved_sym) = resolved_sym {
                let expr = ExprValue::SymbolReference(resolved_sym);
                return Some((key, Some(expr)));
            }; 
            return Some((key, expr.map(|p| p.clone())));
        };
        None
    }
}

#[derive(Debug)]
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

    pub fn scope_ref(&mut self) -> Option<&Scope> {
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
        // let symbol_path = parent_scope.symbol_path().clone();

        let symbols = Symbols::new(Some(&parent_map_id));
        let map_id = symbols.map_id().to_owned();
        self.add_symbol_map(symbols);

        Scope::new_from_parent(&map_id, &parent_scope)
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

    pub fn resolve_unresolved_sym_object(&mut self, sym: &Symbol) -> Option<Symbol> {
        if let &SymbolReferenceType::UnresolvedReference(ref key) = sym.sym_ref() {
            return self.resolve_sym(key);
        };
        None
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

    pub fn resolve_sym_starting_at(&mut self, key: &str, map_id: &str) -> Option<Symbol> {
        let mut cur_map = self.symbol_maps.get(map_id);
        while let Some(map) = cur_map {
            if let Some(sym) = map.get_sym(key) {
                return Some(sym.to_owned());
            };

            cur_map = map.parent_map_id().and_then(|id| self.symbol_maps.get(id));
        };

        None
    }

    pub fn eval_sym(&mut self, sym: &Symbol) -> Option<ExprValue> {

        // let mut cur_sym = sym;
        // match sym.sym_ref() {
        //     &SymbolReferenceType::ResolvedReference(ref key, ResolvedSymbolType::ReferenceToKeyInScope(ref key_ref, Some(ref scope_id)), _) => {
        //         match key_ref {
        //             &KeyReferenceType::UnboundFormalParam => {
        //                 if let Some(ref ref_sym) = self.resolve_sym_starting_at(key, scope_id) {
        //                     return self.eval_sym(ref_sym);
        //                 };

        //             },
        //             _ => ()
        //         };
        //     },

        //     _ => {}
        // };

        // let map_id = self.scope().map_id().to_owned();
        // let mut cur_map = self.symbol_maps.get(&map_id);
        let mut cur_sym = Some(sym);

        while let Some(sym) = cur_sym {
            match sym.sym_ref() {
                &SymbolReferenceType::ResolvedReference(ref sym_key, ResolvedSymbolType::ReferenceToKeyInScope(ref key_ref, Some(ref scope_id)), _) => {
                    match key_ref {
                        &KeyReferenceType::UnboundFormalParam => {
                            // cur_map = self.symbol_maps.get(scope_id);
                            if let Some(map) = self.symbol_maps.get(scope_id) {
                                cur_sym = map.get_sym(sym_key);
                            }
                            continue;
                        }

                        _ => { break; }
                    };
                }

                _ => {}
            };
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

    // pub fn resolve_props<'p, I>(&mut self, props: I) -> SymbolResolver<'p, I>
    //     where I: Iterator<Item = PropValue<'p>>
    // {
    //     SymbolResolver::new(&mut self, props)
    // }

    pub fn reduce_expr_to_string(&self, expr: &ExprValue) -> String {
        match expr {
            &ExprValue::LiteralString(ref s) => format!("{}", s),
            &ExprValue::LiteralNumber(ref n) => format!("{}", n),
            &ExprValue::LiteralArray(_) => format!("[array]"),
            _ => {
                if let Some(expr) = self.reduce_expr(expr) {
                    return self.reduce_expr_to_string(&expr);
                };
                "undefined".to_owned()
            }
        }
    }

    pub fn reduce_expr(&self, expr: &ExprValue) -> Option<ExprValue> {
        if expr.is_literal() { return Some(expr.clone()); }
        match expr {
            &ExprValue::Expr(ref op, box ref l_expr, box ref r_expr) => {
                let l_reduced = self.reduce_expr(&l_expr).unwrap_or_else(|| l_expr.clone());
                let r_reduced = self.reduce_expr(&r_expr).unwrap_or_else(|| r_expr.clone());
                let l_string = match &l_reduced { &ExprValue::LiteralString(..) => true, _ => false };
                let r_string = match &r_reduced { &ExprValue::LiteralString(..) => true, _ => false };

                match (op, &l_reduced, &r_reduced) {
                    (&ExprOp::Add, _, _) if (l_string || r_string) => {
                        let l_string = self.reduce_expr_to_string(&l_reduced);
                        let r_string = self.reduce_expr_to_string(&r_reduced);
                        Some(ExprValue::LiteralString(format!("{}{}", l_string, r_string)))
                    }

                    (&ExprOp::Add, &ExprValue::LiteralNumber(ref l_num), &ExprValue::LiteralNumber(ref r_num)) => {
                        Some(ExprValue::LiteralNumber(l_num + r_num))
                    }

                    _ => None
                }
            },

            _ => None
        }
    }

    pub fn reduce_expr_or_return_same(&self, expr: &ExprValue) -> ExprValue {
        self.reduce_expr(expr).unwrap_or(expr.clone())
    }

    pub fn join_path_as_expr(&mut self, s: Option<&str>) -> ExprValue {
        self.scope().join_path_as_expr(s)
    }

    pub fn join_path(&mut self, s: Option<&str>) -> String {
        self.scope().join_path(self, s)
    }

    pub fn join_path_with(&mut self, s: Option<&str>, last: &str) -> String {
        let key = self.scope().join_path(self, s);
        if key.len() > 0 { format!("{}.{}", key, last) } else { last.to_owned() }
    }

    pub fn join_action_path_as_expr(&mut self, s: Option<&str>) -> ExprValue {
        self.scope().join_action_path_as_expr(s)
    }

    pub fn join_action_path(&mut self, s: Option<&str>) -> String {
        self.scope().join_action_path(self, s)
    }

    pub fn prop(&mut self, key: &str) -> Symbol {
        self.scope().prop(key)
    }

    pub fn param(&mut self, key: &str) -> Symbol {
        self.scope().param(key)
    }

    pub fn unbound_formal_param(&mut self, key: &str) -> Symbol {
        self.scope().unbound_formal_param(key)
    }

    pub fn add_unbound_formal_param(&mut self, key: &str) {
        let formal = self.unbound_formal_param(key);
        self.add_sym(key, formal);
    }

    /// Add prop to component that refers to a key defined in another scope.
    pub fn add_component_prop_ref(&mut self, key: &str, prop_key: &str, scope_id: Option<&str>) {
        let prop = Symbol::component_prop(key, prop_key, scope_id);
        self.add_sym(key, prop);
    }

    /// Add prop to element that refers to a key defined in another scope.
    pub fn add_element_prop_ref(&mut self, key: &str, prop_key: &str, scope_id: Option<&str>) {
        let prop = Symbol::element_prop(key, prop_key, scope_id);
        self.add_sym(key, prop);
    }

    pub fn add_prop(&mut self, prop_key: &str) {
        let prop = self.prop(prop_key);
        self.add_sym(prop_key, prop);
    }

    pub fn add_invocation_prop(&mut self, key: &str, expr: Option<&ExprValue>) {
        let invocation_prop = Symbol::invocation_prop(key, expr);
        self.add_sym(key, invocation_prop);
    }

    pub fn add_param_ref_to(&mut self, key: &str, param_key: &str) {
        let param = self.param(param_key);
        self.add_sym(key, param);
    }
}

#[cfg(test)]
mod tests {
    use std::iter::*;
    use parser::ast::*;
    use scope::context::*;


    // Expressions

    #[test]
    pub fn test_expr_two_numbers() {
        let expr1 = ExprValue::LiteralNumber(1);
        let expr2 = ExprValue::LiteralNumber(2);
        let expr = ExprValue::Expr(ExprOp::Add, Box::new(expr1), Box::new(expr2));

        let ctx = Context::default();
        
        assert_eq!(ctx.reduce_expr(&expr), Some(ExprValue::LiteralNumber(3)));
    }

    // Symbols

    fn create_symbols(key: &str, sym: Symbol, parent_map_id: Option<&str>) -> Symbols {
        let mut symbols = Symbols::new(parent_map_id);
        symbols.add_sym(key, sym);
        symbols
    }

    #[test]
    pub fn test_context_symbol_path_mixed1() {
        let mut ctx = Context::default();
        let mut scope = ctx.scope();

        let expr1 = ExprValue::Expr(ExprOp::Add, Box::new(ExprValue::LiteralNumber(1)), Box::new(ExprValue::LiteralNumber(2)));
        scope.append_path_expr(&expr1);
        scope.append_path_str("test");

        ctx.push_scope(scope);

        let expr = ctx.join_path_as_expr(None);
        assert_eq!(expr, ExprValue::Apply(ExprApplyOp::JoinString(None), Some(vec![
            Box::new(ExprValue::Expr(ExprOp::Add, Box::new(ExprValue::LiteralNumber(1)), Box::new(ExprValue::LiteralNumber(2)))),
            Box::new(ExprValue::LiteralString("test".to_owned()))
        ])));
    }

    #[test]
    pub fn test_context_scope_element_nesting1() {
        let mut ctx = Context::default();

        // Lm
        {
            ctx.push_child_scope();
            ctx.append_path_str("Lm");
            // ctx.add_sym("abc", ctx.prop("xyz3"));
            ctx.add_param_ref_to("abc", "xyz3");
        }

        // Lm.No
        {
            ctx.push_child_scope();
            ctx.append_path_str("No");
            // ctx.add_sym("abc", ctx.prop("xyz2"));
            // ctx.add_sym("def", ctx.prop("def2"));
            ctx.add_param_ref_to("abc", "xyz2");
            ctx.add_param_ref_to("def", "def2");
        }

        // Lm.No.Pq
        {
            ctx.push_child_scope();
            ctx.append_path_str("Pq");
            // ctx.add_sym("abc", ctx.prop("xyz3"));
            ctx.add_param_ref_to("abc", "xyz3");
        }

        // The joined path (dynamic) should be a string join operation
        let expr = ctx.join_path_as_expr(Some("."));
        assert_eq!(expr, ExprValue::Apply(ExprApplyOp::JoinString(Some(".".to_owned())), Some(vec![
            Box::new(ExprValue::LiteralString("Lm".to_owned())),
            Box::new(ExprValue::LiteralString("No".to_owned())),
            Box::new(ExprValue::LiteralString("Pq".to_owned()))
        ])));

        // We should resolve the symbol from the nearest scope where it is defined
        // assert_eq!(ctx.resolve_sym("abc"), Some(Symbol::prop("xyz3")));

        // We should resolve the symbol from the nearest scope where it is defined
        // assert_eq!(ctx.resolve_sym("def"), Some(Symbol::prop("def2")));
    }

    #[derive(Debug, Clone, Default)]
    struct TestProcessor2 {}

    #[derive(Debug, Clone, Default)]
    struct TestOutput2 {}

    type FormalProp<'a> = (&'a str);
    type FormalPropVec<'a> = Vec<FormalProp<'a>>;

    type PropKeyRef = (String, String);
    type PropKeyRefVec = Vec<PropKeyRef>;

    type PropValue<'a> = (&'a str, Option<&'a ExprValue>);
    type PropValueVec<'a> = Vec<PropValue<'a>>;

    impl TestProcessor2 {
        pub fn push_component_definition_scope<'a, I>(&mut self, ctx: &mut Context, _component_ty: &str, formals: I)
          where I: IntoIterator<Item = &'a FormalProp<'a>>
        {
            ctx.push_child_scope();
            for formal in formals {
                ctx.add_unbound_formal_param(formal);
            }
        }

        pub fn push_element_parameter_definition_scope<'a, I>(&mut self, ctx: &mut Context, _element_id: &str, _element_ty: &str, props: I)
          where I: IntoIterator<Item = &'a PropKeyRef>
        {
            let parent_scope_id = ctx.scope().id().to_owned();
            ctx.push_child_scope();
            for prop in props {
                ctx.add_element_prop_ref(&prop.0, &prop.1, Some(&parent_scope_id));
            }
        }

        pub fn push_element_scope(&mut self, ctx: &mut Context, element_id: &str, _element_ty: &str) {
            ctx.push_child_scope();
            ctx.append_path_str(element_id);
        }
    }

    impl TestOutput2 {
        pub fn push_component_instance_invocation_scope<'a, I>(&mut self, ctx: &mut Context, _component_ty: &str, props: I)
          where I: IntoIterator<Item = &'a PropValue<'a>>
        {
            // let parent_scope_id = ctx.scope().id().to_owned();
            ctx.push_child_scope();
            for prop in props {
                ctx.add_invocation_prop(&prop.0, prop.1);
            }
        }

        pub fn push_component_instance_scope(&mut self, ctx: &mut Context, instance_id: &str, _component_ty: &str) {
            ctx.push_child_scope();
            ctx.append_path_str(instance_id);
        }

        pub fn push_element_parameter_definition_scope<'a, I>(&mut self, ctx: &mut Context, element_id: &str, _element_ty: &str, props: I)
          where I: IntoIterator<Item = &'a PropKeyRef>
        {
            let parent_scope_id = ctx.scope().id().to_owned();
            ctx.push_child_scope();
            ctx.append_path_str(element_id);
            for prop in props {
                ctx.add_element_prop_ref(&prop.0, &prop.1, Some(&parent_scope_id));
            }
        }
    }

    #[test]
    pub fn test_context_scope_component_processing1() {
        let mut ctx = Context::default();
        let mut processor = TestProcessor2::default();

        // component Component(todo)
        // Create new component context with unbound formal prop (todo)
        {
            let formals: FormalPropVec = vec![("todo")];
            processor.push_component_definition_scope(&mut ctx, "Component", formals.iter());
        }
        let comp_definition_scope_id = ctx.scope().id().to_owned();

        // within Component definition
        {
            // The local (todo) should be an unbound formal prop (todo)
            assert_eq!(ctx.resolve_sym("todo"),
                // Some(Symbol::ref_prop_in_scope("todo", "todo", Some(&lm_element_scope_id)))
                Some(Symbol::unbound_formal_param("todo", Some(&comp_definition_scope_id)))
            );
        }

        // element Pq invocation
        {
            let props: PropKeyRefVec = vec![
                ("value".into(), "todo".into())
            ];
            processor.push_element_parameter_definition_scope(&mut ctx, "Pq", "input", props.iter());
        }
        // let element_pq_invocation_scope_id = ctx.scope().id().to_owned();

        // element Pq scope
        {
            processor.push_element_scope(&mut ctx, "Pq", "input");

            // The local (todo) should still be an unbound formal param (todo)
            assert_eq!(ctx.resolve_sym("todo"),
                Some(Symbol::unbound_formal_param("todo", Some(&comp_definition_scope_id)))
            );
        }
    }

    #[test]
    pub fn test_context_scope_component_nesting1() {
        let mut ctx = Context::default();
        let mut output = TestOutput2::default();

        // Lm
        // Element: Lm()
        // Invokes: CompNo(todo = store.getState().todo)
        {
            ctx.push_child_scope();
            ctx.append_path_str("Lm");

            // Our element path should be (Lm)
            let expr = ctx.join_path_as_expr(Some("."));
            assert_eq!(expr, ExprValue::Apply(ExprApplyOp::JoinString(Some(".".to_owned())), Some(vec![
                Box::new(ExprValue::LiteralString("Lm".to_owned()))
            ])));
        }
        // let lm_element_scope_id = ctx.scope().id().to_owned();

        // Lm
        // Comp1 definition (loaded)
        // {
        //     output.push_component_definition_param_bindings_scope(&mut ctx);
        // }

        // Lm
        // Invoke: CompNo(todo = store.getState().todo)
        {
            let todo_value = ExprValue::SymbolReference(Symbol::reducer_key("todo"));
            let props: PropValueVec = vec![
                ("todo".into(), Some(&todo_value))
            ];
            output.push_component_instance_invocation_scope(&mut ctx, "Component1", props.iter());

            // Our element path should still be the same (Lm)
            let expr = ctx.join_path_as_expr(Some("."));
            assert_eq!(expr, ExprValue::Apply(ExprApplyOp::JoinString(Some(".".to_owned())), Some(vec![
                Box::new(ExprValue::LiteralString("Lm".to_owned()))
            ])));
        }

        // Lm.Comp1
        // Component contents will be output
        {
            // let parent_scope_id = ctx.scope().id().to_owned();
            output.push_component_instance_scope(&mut ctx, "Comp1", "Component1");

            // The joined path (dynamic) should be a string join operation
            let expr = ctx.join_path_as_expr(Some("."));
            assert_eq!(expr, ExprValue::Apply(ExprApplyOp::JoinString(Some(".".to_owned())), Some(vec![
                Box::new(ExprValue::LiteralString("Lm".to_owned())),
                Box::new(ExprValue::LiteralString("Comp1".to_owned())),
            ])));

            // The local (todo) should resolve to a reducer key reference (todo)
            // assert_eq!(ctx.resolve_sym("todo"), Some(Symbol::param("todo", _)));
            assert_eq!(ctx.resolve_sym("todo"),
                // Some(Symbol::ref_prop_in_scope("todo", "todo", Some(&lm_element_scope_id)))
                Some(Symbol::invocation_prop("todo", Some(&ExprValue::SymbolReference(Symbol::reducer_key("todo")))))
            );
        }

        // Lm.Comp1.Pq
        // Element within component definition
        // Element parameter definition scope
        {
            let props: PropKeyRefVec = vec![
                ("value".into(), "todo".into())
            ];
            output.push_element_parameter_definition_scope(&mut ctx, "Pq", "input", props.iter());
        }

        // The joined path (dynamic) should be a string join operation
        let expr = ctx.join_path_as_expr(Some("."));
        assert_eq!(expr, ExprValue::Apply(ExprApplyOp::JoinString(Some(".".to_owned())), Some(vec![
            Box::new(ExprValue::LiteralString("Lm".to_owned())),
            Box::new(ExprValue::LiteralString("Comp1".to_owned())),
            Box::new(ExprValue::LiteralString("Pq".to_owned()))
        ])));

        // The local var (param) should resolve to a param
        // assert_eq!(ctx.resolve_sym("todo"), Some(Symbol::param("todo", _)));

        // We should resolve the symbol from the nearest scope where it is defined
        // assert_eq!(ctx.resolve_sym("abc"), Some(Symbol::prop("xyz3")));

        // We should resolve the symbol from the nearest scope where it is defined
        // assert_eq!(ctx.resolve_sym("def"), Some(Symbol::prop("def2")));
    }


    #[derive(Debug, Clone, Default)]
    struct TestOutputWriter {}

    impl TestOutputWriter {
        pub fn push_component_instance_scope(&mut self, ctx: &mut Context, element_id: &str) {
            ctx.push_child_scope();
            ctx.append_path_str(element_id);
            ctx.add_param_ref_to("todo", "todo");
            // ctx.add_sym("todo", ctx.param("todo"));
        }
    }

    #[test]
    pub fn test_context_output_writer() {
        let mut ctx = Context::default();
        let mut output = TestOutputWriter::default();

        // Comp1
        output.push_component_instance_scope(&mut ctx, "Comp1");

        // The joined path (dynamic) should be a string join operation
        let expr = ctx.join_path_as_expr(Some("."));
        assert_eq!(expr, ExprValue::Apply(ExprApplyOp::JoinString(Some(".".to_owned())), Some(vec![
            Box::new(ExprValue::LiteralString("Comp1".to_owned())),
        ])));

        // The local var (param) should resolve to a param
        // assert_eq!(ctx.resolve_sym("todo"), Some(Symbol::param("todo", _)));
    }
}