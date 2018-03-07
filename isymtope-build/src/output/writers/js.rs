use std::io;
use std::collections::HashMap;
use std::fmt::Debug;

use error::*;
use ast::*;
use objects::*;
use output::*;

#[derive(Debug, Default, Clone, PartialEq, Eq, Hash)]
pub struct DefaultJsWriter {}

impl ObjectWriter<Primitive, JsOutput> for DefaultJsWriter {
    fn write_object(
        &mut self,
        w: &mut io::Write,
        ctx: &mut OutputContext,
        obj: &Primitive,
    ) -> DocumentProcessingResult<()> {
        match *obj {
            Primitive::Int32Val(n) => write!(w, "{}", n),

            Primitive::BoolVal(b) if b => write!(w, "true"),
            Primitive::BoolVal(_) => write!(w, "false"),

            Primitive::CharVal(c) => write!(w, "'{}'", c),
            Primitive::StringVal(ref s) => write!(w, "\"{}\"", s),
            Primitive::NullVal => write!(w, "null"),
            Primitive::Undefined => write!(w, "undefined"),
        }?;

        Ok(())
    }
}

impl ObjectWriter<LensValue<ProcessedExpression>, JsOutput> for DefaultJsWriter {
    fn write_object(
        &mut self,
        w: &mut io::Write,
        ctx: &mut OutputContext,
        obj: &LensValue<ProcessedExpression>,
    ) -> DocumentProcessingResult<()> {
        debug!(
            "ObjectWriter LensValue<ProcessedExpression> (JS): obj: {:?}",
            obj
        );

        match *obj {
            LensValue::ForLens(ref item_key, box ref coll, _) => {
                // write!(w, "/* for lens: {:?} in ", item_key)?;
                // self.write_object(w, ctx, coll)?;
                // write!(w, " */")?;
                // write!(w, "[]")?;

                Ok(())
            }

            LensValue::GetLens(ref alias, box ref value, _) => {
                // write!(w, "/* get lens: ")?;
                // self.write_object(w, ctx, value)?;
                // write!(w, " as {:?} */", alias)?;

                self.write_object(w, ctx, value)?;

                Ok(())
            }

            LensValue::QueryLens(ref item_key, ref query_call, _) => {
                let name = query_call.name();
                write!(w, "/* query lens: {:?} in {} */", item_key, name)?;
                write!(w, "[]")?;

                Ok(())
            }
        }

        // Ok(())
    }
}

impl ObjectWriter<ExpressionValue<ProcessedExpression>, JsOutput> for DefaultJsWriter {
    fn write_object(
        &mut self,
        w: &mut io::Write,
        ctx: &mut OutputContext,
        obj: &ExpressionValue<ProcessedExpression>,
    ) -> DocumentProcessingResult<()> {
        match *obj {
            ExpressionValue::Expression(ref e) => self.write_object(w, ctx, e),
            ExpressionValue::Primitive(ref p) => self.write_object(w, ctx, p),
            ExpressionValue::Binding(ref b, _) => self.write_object(w, ctx, b),
            ExpressionValue::BindingShape(ref s, _) => self.write_object(w, ctx, s.binding()),
            ExpressionValue::Lens(ref l, _) => self.write_object(w, ctx, l),
            ExpressionValue::SourceLens(_, _) => Ok(()),
            ExpressionValue::Content(_, _) => Ok(()), // _ => Err(reduction_err_bt!())
        }
    }
}

impl ObjectWriter<ExpressionValue<OutputExpression>, JsOutput> for DefaultJsWriter {
    fn write_object(
        &mut self,
        w: &mut io::Write,
        ctx: &mut OutputContext,
        obj: &ExpressionValue<OutputExpression>,
    ) -> DocumentProcessingResult<()> {
        match *obj {
            ExpressionValue::Expression(ref e) => self.write_object(w, ctx, e),
            ExpressionValue::Primitive(ref p) => self.write_object(w, ctx, p),
            ExpressionValue::Binding(ref b, _) => self.write_object(w, ctx, b),
            _ => Err(try_process_from_err!(format!(
                "Unsupported output expression: {:?}",
                obj
            ))),
        }
    }
}

impl ObjectWriter<CommonBindings<ProcessedExpression>, JsOutput> for DefaultJsWriter {
    fn write_object(
        &mut self,
        w: &mut io::Write,
        ctx: &mut OutputContext,
        obj: &CommonBindings<ProcessedExpression>,
    ) -> DocumentProcessingResult<()> {
        eprintln!(
            "ObjectWriter CommonBindings<ProcessedExpression> (JS): obj: {:?}",
            obj
        );

        match *obj {
            CommonBindings::CurrentReducerState(_) => write!(w, "state"),
            CommonBindings::CurrentItem(_) => write!(w, "_item"),
            CommonBindings::CurrentItemIndex => write!(w, "_idx"),
            CommonBindings::NamedReducerKey(ref key, _) => write!(w, "store.getState().{}", key),
            CommonBindings::NamedReducerActionParam(ref ident, _) => write!(w, "action.{}", ident),
            CommonBindings::NamedQueryParam(ref ident, _) => write!(w, "{}", ident),
            CommonBindings::NamedComponentProp(ref ident, _) => write!(w, "props.{}", ident),
            CommonBindings::ComponentPropsObject(_) => write!(w, "props"),
            CommonBindings::NamedEventBoundValue(_, _) => write!(w, "_event.target.value"),
            CommonBindings::CurrentElementValue(_) => write!(w, "_event.target.value"),
            CommonBindings::CurrentElementKeyPath => write!(w, "props.key"),
            CommonBindings::PathAlias(ref path, _) => write!(w, "{}", path),
        }?;

        Ok(())
    }
}

