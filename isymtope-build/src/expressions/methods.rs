use expressions::*;

#[allow(dead_code)]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct ReducedPipelineValue<T>(
    Box<ExpressionValue<T>>,
    Box<Vec<ReducedPipelineComponent<T>>>,
);

impl<T> ReducedPipelineValue<T> {
    pub fn new(e: ExpressionValue<T>, v: Vec<ReducedPipelineComponent<T>>) -> Self {
        ReducedPipelineValue(Box::new(e), Box::new(v))
    }

    pub fn head(&self) -> &ExpressionValue<T> {
        self.0.as_ref()
    }

    pub fn components<'a>(&'a self) -> impl Iterator<Item = &'a ReducedPipelineComponent<T>> {
        let box ref v = self.1;
        v.iter()
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum ReducedPipelineComponent<T> {
    PipelineOp(ReducedMethodCall<T>),
    Member(String),
    ExpressionValue(ExpressionValue<T>),
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum ReducedMethodCall<T> {
    Map(ExpressionValue<T>),
    MapIf(ExpressionValue<T>, ExpressionValue<T>),
    Filter(ExpressionValue<T>),
    Reduce(ExpressionValue<T>, ExpressionValue<T>),
    ReduceIf(ExpressionValue<T>, ExpressionValue<T>, ExpressionValue<T>),
    Uniq(ExpressionValue<T>),
    UniqByKey(String),
    MaxBy(ExpressionValue<T>),
    MinBy(ExpressionValue<T>),
    Count(ExpressionValue<T>),
    FirstWhere(ExpressionValue<T>),
    First,
}
