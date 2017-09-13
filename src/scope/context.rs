// #![allow(dead_code)]

use linked_hash_map::LinkedHashMap;
use itertools::Itertools;

use parser::*;
use scope::*;
use scope::symbols::*;
use processing::*;


#[derive(Debug, Clone)]
pub enum SymbolValueEntry {
    Value(ExprValue),
    Symbol(Symbol)
}

pub type SymbolMap = LinkedHashMap<String, SymbolValueEntry>;

pub type PropValue<'a> = (&'a str, Option<&'a ExprValue>);

#[allow(dead_code)]
#[derive(Debug)]
pub struct ScopeSymbolIter<'a> {
    ctx: &'a mut Context,
    scope_id: Option<String>,
    key: String
}

impl<'a> ScopeSymbolIter<'a>
{
    #[allow(dead_code)]
    pub fn new(ctx: &'a mut Context, key: &str, scope_id: &str) -> Self {
        let scope_id = Some(scope_id.to_owned());
        let key = key.to_owned();

        ScopeSymbolIter {
            ctx: ctx,
            scope_id: scope_id,
            key: key
        }

    }
}

pub enum ScopeSymbolState {
    NoSymbol,
    InterimSymbol(Symbol),
    FinalSymbol(Symbol)
}

impl<'a> Iterator for ScopeSymbolIter<'a>
{
    type Item = ScopeSymbolState;

    fn next(&mut self) -> Option<Self::Item> {
        if self.scope_id.is_none() { return None; }
        let key = self.key.to_owned();

        let scope_id = self.scope_id.as_ref().map(|s| s.to_owned());
        let map_id = scope_id.as_ref().and_then(|scope_id| self.ctx.get_scope(scope_id)).map(|scope| scope.map_id().to_owned());
        let parent_id = scope_id.as_ref().and_then(|scope_id| self.ctx.get_scope(scope_id)).and_then(|scope| scope.parent_id().map(|s| s.to_owned()));

        // let next_key = Some(key.to_owned());

        // Prepare next iteration
        self.scope_id = parent_id.clone();

        let sym = map_id.as_ref()
            .and_then(|map_id| self.ctx.get_map(map_id))
            .and_then(|map| map.get_sym(&key)).map(|s| s.to_owned());

        let next_key = match sym.as_ref().map(|sym| sym.sym_ref()) {
            Some(sym_ref) => {
                match sym_ref {
                    &SymbolReferenceType::Binding(ref binding) => {
                        match binding {
                            // &BindingType::ComponentFormalProp => {
                            //     // Special case, ignore this symbol and continue resolving
                            //     return Some(ScopeSymbolState::InterimSymbol(Symbol::binding(&BindingType::ComponentFormalProp)));
                            // }

                            &BindingType::ComponentPropBinding(ref prop_key) => Some(prop_key.to_owned()),
                            &BindingType::MapItemBinding => {
                                // Special case, ignore this symbol and continue resolving
                                return Some(ScopeSymbolState::InterimSymbol(Symbol::binding(&BindingType::MapItemBinding)));
                            }
                            _ => None
                        }
                    },
                    _ => None
                }
            },
            _ => None
        };

        if let Some(sym) = sym {
            if let Some(next_key) = next_key {
                self.key = next_key;
                return Some(ScopeSymbolState::InterimSymbol(sym));
            } else {
                return Some(ScopeSymbolState::FinalSymbol(sym));
            };
        };

        if parent_id.is_none() { return None; }

        Some(ScopeSymbolState::NoSymbol)
    }
}



#[derive(Debug)]
pub struct Context {
    base_scope_id: String,
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
        let base_scope_id = base_scope.id().to_owned();