impl ObjectWriter<CommonBindings<OutputExpression>, JsOutput> for DefaultJsWriter {
    fn write_object(
        &mut self,
        w: &mut io::Write,
        ctx: &mut OutputContext,
        obj: &CommonBindings<OutputExpression>,
    ) -> DocumentProcessingResult<()> {
        eprintln!(
            "ObjectWriter CommonBindings<OutputExpression> (JS): obj: {:?}",
            obj
        );

        match *obj {
            CommonBindings::CurrentItem(_) => write!(w, "_item"),
            CommonBindings::CurrentItemIndex => write!(w, "_idx"),
            CommonBindings::CurrentElementValue(_) => write!(w, "_event.target.value"),
            _ => Err(try_eval_from_err!(format!(
                "Unsupported output binding: {:?}",
                obj
            )))?,
        }?;

        Ok(())
    }
}

impl ObjectWriter<ParamValue<ProcessedExpression>, JsOutput> for DefaultJsWriter {
    fn write_object(
        &mut self,
        w: &mut io::Write,
        ctx: &mut OutputContext,
        obj: &ParamValue<ProcessedExpression>,
    ) -> DocumentProcessingResult<()> {
        debug!(
            "ObjectWriter ParamValue<ProcessedExpression> (JS): obj: {:?}",
            obj
        );
        self.write_object(w, ctx, obj.value())
    }
}

impl ObjectWriter<PipelineComponentValue<ProcessedExpression>, JsOutput> for DefaultJsWriter {
    fn write_object(
        &mut self,
        w: &mut io::Write,
        ctx: &mut OutputContext,
        obj: &PipelineComponentValue<ProcessedExpression>,
    ) -> DocumentProcessingResult<()> {
        debug!(
            "ObjectWriter PipelineComponentValue<ProcessedExpression> (JS): obj: {:?}",
            obj
        );

        match *obj {
            PipelineComponentValue::Member(ref s) => {
                write!(w, "{}", s)?;
                Ok(())
            }

            PipelineComponentValue::MethodCall(ref s, ref params, _) => {
                write!(w, "{}(", s)?;
                if let &Some(ref params) = params {
                    let mut first = true;
                    for param in params {
                        if !first {
                            write!(w, ",")?;
                        }
                        self.write_object(w, ctx, param)?;
                        first = false;
                    }
                }
                write!(w, ")")?;
                Ok(())
            }
        }
    }
}

fn write_pipeline_head<T: Debug>(
    _self: &mut DefaultJsWriter,
    w: &mut io::Write,
    ctx: &mut OutputContext,
    head: &ExpressionValue<T>,
) -> DocumentProcessingResult<()>
where
    DefaultJsWriter: ObjectWriter<ExpressionValue<T>, JsOutput>,
{
    eprintln!("[JS] write_pipeline_head: head: {:?}", head);
    match *head {
        ExpressionValue::Binding(CommonBindings::CurrentReducerState(_), _)
        | ExpressionValue::Binding(CommonBindings::NamedQueryParam(..), _) => {
            write!(w, "wrap(")?;
            _self.write_object(w, ctx, head)?;
            write!(w, ")")?;
        }

        _ => {
            _self.write_object(w, ctx, head)?;
        }
    };

    Ok(())
}

impl ObjectWriter<PipelineValue<ProcessedExpression>, JsOutput> for DefaultJsWriter {
    fn write_object(
        &mut self,
        w: &mut io::Write,
        ctx: &mut OutputContext,
        obj: &PipelineValue<ProcessedExpression>,
    ) -> DocumentProcessingResult<()> {
        debug!(
            "ObjectWriter PipelineValue<ProcessedExpression> (JS): obj: {:?}",
            obj
        );

        write_pipeline_head(self, w, ctx, obj.head())?;

        if obj.has_components() {
            let components = obj.components();
            for component in components {
                self.write_object(w, ctx, component)?;
            }
            write!(w, ".value")?;
        };

        Ok(())
    }
}

///
/// Filter (sql-like) pipelines
///

impl ObjectWriter<FilterValue<ProcessedExpression>, JsOutput> for DefaultJsWriter {
    fn write_object(
        &mut self,
        w: &mut io::Write,
        ctx: &mut OutputContext,
        obj: &FilterValue<ProcessedExpression>,
    ) -> DocumentProcessingResult<()> {
        eprintln!(
            "ObjectWriter FilterValue<ProcessedExpression> (JS): obj: {:?}",
            obj
        );

        // write!(w, "wrap(")?;
        write_pipeline_head(self, w, ctx, obj.head())?;
        // write!(w, ")")?;

        let components: Vec<_> = obj.components().collect();
        let has_components = !components.is_empty();
        if has_components {
            for component in components {
                // write!(w, ".")?;
                self.write_object(w, ctx, component)?;
            }
            write!(w, ".value")?;
        }

        Ok(())
    }
}

