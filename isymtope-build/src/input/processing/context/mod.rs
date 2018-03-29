use std::rc::Rc;
use std::hash::Hash;
use std::cmp::Eq;
use std::collections::HashSet;

use linked_hash_map::LinkedHashMap;

use error::*;
use common::entries::*;
use traits::*;
use expressions::*;
use ast::*;

pub mod scope;
pub use self::scope::*;

#[derive(Debug, Clone)]
pub struct DefaultProcessingContext<T: Hash + Eq> {
    template: Rc<Template>,

    base_scope_id: String,
    cur_scope_id: String,
    scopes: LinkedHashMap<String, ProcessingScope<T>>,

    // reducers: LinkedHashMap<String, Reducer<ProcessedExpression>>,
    reducer_keys: HashSet<String>,
    default_reducer_key: Option<String>,
}

impl ProcessingContext for DefaultProcessingContext<ProcessedExpression> {
    fn template(&self) -> &Template {
        self.template.as_ref()
    }

    fn add_reducer_key(&mut self, key: String) -> DocumentProcessingResult<()> {
        self.reducer_keys.insert(key);
        Ok(())
    }

    fn is_reducer_key(&self, key: &str) -> DocumentProcessingResult<bool> {
        let res = self.reducer_keys.contains(key);
        Ok(res)
    }

    fn push_child_scope_with_environment(&mut self, environment: ProcessingScopeEnvironment) {
        let parent_id = self.cur_scope_id.to_owned();
        eprintln!("parent_id: {}", parent_id);
        let child: ProcessingScope<ProcessedExpression> =
            ProcessingScope::new(Some(parent_id.clone()), environment.clone());
        let child_id = child.id().to_owned();
        self.scopes.insert(child_id.clone(), child);
        self.cur_scope_id = child_id;
        assert!(
            self.scopes.len() > 1,
            "there must be more than one scope after pushing new scope with environment"
        );

        eprintln!("[ProcessingContext] Pushing child scope [{}] with environment {:?} (parent_id: [{:?}]), there are now {} scopes.",
            self.cur_scope_id, environment, parent_id, self.scopes.len());
        // eprintln!("[OutputContext] scopes: [{:?}]", self.scope_id_vec);
    }

    fn push_child_scope(&mut self) {
        self.push_child_scope_with_environment(Default::default())
    }

    #[allow(dead_code)]
    fn pop_scope(&mut self) {
        assert!(self.scopes.len() > 1);
        assert!(
            self.cur_scope_id != self.base_scope_id,
            "Cannot pop base scope."
        );

        let popped = self.scopes.pop_back().unwrap();
        let parent_id = popped.1.parent_id().unwrap().to_owned();

        self.cur_scope_id = parent_id;
        assert!(self.scopes.len() > 0);
    }

    fn bind_ident(
        &mut self,
        key: String,
        binding: CommonBindings<ProcessedExpression>,
    ) -> DocumentProcessingResult<()> {
        let scope = self.scopes.get_mut(&self.cur_scope_id).unwrap();
        scope.add_ident(key, binding)
    }

    fn must_find_ident(
        &mut self,
        key: &str,
    ) -> DocumentProcessingResult<CommonBindings<ProcessedExpression>> {
        eprintln!(
            "[ProcessingContext] find_ident: cur_scope_id: {}",
            self.cur_scope_id
        );
        assert!(self.scopes.len() > 0);

        must_find_entry(&mut self.scopes, &self.cur_scope_id, key, |scope| {
            scope.get_ident(key)
        })
    }

    fn bind_ident_shape(
        &mut self,
        key: String,
        binding: BindingShape<ProcessedExpression>,
    ) -> DocumentProcessingResult<()> {
        let scope = self.scopes.get_mut(&self.cur_scope_id).unwrap();
        scope.add_ident_shape(key, binding)
    }

    fn find_ident_shape(
        &mut self,
        key: &str,
    ) -> DocumentProcessingResult<Option<BindingShape<ProcessedExpression>>> {
        eprintln!(
            "[ProcessingContext] find_ident_shape: cur_scope_id: {}",
            self.cur_scope_id
        );
        assert!(self.scopes.len() > 0);

        find_entry(&mut self.scopes, &self.cur_scope_id, key, |scope| {
            scope.get_ident_shape(key)
        })
    }

    // Element and value bindings

    fn bind_element_binding(
        &mut self,
        key: String,
        binding: CommonBindings<ProcessedExpression>,
    ) -> DocumentProcessingResult<()> {
        let scope = self.scopes.get_mut(&self.cur_scope_id).unwrap();
        scope.add_element_binding(key, binding)
    }

    fn find_element_binding(
        &mut self,
        key: &str,
    ) -> DocumentProcessingResult<Option<CommonBindings<ProcessedExpression>>> {
        eprintln!(
            "[ProcessingContext] find_element_binding: cur_scope_id: {}",
            self.cur_scope_id
        );
        assert!(self.scopes.len() > 0);

        find_entry(&mut self.scopes, &self.cur_scope_id, key, |scope| {
            scope.get_element_binding(key)
        })
    }

    fn environment(&mut self) -> DocumentProcessingResult<ProcessingScopeEnvironment> {
        let closest = find_match(&mut self.scopes, &self.cur_scope_id, |scope| {
            let env = scope.environment();

            match env { &ProcessingScopeEnvironment::Normal => None, _ => Some(env.to_owned()) }
        })?;

        if let Some(closest) = closest {
            return Ok(closest);
        };

        Ok(ProcessingScopeEnvironment::Normal)

        // let scope = self.scopes.get(&self.cur_scope_id).unwrap();
        // let environment = scope.environment().to_owned();

        // environment
    }
}

impl<T: Hash + Eq> DefaultProcessingContext<T> {
    pub fn for_template(template: Rc<Template>) -> Self {
        DefaultProcessingContext::new(template)
    }

    fn new(template: Rc<Template>) -> Self {
        let base_scope: ProcessingScope<T> = Default::default();
        let base_scope_id = base_scope.id().to_owned();

        let mut ctx = DefaultProcessingContext {
            template: template,

            base_scope_id: base_scope_id.clone(),
            cur_scope_id: base_scope_id.clone(),
            scopes: Default::default(),

            // reducers: Default::default(),
            reducer_keys: Default::default(),
            default_reducer_key: None,
        };

        ctx.scopes.insert(base_scope_id, base_scope);
        ctx
    }

    // pub fn doc(&self) -> &Document { self.document_provider.doc() }
}
