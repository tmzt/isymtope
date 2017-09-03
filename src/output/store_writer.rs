
use std::io;
use std::iter;
use std::slice::Iter;

use parser::ast::*;
use processing::structs::*;

use output::writers::*;
use scope::scope::*;
use scope::context::*;
use scope::bindings::*;


pub trait StoreWriter {
    type E: ExpressionWriter;

    fn write_store_to<'a, I: IntoIterator<Item = (&'a str, &'a ReducerKeyData)>>(&mut self, w: &mut io::Write, expression_writer: &mut Self::E, value_writer: &mut <Self::E as ExpressionWriter>::V, ctx: &mut Context, bindings: &BindingContext, reducers: I) -> Result;
}

#[derive(Debug, Default)]
pub struct StoreWriterJs {}

impl StoreWriterJs {
    #[inline]
    pub fn write_reducer_action(&mut self, w: &mut io::Write, expression_writer: &mut <Self as StoreWriter>::E, value_writer: &mut <<Self as StoreWriter>::E as ExpressionWriter>::V, ctx: &mut Context, bindings: &BindingContext, reducer: &ReducerKeyData, action: &ReducerActionData) -> Result {
        // let action_ty = format!("{}.{}", reducer_scope_key, &action_data.action_type);
        let action_key = ctx.join_action_path_with(Some("."), &action.action_type);

        if let &Some(ActionStateExprType::SimpleReducerKeyExpr(ref expr)) =  &action.state_expr {
            // let mut scope = scope.clone();
            // scope.0.set_default_var("state");

            writeln!(w,
                        "if ('undefined' !== typeof action && '{}' == action.type) \
                        {{",
                        action_key)
                ?;
            write!(w, "  return ")?;
            // write_js_expr_value(w, expr, &self.doc, &scope)?;
            expression_writer.write_expr_to(w, value_writer, ctx, bindings, expr)?;
            writeln!(w, ";")?;
            writeln!(w, "}}")?;
        };

        Ok(())
    }

    #[inline]
    pub fn write_reducer_definition(&mut self, w: &mut io::Write, expression_writer: &mut <Self as StoreWriter>::E, value_writer: &mut <<Self as StoreWriter>::E as ExpressionWriter>::V, ctx: &mut Context, bindings: &BindingContext, reducer: &ReducerKeyData) -> Result {
        ctx.push_child_scope();
        // ctx.append_action_path_str(&reducer.reducer_key);

        let complete_key = ctx.join_action_path_with(Some("."), &reducer.reducer_key);
        writeln!(w, "  function {}Reducer(state, action) {{", complete_key)?;

        if let Some(ref actions) = reducer.actions {
            for ref action in actions {
                self.write_reducer_action(w, expression_writer, value_writer, ctx, bindings, reducer, action)?;
                // let action_ty = format!("{}.{}", reducer_scope_key, &action_data.action_type);
            }
        };

        // Default expression used to initialize state
        write!(w, "    return state || ")?;
        if let Some(ref expr) = reducer.default_expr {
            expression_writer.write_expr_to(w, value_writer, ctx, bindings, expr)?;
        } else {
            write!(w, "null")?;
        }
        writeln!(w, ";")?;

        writeln!(w, "  }}")?;

        ctx.pop_scope();
        Ok(())
    }
}

impl StoreWriter for StoreWriterJs {
    type E = ExpressionWriterJs;

    fn write_store_to<'a, I: IntoIterator<Item = (&'a str, &'a ReducerKeyData)>>(&mut self, w: &mut io::Write, expression_writer: &mut Self::E, value_writer: &mut <Self::E as ExpressionWriter>::V, ctx: &mut Context, bindings: &BindingContext, reducers: I) -> Result {
        // for (_, ref reducer_data) in doc.reducer_key_data.iter() {
        for (reducer_key, reducer) in reducers {
            self.write_reducer_definition(w, expression_writer, value_writer, ctx, bindings, reducer)?;
        }
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
        // let node_1 = DefaultScopeNodeType::LetNode("todos".into(), Some(ExprValue::LiteralNumber(0)));
        // let node_2 = DefaultScopeNodeType::ScopeNode("todos".into(), vec![
        //     ScopeNodeType::ActionNode("add".into(), Some(ActionStateExprType::SimpleReducerKeyExpr(ExprValue::Expr(
        //         ExprOp::Add,
        //         Box::new(ExprValue::SymbolReference(Symbol::unresolved("todos"))),
        //         Box::new(ExprValue::SymbolReference(Symbol::unresolved("value")))
        //     ))), None)
        // ]);
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

    fn prepare_document<'a>(template: &'a Template) -> DocumentState<'a> {
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

        let mut value_writer = ValueWriterJs::default();
        let mut expression_writer = ExpressionWriterJs::default();
        let mut store_writer = StoreWriterJs::default();
        // let reducer_iter = doc.reducer_key_data.iter();
        let reducer_iter = doc.reducers_iter();
        let res = store_writer.write_store_to(&mut s, &mut expression_writer, &mut value_writer, &mut ctx, &bindings, reducer_iter);
        assert!(res.is_ok());
        assert_diff!(str::from_utf8(&s).unwrap(),
r#"  function todosReducer(state, action) {
if ('undefined' !== typeof action && 'TODOS.ADD' == action.type) {
  return state+action.value;
}
    return state || 0;
  }
"#, "\n", 0);
    }
}