impl ObjectWriter<FilterComponentValue<ProcessedExpression>, JsOutput> for DefaultJsWriter {
    fn write_object(
        &mut self,
        w: &mut io::Write,
        ctx: &mut OutputContext,
        obj: &FilterComponentValue<ProcessedExpression>,
    ) -> DocumentProcessingResult<()> {
        eprintln!(
            "ObjectWriter FilterComponentValue<ProcessedExpression> (JS): obj: {:?}",
            obj
        );

        match *obj {
            FilterComponentValue::Where(ref wc, _) => {
                write!(w, ".where(_item => ")?;
                self.write_object(w, ctx, wc)?;
                write!(w, ")")?;

                Ok(())
            }

            FilterComponentValue::Set(ref v, ref wc, _) => {
                write!(w, ".setObject(_item => ({{")?;

                let mut first = true;
                for set_assignment in v {
                    match *set_assignment {
                        FilterSetAssignment::SetMemberTo(ref s, ref e, _) => {
                            if !first {
                                write!(w, ", ")?;
                            }
                            write!(w, "\"{}\": ", s)?;
                            self.write_object(w, ctx, e)?;
                            first = false;
                        }
                    }
                }
                write!(w, "}})")?;

                if let &Some(ref wc) = wc {
                    write!(w, ", _item => ")?;
                    self.write_object(w, ctx, wc)?;
                };

                write!(w, ")")?;

                Ok(())
            }

            FilterComponentValue::Delete(ref wc, _) => {
                write!(w, ".removeObject(_item => ")?;
                self.write_object(w, ctx, wc)?;
                write!(w, ")")?;

                Ok(())
            }

            FilterComponentValue::Unique(ref wc, _) => {
                write!(w, ".unique(_item => ")?;
                self.write_object(w, ctx, wc)?;
                write!(w, ")")?;

                Ok(())
            }
        }
    }
}

impl ObjectWriter<FilterWhereClause<ProcessedExpression>, JsOutput> for DefaultJsWriter {
    fn write_object(
        &mut self,
        w: &mut io::Write,
        ctx: &mut OutputContext,
        obj: &FilterWhereClause<ProcessedExpression>,
    ) -> DocumentProcessingResult<()> {
        debug!(
            "ObjectWriter FilterWhereClause<ProcessedExpression> (JS): obj: {:?}",
            obj
        );

        let anded_conditions = obj.anded_conditions();
        let mut first = true;
        for cond in anded_conditions {
            if !first {
                write!(w, " && ")?;
            }
            self.write_object(w, ctx, cond)?;
            first = false;
        }

        Ok(())
    }
}

///
/// Reduced pipeline
///

impl ObjectWriter<ReducedPipelineValue<ProcessedExpression>, JsOutput> for DefaultJsWriter {
    fn write_object(
        &mut self,
        w: &mut io::Write,
        ctx: &mut OutputContext,
        obj: &ReducedPipelineValue<ProcessedExpression>,
    ) -> DocumentProcessingResult<()> {
        debug!(
            "ObjectWriter ReducedPipelineValue<ProcessedExpression> (JS): obj: {:?}",
            obj
        );

        write!(w, "wrap(")?;
        // write_pipeline_head(self, w, ctx, obj.head())?;
        self.write_object(w, ctx, obj.head())?;
        write!(w, ")")?;

        for component in obj.components() {
            self.write_object(w, ctx, component)?;
        }

        write!(w, ".value")?;
        Ok(())
    }
}

impl ObjectWriter<ReducedPipelineComponent<ProcessedExpression>, JsOutput> for DefaultJsWriter {
    fn write_object(
        &mut self,
        w: &mut io::Write,
        ctx: &mut OutputContext,
        obj: &ReducedPipelineComponent<ProcessedExpression>,
    ) -> DocumentProcessingResult<()> {
        match *obj {
            ReducedPipelineComponent::PipelineOp(ref op) => {
                match *op {
                    ReducedMethodCall::Map(ref expr) => {
                        write!(w, ".map(_item => ")?;
                        self.write_object(w, ctx, expr)?;
                        write!(w, ")")?;
                    }

                    ReducedMethodCall::MapIf(ref expr, ref cond) => {
                        write!(w, ".map(_item => ")?;
                        self.write_object(w, ctx, expr)?;
                        write!(w, ", _item => ")?;
                        self.write_object(w, ctx, cond)?;
                        write!(w, ")")?;
                    }

                    ReducedMethodCall::Filter(ref cond) => {
                        write!(w, ".filter(_item => ")?;
                        self.write_object(w, ctx, cond)?;
                        write!(w, ")")?;
                    }

                    ReducedMethodCall::Reduce(ref expr, ref initial) => {
                        write!(w, ".reduce(_item => ")?;
                        self.write_object(w, ctx, expr)?;
                        write!(w, ", ")?;
                        self.write_object(w, ctx, initial)?;
                        write!(w, ")")?;
                    }

                    ReducedMethodCall::ReduceIf(ref expr, ref cond, ref initial) => {
                        write!(w, ".reduceIf(_item => ")?;
                        self.write_object(w, ctx, expr)?;
                        write!(w, ", ")?;
                        self.write_object(w, ctx, initial)?;
                        write!(w, ", ")?;
                        self.write_object(w, ctx, cond)?;
                        write!(w, ")")?;
                    }

                    ReducedMethodCall::Uniq(ref cond) => {
                        write!(w, ".filter((function(_keys) {{ return function(_item) {{ let _key = item[_key]; ")?;
                        write!(
                            w,
                            "; return !(_keys.has(_key) || !_keys.add(_key) ); }}}})(new Set()))"
                        )?;
                    }

                    ReducedMethodCall::UniqByKey(ref key) => {
                        write!(w, ".filter((function(_keys) {{ return function(_item) {{ let _key = item[_key]; ")?;
                        write!(
                            w,
                            "; return !(_keys.has(_key) || !_keys.add(_key) ); }}}})(new Set()))"
                        )?;
                    }

                    ReducedMethodCall::MinBy(ref expr) => {
                        // write!(w, ".reduce((v, acc) => (v < acc) ? v : acc)")?;
                        write!(w, ".min(_item => ")?;
                        self.write_object(w, ctx, expr)?;
                        write!(w, ")")?;
                    }

                    ReducedMethodCall::MaxBy(ref expr) => {
                        // write!(w, ".reduce((v, acc) => (v > acc) ? v : acc)")?;
                        write!(w, ".max(_item => ")?;
                        self.write_object(w, ctx, expr)?;
                        write!(w, ")")?;
                    }

                    ReducedMethodCall::Count(ref expr) => {
                        write!(w, ".count(_item => ")?;
                        self.write_object(w, ctx, expr)?;
                        write!(w, ")")?;
                    }

                    ReducedMethodCall::FirstWhere(ref cond) => {
                        write!(w, ".first(_item => ")?;
                        self.write_object(w, ctx, cond)?;
                        write!(w, ")")?;
                    }

                    ReducedMethodCall::First => {
                        write!(w, ".first()")?;
                    }
                }
                Ok(())
            }

            ReducedPipelineComponent::Member(ref name) => {
                write!(w, ".{}", name)?;
                Ok(())
            }

            // ReducedPipelineComponent::ExpressionValue(ref expr) => self.write_object(w, ctx, expr)
            _ => Ok(()),
        }
    }
}

