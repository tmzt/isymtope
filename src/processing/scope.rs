
use std::clone::Clone;

use linked_hash_map::LinkedHashMap;

use processing::structs::*;
use parser::ast::*;


#[derive(Debug, Clone, Default)]
pub struct DocumentProcessingScope {
    pub props: SymbolMap,
    pub symbol_map: SymbolMap,
    pub reducer_key_cache: SymbolMap,
    pub block_params: SymbolMap,
    pub params: SymbolMap,
    pub element_value_bindings: SymbolMap,
    pub action_params: SymbolMap,
    pub lens_params: SymbolMap,
}

#[derive(Debug, Clone, Default)]
pub struct ScopeValues {
    prop_values: ValueMap,
    lens_values: ValueMap,
    reducer_cache: ValueMap,
}

impl ScopeValues {
    pub fn add_prop_with_value(&mut self, prop_name: &str, value: &ExprValue) -> &mut Self {
        self.prop_values.insert(prop_name.to_owned(), value.clone());
        self
    }

    pub fn get_prop(&self, key: &str) -> Option<&ExprValue> {
        self.prop_values.get(key)
    }

    pub fn add_lens_param(&mut self, key: &str, value: &ExprValue) -> &mut Self {
        self.lens_values.insert(key.to_owned(), value.clone());
        self
    }

    pub fn get_lens_param(&self, key: &str) -> Option<&ExprValue> {
        self.lens_values.get(key)
    }

    pub fn add_cached_reducer(&mut self, reducer_key: &str, value: &ExprValue) -> &mut Self {
        self.reducer_cache.insert(reducer_key.to_owned(), value.clone());
        self
    }

    pub fn get_reducer_default(&self, key: &str) -> Option<&ExprValue> {
        self.reducer_cache.get(key)
    }
}

impl DocumentProcessingScope {
    pub fn add_param(&mut self, key: &str) -> &mut Self {
        self.params.insert(key.to_owned(), Symbol::param(key));
        self
    }

    pub fn add_action_param(&mut self, key: &str) -> &mut Self {
        self.action_params.insert(key.to_owned(), Symbol::action_param(key));
        self
    }

    pub fn add_prop_with_value(&mut self, prop_name: &str, value: &ExprValue) -> &mut Self {
        self.props.insert(prop_name.to_owned(),
                          Symbol::prop_with_value(prop_name, value));
        self
    }

    pub fn add_loop_var(&mut self, var_name: &str) -> &mut Self {
        self.block_params.insert(var_name.to_owned(), Symbol::loop_var(var_name));
        self
    }

    pub fn add_loop_var_with_value(&mut self, var_name: &str, value: &ExprValue) -> &mut Self {
        self.block_params.insert(var_name.to_owned(),
                                 Symbol::loop_var_with_value(var_name, value));
        self
    }

    pub fn add_for_lens_element_key(&mut self, key: &str) -> &mut Self {
        self.block_params.insert(key.to_owned(), Symbol::for_lens_element_key(key));
        self
    }

    pub fn add_element_value_binding(&mut self, key: &str, element_key: &str) -> &mut Self {
        self.element_value_bindings.insert(key.to_owned(),
                                           Symbol::element_value_binding(key, element_key));
        self
    }

    pub fn with_cached_reducer_key(&mut self, reducer_key: &str) -> &mut Self {
        self.reducer_key_cache.insert(reducer_key.to_owned(), Symbol::reducer_key(reducer_key));
        self
    }

    pub fn add_cached_reducer_key_with_value(&mut self,
                                             reducer_key: &str,
                                             value: &ExprValue)
                                             -> &mut Self {
        self.reducer_key_cache.insert(reducer_key.to_owned(),
                                      Symbol::reducer_key_with_value(reducer_key, value));
        self
    }
}

#[derive(Debug, Clone, Default)]
pub struct ElementOpScope(pub ScopePrefixes, pub DocumentProcessingScope, pub ScopeValues);

impl ElementOpScope {
    pub fn add_loop_var_with_value(&mut self, var_name: &str, value: &ExprValue) -> &mut Self {
        self.1.add_loop_var_with_value(var_name, value);
        self
    }
}

#[derive(Debug, Clone)]
pub enum ElementKeyPrefixType {
    ScopeElementKeyPrefix(String),
}
pub type KeyPrefix = Option<ElementKeyPrefixType>;

#[derive(Debug, Clone)]
pub enum ActionPrefixType {
    ScopeActionPrefix(String),
}
pub type ActionPrefix = Option<ActionPrefixType>;

#[derive(Debug, Clone)]
pub enum VarPrefixType {
    ScopeVarPrefix(String),
}
pub type VarPrefix = Option<VarPrefixType>;

pub type VarDefault = Option<String>;

pub type ExprPrefix = Option<ExprValue>;

pub type ElementIndex = Option<i32>;

pub type BaseElementKeyPrefix = Option<String>;

#[derive(Debug, Clone, Default)]
pub struct ScopePrefixes(KeyPrefix,
                         ActionPrefix,
                         VarPrefix,
                         VarDefault,
                         ExprPrefix,
                         ElementIndex,
                         BaseElementKeyPrefix);

impl ScopePrefixes {
    pub fn set_index(&mut self, idx: i32) -> &mut Self {
        self.5 = Some(idx);
        self
    }

    pub fn clear_index(&mut self) -> &mut Self {
        self.5 = None;
        self
    }

