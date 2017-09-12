// #![allow(dead_code)]

use std::iter;
use linked_hash_map::LinkedHashMap;

use parser::*;
use scope::*;
use processing::*;


#[allow(dead_code)]
pub struct ObjectMemberIter<'a: 'b, 'b, I: IntoIterator<Item = &'b str>> {
    // ctx: &'a mut Context,
    expr: &'a ExprValue,
    cur: Option<&'a ExprValue>,
    key: Option<&'b str>,
    iter: <I as iter::IntoIterator>::IntoIter
}

impl<'a: 'b, 'b, I: IntoIterator<Item = &'b str>> ObjectMemberIter<'a, 'b, I>
{
    #[allow(dead_code)]
    pub fn new(expr: &'a ExprValue, iter: I) -> Self {
        ObjectMemberIter {
            // ctx: ctx,
            expr: expr,
            cur: Some(expr),
            key: None,
            iter: iter.into_iter()
        }
    }
}

#[derive(Debug, PartialEq)]
pub enum ObjectMemberState<'a> {
    InterimResult(&'a ExprValue),
    FinalResult(&'a ExprValue)
}

impl<'a: 'b, 'b, I: IntoIterator<Item = &'b str>> Iterator for ObjectMemberIter<'a, 'b, I>
{
    type Item = ObjectMemberState<'a>;

    fn next(&mut self) -> Option<Self::Item> {
        let mut res: Option<&'a ExprValue> = None;
        let mut next_cur: Option<&'a ExprValue> = None;
        let mut next_key: Option<&str> = None;

        // let cur_key = self.next_key.as_ref().map(|s| s.to_owned());

        // Handle first iteration
        if self.key.is_none() {
            if let Some(next_key) = self.iter.next() {
                self.key = Some(next_key);
            }
        }

        // Get the next part in the path which was cached on the last iteration
        if let Some(key) = self.key {

            // If the next value is an object expression
            if let Some(&ExprValue::LiteralObject(Some(ref props))) = self.cur {
                let mut member_ref = props.iter().filter(|p| p.0 == key).take(1);

                if let Some(member_ref) = member_ref.next() {
                    if let Some(ref expr) = member_ref.1 {
                        res = Some(expr);
                    }
                }
            }
        }

        let mut next_key: Option<&str> = None;

        // Prepare next iteration
        if let Some(res) = res {
            if let Some(key) = self.iter.next() {
                next_key = Some(key);
            }

            if next_key.is_none() {
                return Some(ObjectMemberState::FinalResult(res));
            }

            self.key = next_key;
            self.cur = Some(res);

            return Some(ObjectMemberState::InterimResult(res));
        }

        None
    }
}

pub struct MemberResolver<'a: 'b, 'b,  I: IntoIterator<Item = &'b str>> {
    member_iter: ObjectMemberIter<'a, 'b, I>
    // path_iter: I
}

impl<'a : 'b, 'b, I: IntoIterator<Item = &'b str>> MemberResolver<'a, 'b, I> {
    pub fn new_with_parts(expr: &'a ExprValue, parts: I) -> Self {
        let member_iter: ObjectMemberIter<'a, 'b, I> = ObjectMemberIter::new(expr, parts);

        MemberResolver {
            member_iter: member_iter
        }
    }

    pub fn resolve_member(self) -> Option<&'a ExprValue> {

        for res in self.member_iter {
            match res {
                ObjectMemberState::InterimResult(..) => { continue; }
                ObjectMemberState::FinalResult(expr) => { return Some(expr); }
            };
        }
        None
    }
}

pub fn resolve_member_in_expr<'a, 'b>(expr: &'a ExprValue, path: &'b str) -> Option<&'a ExprValue> {
        let member_resolver = MemberResolver::new_with_parts(expr, path.split("."));
        member_resolver.resolve_member()
}


#[cfg(test)]
mod tests {
    use super::*;
    use parser::*;
    use scope::*;


    #[test]
    pub fn test_members_path1() {
        let obj = ExprValue::LiteralObject(Some(vec![
            ("a".into(), Some(ExprValue::LiteralString("x1".into())))
        ]));

        let path: Vec<String> = vec!["a".into()];

        let mut iter = ObjectMemberIter::new(&obj, path.iter().map(|p| p.as_str()));
        
        assert_eq!(iter.next(), Some(ObjectMemberState::FinalResult(&ExprValue::LiteralString("x1".into()))));
    }

    #[test]
    pub fn test_members_path2() {
        let obj = ExprValue::LiteralObject(Some(vec![
            ("a".into(), Some(ExprValue::LiteralObject(Some(vec![
                ("b".into(), Some(ExprValue::LiteralString("y1".into())))
            ]))))
        ]));

        let path: Vec<String> = vec!["a".into(), "b".into()];

        let mut iter = ObjectMemberIter::new(&obj, path.iter().map(|p| p.as_str()));
        
        assert_eq!(iter.next(), Some(ObjectMemberState::InterimResult(&ExprValue::LiteralObject(Some(vec![ ("b".into(), Some(ExprValue::LiteralString("y1".into()))) ])))));
        assert_eq!(iter.next(), Some(ObjectMemberState::FinalResult(&ExprValue::LiteralString("y1".into()))));
    }

    #[test]
    pub fn test_resolve_member_in_expr1() {
        let obj = ExprValue::LiteralObject(Some(vec![
            ("a".into(), Some(ExprValue::LiteralObject(Some(vec![
                ("b".into(), Some(ExprValue::LiteralString("y1".into())))
            ]))))
        ]));

        let path = "a.b";

        let member_resolver = MemberResolver::new_with_parts(&obj, path.split("."));
        let res = member_resolver.resolve_member();
        assert_eq!(res, Some(&ExprValue::LiteralString("y1".into())));
    }

    #[test]
    pub fn test_resolve_member_in_expr2() {
        let obj = ExprValue::LiteralObject(Some(vec![
            ("a".into(), Some(ExprValue::LiteralObject(Some(vec![
                ("b".into(), Some(ExprValue::LiteralString("y1".into())))
            ]))))
        ]));

        let path = "a.b";
        let res = resolve_member_in_expr(&obj, &path);
        assert_eq!(res, Some(&ExprValue::LiteralString("y1".into())));
    }

    #[test]
    pub fn test_resolve_member_in_expr3() {
        let obj = ExprValue::LiteralObject(Some(vec![
            ("a".into(), Some(ExprValue::LiteralObject(Some(vec![
                ("b".into(), Some(ExprValue::LiteralString("y1".into())))
            ]))))
        ]));

        let res = resolve_member_in_expr(&obj, "a.b");
        assert_eq!(res, Some(&ExprValue::LiteralString("y1".into())));
    }

}