
use std::io;

use processing::structs::*;
use output::writers::*;


#[derive(Debug, Default)]
pub struct ValueWriterJs {}

impl ValueWriter for ValueWriterJs {
    fn write_literal_string(&mut self, w: &mut io::Write, s: &str) -> Result {
        write!(w, "\"{}\"", s)?;
        Ok(())
    }

    fn write_literal_number(&mut self, w: &mut io::Write, n: &i32) -> Result {
        write!(w, "{}", n)?;
        Ok(())
    }

    fn write_literal_bool(&mut self, w: &mut io::Write, b: bool) -> Result {
        if b { write!(w, "true")?; } else { write!(w, "false")?; }
        Ok(())
    }

    fn write_simple_binding(&mut self, w: &mut io::Write, _ctx: &mut Context, _bindings: &BindingContext, binding: &BindingType) -> Result {
        match binding {
            &BindingType::ReducerPathBinding(ref symbol_path) => {
                write!(w, "store.getState().{}", symbol_path)?;
            }
            &BindingType::ActionStateBinding => {
                write!(w, "state")?;
            }
            &BindingType::ActionParamBinding(ref key) => {
                write!(w, "action.{}", key)?;
            }
            &BindingType::ComponentKeyBinding => {
                write!(w, "key")?;
            }
            &BindingType::ComponentPropBinding(ref key) => {
                write!(w, "props.{}", key)?;
            }
            &BindingType::MapIndexBinding => {
                write!(w, "idx")?;
            }
            &BindingType::MapItemBinding => {
                write!(w, "item")?;
            }
            _ => {}
        };
        Ok(())
    }

    fn write_op(&mut self, w: &mut io::Write, op: &ExprOp) -> Result {
        match op {
            &ExprOp::Add => { write!(w, "+")?; },
            _ => {}
        };
        Ok(())
    }

    fn write_undefined(&mut self, w: &mut io::Write) -> Result {
        write!(w, "undefined")?;
        Ok(())
    }
}
impl DynamicValueWriter for ValueWriterJs {}

#[derive(Debug, Default)]
pub struct ExpressionWriterJs {}

// impl ExpressionWriter for ExpressionWriterJs {}
impl ExpressionWriter for ExpressionWriterJs {
    type V = ValueWriterJs;

    fn write_expression(&mut self, w: &mut io::Write, doc: &Document, value_writer: &mut Self::V, ctx: &mut Context, bindings: &BindingContext, op: &ExprOp, left: &ExprValue, right: &ExprValue) -> Result {
        if left.peek_is_array() || right.peek_is_array() {
            self.write_expr_to(w, doc, value_writer, ctx, bindings, left)?;
            write!(w, ".concat(")?;
            self.write_expr_to(w, doc, value_writer, ctx, bindings, right)?;
            write!(w, ")")?;
            return Ok(())
        };

        if left.peek_is_object() || right.peek_is_object() {
            write!(w, "Object.assign({{}}, ")?;
            self.write_expr_to(w, doc, value_writer, ctx, bindings, left)?;
            write!(w, ", ")?;
            self.write_expr_to(w, doc, value_writer, ctx, bindings, right)?;
            write!(w, ")")?;
            return Ok(())
        };

        self.write_expr_to(w, doc, value_writer, ctx, bindings, left)?;
        value_writer.write_op(w, op)?;
        self.write_expr_to(w, doc, value_writer, ctx, bindings, right)?;

        Ok(())
    }

    fn write_apply_expression<'a, I: IntoIterator<Item = &'a ExprValue>>(&mut self, w: &mut io::Write, doc: &Document, value_writer: &mut Self::V, ctx: &mut Context, bindings: &BindingContext, a_op: &ExprApplyOp, arr: Option<I>) -> Result {
        match a_op {
            &ExprApplyOp::JoinString(ref sep) => {
                self.write_array(w, doc, value_writer, ctx, bindings, arr, None)?;
                write!(w, ".join(\"{}\")", sep.as_ref().map_or("", |s| s.as_str()))?;
            },
            _ => {}
        };
        Ok(())
    }

    fn write_array<'a, I: IntoIterator<Item = &'a ExprValue>>(&mut self, w: &mut io::Write, doc: &Document, value_writer: &mut Self::V, ctx: &mut Context, bindings: &BindingContext, arr: Option<I>, _ty: Option<VarType>) -> Result {
        write!(w, "[")?;
        let mut first = true;
        if let Some(arr) = arr {
            for v in arr {
                if !first { write!(w, ", ")?; }
                self.write_expr_to(w, doc, value_writer, ctx, bindings, v)?;
                first = false;
            }
        };
        write!(w, "]")?;
        Ok(())
    }

    fn write_props<'a, I: IntoIterator<Item = &'a Prop>>(&mut self, w: &mut io::Write, doc: &Document, value_writer: &mut Self::V, ctx: &mut Context, bindings: &BindingContext, props: Option<I>) -> Result {
        write!(w, "{{")?;
        if let Some(props) = props {
            let mut first = true;
            for prop in props {
                if !first { write!(w, ", ")?; }
                write!(w, "\"{}\": ", &prop.0)?;
                if let Some(ref v) = prop.1 {
                    self.write_expr_to(w, doc, value_writer, ctx, bindings, v)?;
                }
                first = false;
            };
        };
        write!(w, "}}")?;
        Ok(())
    }