    pub fn set_prefix_expr(&mut self, expr: &ExprValue) -> &mut Self {
        self.4 = Some(expr.to_owned());
        self
    }

    pub fn clear_key(&mut self) -> &mut Self {
        self.5 = None;
        self
    }

    pub fn replace_key(&mut self, key: &str) -> &mut Self {
        self.0 = Some(ElementKeyPrefixType::ScopeElementKeyPrefix(key.to_owned()));
        self
    }

    pub fn append_key(&mut self, key: &str) -> &mut Self {
        let complete_key = self.make_complete_element_key_with(key);
        self.0 = Some(ElementKeyPrefixType::ScopeElementKeyPrefix(complete_key));
        self.5 = None; // Previous index becomes part of key
        self
    }

    pub fn append_action_scope(&mut self, key: &str) -> &mut Self {
        let action_scope = self.make_action_type(key);
        self.1 = Some(ActionPrefixType::ScopeActionPrefix(action_scope));
        self
    }

    pub fn set_default_var(&mut self, var_name: &str) -> &mut Self {
        self.3 = Some(var_name.to_owned());
        self
    }

    pub fn set_base_key(&mut self, base_key: &str) -> &mut Self {
        self.6 = Some(base_key.to_owned());
        self
    }

    pub fn append_base_key(&mut self, base_key: &str) -> &mut Self {
        let mut previous_base_key = self.6.as_ref().map_or("", |s| &s).to_owned();
        previous_base_key.push_str(base_key);
        self.6 = Some(previous_base_key);
        self
    }

    pub fn complete_element_key(&self) -> String {
        let mut key = self.6.as_ref().map_or("", |s| &s).to_owned();
        if let Some(ElementKeyPrefixType::ScopeElementKeyPrefix(ref prefix)) = self.0 {
            key = join_keys(&key, prefix);
        };
        if let Some(ref idx) = self.5 {
            key.push_str(&format!(".{}", idx));
        };

        key
    }

    pub fn prefix_expr(&self) -> Option<&ExprValue> {
        self.4.as_ref()
    }

    pub fn make_complete_element_key_with(&self, suffix: &str) -> String {
        let complete_key = self.complete_element_key();
        join_keys(&complete_key, suffix)
    }

    pub fn make_prefix_expr(&self,
                            value: &ExprValue,
                            idx_sym: Option<&Symbol>)
                            -> Option<ExprValue> {
        self.4.as_ref().map(|s| {
            let mut expr = ExprValue::Expr(ExprOp::Add,
                                           Box::new(s.to_owned()),
                                           Box::new(value.to_owned()));
            if let Some(idx_sym) = idx_sym {
                expr = ExprValue::Expr(ExprOp::Add,
                                       Box::new(expr),
                                       Box::new(ExprValue::SymbolReference(idx_sym.to_owned())))
            };
            expr
        })
    }

    pub fn default_var(&self) -> Option<String> {
        self.3.as_ref().map(|s| s.to_owned())
    }

    pub fn default_action_scope(&self) -> Option<String> {
        match self.1 {
            Some(ActionPrefixType::ScopeActionPrefix(ref prefix)) => Some(format!("{}", prefix)),
            _ => None,
        }
    }

    pub fn make_action_type(&self, key: &str) -> String {
        match self.1 {
            Some(ActionPrefixType::ScopeActionPrefix(ref prefix)) => {
                format!("{}.{}", prefix.to_uppercase(), key.to_uppercase())
            }
            _ => format!("{}", key.to_uppercase()),
        }
    }

    pub fn make_var_name(&self, key: &str) -> String {
        match self.2 {
            Some(VarPrefixType::ScopeVarPrefix(ref prefix)) => format!("{}.{}", prefix, key),
            _ => format!("{}", key),
        }
    }
}

#[inline]
#[allow(dead_code)]
fn join_optionals(a: Option<&str>, b: Option<&str>) -> String {
    let has_a = a.map_or(false, |a| a.len() > 0);
    let has_b = b.map_or(false, |b| b.len() > 0);
    if has_a && has_b {
        format!("{}.{}", a.unwrap_or(""), b.unwrap_or(""))
    } else if has_a {
        a.unwrap_or("").to_owned()
    } else {
        b.unwrap_or("").to_owned()
    }
}

#[inline]
fn join_keys(a: &str, b: &str) -> String {
    let has_a = a.len() > 0;
    let has_b = b.len() > 0;
    if has_a && has_b {
        format!("{}.{}", a, b)
    } else if has_a {
        a.to_owned()
    } else {
        b.to_owned()
    }
}

#[cfg(test)]
mod tests {
    use super::ScopePrefixes;

    #[test]
    pub fn test_scope_prefixes() {
        let mut prefixes = ScopePrefixes::default();
        prefixes.append_key(&"abc")
            .append_key(&"xyz");
        assert_eq!("abc.xyz", prefixes.complete_element_key());

        prefixes.set_index(3);
        assert_eq!("abc.xyz.3", prefixes.complete_element_key());

        prefixes.append_key(&"def");
        assert_eq!("abc.xyz.3.def", prefixes.complete_element_key());
    }

    #[test]
    pub fn test_base_key() {
        let mut prefixes = ScopePrefixes::default();
        prefixes.set_base_key(&"cbase");

        let element_key = "fcd3.aefd";

        let complete_key = prefixes.make_complete_element_key_with(element_key);
        assert_eq!("cbase.fcd3.aefd", complete_key);
    }

}
