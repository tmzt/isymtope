use model::*;
use processing::*;


#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Query (String, Option<FormalPropVec>, Option<Vec<QueryComponent>>);

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum QueryComponent {
    CaseWhere(Box<ExprValue>, Box<ExprValue>)
}

impl Query {
    pub fn new(name: String, params: Option<FormalPropVec>, components: Option<Vec<QueryComponent>>) -> Self {
        Query(name, params, components)
    }

    pub fn name(&self) -> &str { &self.0 }

    #[allow(dead_code)]
    pub fn formal_props_iter<'a>(&'a self) -> Option<impl Iterator<Item = FormalPropRef<'a>>> {
        self.1.as_ref().map(|formals| formals.iter().map(|s| (s.as_str())))
    }

    pub fn components_iter<'a>(&'a self) -> Option<impl Iterator<Item = &'a QueryComponent>> {
        self.2.as_ref().map(|components| components.iter())
    }
}

impl QueryComponent {
    pub fn case_where(val: ExprValue, cond: ExprValue) -> Self {
        QueryComponent::CaseWhere(val.into(), cond.into())
    }    
}

impl QueryComponent {
    pub fn expr(&self) -> Option<&ExprValue> {
        match self {
            &QueryComponent::CaseWhere(box ref val, _) => Some(val)
        }
    }

    pub fn cond(&self) -> Option<&ExprValue> {
        match self {
            &QueryComponent::CaseWhere(_, box ref cond) => Some(cond)
        }
    }
}