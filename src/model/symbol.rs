use model::*;


#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum SymbolReferenceType {
    UnresolvedReference(String),
    UnresolvedPathReference(String),
    Binding(BindingType),
    MemberPath(Box<Symbol>, String),
    InitialValue(Box<Symbol>, Box<Symbol>),
    ResolvedReference(String, ResolvedSymbolType, Option<String>),
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum KeyReferenceType {
    UnboundFormalParam,
    ComponentProp(String),
    InvocationProp(Option<Box<ExprValue>>),
    CurrentElementProp(String),
    UnboundReducerActionParam(String)
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum ResolvedSymbolType {
    ReferenceToKeyInScope(KeyReferenceType, Option<String>),
    ReducerKeyReference(String),
    ParameterReference(String),
    PropReference(String),
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Symbol(SymbolReferenceType, Option<VarType>, Option<Box<ExprValue>>);
// pub type ValueMap = LinkedHashMap<String, ExprValue>;

impl Into<Symbol> for BindingType {
    fn into(self) -> Symbol {
        Symbol(SymbolReferenceType::Binding(self), None, None)
    }
}

impl Symbol {
    #[inline]
    pub fn replace_type(&self, ty: &VarType) -> Self {
        Symbol(self.0.to_owned(), Some(ty.to_owned()), self.2.to_owned())
    }

    pub fn unresolved(key: &str) -> Symbol {
        Symbol(SymbolReferenceType::UnresolvedReference(key.to_owned()),
               None,
               None)
    }

    #[allow(dead_code)]
    pub fn unresolved_path(path: &str) -> Symbol {
        Symbol(SymbolReferenceType::UnresolvedPathReference(path.to_owned()),
               None,
               None)
    }

    #[allow(dead_code)]
    pub fn is_bool(&self) -> bool {
        self.1 == Some(VarType::boolean())
    }

    pub fn binding(binding: &BindingType) -> Symbol {
        Symbol(SymbolReferenceType::Binding(binding.to_owned()), None, None)
    }

    pub fn typed_binding(binding: &BindingType, ty: &VarType) -> Symbol {
        Symbol(SymbolReferenceType::Binding(binding.to_owned()), Some(ty.to_owned()), None)
    }

    pub fn initial_value(initial: &Symbol, after: &Symbol) -> Symbol {
        Symbol(SymbolReferenceType::InitialValue(Box::new(initial.to_owned()), Box::new(after.to_owned())), None, None)
    }

    pub fn member_path(head: Symbol, path: &str) -> Symbol {
        Symbol(SymbolReferenceType::MemberPath(Box::new(head), path.to_owned()), None, None)
    }
    pub fn reducer_key_with(key: &str, ty: Option<&VarType>, value: Option<&ExprValue>) -> Symbol {
        let resolved = ResolvedSymbolType::ReducerKeyReference(key.to_owned());
        let value = value.map(|value| Box::new(value.clone()));
        let ty = ty.map(|ty| ty.clone());
        Symbol(SymbolReferenceType::ResolvedReference(key.to_owned(), resolved, None),
               ty,
               value)
    }

    /// Accessors

    pub fn sym_ref(&self) -> &SymbolReferenceType {
        &self.0
    }

    pub fn ty(&self) -> Option<&VarType> {
        self.1.as_ref()
    }

    #[allow(dead_code)]
    pub fn value(&self) -> Option<&ExprValue> {
        self.2.as_ref().map(|b| b.as_ref())
    }

    #[allow(dead_code)]
    pub fn initial<'a>(&'a self) -> Option<&'a Symbol> {
        match self.sym_ref() { &SymbolReferenceType::InitialValue(box ref initial, _) => Some(initial), _ => None }
    }

    #[allow(dead_code)]
    pub fn resolved_key(&self) -> Option<&str> {
        match self.sym_ref() {
            &SymbolReferenceType::ResolvedReference(ref key, _, _) => Some(key),
            &SymbolReferenceType::Binding(ref binding) => match binding {
                &BindingType::ComponentPropBinding(ref prop_key) => Some(prop_key),
                _ => None
            }
            _ => None
        }
    }

    #[allow(dead_code)]
    pub fn as_single_part_str(&self) -> Option<&str> {
        match self.sym_ref() {
            &SymbolReferenceType::UnresolvedReference(ref key) => Some(key),
            _ => None
        }
    }

    #[allow(dead_code)]
    pub fn as_path_str(&self) -> Option<&str> {
        match self.sym_ref() {
            &SymbolReferenceType::UnresolvedPathReference(ref path) => Some(path),
            &SymbolReferenceType::UnresolvedReference(ref key) => Some(key),
            _ => None
        }
    }

    #[allow(dead_code)]
    pub fn is_transparent(&self) -> bool {
        match self.sym_ref() {
            &SymbolReferenceType::Binding(ref binding) => binding.is_transparent(),
            _ => false
        }
    }
}
