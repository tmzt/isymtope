use parser::*;

#[derive(Debug, Clone, PartialEq)]
pub enum IterMethodPipelineComponent {
    Method(String, Option<Vec<ExprValue>>),
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

#[allow(dead_code)]
#[derive(Debug, Clone, PartialEq)]
pub enum ReducedMethodType {
    Reduce(ExprValue, Option<ExprValue>),
    Map(ExprValue),
    FlatMap(ExprValue),
    FilterMap(ExprValue),
    Any(ExprValue),
    All(ExprValue),
    First,
    Last,
    Take(usize),
    TakeWhile(ExprValue),
    TakeUntil(ExprValue),
    SkipWhile(ExprValue),
    SkipUntil(ExprValue),
    Nth(usize)
}

#[derive(Debug, Clone, PartialEq)]
pub enum ReducedPipelineComponent {
    PipelineOp(ReducedMethodType),
    Symbol(Symbol)
}

pub type IterMethodPipelineComponentVec = Vec<IterMethodPipelineComponent>;
pub type ReducedPipelineComponentVec = Vec<ReducedPipelineComponent>;

/// Simple expression (parameter value)
#[derive(Debug, Clone, PartialEq)]
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
    Apply(ExprApplyOp, Option<Vec<Box<ExprValue>>>),
    ContentNode(Box<ContentNodeType>),
    IterMethodPipeline(Option<Box<ExprValue>>, Option<IterMethodPipelineComponentVec>),
    ReducedPipeline(Option<Box<ExprValue>>, Option<ReducedPipelineComponentVec>),
    Undefined,
}

impl ExprValue {
    #[inline]
    pub fn is_literal(&self) -> bool {
        match self {
            &ExprValue::LiteralArray(..) |
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
    use parser::*;


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