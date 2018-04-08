use std::str;
use std::io;

use itertools::join;

use error::*;
use traits::*;
use objects::*;
use output::*;

#[derive(Debug, Default, Clone)]
pub struct DefaultHtmlWriter {}

///
/// Basic and compound expressions
///

impl ObjectWriter<Expression<OutputExpression>, HtmlOutput> for DefaultHtmlWriter {
    fn write_object(
        &mut self,
        w: &mut io::Write,
        ctx: &mut OutputContext,
        obj: &Expression<OutputExpression>,
    ) -> DocumentProcessingResult<()> {
        debug!(
            "ObjectWriter Expression<OutputExpression> (HTML): obj: {:?}",
            obj
        );

        match *obj {
            Expression::Path(ref p, _) => {
                let expr: ExpressionValue<OutputExpression> = TryEvalFrom::try_eval_from(p, ctx)?;
                self.write_object(w, ctx, &expr)
            }

            Expression::QueryCall(ref query_call, _) => {
                write!(w, "[query_call {}]", query_call.name())?;
                Ok(())
            }

            // Expression::ReducedPipeline(ref p, _) => {
            //     let expr: ExpressionValue<OutputExpression> = TryEvalFrom::try_eval_from(p, ctx)?;
            //     self.write_object(w, ctx, &expr)?;

            //     Ok(())
            // }

            Expression::Group(..) | Expression::BinaryOp(..) | Expression::UnaryOp(..) => {
                let expr: ExpressionValue<OutputExpression> = TryEvalFrom::try_eval_from(obj, ctx)?;
                self.write_object(w, ctx, &expr)
            }

            _ => {
                eprintln!("ObjectWriter Expression<OutputExpression> (HTML): Unsupported Expression: {:?}", obj);
                Err(try_eval_from_err!(format!(
                    "Unsupported expression in HTML writer: {:?}",
                    obj
                )))
            }
        }
    }
}

impl ObjectWriter<Primitive, HtmlOutput> for DefaultHtmlWriter {
    fn write_object(
        &mut self,
        w: &mut io::Write,
        _: &mut OutputContext,
        obj: &Primitive,
    ) -> DocumentProcessingResult<()> {
        debug!(
            "ObjectWriter Primitive<OutputExpression> (HTML): obj: {:?}",
            obj
        );

        match *obj {
            Primitive::Int32Val(n) => write!(w, "{}", n),

            Primitive::BoolVal(b) if b => write!(w, "true"),
            Primitive::BoolVal(_) => write!(w, "false"),

            Primitive::CharVal(c) => write!(w, "{}", c),
            Primitive::StringVal(ref s) => write!(w, "{}", s),
            Primitive::NullVal => write!(w, "[null]"),
            Primitive::Undefined => write!(w, "[undefined]"),
        }?;

        Ok(())
    }
}

impl ObjectWriter<ExpressionValue<OutputExpression>, HtmlOutput> for DefaultHtmlWriter {
    fn write_object(
        &mut self,
        w: &mut io::Write,
        ctx: &mut OutputContext,
        obj: &ExpressionValue<OutputExpression>,
    ) -> DocumentProcessingResult<()> {
        match *obj {
            ExpressionValue::Primitive(ref p) => self.write_object(w, ctx, p),
            ExpressionValue::Expression(ref e) => self.write_object(w, ctx, e),
            ExpressionValue::Lens(..) => Ok(()),
            // ExpressionValue::Binding(ref b, _) => self.write_object(w, b),
            _ => Err(try_eval_from_err!(format!(
                "Unsupported expression value when writing: {:?}",
                obj
            ))),
        }
    }
}

///
/// Content expressions and elements
///

impl ObjectWriter<Block<ProcessedExpression>, HtmlOutput> for DefaultHtmlWriter {
    fn write_object(
        &mut self,
        w: &mut io::Write,
        ctx: &mut OutputContext,
        obj: &Block<ProcessedExpression>,
    ) -> DocumentProcessingResult<()> {
        eprintln!(
            "ObjectWriter Block<ProcessedExpression> (HTML): obj: {:?}",
            obj
        );

        if let Some(ops) = obj.ops() {
            for op in ops {
                self.write_object(w, ctx, op)?;
            }
        }

        Ok(())
    }
}

