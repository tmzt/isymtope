
use std::clone::Clone;

use linked_hash_map::LinkedHashMap;

use processing::structs::*;
use processing::scope::*;
use parser::ast::*;


// #[derive(Debug, Clone)]
// pub enum SymbolValueType {
//     Empty,
//     UnresolvedReference(SymbolReferenceType),
//     ConstantValue(ExprValue)
// }
// pub type SymbolVal = (SymbolValueType, Option<VarType>);
// pub type SymbolValMap = LinkedHashMap<String, SymbolVal>;

// #[derive(Debug, Clone, Default)]
// pub struct ExprEvalScope {
//     pub symbol_values: SymbolValMap
// }

// #[derive(Debug, Clone)]
// pub struct ElementOpScope(pub ScopePrefixes, pub ExprScopeProcessingState, pub Option<ExprEvalScope>);
// impl Default for ElementOpScope { fn default() -> Self { ElementOpScope(Default::default(), Default::default(), None) } }

// impl ElementOpScope {
//     pub fn with_var(self, var_name: &str, symbol: SymbolReferenceType, ty: Option<&VarType>, value: Option<SymbolValueType>) -> Self {
//         // let mut expr_scope = self.1.clone();
//         // expr_scope.symbol_map.insert(var_name.to_owned(), (Some(symbol), ty.map(Clone::clone)));

//         self.1.symbol_map.with_var(var_name);

//         if let Some(ref value) = value {
//             let mut expr_eval = self.2.as_ref().map_or_else(|| Default::default(), |s| s.clone());
//             expr_eval.symbol_values.insert(var_name.to_owned(), (value.clone(), ty.map(Clone::clone)));

//             return ElementOpScope(self.0.clone(), expr_scope, Some(expr_eval));
//         };

//         ElementOpScope(self.0.clone(), expr_scope, self.2.as_ref().map(Clone::clone))
//     }
// }

#[derive(Debug, Clone, Default)]
pub struct ElementOpScope(pub ScopePrefixes, pub DocumentProcessingScope);
// impl Default for ElementOpScope { fn default() -> Self { ElementOpScope(Default::default(), Default::default()) } }

impl ElementOpScope {
    pub fn with_prop(&mut self, prop_name: &str, ty: Option<&VarType>, value: Option<&ExprValue>) -> &mut Self {
        self.1.with_prop(prop_name, ty, value);
        self
    }

    pub fn add_prop_with_value(&mut self, prop_name: &str, value: &ExprValue) -> &mut Self {
        self.1.add_prop_with_value(prop_name, value);
        self
    }

    pub fn add_loop_var_with_value(&mut self, var_name: &str, value: &ExprValue) -> &mut Self {
        self.1.add_loop_var_with_value(var_name, value);
        self
    }

    pub fn with_symbol(&mut self, var_name: &str, sym: &SymbolReferenceType, ty: Option<&VarType>, value: Option<&ExprValue>) -> &mut Self {
        self.1.with_symbol(var_name, sym, ty, value);
        self
    }

    pub fn with_cached_reducer_key(&mut self, reducer_key: &str) -> &mut Self {
        self.1.with_cached_reducer_key(reducer_key);
        self
    }

    pub fn add_cached_reducer_key_with_value(&mut self, reducer_key: &str, value: &ExprValue) -> &mut Self {
        self.1.add_cached_reducer_key_with_value(reducer_key, value);
        self
    }
}

#[derive(Debug, Clone)]
pub enum ElementKeyPrefixType {
    ScopeElementKeyPrefix(String)
}
pub type KeyPrefix = Option<ElementKeyPrefixType>;

#[derive(Debug, Clone)]
pub enum ActionPrefixType {
    ScopeActionPrefix(String)
}
pub type ActionPrefix = Option<ActionPrefixType>;

#[derive(Debug, Clone)]
pub enum VarPrefixType {
    ScopeVarPrefix(String)
}
pub type VarPrefix = Option<VarPrefixType>;

pub type VarDefault = Option<String>;

pub type ExprPrefix = Option<ExprValue>;

#[derive(Debug, Clone, Default)]
pub struct ScopePrefixes (KeyPrefix, ActionPrefix, VarPrefix, VarDefault, ExprPrefix);

pub trait ScopePrefixOperations {
    fn key_prefix(&self, key: &str) -> String;    
    fn prepend_key_prefix(&self, key: &str) -> String;
    fn prepend_var_prefix(&self, key: &str) -> String;
    fn action_prefix(&self, key: &str) -> String;
    fn default_var_scope(&self) -> Option<String>;
    fn default_var(&self) -> Option<String>;
    fn default_action_scope(&self) -> Option<String>;
    fn var_prefix(&self, key: &str) -> String;
    fn key_expr_prefix(&self, key: &str) -> Option<ExprValue>;
}

#[allow(dead_code)]
pub fn add_key_prefix(base: &ScopePrefixes, key: &str) -> ScopePrefixes {
    let key_prefix = base.key_prefix(key);
    ScopePrefixes(Some(ElementKeyPrefixType::ScopeElementKeyPrefix(key_prefix)), base.1.as_ref().map(Clone::clone), base.2.as_ref().map(Clone::clone), base.3.as_ref().map(Clone::clone), base.4.as_ref().map(Clone::clone))
}

