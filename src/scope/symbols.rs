#![allow(dead_code)]

use parser::ast::*;
use parser::util::allocate_element_key;
use scope::*;


#[derive(Debug, Clone)]
pub struct Symbols {
    map_id: String,
    parent_map_id: Option<String>,
    symbol_map: SymbolMap,
    binding_values: BindingOfTypeMap
}

impl Default for Symbols {
    fn default() -> Symbols {
        Symbols::new(None)
    }
}

impl Symbols {
    pub fn new(parent_map_id: Option<&str>) -> Symbols {
        Symbols {
            map_id: allocate_element_key(),
            parent_map_id: parent_map_id.map(|s| s.to_owned()),
            symbol_map: Default::default(),
            binding_values: Default::default()
        }
    }

    pub fn map_id(&self) -> &str {
        &self.map_id
    }

    pub fn parent_map_id(&self) -> Option<&str> {
        self.parent_map_id.as_ref().map(|s| s.as_str())
    }

    pub fn add_sym(&mut self, key: &str, sym: Symbol) {
        self.symbol_map.insert(key.to_owned(), SymbolValueEntry::Symbol(sym));
    }

    pub fn add_value(&mut self, key: &str, expr: ExprValue) {
        self.symbol_map.insert(key.to_owned(), SymbolValueEntry::Value(expr));
    }

    pub fn add_binding_value(&mut self, binding: &BindingType, expr: ExprValue) {
        self.binding_values.insert(binding.to_owned(), expr);
    }

    pub fn get_value_entry(&self, key: &str) -> Option<&SymbolValueEntry> {
        self.symbol_map.get(key)
    }

    pub fn get_sym(&self, key: &str) -> Option<&Symbol> {
        self.symbol_map.get(key).and_then(|r| match r { &SymbolValueEntry::Symbol(ref s) => Some(s), _ => None })
    }

    pub fn get_value(&self, key: &str) -> Option<&ExprValue> {
        self.symbol_map.get(key).and_then(|r| match r { &SymbolValueEntry::Value(ref s) => Some(s), _ => None })
    }

    pub fn get_binding_value(&self, binding: &BindingType) -> Option<&ExprValue> {
        self.binding_values.get(binding)
    }
}

// #[cfg(test)]
// mod tests {
//     use parser::ast::*;
//     use scope::symbols::*;

//     #[test]
//     pub fn test_symbols() {
//         let mut symbols = Symbols::new(None);
//         symbols.add_sym("abc", Symbol::prop("xyz", "s1"));
//         // assert!(match symbols.get_sym("abc").map(|s| s.sym_ref()) { Some(&SymbolReferenceType::ResolvedReference("abc".to_owned(), ResolvedSymbolType::PropReference("abc".to_owned()), _)) => true, _ => false);

//         assert_eq!(symbols.get_sym("abc").map(|s| s.sym_ref()), Some(&SymbolReferenceType::ResolvedReference("xyz".to_owned(), ResolvedSymbolType::PropReference("xyz".to_owned()), Some("s1".to_owned()))));
//     }
// }