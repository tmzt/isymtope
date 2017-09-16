#![allow(dead_code)]

use parser::ast::*;
use scope::context::*;


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
        let binding = BindingType::ReducerPathBinding(reducer_key.to_owned());
        self.binding_map.insert(key.to_owned(), binding);
    }

    pub fn resolve_binding(&self, key: &str) -> Option<&BindingType> {
        self.binding_map.get(key)
    }

    pub fn resolve_sym(&self, sym: &Symbol) -> Option<&BindingType> {
        match sym.sym_ref() {
            &SymbolReferenceType::ResolvedReference(ref sym_key, ResolvedSymbolType::ReferenceToKeyInScope(ref key_ref, Some(ref _scope_id)), _) => {
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
pub struct SymbolBindingResolver<'a, I: Iterator<Item = &'a Symbol>> {
    bindings: &'a mut BindingContext,
    iter: I
}

impl<'a, I: Iterator<Item = &'a Symbol>> SymbolBindingResolver<'a, I>
{
    pub fn new(bindings: &'a mut BindingContext, iter: I) -> Self {
        SymbolBindingResolver { bindings: bindings, iter: iter }
    }
}

impl<'a, I: Iterator<Item = &'a Symbol>> Iterator for SymbolBindingResolver<'a, I>
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

#[derive(Debug)]
pub struct SymbolBindingPropResolver<'a, I: Iterator<Item = PropValue<'a>>> {
    ctx: &'a mut Context,
    bindings: &'a BindingContext,
    iter: I
}

impl<'a, I: Iterator<Item = PropValue<'a>>> SymbolBindingPropResolver<'a, I>
{
    pub fn new(ctx: &'a mut Context, bindings: &'a BindingContext, iter: I) -> Self {
        SymbolBindingPropResolver { ctx: ctx, bindings: bindings, iter: iter }
    }
}

impl<'a, I: Iterator<Item = PropValue<'a>>> Iterator for SymbolBindingPropResolver<'a, I>
{
    type Item = Prop;

    fn next(&mut self) -> Option<Self::Item> {
        if let Some(prop) = self.iter.next() {
            let (key, expr) = prop;
            if let Some(expr) = expr {
                if let &ExprValue::SymbolReference(ref sym) = expr {
                    match sym.sym_ref() {
                        &SymbolReferenceType::ResolvedReference(ref sym_key, ResolvedSymbolType::ReferenceToKeyInScope(KeyReferenceType::UnboundFormalParam, _), _) => {
                            if let Some(resolved_binding) = self.bindings.resolve_binding(sym_key) {
                                let res = ExprValue::Binding(resolved_binding.to_owned());
                                return Some((key.to_owned(), Some(res)));
                            };

                            // if let Some(ref resolved_sym) = self.ctx.resolve_sym(sym_key) {
                            //     if let Some(resolved_binding) = self.bindings.resolve_sym(resolved_sym) {
                            //         let res = ExprValue::Binding(resolved_binding.to_owned());
                            //         return Some((key.to_owned(), Some(res)));
                            //     };
                            // };
                        },
                        _ => { }
                    };
                };
            };
            return Some((key.to_owned(), expr.map(|p| p.clone())));
        };
        None
    }
}


// #[cfg(test)]
// mod tests {
//     use super::*;
//     use parser::ast::*;


//     #[test]
//     pub fn test_bindings_binding_context_1() {
//         let mut bindings = BindingContext::default();
//         bindings.add_key_in_symbols("todo", "todo", "s1");

//         assert_eq!(bindings.resolve_binding("todo"), Some(&BindingType::KeyInSymbolsBinding("todo".into(), "s1".into())));
//     }

//     #[test]
//     pub fn test_bindings_binding_context_resolve_sym1() {
//         let mut bindings = BindingContext::default();
//         bindings.add_reducer_key("todo", "todo");

//         assert_eq!(bindings.resolve_binding("todo"), Some(&BindingType::ReducerPathBinding("todo".into())));
//         let sym = Symbol::unbound_formal_param("todo", Some("s1"));

//         assert_eq!(bindings.resolve_sym(&sym), Some(&BindingType::ReducerPathBinding("todo".into())));
//     }

//     #[test]
//     pub fn test_bindings_binding_context_resolve_symbols1() {
//         let mut bindings = BindingContext::default();
//         bindings.add_reducer_key("todo", "todo");

//         let symbols = vec![
//             Symbol::unbound_formal_param("todo", Some("s1"))
//         ];

//         let binding_iter = SymbolBindingResolver::new(&mut bindings, symbols.iter());
//         assert_eq!(binding_iter.collect::<Vec<BindingType>>(), vec![
//             BindingType::ReducerPathBinding("todo".into())
//         ]);

//         // assert_eq!(bindings.resolve_sym(&sym), Some(&BindingType::ReducerPathBinding("todo".into(), None)));
//     }
// }