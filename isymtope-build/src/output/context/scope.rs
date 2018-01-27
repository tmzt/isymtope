
use std::cmp::Eq;
use std::hash::Hash;
use std::fmt::Debug;

use std::collections::HashMap;

use util::*;
use error::*;
use traits::*;
use expressions::*;

use super::*;


#[derive(Debug, Clone)]
pub struct OutputScope<T: Hash + Eq + Debug> {
    scope_id: String,
    parent_id: Option<String>,

    environment: Option<OutputScopeEnvironment>,
    scoped_values: HashMap<CommonBindings<T>, ExpressionValue<OutputExpression>>,
    loop_values: HashMap<CommonBindings<OutputExpression>, ExpressionValue<OutputExpression>>
}

impl<T: Hash + Eq + Debug> Default for OutputScope<T> {
    fn default() -> Self {
        OutputScope::new(None as Option<String>, Default::default())
    }
}

impl<T: Hash + Eq + Debug> ScopeParentId for OutputScope<T> {
    fn parent_id(&self) -> Option<&str> { self.parent_id.as_ref().map(|s| s.as_str()) }
}

impl<T: Hash + Eq + Debug> OutputScope<T> {

    #[allow(dead_code)]
    pub fn new<P: Into<String>>(parent_id: Option<P>, environment: Option<OutputScopeEnvironment>) -> Self {
        let scope_id = allocate_element_key();
        let parent_id = parent_id.map(|s| s.into());

    	OutputScope {
            scope_id: scope_id,
            parent_id: parent_id,

            environment: environment,

            scoped_values: Default::default(),
            loop_values: Default::default(),
        }
    }

    pub fn id(&self) -> &str { &self.scope_id }

    pub fn add_value(&mut self, common_binding: CommonBindings<T>, value: ExpressionValue<OutputExpression>) -> DocumentProcessingResult<()> {
        self.scoped_values.insert(common_binding, value);
        Ok(())
    }

    pub fn get_value(&mut self, common_binding: &CommonBindings<T>) -> Option<ExpressionValue<OutputExpression>> where T: Clone {
        eprintln!("[OutputContext scope {}] Getting value for binding [{:?}]", self.scope_id, common_binding);
        for binding in self.scoped_values.iter() {
            eprintln!("[OutputContext scope {}] Binding: {:?}", self.scope_id, binding);
        }

        self.scoped_values.get(common_binding).map(|v| v.to_owned())
    }

    pub fn add_loop_value(&mut self, common_binding: CommonBindings<OutputExpression>, value: ExpressionValue<OutputExpression>) -> DocumentProcessingResult<()> {
        self.loop_values.insert(common_binding, value);
        Ok(())
    }

    pub fn get_loop_value(&mut self, common_binding: &CommonBindings<OutputExpression>) -> Option<ExpressionValue<OutputExpression>> {
        eprintln!("[OutputContext scope {}] Getting value for binding (loop value) [{:?}]", self.scope_id, common_binding);
        for binding in self.loop_values.iter() {
            eprintln!("[OutputContext scope {}] Binding: {:?}", self.scope_id, binding);
        }

        self.loop_values.get(common_binding).map(|v| v.to_owned())
    }

    pub fn environment(&self) -> Option<OutputScopeEnvironment> {
        let environment = self.environment.as_ref().map(|s| s.to_owned());
        eprintln!("[OutputContext] getting environment: {:?}", environment);
        environment
    }
}