///
/// Basic and compound expressions
///

impl ObjectWriter<Expression<ProcessedExpression>, JsOutput> for DefaultJsWriter {
    fn write_object(
        &mut self,
        w: &mut io::Write,
        ctx: &mut OutputContext,
        obj: &Expression<ProcessedExpression>,
    ) -> DocumentProcessingResult<()> {
        eprintln!(
            "ObjectWriter Expression<ProcessedExpression> (JS): obj: {:?}",
            obj
        );

        match *obj {
            Expression::Composite(ref c) => self.write_object(w, ctx, c),

            Expression::Path(ref p, _) => self.write_object(w, ctx, p),

            Expression::Ident(ref s, _) => {
                write!(w, "{}", s)?;
                Ok(())
            }

            Expression::RawPath(ref s, _) => {
                write!(w, "{}", s)?;
                Ok(())
            }

            // Expression::Binding(ref b) => {
            //     self.write_object(w, ctx, b)?;
            // }
            Expression::Pipeline(ref p, _) => self.write_object(w, ctx, p),
            Expression::Filter(ref f, _) => self.write_object(w, ctx, f),

            Expression::ReducedPipeline(ref p, _) => self.write_object(w, ctx, p),

            Expression::Group(Some(box ref e)) => {
                write!(w, "(")?;
                self.write_object(w, ctx, e)?;
                write!(w, ")")?;

                Ok(())
            }

            Expression::Group(_) => {
                write!(w, "()")?;
                Ok(())
            }

            Expression::UnaryOp(ref op, box ref a) => {
                match *op {
                    UnaryOpType::Negate => write!(w, "!"),
                }?;

                self.write_object(w, ctx, a)
            }

            Expression::BinaryOp(ref op, box ref a, box ref b) => {
                if let BinaryOpType::Add = *op {
                    match (a, b) {
                        (
                            &ExpressionValue::Binding(CommonBindings::CurrentReducerState(_), _),
                            _,
                        ) => {
                            write!(w, "wrap(")?;
                            self.write_object(w, ctx, a)?;
                            write!(w, ").addObject(")?;
                            self.write_object(w, ctx, b)?;
                            write!(w, ")")?;
                            // write!(w, ").value")?;

                            return Ok(());
                        }

                        (
                            &ExpressionValue::Expression(Expression::Composite(
                                CompositeValue::ObjectValue(_),
                            )),
                            _,
                        )
                        | (
                            _,
                            &ExpressionValue::Expression(Expression::Composite(
                                CompositeValue::ObjectValue(_),
                            )),
                        ) => {
                            write!(w, "Object.assign({{}}, ")?;
                            self.write_object(w, ctx, a)?;
                            write!(w, ", ")?;
                            self.write_object(w, ctx, b)?;
                            write!(w, ")")?;

                            return Ok(());
                        }

                        _ => {}
                    };
                }

                self.write_object(w, ctx, a)?;

                match *op {
                    BinaryOpType::Add => write!(w, " + "),
                    BinaryOpType::Sub => write!(w, " - "),
                    BinaryOpType::Mul => write!(w, " * "),
                    BinaryOpType::Div => write!(w, " / "),
                    BinaryOpType::EqualTo => write!(w, " == "),
                    BinaryOpType::NotEqualTo => write!(w, " != "),
                    BinaryOpType::LessThan => write!(w, " < "),
                    BinaryOpType::LessThanOrEqualTo => write!(w, " <= "),
                    BinaryOpType::GreaterThan => write!(w, " > "),
                    BinaryOpType::GreaterThanOrEqualTo => write!(w, " >= "),
                }?;

                self.write_object(w, ctx, b)?;

                Ok(())
            }

            Expression::QueryCall(ref query_call, _) => self.write_object(w, ctx, query_call),

            _ => {
                eprintln!("ObjectWriter Expression<ProcessedExpression> (JS): Unsupported Expression: {:?}", obj);
                Err(try_process_from_err!(
                    "Unsupported expression in JS writer."
                ))
            }
        }
    }
}

