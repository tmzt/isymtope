
use std::io;

use model::*;
use parser::*;
use scope::*;
use processing::*;
use output::*;


pub trait StoreWriter {
    // type E: ExprWriter;
    type O: OutputWriter;

    fn write_store<'a, I: IntoIterator<Item = (&'a str, &'a ReducerKeyData)>>(&mut self, w: &mut io::Write, doc: &Document, output_writer: &mut Self::O, ctx: &mut Context, bindings: &BindingContext, reducers: I) -> Result;
}

#[derive(Debug, Default)]
pub struct StoreWriterJs {}

impl StoreWriterJs {
    #[inline]
    pub fn write_reducer_action(&mut self, w: &mut io::Write, doc: &Document, output_writer: &mut <Self as StoreWriter>::O, ctx: &mut Context, bindings: &BindingContext, _reducer: &ReducerKeyData, action: &ReducerActionData) -> Result {
        let action_key = ctx.join_action_path_with(Some("."), &action.action_type);

        writeln!(w, "                  /* action: {:?} */", action)?;

        if let Some(ActionStateExprType::SimpleReducerKeyExpr(ref expr)) =  action.state_expr {
            let expr = ctx.map_reducer_expression(doc, &action_key, expr);
            writeln!(w, "                  /* action expr: {:?} */", expr)?;

            writeln!(w,
                        "                  if ('undefined' !== typeof action && '{}' == action.type) \
                                           {{",
                        action_key)
                ?;
            write!(w, "                    return ")?;

            // let expr = match expr {
            //     ExprValue::IterMethodPipeline(ref head, Some(ref parts)) => {
            //         let head = head.as_ref().map(|&box ref head| head);
            //         ctx.reduce_pipeline(head, parts.into_iter())
            //     }
            //     _ => None
            // }.unwrap_or(expr);

            output_writer.write_expr(w, doc, ctx, bindings, &expr)?;
            // expression_writer.write_expr_to(w, value_writer, ctx, bindings, expr)?;
            writeln!(w, ";")?;
            writeln!(w, "                  }}")?;
        };

        Ok(())
    }

    #[inline]
    pub fn write_reducer_definition(&mut self, w: &mut io::Write, doc: &Document, output_writer: &mut <Self as StoreWriter>::O, ctx: &mut Context, bindings: &BindingContext, reducer: &ReducerKeyData) -> Result {
        ctx.push_child_scope();

        let complete_key = ctx.join_action_path_with(Some("_"), &reducer.reducer_key);
        writeln!(w, "")?;
        writeln!(w, "                function {}Reducer(state, action) {{", complete_key)?;

        if let Some(ref actions) = reducer.actions {
            for action in actions {
                self.write_reducer_action(w, doc, output_writer, ctx, bindings, reducer, action)?;
            }
        };

        // Default expression used to initialize state
        write!(w, "                  return state || ")?;
        if let Some(ref expr) = reducer.default_expr {
            output_writer.write_expr(w, doc, ctx, bindings, expr)?;
        } else {
            write!(w, "null")?;
        }
        writeln!(w, ";")?;

        writeln!(w, "                }}")?;

        ctx.pop_scope();
        Ok(())
    }

    #[inline]
    fn write_root_reducer_definition<'a, I: IntoIterator<Item = &'a str>>(&mut self, w: &mut io::Write, _doc: &Document, _output_writer: &mut <Self as StoreWriter>::O, ctx: &mut Context, _bindings: &BindingContext, keys: I) -> Result {
        writeln!(w, "")?;
        writeln!(w, "                var rootReducer = Redux.combineReducers({{")?;
        for reducer_key in keys {
            let complete_key = ctx.join_action_path_with(Some("_"), reducer_key);
            writeln!(w, "                  {}: {}Reducer,", complete_key, complete_key)?;
        }
        writeln!(w, "                }});")?;
        Ok(())
    }
}

impl StoreWriter for StoreWriterJs {
    type O = DefaultOutputWriterJs;

    fn write_store<'a, I: IntoIterator<Item = (&'a str, &'a ReducerKeyData)>>(&mut self, w: &mut io::Write, doc: &Document, output_writer: &mut Self::O, ctx: &mut Context, bindings: &BindingContext, reducers: I) -> Result {
        let mut keys: Vec<String> = Default::default();

        for (_, reducer) in reducers {
            self.write_reducer_definition(w, doc, output_writer, ctx, bindings, reducer)?;
            keys.push(reducer.reducer_key.to_owned());
        }
        self.write_root_reducer_definition(w, doc, output_writer, ctx, bindings, keys.iter().map(|s| s.as_str()))?;
        Ok(())
    }
}


#[cfg(test)]
mod tests {
    use super::*;
    use std::str;
    use processing::*;
    use scope::context::*;
    use scope::bindings::*;
    use processing::structs::*;
    use output::*;


    fn create_template() -> Template {
        let store_nodes = vec![
            DefaultScopeNodeType::LetNode("todos".into(), Some(ExprValue::LiteralNumber(0))),
            DefaultScopeNodeType::ScopeNode("todos".into(), vec![
                ScopeNodeType::ActionNode("add".into(), Some(ActionStateExprType::SimpleReducerKeyExpr(ExprValue::Expr(
                    ExprOp::Add,
                    Box::new(ExprValue::SymbolReference(Symbol::unresolved("todos"))),
                    Box::new(ExprValue::SymbolReference(Symbol::unresolved("value")))
                ))), Some(vec!["value".into()]))
            ])
        ];
        let nodes: Vec<Loc<NodeType, (usize, usize)>> = vec![
            Loc {inner: NodeType::StoreNode(store_nodes), pos: (0,0)},
        ];
        Template { children: nodes }
    }

    fn prepare_document<'a>(template: &'a Template) -> Document {
        let mut ctx = Context::default();
        let mut bindings = BindingContext::default();
        let mut processing = ProcessDocument::from_template(&template);
        assert!(processing.process_document(&mut ctx, &mut bindings).is_ok());
        processing.into()
    }

    #[test]
    pub fn test_output_storewriter() {
        let template = create_template();
        let doc = prepare_document(&template);

        let mut ctx = Context::default();
        let bindings = BindingContext::default();
        let mut s: Vec<u8> = Default::default();

        // let mut value_writer = ValueWriterJs::default();
        // let mut expression_writer = ExpressionWriterJs::default();
        // let mut store_writer = StoreWriterJs::default();

        let mut writers = DefaultOutputWritersBoth::default();
        let mut store_writer = StoreWriterJs::default();
        let reducer_iter = doc.reducers_iter();
        let res = store_writer.write_store(&mut s, &doc, writers.js(), &mut ctx, &bindings, reducer_iter);
        assert!(res.is_ok());
        assert_diff!(str::from_utf8(&s).unwrap(),
r#"
                function todosReducer(state, action) {
                  /* action: ReducerActionData { action_type: "TODOS.ADD", state_expr: Some(SimpleReducerKeyExpr(Expr(Add, SymbolReference(Symbol(Binding(ActionStateBinding), None, None)), SymbolReference(Symbol(Binding(ActionStateBinding), None, None))))), state_ty: Some(Primitive(Number)), default_scope_key: Some("todos") } */
                  if ('undefined' !== typeof action && 'TODOS.ADD' == action.type) {
                    return state+state;
                  }
                  return state || 0;
                }

                var rootReducer = Redux.combineReducers({
                  todos: todosReducer,
                });
"#, "\n", 0);
    }
}