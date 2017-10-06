use model::*;


#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct QueryDefinition(String, Option<Vec<String>>, Option<Vec<QueryDefinitionComponent>>);

impl QueryDefinition {
    pub fn new(name: &str, params: Option<Vec<String>>, components: Option<Vec<QueryDefinitionComponent>>) -> Self {
        QueryDefinition(name.to_owned(), params, components)
    }

    pub fn name(&self) -> &str { &self.0 }

    pub fn params_iter<'a>(&'a self) -> Option<impl Iterator<Item = &'a str>> {
        self.1.as_ref().map(|params| params.iter().map(|s| s.as_str()))
    }

    pub fn components_iter<'a>(&'a self) -> Option<impl Iterator<Item = &'a QueryDefinitionComponent>> {
        self.2.as_ref().map(|components| components.iter())
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum QueryDefinitionComponent {
    CaseWhere(Box<ExprValue>, Box<ExprValue>)
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct UnresolvedQueryInvocation(String, Option<PropVec>, Option<VarType>);

impl UnresolvedQueryInvocation {
    pub fn new(name: &str, props: Option<PropVec>, ty: Option<VarType>) -> Self {
        UnresolvedQueryInvocation(name.to_owned(), props, ty)
    }

    pub fn query_name(&self) -> &str { &self.0 }
    pub fn ty(&self) -> Option<&VarType> { self.2.as_ref() }

    pub fn props_iter<'a>(&'a self) -> Option<impl Iterator<Item = PropRef<'a>>> {
        self.1.as_ref().map(|props| props.iter().map(|s| (s.0.as_str(), s.1.as_ref())))
        // self.1.as_ref().map(|props| props.iter().map(|s| s.0.as_str()))
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct LocalQueryInvocation(String, Option<PropVec>, Option<VarType>);

impl LocalQueryInvocation {
    pub fn new(name: &str, props: Option<PropVec>, ty: Option<VarType>) -> Self {
        LocalQueryInvocation(name.to_owned(), props, ty)
    }

    pub fn query_name(&self) -> &str { &self.0 }
    pub fn ty(&self) -> Option<&VarType> { self.2.as_ref() }

    pub fn props_iter<'a>(&'a self) -> Option<impl Iterator<Item = ActualPropRef<'a>>> {
        self.1.as_ref().map(|props| props.iter().map(|s| (s.0.as_str(), s.1.as_ref())))
    }
}