// Event binding (within attribute list)
impl ObjectWriter<ElementEventBindingOutput<ProcessedExpression>, HtmlOutput> for DefaultHtmlWriter {
    fn write_object(
        &mut self,
        w: &mut io::Write,
        ctx: &mut OutputContext,
        obj: &ElementEventBindingOutput<ProcessedExpression>,
    ) -> DocumentProcessingResult<()> {
        eprintln!(
            "ObjectWriter ElementEventBindingOutput<ProcessedExpression> (HTML): obj: {:?}",
            obj
        );

        let name = obj.0.as_ref().map(|s| s.to_owned()).unwrap_or("click".to_owned());
        let key = &obj.1;

        let props: ObjectValue<OutputExpression> =
            TryEvalFrom::try_eval_from(&obj.2, ctx)?;

        let mut bytes: Vec<u8> = Vec::with_capacity(1024);
        let mut js = DefaultJsWriter::default();
        js.write_object(&mut bytes, ctx, &props)?;

        let props_str = str::from_utf8(bytes.as_slice())?
            .replace("\"", "&quot;")
            .replace("_event", "event");

        write!(
            w,
            " on{}=\"_events.{}(event, {})\"",
            name,
            key,
            props_str
        )?;

        Ok(())
    }
}

fn write_open<'s>(
    _self: &'s mut DefaultHtmlWriter,
    w: &mut io::Write,
    ctx: &mut OutputContext,
    desc: &ElementDescriptor<ProcessedExpression>,
    is_void: bool,
    _comp_desc: Option<&ComponentInstanceDescriptor<OutputExpression>>,
    _idx: Option<i32>,
) -> DocumentProcessingResult<()> {
    let element_key = ctx.get_element_key()?
        .map(|s| format!("{}.{}", s, desc.key()))
        .unwrap_or_else(|| desc.key().to_owned());

    write!(w, "<{}", desc.tag())?;

    // Key
    write!(w, " key=\"{}\"", element_key)?;

    // Props
    let string_props = desc.string_props();

    for prop in desc.props() {
        let (name, expr) = (prop.name(), prop.expr());

        let expr: ExpressionValue<OutputExpression> = TryEvalFrom::try_eval_from(expr, ctx)?;
        let expr: ExpressionValue<OutputExpression> = TryEvalFrom::try_eval_from(&expr, ctx)?;

        let class_props = match expr {
            ExpressionValue::Expression(Expression::Composite(CompositeValue::ObjectValue(ObjectValue(Some(box ref props))))) if name =="class" => Some(props),
            _ => None
        };

        if let Some(props) = class_props {
            let classes: Vec<PropValue<OutputExpression>> = TryEvalFrom::try_eval_from(props, ctx)?;
            let classes: Vec<_> = classes.into_iter().filter_map(|prop| {
                        match prop.value() {
                            &ExpressionValue::Primitive(Primitive::BoolVal(true)) => Some(prop.key().to_owned()),
                            _ => None
                        }
            }).collect();

            if classes.len() > 0 {
                let classes = join(classes.into_iter(), " ");
                write!(w, " class=\"{}\"", classes)?;
            };

            continue;
        };

        // Handle boolean parameters differently
        eprintln!("[html] Writing parameter {}: {:?}", name, expr);
        if let ExpressionValue::Primitive(Primitive::BoolVal(b)) = expr {
            if b {
                write!(w, " {}=\"{}\"", name, name)?;
            };
            continue;
        };

        write!(w, " {}=\"", name)?;
        _self.write_object(w, ctx, &expr)?;
        write!(w, "\"")?;
    }

    // Value binding

    if let Some(value_binding) = desc.value_binding() {
        if desc.tag() == "input" && string_props.get("type").map(|s| s.as_str()) == Some("checkbox")
        {
            if let Some(read_expr) = value_binding.read_expr() {
                let expr: ExpressionValue<OutputExpression> =
                    TryEvalFrom::try_eval_from(read_expr, ctx)?;
                let expr: ExpressionValue<OutputExpression> =
                    TryEvalFrom::try_eval_from(&expr, ctx)?;
                let checked: bool = TryEvalFrom::try_eval_from(&expr, ctx)?;

                if checked {
                    write!(w, " checked=\"checked\"")?;
                };
            };
        };
    };

    // Events
    if let Some(events) = desc.events() {
        for event_binding in events {
            eprintln!("Event binding: {:?}", event_binding);

            let event_output: ElementEventBindingOutput<ProcessedExpression> = event_binding.into();
            _self.write_object(w, ctx, &event_output)?;
        }
    };

    if !is_void {
        write!(w, ">")?;
    } else {
        write!(w, " />")?;
    };

    Ok(())
}

