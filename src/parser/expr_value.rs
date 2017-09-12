use parser::*;

/// Simple expression (parameter value)
#[derive(Debug, Clone, PartialEq)]
pub enum ExprValue {
    LiteralNumber(i32),
    LiteralString(String),
    LiteralArray(Option<Vec<ExprValue>>),
    LiteralObject(Option<Vec<Prop>>),
    LiteralBool(bool),
    DefaultVariableReference,
    SymbolReference(Symbol),
    SymbolPathReference(Vec<Symbol>),
    Binding(BindingType),
    TypedBinding(BindingType, VarType),
    Expr(ExprOp, Box<ExprValue>, Box<ExprValue>),
    Apply(ExprApplyOp, Option<Vec<Box<ExprValue>>>),
    ContentNode(Box<ContentNodeType>),
    // DefaultAction(Option<Vec<String>>, Option<Vec<ActionOpNode>>),
    // Action(String, Option<Vec<String>>, Option<Vec<ActionOpNode>>),
    Undefined,
}

impl ExprValue {
    #[inline]
    pub fn is_literal(&self) -> bool {
        match self {
            &ExprValue::LiteralArray(..) |
            &ExprValue::LiteralString(..) |
            &ExprValue::LiteralNumber(..) => true,
            _ => false,
        }
    }

    #[inline]
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

    #[allow(dead_code)]
    pub fn peek_ty(&self) -> Option<VarType> {
        match self {
            &ExprValue::LiteralNumber(..) => {
                return Some(VarType::Primitive(PrimitiveVarType::Number));
            }

            &ExprValue::LiteralString(..) => {
                return Some(VarType::Primitive(PrimitiveVarType::StringVar));
            }

            &ExprValue::LiteralArray(Some(ref items)) => {
                if !items.is_empty() {
                    if let Some(ref first_item) = items.get(0) {
                        if let Some(var_ty) = first_item.peek_ty() {
                            return Some(VarType::ArrayVar(Some(Box::new(var_ty))));
                        }
                        return Some(VarType::ArrayVar(None));
                    };
                };
                return Some(VarType::ArrayVar(None));
            }

            &ExprValue::SymbolReference(ref sym) => {
                if let Some(ty) = sym.ty() { return Some(ty.to_owned()); }
            }

            _ => {}
        };
        None
    }

    #[inline]
    #[allow(dead_code)]
    pub fn peek_is_array(&self) -> bool {
        match self {
            &ExprValue::LiteralString(..) => true,

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