        let mut ctx = Context {
            base_scope_id: base_scope_id,
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

    pub fn get_scope(&mut self, scope_id: &str) -> Option<&Scope> {
        self.scopes.get(scope_id)
    }

    pub fn get_map(&mut self, map_id: &str) -> Option<&Symbols> {
        self.symbol_maps.get(map_id)
    }

    pub fn create_child_scope(&mut self) -> Scope {
        let parent_map_id = self.scope_ref().unwrap().map_id().to_owned();

        let symbols = Symbols::new(Some(&parent_map_id));
        let map_id = symbols.map_id().to_owned();
        self.add_symbol_map(symbols);

        Scope::new_from_parent(&map_id, self.scope_ref().unwrap())
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

    #[allow(dead_code)]
    pub fn resolve_symbol_to_symbol(&mut self, sym: &Symbol) -> Symbol {
        if let &SymbolReferenceType::UnresolvedReference(ref key) = sym.sym_ref() {
            if let Some(sym) = self.resolve_sym(key) { return sym; }
        };

        if let &SymbolReferenceType::UnresolvedPathReference(ref path) = sym.sym_ref() {
            if let Some(sym) = self.resolve_path_reference_to_symbol(path) { return sym; }
        };
        sym.clone()
    }

    pub fn resolve_path_reference_to_symbol(&mut self, path: &str) -> Option<Symbol> {
        if path.contains(".") {
            let mut splitter = path.split(".");
            let first = splitter.next();
            if let Some(first) = first.and_then(|s| self.resolve_sym(s)) {
                let rest = splitter.join(".");
                return Some(Symbol::member_path(first, &rest));
            }
        } else {
            if let Some(sym) = self.resolve_sym(path) {
                return Some(sym);
            }
        }
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

    #[allow(dead_code)]
    pub fn get_sym(&mut self, key: &str) -> Option<&Symbol> {
        let map_id = self.map_id().to_owned();
        if let Some(map) = self.symbol_maps.get(&map_id) {
            return map.get_sym(key)
        };
        None
    }

    #[allow(dead_code)]
    pub fn get_value(&mut self, key: &str) -> Option<&ExprValue> {
        let map_id = self.map_id().to_owned();
        if let Some(map) = self.symbol_maps.get(&map_id) {
            return map.get_value(key)
        };
        None
    }

    #[allow(dead_code)]
    pub fn resolve_key_from_scope(&mut self, key: &str, scope_id: &str) -> Option<Symbol> {
        let iter = ScopeSymbolIter::new(self, key, scope_id);
        // let mut res: Option<ScopeSymbolState> = None;
        for sym in iter {
            if let ScopeSymbolState::FinalSymbol(sym) = sym {
                return Some(sym);
            };
        }
        None
    }

    #[allow(dead_code)]
    pub fn map_id(&mut self) -> &str {
        self.scope_ref().unwrap().map_id()
    }

    #[allow(dead_code)]
    pub fn parent_map_id(&mut self) -> Option<&str> {
        let map_id = self.map_id().to_owned();
        self.symbol_maps.get(&map_id).unwrap().parent_map_id()
    }

    // #[allow(dead_code)]
    // pub fn resolve_sym_from_parent(&mut self, key: &str) -> Option<Symbol> {
    //     let parent_id = self.parent_map_id().map(|s| s.to_owned());
    //     if let Some(parent_id) = parent_id {
    //         if let Some((_, sym)) = self.search_sym_starting_at(key, &parent_id) {
    //             return Some(sym.to_owned());
    //         };
    //         // return self.resolve_sym_starting_at(key, &parent_id);
    //     };

    //     None
    // }

    // pub fn resolve_sym(&mut self, key: &str) -> Option<Symbol> {
    //     let map_id = self.map_id().to_owned();
    //     if let Some((_, sym)) = self.search_sym_starting_at(key, &map_id) {
    //         return Some(sym.to_owned());
    //     };

    //     None
    // }

    pub fn add_symbol_map(&mut self, map: Symbols) {
        self.symbol_maps.insert(map.map_id().to_owned(), map);
    }

    pub fn add_sym(&mut self, key: &str, sym: Symbol) {
        let map_id = self.scope().map_id().to_owned();
        if let Some(map) = self.symbol_maps.get_mut(&map_id) {
            map.add_sym(key, sym);
        };
    }

    pub fn add_value(&mut self, key: &str, expr: ExprValue) {
        let map_id = self.scope().map_id().to_owned();
        if let Some(map) = self.symbol_maps.get_mut(&map_id) {
            map.add_value(key, expr);
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

    #[allow(dead_code)]
    pub fn append_action_path_expr(&mut self, expr: &ExprValue) {
        if let Some(scope) = self.scope_ref_mut() {
            scope.append_action_path_expr(expr);
        };
    }

    pub fn append_action_path_str(&mut self, s: &str) {
        if let Some(scope) = self.scope_ref_mut() {
            scope.append_action_path_str(s);
        };
    }

    // pub fn resolve_props<'p, I>(&mut self, props: I) -> SymbolResolver<'p, I>
    //     where I: Iterator<Item = PropValue<'p>>
    // {
    //     SymbolResolver::new(&mut self, props)
    // }

    pub fn reduce_expr_to_string(&mut self, expr: &ExprValue) -> String {
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

    pub fn reduce_expr_and_resolve_to_string(&mut self, doc: &Document, expr: &ExprValue) -> Option<String> {
        if let Some(expr) = self.eval_expr(doc, expr) {
            return Some(self.reduce_expr_to_string(&expr));
        };
        None
    }

    #[allow(dead_code)]
    pub fn eval_key(&mut self, doc: &Document, key: &str) -> Option<ExprValue> {
        let scope_id = self.scope_ref().unwrap().id().to_owned();
        if let Some(sym) = self.resolve_key_from_scope(key, &scope_id) {
                return self.eval_sym(doc, &sym);
        };
        None
    }

    #[allow(dead_code)]
    pub fn eval_binding(&mut self, doc: &Document, binding: &BindingType) -> Option<ExprValue> {
        match binding {
            &BindingType::ReducerPathBinding(ref reducer_path) => {
                if let Some(ref reducer_data) = doc.reducer_key_data.get(reducer_path) {
                    if let Some(ref expr) = reducer_data.default_expr {
                        return self.reduce_expr(expr);
                    };
                };
            }
            _ => {}
        };

        None
    }

    #[allow(dead_code)]
    pub fn eval_sym(&mut self, doc: &Document, sym: &Symbol) -> Option<ExprValue> {
        if let Some(resolved_key) = sym.resolved_key() {
            if let Some(expr) = self.get_value(resolved_key) {
                return Some(expr.to_owned());
            };

            let scope_id = self.scope_ref().unwrap().id().to_owned();
            if let Some(sym) = self.resolve_key_from_scope(resolved_key, &scope_id) {
                return self.eval_sym(doc, &sym);
            }
        };

        match sym.sym_ref() {
            &SymbolReferenceType::InitialValue(_, box ref after) => {
                return self.eval_sym(doc, after);
            }

            &SymbolReferenceType::Binding(ref binding) => {
                return self.eval_binding(doc, binding);
            }

            &SymbolReferenceType::MemberPath(box ref first, ref rest) => {
                let first_expr = self.eval_sym(doc, first);
                // let rest_iter = rest.as_ref().map(|rest| rest.iter().map(|s| s.as_str()));
                if let Some(first_expr) = first_expr {
                    if let &Some(ref rest) = rest {
                        if let Some(expr) = first_expr.member_ref(rest.as_str()) {
                            return self.eval_expr(doc, &expr);
                        };
                    };
                    return self.eval_expr(doc, &first_expr);
                };
            }

            _ => {}
        };

        None
    }

    #[allow(dead_code)]
    pub fn eval_expr(&mut self, doc: &Document, expr: &ExprValue) -> Option<ExprValue> {
        if expr.is_literal() { return Some(expr.clone()); }
        match expr {
            &ExprValue::Expr(ref op, box ref l_expr, box ref r_expr) => {
                let l_reduced = self.eval_expr(doc, l_expr);
                let r_reduced = self.eval_expr(doc, r_expr);

                match (op, &l_reduced, &r_reduced) {
                    (&ExprOp::Add, &Some(ref l_reduced), &Some(ref r_reduced)) if l_reduced.peek_is_string() || r_reduced.peek_is_string() => {
                        let l_str = self.reduce_expr_and_resolve_to_string(doc, l_reduced).unwrap_or("undefined".to_owned());
                        let r_str = self.reduce_expr_and_resolve_to_string(doc, r_reduced).unwrap_or("undefined".to_owned());
                        return Some(ExprValue::LiteralString(format!("{}{}", l_str, r_str)));
                    }

                    _ => {}
                };
                
            }

            &ExprValue::SymbolReference(ref sym) => {
                return self.eval_sym(doc, sym);
            }

            &ExprValue::Binding(ref binding) => {
                return self.eval_binding(doc, binding);
                // if let &BindingType::ReducerPathBinding(ref reducer_path) = binding {
                //     if let Some(ref reducer_data) = doc.reducer_key_data.get(reducer_path) {
                //         if let Some(ref expr) = reducer_data.default_expr {
                //             return self.reduce_expr(expr);
                //         };
                //     };
                // };
            }

            _ => {}
        };
        self.reduce_expr(expr)
    }

    pub fn reduce_expr(&mut self, expr: &ExprValue) -> Option<ExprValue> {
        if expr.is_literal() { return Some(expr.clone()); }
        match expr {
            &ExprValue::Expr(ref op, box ref l_expr, box ref r_expr) => {
                let l_reduced = self.reduce_expr(&l_expr);
                let r_reduced = self.reduce_expr(&r_expr);
                let had_reduction = l_reduced.is_some() || r_reduced.is_some();

                let l_reduced = l_reduced.unwrap_or_else(|| l_expr.clone());
                let r_reduced = r_reduced.unwrap_or_else(|| r_expr.clone());

                let l_string = match &l_reduced { &ExprValue::LiteralString(..) => true, _ => false };
                let r_string = match &r_reduced { &ExprValue::LiteralString(..) => true, _ => false };

                match op {
                    &ExprOp::Add if (l_string || r_string) => {
                        return Some(ExprValue::Apply(
                            ExprApplyOp::JoinString(None),
                            Some(vec![
                                Box::new(l_reduced),
                                Box::new(r_reduced)
                            ])
                            // let l_string = self.reduce_expr_to_string(&l_reduced);
                            // let r_string = self.reduce_expr_to_string(&r_reduced);
                            // Some(ExprValue::LiteralString(format!("{}{}", l_string, r_string)))
                        ))
                    }
                    _ => {}
                };

                match (op, &l_reduced, &r_reduced) {
                    // (&ExprOp::Add, _, _) if (l_string || r_string) => {
                    //     Some(ExprValue::Apply(
                    //         ExprApplyOp::JoinString(None),
                    //         Some(vec![
                    //             Box::new(l_reduced.clone()),
                    //             Box::new(r_reduced.clone())
                    //         ])
                    //     ))
                    //     // let l_string = self.reduce_expr_to_string(&l_reduced);
                    //     // let r_string = self.reduce_expr_to_string(&r_reduced);
                    //     // Some(ExprValue::LiteralString(format!("{}{}", l_string, r_string)))
                    // }

                    (&ExprOp::Add, &ExprValue::LiteralNumber(ref l_num), &ExprValue::LiteralNumber(ref r_num)) => {
                        return Some(ExprValue::LiteralNumber(l_num + r_num))
                    }

                    _ => {}
                };

                if had_reduction {
                    // Return the partially reduced expression
                    return Some(
                        ExprValue::Expr(op.to_owned(), Box::new(l_reduced), Box::new(r_reduced))
                    );
                }

                None
            }

            &ExprValue::LiteralObject(ref props) => {
                let props = props.as_ref().map(|props| self.map_props(props.iter()));
                Some(ExprValue::LiteralObject(props))
            }

            // TODO: Fix this in the new regime
            &ExprValue::DefaultVariableReference => {
                let key = "value";
                if let Some(sym) = self.resolve_sym(key) {
                    return Some(ExprValue::SymbolReference(sym));
                }
                None
            }

            &ExprValue::SymbolReference(ref sym) => {
                match sym.sym_ref() {
                    &SymbolReferenceType::UnresolvedReference(ref key) => {
                        if let Some(sym) = self.resolve_sym(key) {
                            return Some(ExprValue::SymbolReference(sym));
                        }
                        None
                    }

                    &SymbolReferenceType::UnresolvedPathReference(ref path) => {
                        if let Some(sym) = self.resolve_path_reference_to_symbol(path) {
                            return Some(ExprValue::SymbolReference(sym));
                        };
                        None
                        // let mut splitter = path.split(".").map(|s| s.to_owned());
                        // let first = splitter.next();
                        // if let Some(first) = first.and_then(|ref s| self.resolve_sym(s)) {
                        //     let rest: Vec<_> = splitter.collect();
                        //     let len = rest.len();
                        //     if len > 0 {
                        //         return Some(ExprValue::SymbolReference(Symbol::member_path_from(first, rest)));
                        //     };
                        //     return Some(ExprValue::SymbolReference(first));
                        // };
                        // None
                    }

                    _ => None
                }
                // Some(ExprValue::LiteralString(format!("{:?}", resolved_sym)))
            }

            _ => None
        }
    }

    pub fn map_props<'a, I: IntoIterator<Item = &'a Prop>>(&mut self, props: I) -> Vec<Prop> {
        props.into_iter().map(|prop| {
            if let Some(expr) = prop.1.as_ref().and_then(|expr| self.reduce_expr(expr)) { return (prop.0.to_owned(), Some(expr)); }
            prop.to_owned()
        }).collect()
    }

    pub fn map_action_ops<'a, I: IntoIterator<Item = &'a ActionOpNode>>(&mut self, action_ops: I) -> Vec<ActionOpNode> {
        action_ops.into_iter().map(|action_op| match action_op {
            &ActionOpNode::DispatchAction(ref action_ty, ref props) => {
                ActionOpNode::DispatchAction(action_ty.to_owned(), props.as_ref().map(|v| self.map_props(v.iter())))
            }
        }).collect()
    }

    pub fn map_event_handler_symbols(&mut self, event_handler: &EventHandler) -> EventHandler {
        match event_handler {
            &EventHandler::Event(ref event_name, ref params, ref action_ops) => {
                let action_ops = action_ops.as_ref().map(|action_ops| self.map_action_ops(action_ops.into_iter()));
                EventHandler::Event(event_name.to_owned(), params.to_owned(), action_ops)
            }
            
            &EventHandler::DefaultEvent(ref params, ref action_ops) => {
                let action_ops = action_ops.as_ref().map(|action_ops| self.map_action_ops(action_ops.into_iter()));
                EventHandler::DefaultEvent(params.to_owned(), action_ops)
            }
        }
    }

    pub fn reduce_expr_or_return_same(&mut self, expr: &ExprValue) -> ExprValue {
        self.reduce_expr(expr).unwrap_or(expr.clone())
    }

    #[allow(dead_code)]
    pub fn join_path_as_expr(&mut self, s: Option<&str>) -> ExprValue {
        self.scope().join_path_as_expr(s)
    }

    pub fn join_path_as_expr_with(&mut self, sep: Option<&str>, last: &str) -> ExprValue {
        self.scope().join_path_as_expr_with(sep, last)
    }

    pub fn join_path(&mut self, s: Option<&str>) -> String {
        self.scope().join_path(self, s)
    }

    pub fn join_path_with(&mut self, s: Option<&str>, last: &str) -> String {
        let key = self.scope().join_path(self, s);
        if key.len() > 0 { format!("{}.{}", key, last) } else { last.to_owned() }
    }

    #[allow(dead_code)]
    pub fn join_action_path_as_expr(&mut self, s: Option<&str>) -> ExprValue {
        self.scope().join_action_path_as_expr(s)
    }

    pub fn join_action_path(&mut self, s: Option<&str>) -> String {
        self.scope().join_action_path(self, s)
    }

    pub fn join_action_path_with(&mut self, sep: Option<&str>, last: &str) -> String {
        let key = self.scope().join_action_path(self, sep);
        if key.len() > 0 { format!("{}.{}", key, last) } else { last.to_owned() }
    }

    #[allow(dead_code)]
    pub fn add_invocation_prop(&mut self, key: &str, expr: Option<&ExprValue>) {
        let invocation_prop = Symbol::invocation_prop(key, expr);
        self.add_sym(key, invocation_prop);
    }

    pub fn add_action_param(&mut self, key: &str) {
        let binding = BindingType::ActionParamBinding(key.to_owned());
        self.add_sym(key, Symbol::binding(&binding));
    }
}


#[cfg(test)]
mod tests {
    use std::iter::*;
    use parser::*;
    use scope::*;
    use scope::symbols::*;
    use processing::*;


    // Expressions

    #[test]
    pub fn test_expr_two_numbers() {
        let expr1 = ExprValue::LiteralNumber(1);
        let expr2 = ExprValue::LiteralNumber(2);
        let expr = ExprValue::Expr(ExprOp::Add, Box::new(expr1), Box::new(expr2));

        let mut ctx = Context::default();
        
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
            // ctx.add_param_ref_to("abc", "xyz3");
        }

        // Lm.No
        {
            ctx.push_child_scope();
            ctx.append_path_str("No");
            // ctx.add_sym("abc", ctx.prop("xyz2"));
            // ctx.add_sym("def", ctx.prop("def2"));
            // ctx.add_param_ref_to("abc", "xyz2");
            // ctx.add_param_ref_to("def", "def2");
        }

        // Lm.No.Pq
        {
            ctx.push_child_scope();
            ctx.append_path_str("Pq");
            // ctx.add_sym("abc", ctx.prop("xyz3"));
            // ctx.add_param_ref_to("abc", "xyz3");
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


    #[test]
    pub fn test_context_reducers_and_actions() {
        let mut ctx = Context::default();

        // Define a new reducer `TODOS`
        {
            ctx.push_child_scope();
            ctx.append_action_path_str("TODOS");
        }

        // Define an action within this reducer
        {
            ctx.push_child_scope();
        }

        // Make the current state available using the reducer key, and as `state`, and `value`.
        {
            let binding = BindingType::ActionStateBinding;
            let sym = Symbol::binding(&binding);
            ctx.add_sym("todos", sym.clone());
            ctx.add_sym("state", sym.clone());
            ctx.add_sym("value", sym.clone());
        }

        // Define an action param `entry` within this action
        let action_scope_id = ctx.scope().id().to_owned();
        {
            ctx.add_action_param("entry")
        }

        // Action ADD
        {
            ctx.push_child_scope();

            // Reference an action param `entry` within this action
            assert_eq!(ctx.resolve_sym("entry"),
                // Some(Symbol::ref_prop_in_scope("todo", "todo", Some(&lm_element_scope_id)))
                Some(Symbol::binding(&BindingType::ActionParamBinding("entry".into())))
                // Some(Symbol::unbound_action_param("message", Some(&action_scope_id)))
            );

            // Reference the current state as local (state)
            assert_eq!(ctx.resolve_sym("state"),
                Some(Symbol::binding(&BindingType::ActionStateBinding))
                // Some(Symbol::reducer_key("TODOS"))
            );

            // Reference the current state as local (todos)
            assert_eq!(ctx.resolve_sym("todos"),
                Some(Symbol::binding(&BindingType::ActionStateBinding))
                // Some(Symbol::reducer_key("TODOS"))
            );

            // Reference the current state as local (state)
            assert_eq!(ctx.resolve_sym("value"),
                Some(Symbol::binding(&BindingType::ActionStateBinding))
                // Some(Symbol::reducer_key("TODOS"))
            );

        }
    }

    fn create_document_with_data<'a>(template: &'a Template, reducer_key: &str, default_expr: ExprValue) -> Document {
        let mut ctx = Context::default();
        let mut bindings = BindingContext::default();
        // let mut processing = ProcessDocument::from_template(&template);
        let mut state = DocumentProcessingState::default();
        state.reducer_key_data = Default::default();

        let mut reducer = ReducerKeyData::default();
        reducer.default_expr = Some(default_expr);

        state.reducer_key_data.insert(reducer_key.to_owned(), reducer);
        // state.reducer_key_data.default_expr = Some(default_expr);
        state.default_reducer_key = Some(reducer_key.to_owned());
        // assert!(processing.process_document(&mut ctx, &mut bindings).is_ok());
        state.into()
    }

    #[test]
    pub fn test_context_component_props() {
        let mut ctx = Context::default();
        let template = Template::new(vec![]);
        let doc = create_document_with_data(&template, "TODOS".into(), ExprValue::LiteralArray(Some(vec![ExprValue::LiteralString("One".into())])));

        // Define a new reducer path binding (todos)
        {
            let binding = BindingType::ReducerPathBinding("TODOS".into());
            ctx.add_sym("todos", Symbol::binding(&binding));
        }

        // Define a forlens (not supported in Context yet)
        {
            ctx.push_child_scope();
        }

        // Define a ComponentPropBinding
        {
            ctx.push_child_scope();
            let binding = BindingType::ComponentPropBinding("todos".into());
            ctx.add_sym("todo", Symbol::binding(&binding));
        }

        let scope_id = ctx.scope_ref().unwrap().id().to_owned();

        // Add reducer path binding
        {
            ctx.push_child_scope();
            assert_eq!(
                ctx.resolve_key_from_scope("todo", &scope_id),
                Some(Symbol::binding(&BindingType::ReducerPathBinding("TODOS".into())))
            );
        }

        // Assert default expression
        {
            assert!(doc.reducer_key_data.contains_key("TODOS"));
            assert_eq!(doc.reducer_key_data.get("TODOS").unwrap().default_expr, Some(ExprValue::LiteralArray(Some(vec![ExprValue::LiteralString("One".into())]))));
        }

        // Evaluate reducer path binding
        {
            let sym  = Symbol::binding(&BindingType::ReducerPathBinding("TODOS".into()));
            assert_eq!(ctx.eval_sym(&doc, &sym), Some(ExprValue::LiteralArray(Some(vec![ExprValue::LiteralString("One".into())]))));
        }

        // Resolve reducer path symbol
        {
            ctx.push_child_scope();
            assert_eq!(
                ctx.resolve_key_from_scope("todo", &scope_id),
                Some(Symbol::binding(&BindingType::ReducerPathBinding("TODOS".into())))
            );
        }
    }

    // type FormalProp<'a> = (&'a str);
    // type FormalPropVec<'a> = Vec<FormalProp<'a>>;

    // type PropKeyRef = (String, String);
    // type PropKeyRefVec = Vec<PropKeyRef>;

    // type PropValue<'a> = (&'a str, Option<&'a ExprValue>);
    // type PropValueVec<'a> = Vec<PropValue<'a>>;

    // impl TestOutput2 {
    //     pub fn push_component_instance_invocation_scope<'a, I>(&mut self, ctx: &mut Context, _component_ty: &str, props: I)
    //       where I: IntoIterator<Item = &'a PropValue<'a>>
    //     {
    //         // let parent_scope_id = ctx.scope().id().to_owned();
    //         ctx.push_child_scope();
    //         for prop in props {
    //             ctx.add_invocation_prop(&prop.0, prop.1);
    //         }
    //     }

    //     pub fn push_component_instance_scope(&mut self, ctx: &mut Context, instance_id: &str, _component_ty: &str) {
    //         ctx.push_child_scope();
    //         ctx.append_path_str(instance_id);
    //     }

    //     pub fn push_element_parameter_definition_scope<'a, I>(&mut self, ctx: &mut Context, element_id: &str, _element_ty: &str, props: I)
    //       where I: IntoIterator<Item = &'a PropKeyRef>
    //     {
    //         let parent_scope_id = ctx.scope().id().to_owned();
    //         ctx.push_child_scope();
    //         ctx.append_path_str(element_id);
    //         for prop in props {
    //             ctx.add_element_prop_ref(&prop.0, &prop.1, Some(&parent_scope_id));
    //         }
    //     }
    // }

    // #[test]
    // pub fn test_context_scope_component_nesting1() {
    //     let mut ctx = Context::default();
    //     let mut output = TestOutput2::default();

    //     // Lm
    //     // Element: Lm()
    //     // Invokes: CompNo(todo = store.getState().todo)
    //     {
    //         ctx.push_child_scope();
    //         ctx.append_path_str("Lm");

    //         // Our element path should be (Lm)
    //         let expr = ctx.join_path_as_expr(Some("."));
    //         assert_eq!(expr, ExprValue::Apply(ExprApplyOp::JoinString(Some(".".to_owned())), Some(vec![
    //             Box::new(ExprValue::LiteralString("Lm".to_owned()))
    //         ])));
    //     }
    //     // let lm_element_scope_id = ctx.scope().id().to_owned();

    //     // Lm
    //     // Comp1 definition (loaded)
    //     // {
    //     //     output.push_component_definition_param_bindings_scope(&mut ctx);
    //     // }

    //     // Lm
    //     // Invoke: CompNo(todo = store.getState().todo)
    //     {
    //         let todo_value = ExprValue::SymbolReference(Symbol::reducer_key("todo"));
    //         let props: PropValueVec = vec![
    //             ("todo".into(), Some(&todo_value))
    //         ];
    //         output.push_component_instance_invocation_scope(&mut ctx, "Component1", props.iter());

    //         // Our element path should still be the same (Lm)
    //         let expr = ctx.join_path_as_expr(Some("."));
    //         assert_eq!(expr, ExprValue::Apply(ExprApplyOp::JoinString(Some(".".to_owned())), Some(vec![
    //             Box::new(ExprValue::LiteralString("Lm".to_owned()))
    //         ])));
    //     }

    //     // Lm.Comp1
    //     // Component contents will be output
    //     {
    //         // let parent_scope_id = ctx.scope().id().to_owned();
    //         output.push_component_instance_scope(&mut ctx, "Comp1", "Component1");

    //         // The joined path (dynamic) should be a string join operation
    //         let expr = ctx.join_path_as_expr(Some("."));
    //         assert_eq!(expr, ExprValue::Apply(ExprApplyOp::JoinString(Some(".".to_owned())), Some(vec![
    //             Box::new(ExprValue::LiteralString("Lm".to_owned())),
    //             Box::new(ExprValue::LiteralString("Comp1".to_owned())),
    //         ])));

    //         // The local (todo) should resolve to a reducer key reference (todo)
    //         // assert_eq!(ctx.resolve_sym("todo"), Some(Symbol::param("todo", _)));
    //         assert_eq!(ctx.resolve_sym("todo"),
    //             // Some(Symbol::ref_prop_in_scope("todo", "todo", Some(&lm_element_scope_id)))
    //             Some(Symbol::invocation_prop("todo", Some(&ExprValue::SymbolReference(Symbol::reducer_key("todo")))))
    //         );
    //     }

    //     // Lm.Comp1.Pq
    //     // Element within component definition
    //     // Element parameter definition scope
    //     {
    //         let props: PropKeyRefVec = vec![
    //             ("value".into(), "todo".into())
    //         ];
    //         output.push_element_parameter_definition_scope(&mut ctx, "Pq", "input", props.iter());
    //     }

    //     // The joined path (dynamic) should be a string join operation
    //     let expr = ctx.join_path_as_expr(Some("."));
    //     assert_eq!(expr, ExprValue::Apply(ExprApplyOp::JoinString(Some(".".to_owned())), Some(vec![
    //         Box::new(ExprValue::LiteralString("Lm".to_owned())),
    //         Box::new(ExprValue::LiteralString("Comp1".to_owned())),
    //         Box::new(ExprValue::LiteralString("Pq".to_owned()))
    //     ])));

    //     // The local var (param) should resolve to a param
    //     // assert_eq!(ctx.resolve_sym("todo"), Some(Symbol::param("todo", _)));

    //     // We should resolve the symbol from the nearest scope where it is defined
    //     // assert_eq!(ctx.resolve_sym("abc"), Some(Symbol::prop("xyz3")));

    //     // We should resolve the symbol from the nearest scope where it is defined
    //     // assert_eq!(ctx.resolve_sym("def"), Some(Symbol::prop("def2")));
    // }
}