impl ObjectWriter<Expression<OutputExpression>, JsOutput> for DefaultJsWriter {
    fn write_object(
        &mut self,
        w: &mut io::Write,
        ctx: &mut OutputContext,
        obj: &Expression<OutputExpression>,
    ) -> DocumentProcessingResult<()> {
        eprintln!(
            "ObjectWriter Expression<OutputExpression> (JS): obj: {:?}",
            obj
        );

        // if obj.is_array_of_objects() {
        //     let mut first = true;
        //     write!(w, "new Map([")?;
        //     for param in params {
        //         if !first {
        //             write!(w, ", ")?;
        //         }
        //         self.write_object(w, ctx, param.value())?;
        //         first = false;
        //     }
        //     write!(w, "]).map(_item => [_item.id, _item]))")?;
        //     return Ok(());
        // };

        match *obj {
            Expression::Composite(ref c) => self.write_object(w, ctx, c),

            Expression::Path(ref p, _) => self.write_object(w, ctx, p),

            Expression::Ident(ref s, _) => {
                write!(w, "{}", s)?;
                Ok(())
            }

            Expression::RawPath(ref s, _) => {
                write!(w, "{}", s)?;
                Ok(())
            }

            _ => {
                eprintln!(
                    "ObjectWriter Expression<OutputExpression> (JS): Unsupported Expression: {:?}",
                    obj
                );
                Err(try_process_from_err!(
                    "Unsupported expression in JS writer."
                ))
            }
        }
    }
}

impl ObjectWriter<QueryCall<ProcessedExpression>, JsOutput> for DefaultJsWriter {
    fn write_object(
        &mut self,
        w: &mut io::Write,
        ctx: &mut OutputContext,
        obj: &QueryCall<ProcessedExpression>,
    ) -> DocumentProcessingResult<()> {
        let name = obj.name();
        let params = obj.params();

        write!(w, "query_{}(store", name)?;

        // let mut first = true;
        for param in params {
            // if !first { write!(w, ", ")?; }
            write!(w, ", ")?;
            self.write_object(w, ctx, param)?;
            // first = false;
        }

        write!(w, ")")?;

        Ok(())
    }
}

impl ObjectWriter<QueryParamValue<ProcessedExpression>, JsOutput> for DefaultJsWriter {
    fn write_object(
        &mut self,
        w: &mut io::Write,
        ctx: &mut OutputContext,
        obj: &QueryParamValue<ProcessedExpression>,
    ) -> DocumentProcessingResult<()> {
        self.write_object(w, ctx, obj.value())
    }
}

fn write_composite<T>(
    _self: &mut DefaultJsWriter,
    w: &mut io::Write,
    ctx: &mut OutputContext,
    obj: &CompositeValue<T>,
) -> DocumentProcessingResult<()>
where
    DefaultJsWriter: ObjectWriter<ExpressionValue<T>, JsOutput>,
{
    match *obj {
        CompositeValue::ObjectValue(Some(box ref props)) => {
            let mut first = true;
            write!(w, "{{")?;
            for prop in props {
                if !first {
                    write!(w, ", ")?;
                }
                write!(w, "\"{}\": ", prop.key())?;
                _self.write_object(w, ctx, prop.value())?;
                first = false;
            }
            write!(w, "}}")?;
            Ok(())
        }

        CompositeValue::ObjectValue(_) => {
            write!(w, "{{}}")?;
            Ok(())
        }

        CompositeValue::ArrayValue(Some(box ref params)) => {
            let mut first = true;
            write!(w, "[")?;
            for param in params {
                if !first {
                    write!(w, ", ")?;
                }
                _self.write_object(w, ctx, param.value())?;
                first = false;
            }
            write!(w, "]")?;
            Ok(())
        }

        CompositeValue::ArrayValue(_) => {
            write!(w, "[]")?;
            Ok(())
        } // _ => Err(reduction_err_bt!())
    }.into()
}

impl ObjectWriter<CompositeValue<ProcessedExpression>, JsOutput> for DefaultJsWriter {
    fn write_object(
        &mut self,
        w: &mut io::Write,
        ctx: &mut OutputContext,
        obj: &CompositeValue<ProcessedExpression>,
    ) -> DocumentProcessingResult<()> {
        write_composite(self, w, ctx, obj)
    }
}

impl ObjectWriter<CompositeValue<OutputExpression>, JsOutput> for DefaultJsWriter {
    fn write_object(
        &mut self,
        w: &mut io::Write,
        ctx: &mut OutputContext,
        obj: &CompositeValue<OutputExpression>,
    ) -> DocumentProcessingResult<()> {
        write_composite(self, w, ctx, obj)
    }
}

impl ObjectWriter<PathValue<ProcessedExpression>, JsOutput> for DefaultJsWriter {
    fn write_object(
        &mut self,
        w: &mut io::Write,
        ctx: &mut OutputContext,
        obj: &PathValue<ProcessedExpression>,
    ) -> DocumentProcessingResult<()> {
        debug!(
            "ObjectWriter PathValue<ProcessedExpression> (JS): obj: {:?}",
            obj
        );

        write_pipeline_head(self, w, ctx, obj.head())?;

        if let Some(components) = obj.components() {
            for component in components {
                write!(w, ".{}", component)?;
            }
        }

        Ok(())
    }
}

impl ObjectWriter<PathComponentValue<ProcessedExpression>, JsOutput> for DefaultJsWriter {
    fn write_object(
        &mut self,
        w: &mut io::Write,
        ctx: &mut OutputContext,
        obj: &PathComponentValue<ProcessedExpression>,
    ) -> DocumentProcessingResult<()> {
        debug!(
            "ObjectWriter PathComponentValue<ProcessedExpression> (JS): obj: {:?}",
            obj
        );

        match *obj {
            PathComponentValue::Member(ref s, _) => {
                write!(w, "{}", s)?;
                Ok(())
            }

            PathComponentValue::MethodCall(ref s, ref params, _) => {
                write!(w, "{}(", s)?;
                if let &Some(ref params) = params {
                    let mut first = true;
                    for param in params {
                        if !first {
                            write!(w, ",")?;
                        }
                        self.write_object(w, ctx, param)?;
                        first = false;
                    }
                }
                write!(w, ")")?;
                Ok(())
            }
        }
    }
}

