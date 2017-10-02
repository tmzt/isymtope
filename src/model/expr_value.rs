use std::iter;
use model::*;
use parser::*;


pub trait AsStaticString {
  fn as_static_string(&self) -> String;
}

pub trait AsExpr {
    fn as_expr(&self) -> ExprValue;
}

pub type IterMethodPipelineParam = (ExprValue, Option<ExprValue>);

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum IterMethodPipelineComponent {
    Method(String, Option<Vec<IterMethodPipelineParam>>),
    PathComponent(String)
}

impl IterMethodPipelineComponent {
    pub fn is_path(&self) -> bool {
        if let &IterMethodPipelineComponent::PathComponent(..) = self {
            return true;
        }
        false
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum FilterPipelineComponent {
    Set(Option<PropVec>, Option<FilterPipelineWhereClause>),
    Where(FilterPipelineWhereClause),
    Unique(Symbol)
}

impl FilterPipelineComponent {
    #[allow(dead_code)]
    pub fn where_clause(first: ExprValue, rest: Option<Vec<ExprValue>>) -> FilterPipelineWhereClause {
        FilterPipelineWhereClause(first, rest)
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct FilterPipelineWhereClause(ExprValue, Option<Vec<ExprValue>>);

impl FilterPipelineWhereClause {
    pub fn first_cond(&self) -> &ExprValue { &self.0 }

    pub fn as_expr(&self) -> ExprValue {
        let first = &self.0;
        match &self.1 {
            &Some(ref and_values) => {
                let params: Vec<_> = iter::once(first).chain(and_values.iter()).cloned().collect();

                ExprValue::BoolGroupExpr(BoolGroupOp::All, params.into())
            },

            _ => first.to_owned()
        }

    }
}

#[allow(dead_code)]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum ReducedMethodType {
    Reduce(ExprValue, Option<ExprValue>),
    ReduceIf(ExprValue, ExprValue, Option<ExprValue>),
    Map(ExprValue),
    MapIf(ExprValue, ExprValue),
    FlatMap(ExprValue),
    FilterMap(ExprValue),
    UniqMember(Symbol),
    Uniq(ExprValue),
    Any(ExprValue),
    All(ExprValue),
    First,
    Last,
    Max,
    Min,
    Take(usize),
    TakeWhile(ExprValue),
    TakeUntil(ExprValue),
    SkipWhile(ExprValue),
    SkipUntil(ExprValue),
    Nth(usize)
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum ReducedPipelineComponent {
    PipelineOp(ReducedMethodType),
    Symbol(Symbol)
}

pub type IterMethodPipelineComponentVec = Vec<IterMethodPipelineComponent>;
pub type ReducedPipelineComponentVec = Vec<ReducedPipelineComponent>;
pub type FilterPipelineComponentVec = Vec<FilterPipelineComponent>;

#[derive(Debug, Clone, PartialEq)]
pub enum InstanceKey<'a> {
  Static(&'a str),
  Dynamic(&'a ExprValue)
}

impl<'a> AsStaticString for InstanceKey<'a> {
  fn as_static_string(&self) -> String {
    match self {
      &InstanceKey::Static(s) => s.to_owned(),
      &InstanceKey::Dynamic(_) => "undefined".into()
    }
  }
}

impl<'a> AsExpr for InstanceKey<'a> {
  fn as_expr(&self) -> ExprValue {
    match self {
      &InstanceKey::Static(s) => ExprValue::LiteralString(s.to_owned()),
      &InstanceKey::Dynamic(e) => e.to_owned()
    }
  }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum StaticValue {
    StaticString(String),
    StaticNumber(i32),
    StaticBool(bool)
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum ReducedValue {
  Static(StaticValue),
  Dynamic(ExprValue)
}

impl AsStaticString for ReducedValue {
  fn as_static_string(&self) -> String {
    match self {
      &ReducedValue::Static(ref s) => match s {
          &StaticValue::StaticString(ref s) => s.to_owned(),
          &StaticValue::StaticNumber(n) => format!("{}", n),
          &StaticValue::StaticBool(b) => format!("{}", b)
      },
      &ReducedValue::Dynamic(_) => "undefined".into()
    }
  }
}

impl AsExpr for ReducedValue {
  fn as_expr(&self) -> ExprValue {
    match self {
      &ReducedValue::Static(ref s) => match s {
          &StaticValue::StaticString(ref s) => ExprValue::LiteralString(s.to_owned()),
          &StaticValue::StaticNumber(n) => ExprValue::LiteralNumber(n),
          &StaticValue::StaticBool(b) => ExprValue::LiteralBool(b)
      },
      &ReducedValue::Dynamic(ref expr) => expr.to_owned()
    }
  }
}

/// Operators
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum ExprOp {
    Add,
    Sub,
    Mul,
    Div,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum ExprApplyOp {
    JoinString(Option<String>),
    // Sum
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum TestOp {
    Negate,
    EqualTo,
    NotEqualTo,
    GreaterThan,
    LessThan,
    GreaterThanOrEqualTo,
    LessThanOrEqualTo,
    IsUndefined,
    IsNaN
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum BoolGroupOp {
    All,
    One,
    Zero
}

/// Simple expression (parameter value)
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum ExprValue {
    Group(Option<Box<ExprValue>>),
    LiteralNumber(i32),
    LiteralString(String),
    LiteralArray(Option<Vec<ExprValue>>),
    LiteralObject(Option<Vec<Prop>>),
    LiteralBool(bool),
    SymbolReference(Symbol),
    Binding(BindingType),
    Expr(ExprOp, Box<ExprValue>, Box<ExprValue>),
    TestValue(TestOp, Box<ExprValue>, Option<Box<ExprValue>>),
    BoolGroupExpr(BoolGroupOp, Box<Vec<ExprValue>>),
    Apply(ExprApplyOp, Option<Vec<Box<ExprValue>>>),
    ContentNode(Box<ContentNodeType>),
    IterMethodPipeline(Option<Box<ExprValue>>, Option<IterMethodPipelineComponentVec>),
    ReducedPipeline(Option<Box<ExprValue>>, Option<ReducedPipelineComponentVec>),
    FilterPipeline(Option<Box<ExprValue>>, Option<FilterPipelineComponentVec>),
    Undefined,
}

impl ExprValue {

    #[inline]
    pub fn is_literal_primitive(&self) -> bool {
        match self {
            &ExprValue::LiteralString(..) |
            &ExprValue::LiteralNumber(..) |
            &ExprValue::LiteralBool(..) => true,
            _ => false,
        }
    }

    #[inline]
    #[allow(dead_code)]
    pub fn is_array(&self) -> bool {
        match self {
            &ExprValue::LiteralArray(..) => true,
            _ => false,
        }
    }

    #[inline]
    #[allow(dead_code)]
    pub fn is_literal_string(&self) -> bool {
        match self {
            &ExprValue::LiteralString(..) => true,
            _ => false,
        }
    }

    #[inline]
    #[allow(dead_code)]
    pub fn string_value(&self) -> Option<&str> {
        match self {
            &ExprValue::LiteralString(ref s) => Some(s.as_str()),
            _ => None
        }
    }

    #[inline]
    #[allow(dead_code)]
    pub fn reduce_to_string(&self) -> Option<String> {
        match self {
            &ExprValue::LiteralString(ref s) => Some(s.to_owned()),
            &ExprValue::Apply(ExprApplyOp::JoinString(ref sep), Some(ref arr)) => {
                // let res = arr.iter().map(|&box ref e| match e { &ExprValue::LiteralString(ref s) => s.to_owned(), _ => "undefined".to_owned() }).join(sep.as_ref().map_or("", |s| s.as_str()));
                let mut res = String::new();
                let mut wrote_first = false;
                for &box ref e in arr {
                    if let &ExprValue::LiteralString(ref s) = e {
                        if wrote_first { if let &Some(ref sep) = sep { res.push_str(sep); } }
                        res.push_str(s);
                        wrote_first = true;
                    };
                    return None;
                }
                Some(res)
            },
            _ => None,
        }
    }

    #[allow(dead_code)]
    pub fn peek_ty(&self) -> Option<VarType> {
        match self {
            &ExprValue::LiteralNumber(..) => Some(VarType::Primitive(PrimitiveVarType::Number)),
            &ExprValue::LiteralString(..) => Some(VarType::Primitive(PrimitiveVarType::StringVar)),
            &ExprValue::LiteralArray(..) => self.peek_array_ty(),
            &ExprValue::LiteralObject(..) => Some(VarType::ObjectVar),

            &ExprValue::SymbolReference(ref sym) => sym.ty().map(|sym| sym.to_owned()),

            &ExprValue::Expr(_, box ref l_expr, box ref r_expr) => {
                l_expr.peek_ty().or_else(|| r_expr.peek_ty())
            }

            _ => None
        }
    }

    #[inline]
    #[allow(dead_code)]
    pub fn peek_array_ty(&self) -> Option<VarType> {
        match self {
            &ExprValue::LiteralArray(Some(ref arr)) => {
                if arr.len() > 0 {
                    let ty = arr.get(0).and_then(|e| e.peek_ty().map(|e| Box::new(e)));
                    return Some(VarType::ArrayVar(ty));
                };

                Some(VarType::ArrayVar(None))
            }

            &ExprValue::Expr(_, box ref l_expr, box ref r_expr) => {
                l_expr.peek_array_ty().or_else(|| r_expr.peek_array_ty())
            }

            _ => None
        }
    }

    #[inline]
    #[allow(dead_code)]
    pub fn peek_is_array(&self) -> bool {
        match self {
            &ExprValue::LiteralArray(..) => true,

            &ExprValue::Expr(_, box ref l_expr, box ref r_expr) => {
                l_expr.peek_is_array() || r_expr.peek_is_array()
            }

            &ExprValue::SymbolReference(ref sym) => {
                if let Some(&VarType::ArrayVar(..)) = sym.ty() { true } else { false }
            }

            _ => false
        }
    }

    #[inline]
    #[allow(dead_code)]
    pub fn peek_is_object(&self) -> bool {
        match self {
            &ExprValue::LiteralObject(..) => true,

            &ExprValue::Expr(_, box ref l_expr, box ref r_expr) => {
                l_expr.peek_is_object() || r_expr.peek_is_object()
            }

            &ExprValue::SymbolReference(ref sym) => {
                if let Some(&VarType::ObjectVar) = sym.ty() { true } else { false }
            }

            _ => false
        }
    }

    #[inline]
    #[allow(dead_code)]
    pub fn peek_is_string(&self) -> bool {
        match self {
            &ExprValue::LiteralString(..) => true,

            &ExprValue::Expr(_, box ref l_expr, box ref r_expr) => {
                l_expr.peek_is_string() || r_expr.peek_is_string()
            }

            &ExprValue::SymbolReference(ref sym) if sym.ty() == Some(&VarType::string()) => true,

            _ => false
        }
    }

    #[allow(dead_code)]
    pub fn initial_value_expr(&self) -> Option<ExprValue> {
        if let &ExprValue::SymbolReference(ref sym) = self {
            if let &SymbolReferenceType::InitialValue(box ref initial, _) = sym.sym_ref() {
                return Some(ExprValue::SymbolReference(initial.to_owned()));
            };
        };

        None
    }

    #[allow(dead_code)]
    pub fn member_ref<'a>(&'a self, path: &str) -> Option<&'a ExprValue> {
        let member_resolver = MemberResolver::new_with_parts(self, path.split("."));
        member_resolver.resolve_member()
    }
}

#[cfg(test)]
mod tests {
    use model::*;


    #[test]
    pub fn test_expr_member_ref() {
        let obj = ExprValue::LiteralObject(Some(vec![
            ("a".into(), Some(ExprValue::LiteralObject(Some(vec![
                ("b".into(), Some(ExprValue::LiteralString("y1".into())))
            ]))))
        ]));

        let res = obj.member_ref("a.b");
        assert_eq!(res, Some(&ExprValue::LiteralString("y1".into())));
    }

}