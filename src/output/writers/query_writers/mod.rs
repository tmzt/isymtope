use std::io;

use processing::*;
use scope::*;
use output::*;


pub trait QueryWriter {
    fn write_query(&mut self, w: &mut io::Write, doc: &Document, ctx: &mut Context, bindings: &BindingContext, query: &Query) -> Result;
}

impl<O: OutputWriter + ExprWriter + JavascriptUtilWriter> QueryWriter for O {
    fn write_query(&mut self, w: &mut io::Write, doc: &Document, ctx: &mut Context, bindings: &BindingContext, query: &Query) -> Result {
        let name = query.name();
        writeln!(w, "                function query_{}(props) {{", name)?;
        if let Some(iter) = query.components_iter() {
            for query_comp in iter {
                let (val, cond) = (query_comp.expr(), query_comp.cond());
                if val.is_some() && cond.is_some() {
                    write!(w, "                  if (")?;
                    self.write_expr(w, doc, ctx, bindings, cond.unwrap())?;
                    write!(w, ") {{ return ")?;
                    self.write_expr(w, doc, ctx, bindings, val.unwrap())?;
                    writeln!(w, "; }}")?;
                }
            }
        };
        writeln!(w, "                }}")?;
        Ok(())
    }
}