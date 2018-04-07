use std::collections::hash_map::{HashMap, Entry};
use std::iter::FromIterator;

use error::*;
use super::*;

pub trait ExpressionVisitor<T> {
    fn visit_all<F: FnMut(&ExpressionValue<T>) -> DocumentProcessingResult<()>>(&mut self, n: &ExpressionValue<T>, f: &mut F) -> DocumentProcessingResult<()>;

    fn visit_all_value_bindings<F: FnMut(Option<&str>, &CommonBindings<T>) -> DocumentProcessingResult<()>>(&mut self, n: &ExpressionValue<T>, f: &mut F) -> DocumentProcessingResult<()>;
}

#[derive(Debug, Default)]
pub struct DefaultExpressionVisitor;

impl<T> ExpressionVisitor<T> for DefaultExpressionVisitor {
    fn visit_all<F: FnMut(&ExpressionValue<T>) -> DocumentProcessingResult<()>>(&mut self, n: &ExpressionValue<T>, f: &mut F) -> DocumentProcessingResult<()> {
        // Visit node first
        f(n)?;

        // Visit children
        match *n {
            ExpressionValue::Expression(Expression::Composite(CompositeValue::ObjectValue(ObjectValue(Some(box ref children))))) => {
                for child in children {
                    self.visit_all(child.value(), f)?;
                }
            }

            ExpressionValue::Expression(Expression::Composite(CompositeValue::ArrayValue(ArrayValue(Some(box ref children))))) => {
                for child in children {
                    self.visit_all(child.value(), f)?;
                }
            }

            _ => {}
        };
        Ok(())
    }

    fn visit_all_value_bindings<F: FnMut(Option<&str>, &CommonBindings<T>) -> DocumentProcessingResult<()>>(&mut self, n: &ExpressionValue<T>, f: &mut F) -> DocumentProcessingResult<()> {
        // Visit children
        match *n {
            ExpressionValue::Expression(Expression::Composite(CompositeValue::ObjectValue(ObjectValue(Some(box ref children))))) => {
                for child in children {
                    if let ExpressionValue::Binding(ref binding, _) = child.value() {
                        f(Some(child.key()), binding)?;
                        continue;
                    };

                    self.visit_all_value_bindings(child.value(), f)?;
                }
            }

            ExpressionValue::Expression(Expression::Composite(CompositeValue::ArrayValue(ArrayValue(Some(box ref children))))) => {
                for child in children {
                    if let ExpressionValue::Binding(ref binding, _) = child.value() {
                        f(None, binding)?;
                        continue;
                    };

                    self.visit_all_value_bindings(child.value(), f)?;
                }
            }

            _ => {}
        };

        Ok(())
    }
}

pub trait ExpressionValueBindingExt<T> {
    fn merge_all_value_bindings_into(&self, into: &mut HashMap<String, PropValue<T>>) -> DocumentProcessingResult<()>;
}

impl<T: Clone> ExpressionValueBindingExt<T> for ExpressionValue<T> {
    fn merge_all_value_bindings_into(&self, into: &mut HashMap<String, PropValue<T>>) -> DocumentProcessingResult<()> {
        let mut visitor = DefaultExpressionVisitor;

        visitor.visit_all_value_bindings(self, &mut |key, binding| {
            if let Some(key) = key {
                let mut suffix = 0;
                while suffix < 6 {
                    let dest_key = if suffix > 0 { format!("{}{}", key, suffix) } else { key.to_owned() };
                    let entry = into.entry(dest_key.clone());
                    match entry {
                        Entry::Occupied(_) => {
                            suffix += 1;
                            continue;
                        }

                        Entry::Vacant(v) => {
                            let prop = PropValue::new(dest_key, ExpressionValue::Binding(binding.to_owned(), Default::default()), None);
                            v.insert(prop);
                            break;
                        }
                    }
                };
            };
            Ok(())
        })
    }
}

impl<T> FromIterator<PropValue<T>> for ObjectValue<T> {
    fn from_iter<I: IntoIterator<Item = PropValue<T>>>(iter: I) -> Self {
        let props: Vec<PropValue<T>> = iter.into_iter().collect();
        ObjectValue(Some(Box::new(props)))
    }
}

impl<T> Into<ExpressionValue<T>> for ObjectValue<T> {
    fn into(self) -> ExpressionValue<T> {
        ExpressionValue::Expression(Expression::Composite(CompositeValue::ObjectValue(self)))
    }
}

