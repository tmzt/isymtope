use std::fmt;
use std::cmp::Eq;
use std::hash::Hash;

use std::collections::HashMap;

use isymtope_ast_common::*;

#[derive(Clone)]
pub struct ProcessingScope<T: Hash + Eq> {
    scope_id: String,
    parent_id: Option<String>,

    environment: ProcessingScopeEnvironment,

    scoped_idents: HashMap<String, CommonBindings<T>>,
    shaped_idents: HashMap<String, BindingShape<T>>,
    element_bindings: HashMap<String, CommonBindings<T>>,
}

impl<T: Hash + Eq> Default for ProcessingScope<T> {
    fn default() -> Self {
        ProcessingScope::new(None as Option<String>, Default::default())
    }
}

impl<T: Hash + Eq> ScopeParentId for ProcessingScope<T> {
    fn parent_id(&self) -> Option<&str> {
        self.parent_id.as_ref().map(|s| s.as_str())
    }
}

impl<T: Hash + Eq> fmt::Debug for ProcessingScope<T> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{} ({:?})", self.id(), self.environment())
    }
}

impl<T: Hash + Eq> ProcessingScope<T> {
    #[allow(dead_code)]
    pub fn new<P: Into<String>>(
        parent_id: Option<P>,
        environment: ProcessingScopeEnvironment,
    ) -> Self {
        let scope_id = allocate_element_key();
        let parent_id = parent_id.map(|s| s.into());

        ProcessingScope {
            scope_id: scope_id,
            parent_id: parent_id,

            environment: environment,

            scoped_idents: Default::default(),
            shaped_idents: Default::default(),
            element_bindings: Default::default(),
        }
    }

    pub fn id(&self) -> &str {
        &self.scope_id
    }

    pub fn add_ident(
        &mut self,
        key: String,
        binding: CommonBindings<T>,
    ) -> DocumentProcessingResult<()> {
        self.scoped_idents.insert(key, binding);
        Ok(())
    }

    pub fn get_ident(&mut self, key: &str) -> Option<CommonBindings<T>>
    where
        T: Clone,
    {
        self.scoped_idents.get(key).map(|v| v.to_owned())
    }

    pub fn add_ident_shape(
        &mut self,
        key: String,
        binding: BindingShape<T>,
    ) -> DocumentProcessingResult<()> {
        self.shaped_idents.insert(key, binding);
        Ok(())
    }

    pub fn get_ident_shape(&mut self, key: &str) -> Option<BindingShape<T>>
    where
        T: Clone,
    {
        self.shaped_idents.get(key).map(|v| v.to_owned())
    }

    pub fn add_element_binding(
        &mut self,
        key: String,
        common_binding: CommonBindings<T>,
    ) -> DocumentProcessingResult<()> {
        self.element_bindings.insert(key, common_binding);
        Ok(())
    }

    pub fn get_element_binding(&mut self, key: &str) -> Option<CommonBindings<T>>
    where
        T: Clone,
    {
        self.element_bindings.get(key).map(|v| v.to_owned())
    }

    pub fn environment(&self) -> &ProcessingScopeEnvironment {
        &self.environment
    }
}