impl ObjectWriter<ComponentInstanceDescriptor<ProcessedExpression>, HtmlOutput>
    for DefaultHtmlWriter
{
    fn write_object(
        &mut self,
        w: &mut io::Write,
        ctx: &mut OutputContext,
        comp_desc: &ComponentInstanceDescriptor<ProcessedExpression>,
    ) -> DocumentProcessingResult<()> {
        debug!(
            "ObjectWriter ComponentInstanceDescriptor<ProcessedExpression> (HTML): comp_desc: {:?}",
            comp_desc
        );

        ctx.push_child_scope();

        if let Some(component_props) = comp_desc.props() {
            for prop in component_props {
                let expr: ExpressionValue<OutputExpression> =
                    TryEvalFrom::try_eval_from(prop.expr(), ctx)?;
                let binding =
                    CommonBindings::NamedComponentProp(prop.name().to_owned(), Default::default());
                eprintln!(
                    "[HTML] write_comp_desc: adding binding [{:?}] with value [{:?}].",
                    binding, expr
                );
                ctx.bind_value(binding, expr)?;
            }
        };

        let component_tag = comp_desc.tag();
        let component = ctx.doc()
            .component(component_tag)
            .map(|c| c.to_owned())
            .expect("component expected to exist in write_comp_desc");
        let block = component.block();

        self.write_object(w, ctx, block)?;

        ctx.pop_scope();
        Ok(())
    }
}

