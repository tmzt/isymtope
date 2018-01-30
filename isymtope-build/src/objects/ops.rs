use std::marker::PhantomData;
use std::hash::Hash;
use std::fmt::Debug;
use std::collections::HashMap;

use error::*;
use traits::*;
use expressions::*;
use ast::*;
use output::*;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct ElementDescriptor<T>(
    String,
    String,
    Vec<ElementPropValue<T>>,
    Option<Vec<ElementEventBindingName<T>>>,
    Option<ElementValueBinding<T>>,
    bool,
);

impl<T> ElementDescriptor<T> {
    pub fn new(
        tag: String,
        key: String,
        props: Vec<ElementPropValue<T>>,
        event_bindings: Option<Vec<ElementEventBindingName<T>>>,
        value_binding: Option<ElementValueBinding<T>>,
        is_map: bool,
    ) -> Self {
        ElementDescriptor(tag, key, props, event_bindings, value_binding, is_map)
    }

    pub fn tag(&self) -> &str {
        self.0.as_str()
    }
    pub fn key(&self) -> &str {
        self.1.as_str()
    }

    pub fn props<'a>(&'a self) -> impl Iterator<Item = &'a ElementPropValue<T>> {
        self.2.iter()
    }

    pub fn events<'a>(&'a self) -> Option<impl Iterator<Item = &'a ElementEventBindingName<T>>> {
        self.3.as_ref().map(|v| v.iter())
    }

    pub fn value_binding(&self) -> Option<&ElementValueBinding<T>> {
        self.4.as_ref()
    }

    pub fn is_map(&self) -> bool {
        self.5
    }
}

impl TryProcessFrom<ElementDescriptor<SourceExpression>>
    for ElementDescriptor<ProcessedExpression>
{
    fn try_process_from(
        src: &ElementDescriptor<SourceExpression>,
        ctx: &mut ProcessingContext,
    ) -> DocumentProcessingResult<Self> {
        let props: Vec<ElementPropValue<ProcessedExpression>> =
            TryProcessFrom::try_process_from(&src.2, ctx)?;
        let event_bindings: Option<Vec<ElementEventBindingName<ProcessedExpression>>> =
            TryProcessFrom::try_process_from(&src.3, ctx)?;
        let value_binding: Option<ElementValueBinding<ProcessedExpression>> =
            TryProcessFrom::try_process_from(&src.4, ctx)?;
        let map_key = src.5.to_owned();

        Ok(ElementDescriptor(
            src.0.to_owned(),
            src.1.to_owned(),
            props,
            event_bindings,
            value_binding,
            map_key,
        ))
    }
}

// impl<I, O> TryEvalFrom<ElementDescriptor<I>> for ElementDescriptor<O> where ExpressionValue<O>: TryEvalFrom<ExpressionValue<I>>, I: ::std::fmt::Debug, O: ::std::fmt::Debug {
//     fn try_eval_from(src: &ElementDescriptor<I>, ctx: &mut OutputContext<ProcessedExpression>) -> DocumentProcessingResult<Self> {
//         let props: Vec<ElementPropValue<O>> = TryEvalFrom::try_eval_from(&src.2, ctx)?;
//         let event_bindings: Option<Vec<ElementEventBindingName<O>>> = TryEvalFrom::try_eval_from(&src.3, ctx)?;
//         let value_binding: Option<ElementValueBinding<O>> = TryEvalFrom::try_eval_from(&src.4, ctx)?;

//         Ok(ElementDescriptor(src.0.to_owned(), src.1.to_owned(), props, event_bindings, value_binding))
//     }
// }

impl<T> TryEvalFrom<ElementDescriptor<T>> for ElementDescriptor<OutputExpression>
where
    ExpressionValue<OutputExpression>: TryEvalFrom<ExpressionValue<T>>,
    T: Debug + Hash + Eq + Clone,
{
    fn try_eval_from(
        src: &ElementDescriptor<T>,
        ctx: &mut OutputContext<ProcessedExpression>,
    ) -> DocumentProcessingResult<Self> {
        eprintln!(
            "TryEval ElementDescriptor -> ElementDescriptor src: {:?}",
            src
        );

        let props: Vec<ElementPropValue<OutputExpression>> =
            TryEvalFrom::try_eval_from(&src.2, ctx)?;
        eprintln!(
            "TryEval ElementDescriptor -> ElementDescriptor props: {:?}",
            props
        );

        let event_bindings: Option<Vec<ElementEventBindingName<OutputExpression>>> =
            TryEvalFrom::try_eval_from(&src.3, ctx)?;
        let value_binding: Option<ElementValueBinding<OutputExpression>> =
            TryEvalFrom::try_eval_from(&src.4, ctx)?;
        let map_key = src.5.to_owned();

        Ok(ElementDescriptor(
            src.0.to_owned(),
            src.1.to_owned(),
            props,
            event_bindings,
            value_binding,
            map_key,
        ))
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct ComponentInstanceDescriptor<T>(
    ElementDescriptor<T>,
    Option<String>,
    Option<Vec<ElementPropValue<T>>>,
);

impl<T> ComponentInstanceDescriptor<T> {
    pub fn new(
        desc: ElementDescriptor<T>,
        s: Option<String>,
        component_props: Option<Vec<ElementPropValue<T>>>,
    ) -> Self {
        ComponentInstanceDescriptor(desc, s, component_props)
    }

    pub fn desc(&self) -> &ElementDescriptor<T> {
        &self.0
    }

    pub fn merge_props(self, new_props: Vec<ElementPropValue<T>>) -> Self {
        let element_desc = self.0;
        let s = self.1;

        let props: Vec<_> = self.2.map(|v| v.into_iter().collect()).unwrap_or_default();

        let props: HashMap<String, ElementPropValue<T>> = props
            .into_iter()
            .chain(new_props.into_iter())
            .map(|m| (m.name().to_owned(), m))
            .collect();

        let props: Vec<_> = props.into_iter().map(|(_, p)| p).collect();

        ComponentInstanceDescriptor(element_desc, s, Some(props))
    }

    pub fn tag(&self) -> &str {
        self.0.tag()
    }
    pub fn key(&self) -> &str {
        self.0.key()
    }

    pub fn props<'a>(&'a self) -> Option<impl Iterator<Item = &'a ElementPropValue<T>>> {
        self.2.as_ref().map(|v| v.iter())
    }
}

