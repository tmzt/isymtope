use std::rc::Rc;
use std::fmt::Debug;
use std::hash::Hash;
use std::cmp::Eq;

use linked_hash_map::LinkedHashMap;

use error::*;
use common::*;
use traits::*;
use expressions::*;
use objects::*;
use input::*;

pub mod scope;
pub use self::scope::*;

#[derive(Debug, Clone)]
pub struct DefaultOutputContext {
    base_scope_id: String,
    cur_scope_id: String,
    scopes: LinkedHashMap<String, OutputScope<ProcessedExpression>>,
    scope_id_vec: Vec<String>,
    state_provider: Option<Rc<ReducerStateProvider>>,
    document_provider: Rc<DocumentProvider>,
}

impl OutputContext for DefaultOutputContext {

    fn doc(&self) -> &Document {
        self.document_provider.doc()
    }

    fn reducer_value(
        &mut self,
        key: &str,
    ) -> DocumentProcessingResult<ExpressionValue<OutputExpression>> {
        eprintln!("[Expression eval] Getting state for reducer key [{}]", key);

        if let Some(state_provider) = self.state_provider.as_ref() {
            let state_provider = state_provider.as_ref();
            let value = state_provider.get(key)?;

            if let Some(value) = value {
                return Ok(value.to_owned());
            }
        };

        let default_value: Option<ExpressionValue<ProcessedExpression>> = {
            self.doc()
                .reducer(key)
                .and_then(|r| r.default_value())
                .map(|r| r.to_owned())
        };

        if let Some(ref default_value) = default_value {
            let default_value = TryEvalFrom::try_eval_from(default_value, self)?;
            return Ok(default_value);
        };

        Err(try_eval_from_err!(format!(
            "Cannot get value for reducer key [{}]",
            key
        )))
    }

    fn push_child_scope_with_environment(&mut self, environment: OutputScopeEnvironment) {
        self.do_push(Some(environment));
    }

    fn push_child_scope(&mut self) {
        self.do_push(None);
    }

    #[allow(dead_code)]
    fn pop_scope(&mut self) {
        assert!(self.scopes.len() > 1);
        assert!(
            self.cur_scope_id != self.base_scope_id,
            "Cannot pop base scope."
        );

        let popped_scope_id = self.cur_scope_id.to_owned();
        let popped = self.scopes.pop_back().unwrap();
        let parent_id = popped.1.parent_id().unwrap().to_owned();

        self.cur_scope_id = parent_id;
        assert!(self.scopes.len() > 0);

        assert!(self.scope_id_vec.len() > 0);
        self.scope_id_vec.pop();

        eprintln!(
            "[OutputContext] Popping scope [{}], current is now [{}], there are now {} scopes.",
            popped_scope_id,
            self.cur_scope_id,
            self.scopes.len()
        );
        eprintln!("[OutputContext] scopes: [{:?}]", self.scope_id_vec);
    }

    fn bind_value(
        &mut self,
        binding: CommonBindings<ProcessedExpression>,
        value: ExpressionValue<OutputExpression>,
    ) -> DocumentProcessingResult<()> {
        let scope = self.scopes.get_mut(&self.cur_scope_id).unwrap();
        eprintln!(
            "[OutputContext] adding binding [{:?}] with value [{:?}] to scope [{}]",
            binding, value, self.cur_scope_id
        );
        scope.add_value(binding, value)
    }

    fn find_value(
        &mut self,
        binding: &CommonBindings<ProcessedExpression>,
    ) -> DocumentProcessingResult<Option<ExpressionValue<OutputExpression>>> {
        eprintln!(
            "[OutputContext]  Looking for value for binding [{:?}]",
            binding
        );

        find_entry(&mut self.scopes, &self.cur_scope_id, binding, |scope| {
            scope.get_value(binding)
        })
    }

    fn must_find_value(
        &mut self,
        binding: &CommonBindings<ProcessedExpression>,
    ) -> DocumentProcessingResult<ExpressionValue<OutputExpression>> {
        eprintln!(
            "[OutputContext]  Looking for value for binding [{:?}]",
            binding
        );

        must_find_entry(&mut self.scopes, &self.cur_scope_id, binding, |scope| {
            scope.get_value(binding)
        })
    }

