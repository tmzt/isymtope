#![allow(dead_code)]

use linked_hash_map::LinkedHashMap;

use parser::ast::*;


#[derive(Debug, Default)]
pub struct BindingContext {
    binding_map: BindingMap
}

impl BindingContext {
    pub fn add_expr(&mut self, key: &str, expr: &ExprValue) {
        let binding = BindingType::ExpressionBinding(Box::new(expr.to_owned()));
        self.binding_map.insert(key.to_owned(), binding);
    }

    pub fn add_key_in_symbols(&mut self, key: &str, ref_key: &str, map_id: &str) {
        let binding = BindingType::KeyInSymbolsBinding(ref_key.to_owned(), map_id.to_owned());
        self.binding_map.insert(key.to_owned(), binding);
    }

    pub fn add_reducer_key(&mut self, key: &str, reducer_key: &str) {
        let binding = BindingType::ReducerPathBinding(reducer_key.to_owned(), None);
        self.binding_map.insert(key.to_owned(), binding);
    }

    pub fn add_reducer_key_and_path<'a, I: IntoIterator<Item = &'a str>>(&mut self, key: &str, reducer_key: &str, reducer_path: I) {
        let reducer_path = Some(reducer_path.into_iter().map(|s| s.to_owned()).collect());
        let binding = BindingType::ReducerPathBinding(reducer_key.to_owned(), reducer_path);
        self.binding_map.insert(key.to_owned(), binding);
    }

    pub fn resolve_binding(&self, key: &str) -> Option<&BindingType> {
        self.binding_map.get(key)
    }

    pub fn resolve_sym(&self, sym: &Symbol) -> Option<&BindingType> {
        match sym.sym_ref() {
            &SymbolReferenceType::ResolvedReference(ref sym_key, ResolvedSymbolType::ReferenceToKeyInScope(ref key_ref, Some(ref scope_id)), _) => {
                match key_ref {
                    &KeyReferenceType::UnboundFormalParam => {
                        if let Some(binding) = self.resolve_binding(sym_key) {
                            return Some(binding);
                        }
                    }

                    _ => {}
                };
            }

            _ => {}
        };

        None
    }
}

#[derive(Debug)]
pub struct SymbolBindingsResolver<'a, I: Iterator<Item = &'a Symbol>> {
    bindings: &'a mut BindingContext,
    iter: I
}

impl<'a, I: Iterator<Item = &'a Symbol>> SymbolBindingsResolver<'a, I>
{
    pub fn new(bindings: &'a mut BindingContext, iter: I) -> Self {
        SymbolBindingsResolver { bindings: bindings, iter: iter }
    }
}

impl<'a, I: Iterator<Item = &'a Symbol>> Iterator for SymbolBindingsResolver<'a, I>
{
    type Item = BindingType;

    fn next(&mut self) -> Option<Self::Item> {
        if let Some(sym) = self.iter.next() {
            if let Some(resolved_binding) = self.bindings.resolve_sym(sym) {
                return Some(resolved_binding.to_owned());
            }; 
        };
        None
    }
}


#[cfg(test)]
mod tests {
    use super::*;
    use parser::ast::*;


    #[test]
    pub fn test_bindings_binding_context_1() {
        let mut bindings = BindingContext::default();
        bindings.add_key_in_symbols("todo", "todo", "s1");

        assert_eq!(bindings.resolve_binding("todo"), Some(&BindingType::KeyInSymbolsBinding("todo".into(), "s1".into())));
    }

    #[test]
    pub fn test_bindings_binding_context_resolve_sym1() {
        let mut bindings = BindingContext::default();
        bindings.add_reducer_key("todo", "todo");

        assert_eq!(bindings.resolve_binding("todo"), Some(&BindingType::ReducerPathBinding("todo".into(), None)));
        let sym = Symbol::unbound_formal_param("todo", Some("s1"));

        assert_eq!(bindings.resolve_sym(&sym), Some(&BindingType::ReducerPathBinding("todo".into(), None)));
    }

    #[test]
    pub fn test_bindings_binding_context_resolve_symbols1() {
        let mut bindings = BindingContext::default();
        bindings.add_reducer_key("todo", "todo");

        let symbols = vec![
            Symbol::unbound_formal_param("todo", Some("s1"))
        ];

        let binding_iter = SymbolBindingsResolver::new(&mut bindings, symbols.iter());
        assert_eq!(binding_iter.collect::<Vec<BindingType>>(), vec![
            BindingType::ReducerPathBinding("todo".into(), None)
        ]);

        // assert_eq!(bindings.resolve_sym(&sym), Some(&BindingType::ReducerPathBinding("todo".into(), None)));
    }
}