impl TryProcessFrom<ComponentInstanceDescriptor<SourceExpression>>
    for ComponentInstanceDescriptor<ProcessedExpression>
{
    fn try_process_from(
        src: &ComponentInstanceDescriptor<SourceExpression>,
        ctx: &mut ProcessingContext,
    ) -> DocumentProcessingResult<Self> {
        eprintln!(
            "TryProcess ComponentInstanceDescriptor -> ComponentInstanceDescriptor src: {:?}",
            src
        );

        let desc: ElementDescriptor<ProcessedExpression> =
            TryProcessFrom::try_process_from(&src.0, ctx)?;
        let component_props: Option<Vec<ElementPropValue<ProcessedExpression>>> =
            TryProcessFrom::try_process_from(&src.2, ctx)?;

        Ok(ComponentInstanceDescriptor(
            desc,
            src.1.to_owned(),
            component_props,
        ))
    }
}

impl TryEvalFrom<ComponentInstanceDescriptor<ProcessedExpression>>
    for ComponentInstanceDescriptor<OutputExpression>
{
    fn try_eval_from(
        src: &ComponentInstanceDescriptor<ProcessedExpression>,
        ctx: &mut OutputContext<ProcessedExpression>,
    ) -> DocumentProcessingResult<Self> {
        eprintln!(
            "TryEval ComponentInstanceDescriptor -> ComponentInstanceDescriptor src: {:?}",
            src
        );

        let component_props: Option<Vec<ElementPropValue<OutputExpression>>> =
            TryEvalFrom::try_eval_from(&src.2, ctx)?;
        eprintln!("TryEval ComponentInstanceDescriptor -> ComponentInstanceDescriptor component_props: {:?}", component_props);

        ctx.push_child_scope();

        if let Some(ref component_props) = component_props {
            for prop in component_props {
                let name = prop.name().to_owned();
                let expr = prop.expr();
                let binding = CommonBindings::NamedComponentProp(name.clone(), Default::default());

                eprintln!("TryEval ComponentInstanceDescriptor -> ComponentInstanceDescriptor binding ident [{}] to [{:?}]", name, binding);
                let expr: ExpressionValue<OutputExpression> =
                    TryEvalFrom::try_eval_from(expr, ctx)?;

                eprintln!("TryEval ComponentInstanceDescriptor -> ComponentInstanceDescriptor adding binding [{}] with value [{:?}]", name, binding);
                ctx.bind_value(binding, expr)?;
            }
        }

        let desc: ElementDescriptor<OutputExpression> = TryEvalFrom::try_eval_from(&src.0, ctx)?;

        ctx.pop_scope();

        Ok(ComponentInstanceDescriptor(
            desc,
            src.1.to_owned(),
            component_props,
        ))
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum ElementOp<T> {
    ElementOpen(ElementDescriptor<T>, PhantomData<T>),
    ElementVoid(ElementDescriptor<T>, PhantomData<T>),

    ElementClose(String),
    WriteValue(ExpressionValue<T>, String),

    SkipNode,

    InstanceComponent(ComponentInstanceDescriptor<T>, PhantomData<T>),
    MapInstanceComponent(
        ComponentInstanceDescriptor<T>,
        Option<String>,
        ExpressionValue<T>,
        PhantomData<T>,
    ),

    StartBlock(String),
    EndBlock(String),
    MapCollection(String, Option<String>, ExpressionValue<T>, PhantomData<T>),
}

impl TryProcessFrom<ElementOp<SourceExpression>> for ElementOp<ProcessedExpression> {
    fn try_process_from(
        src: &ElementOp<SourceExpression>,
        ctx: &mut ProcessingContext,
    ) -> DocumentProcessingResult<Self> {
        match *src {
            ElementOp::ElementOpen(ref desc, _) => Ok(ElementOp::ElementOpen(
                TryProcessFrom::try_process_from(desc, ctx)?,
                Default::default(),
            )),
            ElementOp::ElementVoid(ref desc, _) => Ok(ElementOp::ElementOpen(
                TryProcessFrom::try_process_from(desc, ctx)?,
                Default::default(),
            )),
            ElementOp::ElementClose(ref s) => Ok(ElementOp::ElementClose(s.to_owned())),

            ElementOp::WriteValue(ref e, ref s) => {
                eprintln!("TryProcess ElementOp -> ElementOp WriteValue: e: {:?}", e);
                Ok(ElementOp::WriteValue(
                    TryProcessFrom::try_process_from(e, ctx)?,
                    s.to_owned(),
                ))
            }

            ElementOp::SkipNode => Ok(ElementOp::SkipNode),

            ElementOp::StartBlock(ref s) => Ok(ElementOp::StartBlock(s.to_owned())),
            ElementOp::EndBlock(ref s) => Ok(ElementOp::EndBlock(s.to_owned())),

            ElementOp::MapCollection(ref s, ref k, ref e, _) => Ok(ElementOp::MapCollection(
                s.to_owned(),
                k.as_ref().map(|s| s.to_owned()),
                TryProcessFrom::try_process_from(e, ctx)?,
                Default::default(),
            )),

            ElementOp::InstanceComponent(ref comp_desc, _) => Ok(ElementOp::InstanceComponent(
                TryProcessFrom::try_process_from(comp_desc, ctx)?,
                Default::default(),
            )),

            ElementOp::MapInstanceComponent(ref comp_desc, ref item_key, ref coll, _) => {
                let comp_desc: ComponentInstanceDescriptor<ProcessedExpression> =
                    TryProcessFrom::try_process_from(comp_desc, ctx)?;
                let coll: ExpressionValue<ProcessedExpression> =
                    TryProcessFrom::try_process_from(coll, ctx)?;

                Ok(ElementOp::MapInstanceComponent(
                    comp_desc,
                    item_key.to_owned(),
                    coll,
                    Default::default(),
                ))
            } // _ => Ok(*src.to_owned())
              // _ => Err(reduction_err_bt!())
        }
    }
}

/// Evaluate against `OutputContext<ProcessedExpression>`.

impl TryEvalFrom<ElementOp<ProcessedExpression>> for ElementOp<OutputExpression> {
    fn try_eval_from(
        src: &ElementOp<ProcessedExpression>,
        ctx: &mut OutputContext<ProcessedExpression>,
    ) -> DocumentProcessingResult<Self> {
        match *src {
            ElementOp::ElementOpen(ref desc, _) => Ok(ElementOp::ElementOpen(
                TryEvalFrom::try_eval_from(desc, ctx)?,
                Default::default(),
            )),
            ElementOp::ElementVoid(ref desc, _) => Ok(ElementOp::ElementOpen(
                TryEvalFrom::try_eval_from(desc, ctx)?,
                Default::default(),
            )),
            ElementOp::ElementClose(ref s) => Ok(ElementOp::ElementClose(s.to_owned())),

            ElementOp::WriteValue(ref e, ref s) => {
                eprintln!("TryEval ElementOp -> ElementOp WriteValue: e: {:?}", e);
                Ok(ElementOp::WriteValue(
                    TryEvalFrom::try_eval_from(e, ctx)?,
                    s.to_owned(),
                ))
            }

            ElementOp::StartBlock(ref s) => Ok(ElementOp::StartBlock(s.to_owned())),
            ElementOp::EndBlock(ref s) => Ok(ElementOp::EndBlock(s.to_owned())),

            ElementOp::SkipNode => Ok(ElementOp::SkipNode),

            ElementOp::MapCollection(ref s, ref k, ref e, _) => Ok(ElementOp::MapCollection(
                s.to_owned(),
                k.as_ref().map(|s| s.to_owned()),
                TryEvalFrom::try_eval_from(e, ctx)?,
                Default::default(),
            )),

            ElementOp::InstanceComponent(ref comp_desc, _) => Ok(ElementOp::InstanceComponent(
                TryEvalFrom::try_eval_from(comp_desc, ctx)?,
                Default::default(),
            )),

            ElementOp::MapInstanceComponent(ref comp_desc, ref item_key, ref coll, _) => {
                let comp_desc: ComponentInstanceDescriptor<OutputExpression> =
                    TryEvalFrom::try_eval_from(comp_desc, ctx)?;
                let coll: ExpressionValue<OutputExpression> =
                    TryEvalFrom::try_eval_from(coll, ctx)?;

                Ok(ElementOp::MapInstanceComponent(
                    comp_desc,
                    item_key.to_owned(),
                    coll,
                    Default::default(),
                ))
            } // _ => Ok(*src.to_owned())
              // _ => Err(reduction_err_bt!())
        }
    }
}