#[cfg(test)]
mod tests {
    use std::collections::HashMap;
    use expressions::*;
    use expressions::expression::PropValue;
    use super::*;

    fn do_test_visit_all_value_bindings() -> DocumentProcessingResult<()> {
        let obj: ExpressionValue<ProcessedExpression> = ExpressionValue::Expression(Expression::Composite(CompositeValue::ObjectValue(ObjectValue(Some(Box::new(vec![
            PropValue::new("entry".to_owned(), ExpressionValue::Expression(Expression::Composite(CompositeValue::ObjectValue(ObjectValue(Some(Box::new(vec![
                PropValue::new("name".to_owned(), ExpressionValue::Binding(CommonBindings::NamedElementBoundValue("abc".to_owned(), Default::default()), Default::default()), None),
                PropValue::new("years".to_owned(), ExpressionValue::Binding(CommonBindings::NamedElementBoundValue("def".to_owned(), Default::default()), Default::default()), None)
            ])))))), None),
            PropValue::new("task".to_owned(), ExpressionValue::Expression(Expression::Composite(CompositeValue::ObjectValue(ObjectValue(Some(Box::new(vec![
                PropValue::new("name".to_owned(), ExpressionValue::Binding(CommonBindings::NamedElementBoundValue("ghi".to_owned(), Default::default()), Default::default()), None),
                PropValue::new("hours_worked".to_owned(), ExpressionValue::Binding(CommonBindings::NamedElementBoundValue("jkl".to_owned(), Default::default()), Default::default()), None)
            ])))))), None)
        ]))))));

        let mut results: Vec<(Option<String>, CommonBindings<ProcessedExpression>)> = Default::default();
        let mut visitor = DefaultExpressionVisitor;

        visitor.visit_all_value_bindings(&obj, &mut |key, binding| {
            let key = key.map(|s| s.to_owned());
            let binding = binding.to_owned();

            results.push((key, binding));
            Ok(())
        })?;

        let expected: Vec<(Option<String>, CommonBindings<ProcessedExpression>)> = vec![
                (Some("name".to_owned()), CommonBindings::NamedElementBoundValue("abc".to_owned(), Default::default())),
                (Some("years".to_owned()), CommonBindings::NamedElementBoundValue("def".to_owned(), Default::default())),
                (Some("name".to_owned()), CommonBindings::NamedElementBoundValue("ghi".to_owned(), Default::default())),
                (Some("hours_worked".to_owned()), CommonBindings::NamedElementBoundValue("jkl".to_owned(), Default::default()))
        ];

        // let expected: Vec<(Option<String>, ExpressionValue<ProcessedExpression>)> = vec![
        //         (Some("name".to_owned()), ExpressionValue::Binding(CommonBindings::NamedElementBoundValue("abc".to_owned(), Default::default()), Default::default())),
        //         (Some("years".to_owned()), ExpressionValue::Binding(CommonBindings::NamedElementBoundValue("def".to_owned(), Default::default()), Default::default())),
        //         (Some("name".to_owned()), ExpressionValue::Binding(CommonBindings::NamedElementBoundValue("ghi".to_owned(), Default::default()), Default::default())),
        //         (Some("hours_worked".to_owned()), ExpressionValue::Binding(CommonBindings::NamedElementBoundValue("jkl".to_owned(), Default::default()), Default::default()))
        // ];

        // let expected: Vec<(Option<String>, PropValue<ProcessedExpression>)> = vec![
        //         (Some("name".to_owned()), PropValue::new("name".to_owned(), ExpressionValue::Binding(CommonBindings::NamedElementBoundValue("abc".to_owned(), Default::default()), Default::default()), None)),
        //         (Some("years".to_owned()), PropValue::new("years".to_owned(), ExpressionValue::Binding(CommonBindings::NamedElementBoundValue("def".to_owned(), Default::default()), Default::default()), None)),
        //         (Some("name".to_owned()), PropValue::new("name".to_owned(), ExpressionValue::Binding(CommonBindings::NamedElementBoundValue("ghi".to_owned(), Default::default()), Default::default()), None)),
        //         (Some("hours_worked".to_owned()), PropValue::new("hours_worked".to_owned(), ExpressionValue::Binding(CommonBindings::NamedElementBoundValue("jkl".to_owned(), Default::default()), Default::default()), None))
        // ];

        assert_eq!(expected, results);
        Ok(())
    }

    #[test]
    fn test_visit_all_value_bindings() {
        assert!(do_test_visit_all_value_bindings().is_ok());
    }

