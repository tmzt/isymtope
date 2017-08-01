
use std::clone::Clone;

use linked_hash_map::LinkedHashMap;

use processing::structs::*;
use parser::ast::*;


// #[derive(Debug, Clone)]
// pub enum SymbolValueType {
//     Empty,
//     UnresolvedReference(SymbolReferenceType),
//     ConstantValue(ExprValue)
// }
// pub type SymbolVal = (SymbolValueType, Option<VarType>);
// pub type SymbolValMap = LinkedHashMap<String, SymbolVal>;

#[derive(Debug, Clone, Default)]
pub struct DocumentProcessingScope {
    pub props: SymbolMap,
    pub symbol_map: SymbolMap,
    pub reducer_key_cache: SymbolMap
}

// #[derive(Debug, Clone)]
// impl Default for ElementOpScope { fn default() -> Self { ElementOpScope(Default::default(), Default::default(), None) } }

impl DocumentProcessingScope {
    pub fn with_prop(&mut self, prop_name: &str, ty: Option<&VarType>, value: Option<&ExprValue>) -> &mut Self {
        let mut sym = SymbolReferenceType::PropReference(prop_name.to_owned());
        self.props.insert(prop_name.to_owned(), Symbol(sym, ty.map(Clone::clone), value.map(|value| Box::new(value.clone()))));
        self
    }

    pub fn add_prop_with_value(&mut self, prop_name: &str, value: &ExprValue) -> &mut Self {
        self.symbol_map.insert(prop_name.to_owned(), Symbol::prop_with_value(prop_name, value));
        self
    }

    pub fn add_loop_var(&mut self, var_name: &str) -> &mut Self {
        self.symbol_map.insert(var_name.to_owned(), Symbol::loop_var(var_name));
        self
    }

    pub fn add_loop_var_with_value(&mut self, var_name: &str, value: &ExprValue) -> &mut Self {
        self.symbol_map.insert(var_name.to_owned(), Symbol::loop_var_with_value(var_name, value));
        self
    }

    pub fn with_symbol(&mut self, var_name: &str, sym: &SymbolReferenceType, ty: Option<&VarType>, value: Option<&ExprValue>) -> &mut Self {
        self.symbol_map.insert(var_name.to_owned(), Symbol(sym.clone(), ty.map(Clone::clone), value.map(|value| Box::new(value.clone()))));
        self
    }

    pub fn with_cached_reducer_key(&mut self, reducer_key: &str) -> &mut Self {
        self.reducer_key_cache.insert(reducer_key.to_owned(), Symbol::reducer_key(reducer_key));
        self
    }

    pub fn add_cached_reducer_key_with_value(&mut self, reducer_key: &str, value: &ExprValue) -> &mut Self {
        self.reducer_key_cache.insert(reducer_key.to_owned(), Symbol::reducer_key_with_value(reducer_key, value));
        self
    }

}