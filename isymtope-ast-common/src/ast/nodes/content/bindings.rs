use std::marker::PhantomData;
use std::hash::Hash;
use std::fmt::Debug;

use error::*;
use util::*;
use traits::*;
use expressions::*;
use ast::*;

#[allow(dead_code)]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum ElementBinding<T> {
    Event(ElementEventBinding<T>, PhantomData<T>),
    Value(ElementValueBinding<T>, PhantomData<T>),
}

impl TryProcessFrom<ElementBinding<SourceExpression>> for ElementBinding<ProcessedExpression> {
    fn try_process_from(
        src: &ElementBinding<SourceExpression>,
        ctx: &mut ProcessingContext,
    ) -> DocumentProcessingResult<Self> {
        Ok(match *src {
            ElementBinding::Event(ref event_binding, _) => ElementBinding::Event(
                TryProcessFrom::try_process_from(event_binding, ctx)?,
                Default::default(),
            ),
            ElementBinding::Value(ref value_binding, _) => ElementBinding::Value(
                TryProcessFrom::try_process_from(value_binding, ctx)?,
                Default::default(),
            ),
        })
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct ElementEventBinding<T>(Option<String>, FormalParams<T>, Option<Vec<ActionOp<T>>>);

impl<T> ElementEventBinding<T> {
    pub fn new(
        event: Option<String>,
        params: FormalParams<T>,
        action_ops: Option<Vec<ActionOp<T>>>,
    ) -> Self {
        ElementEventBinding(event, params, action_ops)
    }

    pub fn name(&self) -> Option<&str> {
        self.0.as_ref().map(|s| s.as_str())
    }

    pub fn actions<'a>(&'a self) -> Option<impl Iterator<Item = &'a ActionOp<T>>> {
        self.2.as_ref().map(|v| v.iter())
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct ElementEventProp<T>(String, ExpressionValue<T>, String);

impl TryProcessFrom<ElementEventProp<SourceExpression>> for ElementEventProp<ProcessedExpression> {
    fn try_process_from(
        src: &ElementEventProp<SourceExpression>,
        ctx: &mut ProcessingContext,
    ) -> DocumentProcessingResult<Self> {
         eprintln!(
            "TryProcess ElementEventProp -> ElementEventProp: src: {:?}",
            src
        );

       let prop: ExpressionValue<ProcessedExpression> =
            TryProcessFrom::try_process_from(&src.1, ctx)?;
        Ok(ElementEventProp(src.0.to_owned(), prop, src.2.to_owned()))
    }
}

impl<T> TryEvalFrom<ElementEventProp<T>> for ElementEventProp<OutputExpression>
where
    ExpressionValue<OutputExpression>: TryEvalFrom<ExpressionValue<T>>,
    T: Debug + Hash + Eq,
{
    fn try_eval_from(
        src: &ElementEventProp<T>,
        ctx: &mut OutputContext,
    ) -> DocumentProcessingResult<Self> {
         eprintln!(
            "TryEval ElementEventProp -> ElementEventProp: src: {:?}",
            src
        );

        let prop: ExpressionValue<OutputExpression> = TryEvalFrom::try_eval_from(&src.1, ctx)?;
        Ok(ElementEventProp(src.0.to_owned(), prop, src.2.to_owned()))
    }
}

fn create_element_event_props<T>(event: &ElementEventBinding<T>) -> Vec<ElementEventProp<T>>
where
    T: Clone + Debug,
{
    let actions: Vec<_> = event.actions().map(|v| v.collect()).unwrap_or_default();

    let event_prop_aliases: Vec<_> = actions
        .iter()
        .flat_map(|action| {
            let dispatch_iter: Vec<_> = match **action {
                ActionOp::DispatchAction(_, Some(box ref props), _)
                | ActionOp::DispatchActionTo(_, Some(box ref props), _, _) => Some(
                    props
                        .into_iter()
                        .map(|prop| (prop.key().to_owned(), prop.value().to_owned())),
                ),
                _ => None,
            }.into_iter()
                .flat_map(|v| v)
                .collect();

            let navigate_iter: Vec<_> = match **action {
                ActionOp::Navigate(ref path, _) => {
                    Some(vec![("path".to_owned(), path.to_owned())].into_iter())
                }
                _ => None,
            }.into_iter()
                .flat_map(|v| v)
                .collect();
            dispatch_iter.into_iter().chain(navigate_iter.into_iter())
        })
        .collect();
    eprintln!("[bindings] event_prop_aliases: {:?}", event_prop_aliases);

    let event_prop_aliases: Vec<_> = event_prop_aliases
        .into_iter()
        .filter_map(|(alias, prop)| match prop {
            ExpressionValue::Expression(Expression::Path(ref path_value, _)) => Some(
                ElementEventProp(alias, prop.to_owned(), path_value.complete_string()),
            ),
            ExpressionValue::Binding(CommonBindings::NamedComponentProp(ref s, _), _) => Some(
                ElementEventProp(alias, prop.to_owned(), format!("props.{}", s)),
            ),
            ExpressionValue::Binding(CommonBindings::CurrentElementValue(_), _) => Some(
                ElementEventProp(alias, prop.to_owned(), format!("_event.target.value"))
            ),
            ExpressionValue::Binding(CommonBindings::NamedElementBoundValue(ref element_key, _), _) => Some(
                ElementEventProp(alias, prop.to_owned(), format!("document.querySelector(\"[key = '\" + props.key + \"xxx.{}']\").value", element_key))
            ),
            // ExpressionValue::Expression(Expression::Composite(CompositeValue::ObjectValue(Some(props)))) => {
            // }
            _ => None,
        })
        .collect();
    eprintln!("[bindings] event_prop_aliases: {:?}", event_prop_aliases);

    event_prop_aliases
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct ElementEventBindingName<T>(
    String,
    String,
    ElementEventBinding<T>,
    bool,
    Vec<ElementEventProp<T>>,
);

impl<T: Clone + Debug> ElementEventBindingName<T> {
    pub fn create(event: ElementEventBinding<T>) -> Self {
        let (event_name, is_enterkey) = match event.name() {
            Some("enterkey") => ("keypress".into(), true),
            Some(s) => (format!("{}", s), false),
            _ => ("click".into(), false),
        };

        debug!("[event binding] generating binding name");
        let binding_name = allocate_element_key();
        debug!("[event binding] binding_name: {}", binding_name);
        let event_props = create_element_event_props(&event);

        ElementEventBindingName(binding_name, event_name, event, is_enterkey, event_props)
    }

    pub fn key(&self) -> String {
        let name = self.name();
        let event_name = self.event_name();

        format!("event_{}x_{}", name, event_name)
    }

    pub fn name(&self) -> &str {
        &self.0
    }

    pub fn event_name(&self) -> &str {
        &self.1
    }

    pub fn event(&self) -> &ElementEventBinding<T> {
        &self.2
    }

    pub fn is_enterkey(&self) -> bool {
        self.3
    }

    pub fn props(&self) -> impl Iterator<Item = (&str, &ExpressionValue<T>, &str)> {
        self.4.iter().map(|p| (p.0.as_str(), &p.1, p.2.as_str()))
    }
}

impl TryProcessFrom<ElementEventBinding<SourceExpression>>
    for ElementEventBinding<ProcessedExpression>
{
    fn try_process_from(
        src: &ElementEventBinding<SourceExpression>,
        ctx: &mut ProcessingContext,
    ) -> DocumentProcessingResult<Self> {
         eprintln!(
            "TryProcess ElementEventBinding -> ElementEventBinding: src: {:?}",
            src
        );

        let formal_params: FormalParams<ProcessedExpression> =
            TryProcessFrom::try_process_from(&src.1, ctx)?;

        ctx.push_child_scope_with_environment(ProcessingScopeEnvironment::ElementActions);
        let action_ops: Option<Vec<ActionOp<ProcessedExpression>>> =
            TryProcessFrom::try_process_from(&src.2, ctx)?;
        ctx.pop_scope();

        Ok(ElementEventBinding(
            src.0.clone(),
            formal_params,
            action_ops,
        ))
    }
}

impl TryProcessFrom<ElementEventBindingName<SourceExpression>>
    for ElementEventBindingName<ProcessedExpression>
{
    fn try_process_from(
        src: &ElementEventBindingName<SourceExpression>,
        ctx: &mut ProcessingContext,
    ) -> DocumentProcessingResult<Self> {
         eprintln!(
            "TryProcess ElementEventBindingName -> ElementEventBindingName: src: {:?}",
            src
        );

        let name = src.name().to_owned();
        let event_name = src.event_name().to_owned();
        let event: ElementEventBinding<ProcessedExpression> =
            TryProcessFrom::try_process_from(src.event(), ctx)?;
        let is_enterkey = src.is_enterkey();

        let props: Vec<ElementEventProp<ProcessedExpression>> =
            TryProcessFrom::try_process_from(&src.4, ctx)?;

        Ok(ElementEventBindingName(
            name,
            event_name,
            event,
            is_enterkey,
            props,
        ))
    }
}

impl<T> TryEvalFrom<ElementEventBindingName<T>> for ElementEventBindingName<OutputExpression>
where
    ExpressionValue<OutputExpression>: TryEvalFrom<ExpressionValue<T>>,
    T: Debug + Hash + Eq + Clone,
{
    fn try_eval_from(
        src: &ElementEventBindingName<T>,
        ctx: &mut OutputContext,
    ) -> DocumentProcessingResult<Self> {
        let name = src.name().to_owned();
        let event_name = src.event_name().to_owned();
        let event: ElementEventBinding<OutputExpression> =
            TryEvalFrom::try_eval_from(src.event(), ctx)?;
        let is_enterkey = src.is_enterkey();

        let props: Vec<ElementEventProp<OutputExpression>> =
            TryEvalFrom::try_eval_from(&src.4, ctx)?;

        Ok(ElementEventBindingName(
            name,
            event_name,
            event,
            is_enterkey,
            props,
        ))
    }
}

impl<T> TryEvalFrom<ElementEventBinding<T>> for ElementEventBinding<OutputExpression>
where
    ExpressionValue<OutputExpression>: TryEvalFrom<ExpressionValue<T>>,
    T: Debug + Hash + Eq + Clone,
{
    fn try_eval_from(
        src: &ElementEventBinding<T>,
        ctx: &mut OutputContext,
    ) -> DocumentProcessingResult<Self> {
        let formal_params: FormalParams<OutputExpression> =
            TryEvalFrom::try_eval_from(&src.1, ctx)?;
        let action_ops: Option<Vec<ActionOp<OutputExpression>>> =
            TryEvalFrom::try_eval_from(&src.2, ctx)?;

        Ok(ElementEventBinding(
            src.0.clone(),
            formal_params,
            action_ops,
        ))
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct ElementValueBinding<T>(
    ExpressionValue<T>,
    Option<String>,
    Option<ExpressionValue<T>>,
);

impl<T> ElementValueBinding<T> {
    pub fn new(
        e: ExpressionValue<T>,
        alias: Option<String>,
        read_expr: Option<ExpressionValue<T>>,
    ) -> Self {
        ElementValueBinding(e, alias, read_expr)
    }

    pub fn expr(&self) -> &ExpressionValue<T> {
        &self.0
    }
    pub fn ident(&self) -> Option<&str> {
        self.1.as_ref().map(|s| s.as_str())
    }
    pub fn read_expr(&self) -> Option<&ExpressionValue<T>> {
        self.2.as_ref()
    }
}

impl<I, O> TryProcessFrom<ElementValueBinding<I>> for ElementValueBinding<O>
where
    ExpressionValue<O>: TryProcessFrom<ExpressionValue<I>>,
    I: ::std::fmt::Debug,
    O: ::std::fmt::Debug,
{
    fn try_process_from(
        src: &ElementValueBinding<I>,
        ctx: &mut ProcessingContext,
    ) -> DocumentProcessingResult<Self> {
        eprintln!(
            "TryProcess ElementValueBinding -> ElementValueBinding: src: {:?}",
            src
        );

        let read_expr = TryProcessFrom::try_process_from(&src.2, ctx)?;

        Ok(ElementValueBinding(
            TryProcessFrom::try_process_from(&src.0, ctx)?,
            src.1.to_owned(),
            read_expr,
        ))
    }
}

impl<I, O> TryEvalFrom<ElementValueBinding<I>> for ElementValueBinding<O>
where
    ExpressionValue<O>: TryEvalFrom<ExpressionValue<I>>,
    I: ::std::fmt::Debug,
    O: ::std::fmt::Debug,
{
    fn try_eval_from(
        src: &ElementValueBinding<I>,
        ctx: &mut OutputContext,
    ) -> DocumentProcessingResult<Self> {
        eprintln!(
            "TryEval ElementValueBinding -> ElementValueBinding: src: {:?}",
            src
        );

        let read_expr = TryEvalFrom::try_eval_from(&src.2, ctx)?;

        Ok(ElementValueBinding(
            TryEvalFrom::try_eval_from(&src.0, ctx)?,
            src.1.to_owned(),
            read_expr,
        ))
    }
}