#[allow(dead_code)]
pub fn add_var_prefix(base: &ScopePrefixes, key: &str) -> ScopePrefixes {
    let key_prefix = base.var_prefix(key);
    ScopePrefixes(base.0.as_ref().map(Clone::clone), base.1.as_ref().map(Clone::clone), Some(VarPrefixType::ScopeVarPrefix(key_prefix)), base.3.as_ref().map(Clone::clone), base.4.as_ref().map(Clone::clone))
}

#[allow(dead_code)]
pub fn add_action_prefix(base: &ScopePrefixes, key: &str) -> ScopePrefixes {
    let key_prefix = base.action_prefix(key);
    ScopePrefixes(base.0.as_ref().map(Clone::clone), Some(ActionPrefixType::ScopeActionPrefix(key_prefix)), base.2.as_ref().map(Clone::clone), base.3.as_ref().map(Clone::clone), base.4.as_ref().map(Clone::clone))
}

#[allow(dead_code)]
pub fn prepend_key_prefix(base: &ScopePrefixes, key: &str) -> ScopePrefixes {
    let key_prefix = base.prepend_key_prefix(key);
    ScopePrefixes(Some(ElementKeyPrefixType::ScopeElementKeyPrefix(key_prefix)), base.1.as_ref().map(Clone::clone), base.2.as_ref().map(Clone::clone), base.3.as_ref().map(Clone::clone), base.4.as_ref().map(Clone::clone))
}

#[allow(dead_code)]
pub fn prepend_var_prefix(base: &ScopePrefixes, key: &str) -> ScopePrefixes {
    let key_prefix = base.prepend_var_prefix(key);
    ScopePrefixes(base.0.as_ref().map(Clone::clone), base.1.as_ref().map(Clone::clone), Some(VarPrefixType::ScopeVarPrefix(key_prefix)), base.3.as_ref().map(Clone::clone), base.4.as_ref().map(Clone::clone))
}

#[allow(dead_code)]
pub fn with_default_var(base: &ScopePrefixes, default_var: &str) -> ScopePrefixes {
    ScopePrefixes(base.0.as_ref().map(Clone::clone), base.1.as_ref().map(Clone::clone), base.2.as_ref().map(Clone::clone), Some(default_var.to_owned()), base.4.as_ref().map(Clone::clone))
}

#[allow(dead_code)]
pub fn with_key_expr_prefix(base: &ScopePrefixes, expr: ExprValue) -> ScopePrefixes {
    ScopePrefixes(base.0.as_ref().map(Clone::clone), base.1.as_ref().map(Clone::clone), base.2.as_ref().map(Clone::clone), base.3.as_ref().map(Clone::clone), Some(expr))
}

impl ScopePrefixOperations for ScopePrefixes {
    fn key_prefix(&self, key: &str) -> String {
        match self.0 {
            Some(ElementKeyPrefixType::ScopeElementKeyPrefix(ref prefix)) => {
                format!("{}.{}", prefix, key)
            },
            _ => format!("{}", key)
        }
    }

    fn prepend_key_prefix(&self, key: &str) -> String {
        match self.0 {
            Some(ElementKeyPrefixType::ScopeElementKeyPrefix(ref prefix)) => {
                format!("{}.{}", key, prefix)
            },
            _ => format!("{}", key)
        }
    }

    fn action_prefix(&self, key: &str) -> String {
        match self.1 {
            Some(ActionPrefixType::ScopeActionPrefix(ref prefix)) => {
                format!("{}.{}", prefix.to_uppercase(), key.to_uppercase())
            },
            _ => format!("{}", key.to_uppercase())
        }
    }

    fn default_action_scope(&self) -> Option<String> {
        match self.1 {
            Some(ActionPrefixType::ScopeActionPrefix(ref prefix)) => {
                Some(format!("{}", prefix))
            },
            _ => None
        }
    }

    fn default_var_scope(&self) -> Option<String> {
        match self.2 {
            Some(VarPrefixType::ScopeVarPrefix(ref prefix)) => {
                Some(format!("{}", prefix))
            },
            _ => None
        }
    }

    fn default_var(&self) -> Option<String> {
        match self.3 {
            Some(ref default_var) => {
                Some(format!("{}", default_var))
            },
            _ => None
        }
    }

    fn var_prefix(&self, key: &str) -> String {
        match self.2 {
            Some(VarPrefixType::ScopeVarPrefix(ref prefix)) => {
                format!("{}.{}", prefix, key)
            },
            _ => format!("{}", key)
        }
    }

    fn prepend_var_prefix(&self, key: &str) -> String {
        match self.2 {
            Some(VarPrefixType::ScopeVarPrefix(ref prefix)) => {
                format!("{}.{}", key, prefix)
            },
            _ => format!("{}", key)
        }
    }

    fn key_expr_prefix(&self, key: &str) -> Option<ExprValue> {
        self.4.as_ref().map(Clone::clone)
    }
}
