pub mod html;
pub mod js;
pub mod page_data;
pub mod page_templates;

pub use self::html::*;
pub use self::js::*;
pub use self::page_data::*;
pub use self::page_templates::*;

use isymtope_ast_common::*;

#[derive(Debug)]
pub struct  ActionOpOutput<T>(pub Option<String>, pub ActionOp<T>);

#[derive(Debug)]
pub struct ElementEventBindingOutput<T>(pub Option<String>, String, pub ObjectValue<T>);

impl<'a> Into<ElementEventBindingOutput<ProcessedExpression>> for &'a ElementEventBindingName<ProcessedExpression> {
    fn into(self) -> ElementEventBindingOutput<ProcessedExpression> {
        let event = self.event();
        let name = event.name().map(|s| s.to_owned());
        let key = self.key().to_owned();

        let actions: Vec<_> = event
            .actions()
            .map(|v| v.into_iter().collect())
            .unwrap_or_default();
        eprintln!("Event actions: {:?}", &actions);

        let action_props: ObjectValue<ProcessedExpression> = actions
            .into_iter()
            .enumerate()
            .flat_map(|(ctr, action)| {
                let key = format!("a{}", ctr);
                let prop = match *action {
                    ActionOp::DispatchAction(_, Some(box ref props), _)
                    | ActionOp::DispatchActionTo(_, Some(box ref props), _, _) => {
                        let props: ObjectValue<ProcessedExpression> = props.iter().cloned().collect();
                        Some(PropValue::new(key, props.into(), None))
                    }
                    ActionOp::Navigate(ref path, _) => {
                        // let props: ObjectValue<ProcessedExpression> = vec!["path".to_owned(), path.to_owned(), None)].into_iter().collect();
                        Some(PropValue::new(key, path.to_owned(), None))
                    }
                    _ => None
                };
                prop
            })
            .collect();

        ElementEventBindingOutput(name, key, action_props)
    }
}