impl ObjectWriter<PathValue<OutputExpression>, JsOutput> for DefaultJsWriter {
    fn write_object(
        &mut self,
        w: &mut io::Write,
        ctx: &mut OutputContext,
        obj: &PathValue<OutputExpression>,
    ) -> DocumentProcessingResult<()> {
        debug!(
            "ObjectWriter PathValue<OutputExpression> (JS): obj: {:?}",
            obj
        );

        write_pipeline_head(self, w, ctx, obj.head())?;

        if let Some(components) = obj.components() {
            for component in components {
                write!(w, ".{}", component)?;
            }
        }

        Ok(())
    }
}

impl ObjectWriter<PathComponentValue<OutputExpression>, JsOutput> for DefaultJsWriter {
    fn write_object(
        &mut self,
        w: &mut io::Write,
        ctx: &mut OutputContext,
        obj: &PathComponentValue<OutputExpression>,
    ) -> DocumentProcessingResult<()> {
        debug!(
            "ObjectWriter PathComponentValue<OutputExpression> (JS): obj: {:?}",
            obj
        );

        match *obj {
            PathComponentValue::Member(ref s, _) => {
                write!(w, "{}", s)?;
                Ok(())
            }

            PathComponentValue::MethodCall(ref s, ref params, _) => {
                write!(w, "{}(...)", s)?;
                // if let &Some(ref params) = params {
                //     let mut first = true;
                //     for param in params {
                //         if !first { write!(w, ",")?; }
                //         self.write_object(w, ctx, param)?;
                //         first = false;
                //     }
                // }
                // write!(w, ")")?;
                Ok(())
            }
        }
    }
}

///
/// Components
///

impl ObjectWriter<Component<ProcessedExpression>, JsOutput> for DefaultJsWriter {
    fn write_object(
        &mut self,
        w: &mut io::Write,
        ctx: &mut OutputContext,
        obj: &Component<ProcessedExpression>,
    ) -> DocumentProcessingResult<()> {
        debug!(
            "ObjectWriter Component<ProcessedExpression> (JS): obj: {:?}",
            obj
        );

        ctx.push_child_scope_with_environment(OutputScopeEnvironment::Component);

        // NOTE: Writing of the function has moved to the internal template

        let block = obj.block();
        self.write_object(w, ctx, block)?;

        ctx.pop_scope();
        Ok(())
    }
}

///
/// Content expressions and elements
///