    fn write_binding(&mut self, w: &mut io::Write, doc: &Document, value_writer: &mut Self::V, ctx: &mut Context, bindings: &BindingContext, binding: &BindingType) -> Result {
        match binding {
            &BindingType::DOMElementAttributeBinding(ref complete_key, ref attr_name) => {
                let path_expr = ctx.join_path_as_expr_with(Some("."), complete_key);
                write!(w, "document.querySelector(\"[key='\" + ")?;
                self.write_expr_to(w, doc, value_writer, ctx, bindings, &path_expr)?;
                write!(w, " + \"']\").getAttribute(\"{}\")", attr_name)?;
                Ok(())
            },
            &BindingType::DOMInputElementValueBinding(ref complete_key) => {
                let path_expr = ctx.join_path_as_expr_with(Some("."), complete_key);
                write!(w, "document.querySelector(\"[key='\" + ")?;
                self.write_expr_to(w, doc, value_writer, ctx, bindings, &path_expr)?;
                write!(w, " + \"']\").value")?;
                Ok(())
            }
            _ => value_writer.write_simple_binding(w, ctx, bindings, binding)
        }
    }

    fn write_group(&mut self, w: &mut io::Write, doc: &Document, value_writer: &mut Self::V, ctx: &mut Context, bindings: &BindingContext, inner_expr: Option<&ExprValue>) -> Result {
        write!(w, "(")?;
        if let Some(ref inner_expr) = inner_expr {
            self.write_expr_to(w, doc, value_writer, ctx, bindings, inner_expr)?;
        };
        write!(w, ")")?;
        Ok(())
    }

    fn write_symbol(&mut self, w: &mut io::Write, doc: &Document, value_writer: &mut Self::V, ctx: &mut Context, bindings: &BindingContext, sym: &Symbol) -> Result {
        match sym.sym_ref() {
            &SymbolReferenceType::InitialValue(_, box ref after) => self.write_symbol(w, doc, value_writer, ctx, bindings, after),
            &SymbolReferenceType::Binding(ref binding) => self.write_binding(w, doc, value_writer, ctx, bindings, binding),
            &SymbolReferenceType::MemberPath(box ref first, ref parts) => {
                self.write_symbol(w, doc, value_writer, ctx, bindings, first)?;
                if let &Some(ref parts) = parts {
                    write!(w, ".{}", parts)?;
                    // for part in parts.iter() {
                    //     write!(w, ".{}", part)?;
                    // };
                };
                Ok(())
            }
            _ => Ok(())
        }
    }

}


#[cfg(test)]
mod tests {
    use super::*;
    use std::str;
    use std::io::Write;
    use scope::*;
    use output::*;


    fn create_document<'a>(template: &'a Template) -> Document {
        let mut ctx = Context::default();
        let mut bindings = BindingContext::default();
        let mut processing = ProcessDocument::from_template(&template);
        assert!(processing.process_document(&mut ctx, &mut bindings).is_ok());
        processing.into()
    }

    #[test]
    fn test_stream_writers_value_writer_js_write_simple_binding1() {
        let template = Template::new(vec![]);
        let doc = create_document(&template);

        let mut value_writer = ValueWriterJs::default();
        let mut ctx = Context::default();
        let binding = BindingType::ReducerPathBinding("todo".into());

        {
            let mut s: Vec<u8> = Default::default();
            let bindings = BindingContext::default();
            let res = value_writer.write_simple_binding(&mut s, &mut ctx, &bindings, &binding);
            assert!(res.is_ok());
            assert_eq!(str::from_utf8(&s), Ok("store.getState().todo".into()));
        }

        {
            let mut s: Vec<u8> = Default::default();
            let bindings = BindingContext::default();
            // let mut expr_writer = DefaultExpressionWriter::default();
            // let mut expr_writer = ExpressionWriterJs::default();
            let expr = ExprValue::Binding(binding.clone());

            // let mut writer: DefaultOutputWriter<ValueWriterJs, ExpressionWriterJs, ElementOpsStreamWriterJs> = DefaultOutputWriter::default();
            let mut writer = DefaultOutputWriterJs::default();

            let res = writer.write_expr(&mut s, &doc, &mut ctx, &bindings, &expr);
            assert!(res.is_ok());
            assert_eq!(str::from_utf8(&s), Ok("store.getState().todo".into()));
        }
    }

    #[test]
    fn test_stream_writers_value_writer_js_write_dynamic_expression1() {
        let template = Template::new(vec![]);
        let doc = create_document(&template);

        let bindings = BindingContext::default();
        let mut ctx = Context::default();
        let binding = BindingType::ReducerPathBinding("todo".into());
        let literal_string = ExprValue::LiteralString("test".into());

        let expr = ExprValue::Expr(ExprOp::Add,
            Box::new(ExprValue::Binding(binding.clone())),
            Box::new(literal_string.clone())
        );

        let mut writers = DefaultOutputWritersBoth::default();
        let mut s: Vec<u8> = Default::default();
        let res = writers.js().write_expr(&mut s, &doc, &mut ctx, &bindings, &expr);
        assert!(res.is_ok());
        assert_eq!(str::from_utf8(&s), Ok("store.getState().todo+\"test\"".into()));        
    }

    #[test]
    fn test_stream_writers_writerjs_write_dynamic_expression1() {
        let template = Template::new(vec![]);
        let doc = create_document(&template);

        let bindings = BindingContext::default();
        let mut ctx = Context::default();

        let binding = BindingType::ReducerPathBinding("todo".into());
        let literal_string = ExprValue::LiteralString("test".into());

        let expr = ExprValue::Expr(ExprOp::Add,
            Box::new(ExprValue::Binding(binding.clone())),
            Box::new(literal_string.clone())
        );

        let mut s: Vec<u8> = Default::default();
        let mut writers = DefaultOutputWritersBoth::default();

        let res = writers.js().write_expr(&mut s, &doc, &mut ctx, &bindings, &expr);
        assert!(res.is_ok());
        assert_eq!(str::from_utf8(&s), Ok("store.getState().todo+\"test\"".into()));
    }
}