    #[test]
    fn test_object_from_props() {
        use super::*;
        use expressions::expression::*;

        let props: Vec<PropValue<ProcessedExpression>> = vec![
                PropValue::new("name".to_owned(), ExpressionValue::Binding(CommonBindings::NamedElementBoundValue("abc".to_owned(), Default::default()), Default::default()), None),
                PropValue::new("years".to_owned(), ExpressionValue::Binding(CommonBindings::NamedElementBoundValue("def".to_owned(), Default::default()), Default::default()), None)
        ];

        let obj: ObjectValue<ProcessedExpression> = props.into_iter().collect();

        let expected: ObjectValue<ProcessedExpression> = ObjectValue(Some(Box::new(vec![
                PropValue::new("name".to_owned(), ExpressionValue::Binding(CommonBindings::NamedElementBoundValue("abc".to_owned(), Default::default()), Default::default()), None),
                PropValue::new("years".to_owned(), ExpressionValue::Binding(CommonBindings::NamedElementBoundValue("def".to_owned(), Default::default()), Default::default()), None)
        ])));

        assert_eq!(expected, obj);
    }

    #[cfg(test)]
    fn do_test_merge_all_value_bindings() -> DocumentProcessingResult<()> {
        use super::*;

        let action1: ExpressionValue<ProcessedExpression> = ExpressionValue::Expression(Expression::Composite(CompositeValue::ObjectValue(ObjectValue(Some(Box::new(vec![
            PropValue::new("entry".to_owned(), ExpressionValue::Expression(Expression::Composite(CompositeValue::ObjectValue(ObjectValue(Some(Box::new(vec![
                PropValue::new("name".to_owned(), ExpressionValue::Binding(CommonBindings::NamedElementBoundValue("abc".to_owned(), Default::default()), Default::default()), None),
                PropValue::new("years".to_owned(), ExpressionValue::Binding(CommonBindings::NamedElementBoundValue("def".to_owned(), Default::default()), Default::default()), None)
            ])))))), None),
        ]))))));

        let action2: ExpressionValue<ProcessedExpression> = ExpressionValue::Expression(Expression::Composite(CompositeValue::ObjectValue(ObjectValue(Some(Box::new(vec![
            PropValue::new("task".to_owned(), ExpressionValue::Expression(Expression::Composite(CompositeValue::ObjectValue(ObjectValue(Some(Box::new(vec![
                PropValue::new("name".to_owned(), ExpressionValue::Binding(CommonBindings::NamedElementBoundValue("ghi".to_owned(), Default::default()), Default::default()), None),
                PropValue::new("hours_worked".to_owned(), ExpressionValue::Binding(CommonBindings::NamedElementBoundValue("jkl".to_owned(), Default::default()), Default::default()), None)
            ])))))), None)
        ]))))));

        let mut merged: HashMap<String, PropValue<ProcessedExpression>> = Default::default();
        action1.merge_all_value_bindings_into(&mut merged)?;
        action2.merge_all_value_bindings_into(&mut merged)?;

        let mut merged_props: Vec<PropValue<ProcessedExpression>> = merged.values().cloned().collect();
        merged_props.sort_by(|a, b| a.key().cmp(b.key()));

        let merged: ObjectValue<ProcessedExpression> = merged_props.into_iter().collect();
        let merged = ExpressionValue::Expression(Expression::Composite(CompositeValue::ObjectValue(merged)));

        let expected = ExpressionValue::Expression(Expression::Composite(CompositeValue::ObjectValue(ObjectValue(Some(Box::new(vec![
                PropValue::new("hours_worked".to_owned(), ExpressionValue::Binding(CommonBindings::NamedElementBoundValue("jkl".to_owned(), Default::default()), Default::default()), None),
                PropValue::new("name".to_owned(), ExpressionValue::Binding(CommonBindings::NamedElementBoundValue("abc".to_owned(), Default::default()), Default::default()), None),
                PropValue::new("name1".to_owned(), ExpressionValue::Binding(CommonBindings::NamedElementBoundValue("ghi".to_owned(), Default::default()), Default::default()), None),
                PropValue::new("years".to_owned(), ExpressionValue::Binding(CommonBindings::NamedElementBoundValue("def".to_owned(), Default::default()), Default::default()), None)
        ]))))));

        assert_eq!(expected, merged);
        Ok(())
    }

    #[test]
    fn test_merge_all_value_bindings() {
        assert!(do_test_merge_all_value_bindings().is_ok());
    }
}