impl ObjectWriter<Block<ProcessedExpression>, JsOutput> for DefaultJsWriter {
    fn write_object(
        &mut self,
        w: &mut io::Write,
        ctx: &mut OutputContext,
        obj: &Block<ProcessedExpression>,
    ) -> DocumentProcessingResult<()> {
        debug!(
            "ObjectWriter Block<ProcessedExpression> (JS): obj: {:?}",
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

fn write_open<'s>(
    _self: &'s mut DefaultJsWriter,
    w: &mut io::Write,
    ctx: &mut OutputContext,
    desc: &ElementDescriptor<ProcessedExpression>,
    is_void: bool,
) -> DocumentProcessingResult<()> {
    let tag = desc.tag();
    let element_key = ctx.get_element_key()?
        .map(|s| format!("{}.{}", s, desc.key()))
        .unwrap_or_else(|| desc.key().to_owned());

    if !is_void {
        write!(w, "IncrementalDOM.elementOpen(\"{}\", ", tag)?;
    } else {
        write!(w, "IncrementalDOM.elementVoid(\"{}\", ", tag)?;
    };

    // Is this element being emitted within a component definition (function)?
    let key_string = match ctx.environment()? {
        Some(OutputScopeEnvironment::Component) => format!("props.key + \".{}\"", element_key),
        _ => format!("\"{}\"", element_key),
    };

    write!(w, "{}, [", key_string)?;

    let mut first = true;

    write!(w, "\"key\", {}", key_string)?;
    first = false;

    if let Some(events) = desc.events() {
        for event_binding in events {
            if !first {
                write!(w, ", ")?;
            }
            write!(w, "\"on{}\", ", event_binding.event_name())?;
            // write!(w, "e => {}(e, props)", event_binding.key())?;
            write!(w, "e => {}(e, {{", event_binding.key())?;

            let props: HashMap<String, String> =event_binding.props().map(|(alias, _, path_string)| (alias.to_string(), path_string.to_string())).collect();

            let mut first_prop = true;
            for (ref alias, ref path_string) in props {
                if !first_prop {
                    write!(w, ", ")?;
                }
                write!(w, "{}: {}", alias, path_string)?;
                first_prop = false;
            }

            write!(w, "}})")?;
            first = false;
        }
    };

    let props: Vec<_> = desc.props()
        .map(|prop| (prop.name().to_owned(), prop.expr().to_owned()))
        .collect();

    let const_props: Vec<_> = props
        .iter()
        .filter_map(|prop| match prop {
            _ if prop.1.is_primitive() => Some(prop.to_owned()),
            _ => None,
        })
        .collect();
    let eval_props: Vec<_> = props
        .iter()
        .filter_map(|prop| match prop {
            _ if !prop.1.is_primitive() => Some(prop.to_owned()),
            _ => None,
        })
        .collect();

    // let const_prop_map: HashMap<_, _> = const_props.iter().map(|prop| (prop.name(), prop.to_owned()))

    // let eval_props: Vec<(String, ExpressionValue<OutputExpression>)> = TryEvalFrom::try_eval_from(&eval_props, ctx)?;

    // statics
    for prop in const_props {
        if !first {
            write!(w, ", ")?;
        }
        let (name, expr) = prop;
        write!(w, "\"{}\", ", name)?;
        _self.write_object(w, ctx, &expr)?;
        first = false;
    }
    write!(w, "]")?;

    // attributes
    for prop in eval_props {
        write!(w, ", ")?;

        let (name, expr) = prop;
        let use_classes = name == "class" && expr.is_object();

        if (tag == "input" || tag == "button") && name == "disabled" {
            _self.write_object(w, ctx, &expr)?;
            write!(w, " ? 'disabled' : null, ")?;

            _self.write_object(w, ctx, &expr)?;
            write!(w, " ? 'disabled' : null")?;

            continue;
        };

        write!(w, "\"{}\", ", name)?;
        if use_classes {
            write!(w, "classes(")?;
        }
        _self.write_object(w, ctx, &expr)?;
        if use_classes {
            write!(w, ")")?;
        }
    }

    let string_props = desc.string_props();

    if let Some(value_binding) = desc.value_binding() {
        if desc.tag() == "input" && string_props.get("type").map(|s| s.as_str()) == Some("checkbox")
        {
            if let Some(read_expr) = value_binding.read_expr() {
                write!(w, ", ")?;

                _self.write_object(w, ctx, read_expr)?;
                write!(w, " ? 'checked' : null, ")?;

                _self.write_object(w, ctx, read_expr)?;
                write!(w, " ? 'checked' : null")?;
            };
        };
    };

    writeln!(w, ");")?;

    Ok(())
}

fn write_comp_desc<'s>(
    _self: &'s mut DefaultJsWriter,
    w: &mut io::Write,
    ctx: &mut OutputContext,
    comp_desc: &ComponentInstanceDescriptor<ProcessedExpression>,
    item_key: Option<&str>,
    is_map: bool,
) -> DocumentProcessingResult<()> {
    let component_key = comp_desc.key();
    let tag = comp_desc.tag();

    // let component_props: Vec<ElementPropValue<ProcessedExpression>> = props.as_ref().map(|v| v.iter().map(|p| p.to_owned()).collect()).unwrap_or_default();
    let component_props = comp_desc.props();

    // Is this component instance (function call) being emitted within a component definition (function)?
    let key_string = match ctx.environment()? {
        Some(OutputScopeEnvironment::Component) if is_map => {
            format!("props.key + \".{}.\" + _item[0]", component_key)
        }
        Some(OutputScopeEnvironment::Component) => format!("props.key + \".{}\"", component_key),
        _ => format!("\"{}\"", component_key),
    };

    write!(w, "{}Component(", tag)?;
    if let Some(component_props) = component_props {
        write!(w, "{{\"key\": {}", key_string)?;

        // let props = component_props.iter()
        //     .map(|prop| {
        //         let val;

        //         if let Some(item_key) = item_key {
        //             val = "_item[1]";
        //         } else {
        //             val = prop.key().to_owned();
        //         };

        //         val
        //     });

        // if let Some(item_key) = item_key {
        //     if !first {
        //         write!(w, ", ")?;
        //     }
        //     write!(w, "{}: _item[1]", item_key)?;
        //     first = false;
        // };

        for prop in component_props {
            let key = prop.name();

            write!(w, ", {}: ", prop.name())?;
            if item_key == Some(key) {
                write!(w, "_item[1]")?;
            } else {
                _self.write_object(w, ctx, prop.expr())?;
            };
        }

        // if is_map {
        //     // FIXME
        //     let key = format!("\"{}_\" + _item[1].id", comp_desc.key());

        //     if !first { write!(w, ", ")?; }
        //     write!(w, "key: {}", key)?;
        //     first = false;
        // }

        write!(w, "}}")?;
    };
    writeln!(w, ");")?;

    Ok(())
}

impl ObjectWriter<ElementOp<ProcessedExpression>, JsOutput> for DefaultJsWriter {
    fn write_object(
        &mut self,
        w: &mut io::Write,
        ctx: &mut OutputContext,
        obj: &ElementOp<ProcessedExpression>,
    ) -> DocumentProcessingResult<()> {
        debug!(
            "ObjectWriter ElementOp<ProcessedExpression> (JS): obj: {:?}",
            obj
        );

        write!(w, "        ")?;

        match *obj {
            ElementOp::ElementOpen(ref desc, _) => {
                // ctx.push_child_scope();
                // ctx.bind_element_key(desc.key(), None)?;
                write_open(self, w, ctx, desc, false)?;
                Ok(())
            }

            ElementOp::ElementClose(ref tag) => {
                writeln!(w, "IncrementalDOM.elementClose(\"{}\");", tag)?;
                // ctx.pop_scope();
                Ok(())
            }

            ElementOp::ElementVoid(ref desc, _) => {
                // ctx.push_child_scope();
                // ctx.bind_element_key(desc.key(), None)?;
                write_open(self, w, ctx, desc, true)?;
                // ctx.pop_scope();
                Ok(())
            }

            ElementOp::SkipNode => {
                writeln!(w, "IncrementalDOM.skipNode();")?;
                Ok(())
            }

            ElementOp::StartBlock(_) => Ok(()),

            ElementOp::EndBlock(_) => Ok(()),

            ElementOp::MapCollection(_, _, _, _) => Ok(()),

            ElementOp::WriteValue(ref e, ref key) => {
                write!(w, "IncrementalDOM.text(")?;
                self.write_object(w, ctx, e)?;
                writeln!(w, ");")?;

                Ok(())
            }

            ElementOp::InstanceComponent(ref comp_desc, _) => {
                // ctx.bind_element_key(comp_desc.desc().key(), None)?;
                write_comp_desc(self, w, ctx, comp_desc, None, false)?;

                Ok(())
            }

            ElementOp::MapInstanceComponent(ref comp_desc, ref item_key, ref coll, _) => {
                // ctx.bind_element_key(comp_desc.desc().key(), None)?;

                write!(w, "for (const _item of wrap(")?;
                self.write_object(w, ctx, coll)?;
                writeln!(w, ").enumerateWithKeys().value) {{ ")?;

                let item_key = item_key.as_ref().map(|s| s.as_str());
                write_comp_desc(self, w, ctx, comp_desc, item_key, true)?;
                writeln!(w, "}}")?;

                Ok(())
            }
        }
    }
}

/// Store and reducers

impl ObjectWriter<ReducerAction<ProcessedExpression>, JsOutput> for DefaultJsWriter {
    fn write_object(
        &mut self,
        w: &mut io::Write,
        ctx: &mut OutputContext,
        action: &ReducerAction<ProcessedExpression>,
    ) -> DocumentProcessingResult<()> {
        let expr = action.expr();

        if expr.is_none() {
            write!(w, "return state || null;")?;
            return Ok(());
        };

        let expr = expr.unwrap();

        write!(w, "return state || ")?;
        self.write_object(w, ctx, expr)?;
        write!(w, ";")?;

        Ok(())
    }
}

/// Queries

impl ObjectWriter<Query<ProcessedExpression>, JsOutput> for DefaultJsWriter {
    fn write_object(
        &mut self,
        w: &mut io::Write,
        ctx: &mut OutputContext,
        query: &Query<ProcessedExpression>,
    ) -> DocumentProcessingResult<()> {
        for component in query.components() {
            eprintln!(
                "ObjectWriter QueryComponentValue<ProcessedExpression> (JS): component: {:?}",
                component
            );

            match *component {
                QueryComponent::CaseWhere(box ref expr, box ref cond, _) => {
                    write!(w, "        if (")?;
                    self.write_object(w, ctx, cond)?;
                    write!(w, ") {{ return ")?;
                    self.write_object(w, ctx, expr)?;
                    writeln!(w, "; }}")?;
                }
            }
        }

        Ok(())
    }
}

/// Actions

fn write_action_prop_value<'s>(
    _self: &'s mut DefaultJsWriter,
    w: &mut io::Write,
    ctx: &mut OutputContext,
    prop: &PropValue<ProcessedExpression>,
) -> DocumentProcessingResult<()> {
    eprintln!("[Action Prop Value] prop: {:?}", prop);

    // // Special case: lookup value by path alias
    // if let ExpressionValue::Expression(Expression::Path(ref path_value, _)) = *expr {
    //     let alias_string = path_value.component_string();
    //     let binding = CommonBindings::PathAlias(alias_string, Default::default());
    //     if let Some(alias_target) = ctx.find_value(&binding)? {
    //         if let ExpressionValue::Expression(Expression::RawPath(ref s, _)) = alias_target {
    //             // write!(w, "props.{}", s)?;
    //             write!(w, "{}", s)?;
    //             return Ok(());
    //         };
    //         // return _self.write_object(w, ctx, &alias_target);
    //     };
    // };
    let value = prop.value();

    let is_primitive = value.is_primitive();
    let environment = ctx.environment()?;

    if !is_primitive && environment != Some(OutputScopeEnvironment::RouteDispatchAction) {
        let alias = prop.key().to_owned();
        let binding: CommonBindings<ProcessedExpression> = CommonBindings::NamedComponentProp(alias.clone(), Default::default());
        let expr = ExpressionValue::Binding(binding, Default::default());

        return _self.write_object(w, ctx, &expr);
    };

    _self.write_object(w, ctx, value)
}