impl ObjectWriter<ElementOp<ProcessedExpression>, HtmlOutput> for DefaultHtmlWriter {
    fn write_object(
        &mut self,
        w: &mut io::Write,
        ctx: &mut OutputContext,
        el: &ElementOp<ProcessedExpression>,
    ) -> DocumentProcessingResult<()> {
        debug!(
            "ObjectWriter ElementOp<ProcessedExpression> (HTML): el: {:?}",
            el
        );

        match *el {
            ElementOp::ElementOpen(ref desc, _) => {
                ctx.push_child_scope();
                // ctx.bind_element_key(desc.key(), None)?;
                write_open(self, w, ctx, desc, false, None, None)?;
                Ok(())
            }

            ElementOp::ElementClose(ref tag) => {
                write!(w, "</{}>", tag)?;
                ctx.pop_scope();
                Ok(())
            }

            ElementOp::ElementVoid(ref desc, _) => {
                ctx.push_child_scope();
                // ctx.bind_element_key(desc.key(), None)?;
                write_open(self, w, ctx, desc, true, None, None)?;
                ctx.pop_scope();
                Ok(())
            }

            ElementOp::SkipNode => Ok(()),

            ElementOp::SkipOuterElement(ref e) | ElementOp::SkipElement(ref e) => {
                match *e {
                    SkipElementOp::ElementOpen(ref desc, _) => {
                        ctx.push_child_scope();
                        write_open(self, w, ctx, desc, false, None, None)?;
                        Ok(())
                    }

                    SkipElementOp::ElementClose(ref tag) => {
                        write!(w, "</{}>", tag)?;
                        ctx.pop_scope();
                        Ok(())
                    }

                    SkipElementOp::ElementVoid(ref desc, _) => {
                        ctx.push_child_scope();
                        write_open(self, w, ctx, desc, true, None, None)?;
                        ctx.pop_scope();
                        Ok(())
                    }

                    SkipElementOp::WriteValue(ref expr, _) => {
                        let expr: ExpressionValue<OutputExpression> =
                            TryEvalFrom::try_eval_from(expr, ctx)?;

                        eprintln!(
                            "ObjectWriter SkipElementOp<ProcessedExpression> (HTML) WriteValue expr: {:?}",
                            expr
                        );
                        self.write_object(w, ctx, &expr)
                    }
                }
            }

            ElementOp::StartBlock(_) => Ok(()),

            ElementOp::EndBlock(_) => Ok(()),

            ElementOp::MapCollection(_, _, _, _) => Ok(()),

            ElementOp::WriteValue(ref expr, _) => {
                let expr: ExpressionValue<OutputExpression> =
                    TryEvalFrom::try_eval_from(expr, ctx)?;

                eprintln!(
                    "ObjectWriter ElementOp<ProcessedExpression> (HTML) WriteValue expr: {:?}",
                    expr
                );
                self.write_object(w, ctx, &expr)
            }

            ElementOp::InstanceComponent(ref comp_desc, _) => {
                eprintln!("ObjectWriter ElementOp<ProcessedExpression> (HTML) InstanceComponent comp_desc: {:?}", comp_desc);
                ctx.push_child_scope();
                ctx.bind_element_key(comp_desc.desc().key(), None)?;

                // write_component(self, w, ctx, comp_desc, None, None)
                // write_comp_desc(self, w, ctx, comp_desc, None)?;
                self.write_object(w, ctx, comp_desc)?;
                ctx.pop_scope();
                Ok(())
            }

            ElementOp::MapInstanceComponent(ref comp_desc, ref item_key, ref coll, _) => {
                eprintln!("ObjectWriter ElementOp<ProcessedExpression> (HTML) MapInstanceComponent comp_desc: {:?}", comp_desc);
                eprintln!("ObjectWriter ElementOp<ProcessedExpression> (HTML) MapInstanceComponent item_key: {:?}", item_key);
                eprintln!("ObjectWriter ElementOp<ProcessedExpression> (HTML) MapInstanceComponent coll: {:?}", coll);

                // write!(w, "<!-- map_component: {} () {{ -->", comp_desc.tag())?;

                let coll: ExpressionValue<OutputExpression> =
                    TryEvalFrom::try_eval_from(coll, ctx)?;
                let coll: Option<Vec<ExpressionValue<OutputExpression>>> =
                    TryEvalFrom::try_eval_from(&coll, ctx)?;
                if let Some(coll) = coll {
                    for (idx, item) in (0i32..).zip(coll.iter()) {
                        ctx.push_child_scope_with_environment(
                            OutputScopeEnvironment::MappedComponentInstance,
                        );

                        ctx.bind_element_key(comp_desc.key(), Some(idx))?;

                        // CurrentItem
                        let binding = CommonBindings::CurrentItem(Default::default());
                        ctx.bind_loop_value(binding, item.to_owned())?;

                        // CurrentItemIndex
                        let binding = CommonBindings::CurrentItemIndex;
                        ctx.bind_loop_value(
                            binding,
                            ExpressionValue::Primitive(Primitive::Int32Val(idx)),
                        )?;

                        // write_component(self, w, ctx, comp_desc, item_key.as_ref().map(|s| s.as_str()), Some(idx))?;
                        // write_comp_desc(self, w, ctx, comp_desc, None)?;
                        self.write_object(w, ctx, comp_desc)?;

                        ctx.pop_scope();
                    }
                }

                // write!(w, "<!-- }} -->")?;

                Ok(())
            }
        }
    }
}