    fn bind_loop_value(
        &mut self,
        binding: CommonBindings<OutputExpression>,
        value: ExpressionValue<OutputExpression>,
    ) -> DocumentProcessingResult<()> {
        let scope = self.scopes.get_mut(&self.cur_scope_id).unwrap();
        eprintln!(
            "[OutputContext] adding binding [{:?}] with value [{:?}] to scope [{}]",
            binding, value, self.cur_scope_id
        );
        scope.add_loop_value(binding, value)
    }

    fn must_find_loop_value(
        &mut self,
        binding: &CommonBindings<OutputExpression>,
    ) -> DocumentProcessingResult<ExpressionValue<OutputExpression>> {
        eprintln!(
            "[OutputContext]  Looking for value for binding [{:?}]",
            binding
        );

        must_find_entry(&mut self.scopes, &self.cur_scope_id, binding, |scope| {
            scope.get_loop_value(binding)
        })
    }

    fn environment(&mut self) -> DocumentProcessingResult<Option<OutputScopeEnvironment>> {
        find_entry(&mut self.scopes, &self.cur_scope_id, 0, |scope| {
            scope.environment()
        })
    }

    fn bind_element_key(
        &mut self,
        key: &str,
        idx: Option<i32>,
    ) -> DocumentProcessingResult<()> {
        let binding = CommonBindings::CurrentElementKeyPath;

        let element_key = match (self.find_value(&binding)?, idx) {
            (Some(ExpressionValue::Primitive(Primitive::StringVal(ref prefix))), Some(idx)) => {
                format!("{}.{}.{}", prefix, key, idx)
            }
            (Some(ExpressionValue::Primitive(Primitive::StringVal(ref prefix))), _) => {
                format!("{}.{}", prefix, key)
            }
            (_, Some(idx)) => format!("{}.{}", key, idx),
            (_, _) => key.to_owned(),
        };

        eprintln!(
            "[OutputContext] bind_element_key: adding binding for CurrentElementKeyPath: {}",
            element_key
        );
        self.bind_value(
            binding,
            ExpressionValue::Primitive(Primitive::StringVal(element_key)),
        )
    }

    fn get_element_key(&mut self) -> DocumentProcessingResult<Option<String>> {
        let binding = CommonBindings::CurrentElementKeyPath;

        if let Some(ExpressionValue::Primitive(Primitive::StringVal(ref key))) =
            self.find_value(&binding)?
        {
            return Ok(Some(key.to_owned()));
        };

        Ok(None)
    }

    fn must_get_element_key(&mut self) -> DocumentProcessingResult<String> {
        let binding = CommonBindings::CurrentElementKeyPath;

        if let Some(ExpressionValue::Primitive(Primitive::StringVal(ref key))) =
            self.find_value(&binding)?
        {
            return Ok(key.to_owned());
        };

        Err(try_eval_from_err!("Missing element key"))
    }
}

impl DefaultOutputContext {
    pub fn create(
        document_provider: Rc<DocumentProvider>,
        state_provider: Option<Rc<ReducerStateProvider>>,
    ) -> Self {
        Self::new(document_provider, None, state_provider)
    }

    fn new(
        document_provider: Rc<DocumentProvider>,
        base_scope: Option<OutputScope<ProcessedExpression>>,
        state_provider: Option<Rc<ReducerStateProvider>>,
    ) -> Self {
        let base_scope = base_scope.unwrap_or_default();
        let base_scope_id = base_scope.id().to_owned();

        let mut ctx = DefaultOutputContext {
            base_scope_id: base_scope_id.clone(),
            cur_scope_id: base_scope_id.clone(),
            scopes: Default::default(),
            scope_id_vec: Default::default(),
            state_provider: state_provider,
            document_provider: document_provider,
        };

        ctx.scopes.insert(base_scope_id.clone(), base_scope);
        ctx.scope_id_vec.push(base_scope_id);
        ctx
    }

    fn do_push(&mut self, environment: Option<OutputScopeEnvironment>) {
        let parent_id = self.cur_scope_id.clone();
        let child: OutputScope<ProcessedExpression> = OutputScope::new(Some(parent_id.clone()), environment.clone());
        let child_id = child.id().to_owned();
        self.scopes.insert(child_id.clone(), child);
        assert!(self.scopes.len() > 1);

        self.scope_id_vec.push(child_id.clone());
        self.cur_scope_id = child_id;

        eprintln!("[OutputContext] Pushing scope [{}] (parent_id: [{:?}]) (environment: {:?}), there are now {} scopes.", self.cur_scope_id, parent_id, environment, self.scopes.len());
        eprintln!("[OutputContext] scopes: [{:?}]", self.scope_id_vec);
    }
}