impl ObjectWriter<ActionOp<ProcessedExpression>, JsOutput> for DefaultJsWriter {
    fn write_object(
        &mut self,
        w: &mut io::Write,
        ctx: &mut OutputContext,
        obj: &ActionOp<ProcessedExpression>,
    ) -> DocumentProcessingResult<()> {
        match *obj {
            ActionOp::DispatchAction(ref name, ref props, _)
            | ActionOp::DispatchActionTo(ref name, ref props, _, _) => {
                let action = match *obj {
                    ActionOp::DispatchActionTo(_, _, ref target, _) => {
                        format!("{}.{}", target.to_uppercase(), name.to_uppercase())
                    }
                    _ => name.to_uppercase(),
                };
                write!(w, "            store.dispatch({{type: \"{}\"", action)?;
                if let Some(box ref props) = *props {
                    for prop in props {
                        write!(w, ", \"{}\": ", prop.key())?;
                        write_action_prop_value(self, w, ctx, prop)?;
                    }
                };
                writeln!(w, "}});")?;
            }

            ActionOp::Navigate(ref prop, _) => {
                write!(w, "            window._go(")?;
                let prop = PropValue::new("path".to_owned(), prop.to_owned(), None);
                write_action_prop_value(self, w, ctx, &prop)?;
                write!(w, ");")?;
            }
        };

        Ok(())
    }
}

/// Routes

impl ObjectWriter<RouteActionValue<ProcessedExpression>, JsOutput> for DefaultJsWriter {
    fn write_object(
        &mut self,
        w: &mut io::Write,
        ctx: &mut OutputContext,
        obj: &RouteActionValue<ProcessedExpression>,
    ) -> DocumentProcessingResult<()> {
        match *obj {
            RouteActionValue::Block(ref block, _) => {
                self.write_object(w, ctx, block)?;
            }

            RouteActionValue::Actions(Some(ref actions), _) => for action in actions {
                ctx.push_child_scope_with_environment(OutputScopeEnvironment::RouteDispatchAction);
                self.write_object(w, ctx, action)?;
                ctx.pop_scope();
            },
            RouteActionValue::Actions(..) => {}
        }

        Ok(())
    }
}
