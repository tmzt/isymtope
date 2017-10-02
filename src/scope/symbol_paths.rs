#![allow(dead_code)]

use parser::ast::*;
use scope::context::*;


#[derive(Debug, Clone, PartialEq)]
pub enum SymbolPathComponent {
    StaticPathComponent(String),
    EvalPathComponent(ExprValue)
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct SymbolPathScope(Option<Vec<SymbolPathComponent>>, Option<Symbol>);

impl SymbolPathScope {
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
    pub fn join_as_str(&self, ctx: &mut Context, s: Option<&str>) -> String {
        let str_components: Option<Vec<String>> = self.0.as_ref().map(|symbol_path| symbol_path.iter()
            .map(|component| match component {
                &SymbolPathComponent::StaticPathComponent(ref s) => s.to_owned(),
                &SymbolPathComponent::EvalPathComponent(ref expr) => ctx.reduce_expr_to_string(expr)
            }).collect());

        let res = str_components.and_then(|str_components| {
            if str_components.len() > 0 { Some(str_components.join(s.unwrap_or(""))) } else { None }
        });

        res.unwrap_or("".into())
    }

    #[inline]
    pub fn join_as_expr(&self, s: Option<&str>) -> ExprValue {
        let expr_components: Option<Vec<Box<ExprValue>>> = self.0.as_ref().map(|symbol_path| symbol_path.iter()
            .map(|component| match component {
                &SymbolPathComponent::StaticPathComponent(ref s) => Box::new(ExprValue::LiteralString(s.to_owned())),
                &SymbolPathComponent::EvalPathComponent(ref expr) => Box::new(expr.to_owned())
            }).collect());

        let components = expr_components.and_then(|expr_components| {
            if expr_components.len() > 0 { Some(expr_components) } else { None }
        });

        let join_opt = s.map(|s| s.to_owned());
        ExprValue::Apply(ExprApplyOp::JoinString(join_opt), components)
    }

    #[inline]
    pub fn join_as_expr_with_expr(&self, sep: Option<&str>, last: &ExprValue) -> ExprValue {
        // Handle special case first
        if self.0.is_none() { return last.to_owned(); }

        let mut parts: Vec<Box<ExprValue>> = Default::default();

        if let Some(iter) = self.0.as_ref().map(|symbol_path| symbol_path.iter()) {
            let arr: Vec<_> = iter.map(|component| match component {
                &SymbolPathComponent::StaticPathComponent(ref s) => Box::new(ExprValue::LiteralString(s.to_owned())),
                &SymbolPathComponent::EvalPathComponent(ref expr) => Box::new(expr.to_owned())
            }).collect();

            if !arr.is_empty() { parts.extend(arr); }
            parts.push(Box::new(last.to_owned()));
        };

        let join_opt = sep.map(|s| s.to_owned());
        ExprValue::Apply(ExprApplyOp::JoinString(join_opt), Some(parts))
    }

    #[inline]
    pub fn join_as_expr_with(&self, sep: Option<&str>, last: &str) -> ExprValue {
        let last = ExprValue::LiteralString(last.to_owned());

        self.join_as_expr_with_expr(sep, &last)
    }
}


#[cfg(test)]
mod tests {
    use super::*;
    use parser::ast::*;


    #[test]
    pub fn test_symbol_path_empty() {
        let symbol_path_scope = SymbolPathScope::default();
        let expr = symbol_path_scope.join_as_expr(None);
        assert_eq!(expr, ExprValue::Apply(ExprApplyOp::JoinString(None), None));
    }

    #[test]
    pub fn test_symbol_path_expr1() {
        let mut symbol_path_scope = SymbolPathScope::default();
        let expr1 = ExprValue::Expr(ExprOp::Add, Box::new(ExprValue::LiteralNumber(1)), Box::new(ExprValue::LiteralNumber(2)));
        let expr2 = ExprValue::LiteralString("test".to_owned());

        symbol_path_scope.append_expr(&expr1);
        symbol_path_scope.append_expr(&expr2);

        let expr = symbol_path_scope.join_as_expr(None);
        assert_eq!(expr, ExprValue::Apply(ExprApplyOp::JoinString(None), Some(vec![
            Box::new(ExprValue::Expr(ExprOp::Add, Box::new(ExprValue::LiteralNumber(1)), Box::new(ExprValue::LiteralNumber(2)))),
            Box::new(ExprValue::LiteralString("test".to_owned()))
        ])));
    }

    #[test]
    pub fn test_symbol_path_mixed1() {
        let mut symbol_path_scope = SymbolPathScope::default();
        let expr1 = ExprValue::Expr(ExprOp::Add, Box::new(ExprValue::LiteralNumber(1)), Box::new(ExprValue::LiteralNumber(2)));

        symbol_path_scope.append_expr(&expr1);
        symbol_path_scope.append_str("test");

        let expr = symbol_path_scope.join_as_expr(None);
        assert_eq!(expr, ExprValue::Apply(ExprApplyOp::JoinString(None), Some(vec![
            Box::new(ExprValue::Expr(ExprOp::Add, Box::new(ExprValue::LiteralNumber(1)), Box::new(ExprValue::LiteralNumber(2)))),
            Box::new(ExprValue::LiteralString("test".to_owned()))
        ])));
    }
}