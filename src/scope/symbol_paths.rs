#![allow(dead_code)]

use std::ops::{Add, AddAssign};

use model::*;


#[derive(Debug, Clone, PartialEq)]
pub enum SymbolPathComponent {
    StaticPathComponent(String),
    EvalPathComponent(ExprValue)
}

impl AsExpr for SymbolPathComponent {
    fn as_expr(&self) -> ExprValue {
        match self {
            &SymbolPathComponent::StaticPathComponent(ref s) => ExprValue::LiteralString(s.to_owned()),
            &SymbolPathComponent::EvalPathComponent(ref e) => e.to_owned()
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct SymbolPathScope(Option<Vec<SymbolPathComponent>>, Option<Symbol>, Option<String>);

// impl<'a, 'b> Add<&'b ExprValue> for &'a SymbolPathScope {
//     type Output = ExprValue;
//     fn add(self, rhs: &'b ExprValue) -> ExprValue { self.do_join(None, Some(rhs)) }
// }

impl<'a, Rhs: AsExpr> Add<Rhs> for &'a SymbolPathScope {
    type Output = ExprValue;
    fn add(self, rhs: Rhs) -> ExprValue { self.do_join(None, Some(&rhs)) }
}

impl<'a, Rhs: AsExpr> AddAssign<Rhs> for &'a mut SymbolPathScope {
    fn add_assign(&mut self, rhs: Rhs) { let expr = rhs.as_expr(); self.append_expr(&expr); }
}

impl SymbolPathScope {
    pub fn with_sep(sep: &str) -> Self {
    	SymbolPathScope(Default::default(), Default::default(), Some(sep.to_owned()))
    }

    #[inline]
    pub fn append_expr(&mut self, expr: &ExprValue) {
        let comp = SymbolPathComponent::EvalPathComponent(expr.to_owned());
        if let Some(ref mut v) = self.0 {
            v.push(comp);
        } else {
            self.0 = Some(vec![comp]);
        };
    }

    #[inline]
    pub fn append_str(&mut self, s: &str) {
        let comp = SymbolPathComponent::StaticPathComponent(s.to_owned());
        if let Some(ref mut v) = self.0 {
            v.push(comp);
        } else {
            self.0 = Some(vec![comp]);
        };
    }

    #[inline]
    fn do_join<T: AsExpr + ?Sized>(&self, override_sep: Option<Option<&str>>, last: Option<&T>) -> ExprValue {
        let sep = override_sep.unwrap_or(self.2.as_ref().map(|s| s.as_str()));

        let last_map = last.map(|last| SymbolPathComponent::EvalPathComponent(last.as_expr())).to_owned();
        let components_iter = self.0.as_ref().map(|v| v.into_iter()).into_iter().flat_map(|m| m)
            .chain(last_map.as_ref().into_iter());

        let components: Vec<_> = components_iter.collect();

        let join_opt = sep.map(|s| s.to_owned());
        let components = if !components.is_empty() { Some(components.into_iter().map(|c| c.as_expr().into()).collect()) } else { None };
        ExprValue::Apply(ExprApplyOp::JoinString(join_opt), components)
    }

    #[inline]
    pub fn path_expr(&self) -> ExprValue { self.do_join(None, None as Option<&ExprValue>) }

    #[inline]
    pub fn path_expr_using(&self, s: &str) -> ExprValue { self.do_join(Some(Some(s)), None as Option<&AsExpr>) }

    #[inline]
    pub fn path_expr_with<T: AsExpr + ?Sized>(&self, last: &T) -> ExprValue { self.do_join(None, Some(last)) }

    #[inline]
    pub fn path_expr_using_with<T: AsExpr + ?Sized>(&self, s: &str, last: &T) -> ExprValue { self.do_join(Some(Some(s)), Some(last)) }
}


#[cfg(test)]
mod tests {
    use super::*;


    #[test]
    pub fn test_symbol_path_expr1() {
        let mut symbol_path_scope = SymbolPathScope::with_sep(".");
        let expr1 = ExprValue::Expr(ExprOp::Add, Box::new(ExprValue::LiteralNumber(1)), Box::new(ExprValue::LiteralNumber(2)));
        let expr2 = ExprValue::LiteralString("test".to_owned());

        symbol_path_scope.append_expr(&expr1);
        symbol_path_scope.append_expr(&expr2);

        let expr = symbol_path_scope.path_expr();
        assert_eq!(expr, ExprValue::Apply(ExprApplyOp::JoinString(None), Some(vec![
            Box::new(ExprValue::Expr(ExprOp::Add, Box::new(ExprValue::LiteralNumber(1)), Box::new(ExprValue::LiteralNumber(2)))),
            Box::new(ExprValue::LiteralString("test".to_owned()))
        ])));
    }

    #[test]
    pub fn test_symbol_path_mixed1() {
        let mut symbol_path_scope = SymbolPathScope::with_sep(".");
        let expr1 = ExprValue::Expr(ExprOp::Add, Box::new(ExprValue::LiteralNumber(1)), Box::new(ExprValue::LiteralNumber(2)));

        symbol_path_scope.append_expr(&expr1);
        symbol_path_scope.append_str("test");

        let expr = symbol_path_scope.path_expr();
        assert_eq!(expr, ExprValue::Apply(ExprApplyOp::JoinString(None), Some(vec![
            Box::new(ExprValue::Expr(ExprOp::Add, Box::new(ExprValue::LiteralNumber(1)), Box::new(ExprValue::LiteralNumber(2)))),
            Box::new(ExprValue::LiteralString("test".to_owned()))
        ])));
    }
}
