
use std::io;
use std::fmt;
use parser::ast::*;

mod format_html {
    use std::fmt::{self, Write};
    use parser::ast::*;

    pub struct FormatHtml {
    }

    impl FormatHtml {
        pub fn new() -> FormatHtml {
            FormatHtml {}
        }

        pub fn write_computed_expr_value(&self, w: &mut fmt::Write, node: &ExprValue) -> fmt::Result {
            match node {
                &ExprValue::LiteralString(ref s) => { write!(w, "{}", s)?; },
                _ => {}
            }
            Ok(())
        }

        pub fn write_html_content(&self, w : &mut fmt::Write, node: &ContentNodeType) -> fmt::Result {
            // Write node
            match node {
                &ContentNodeType::ElementNode(ref element_data) => {
                    let element_tag = element_data.element_ty.to_uppercase();
                    let mut attrs_str = String::new();

                    if let Some(ref attrs) = element_data.attrs {
                        for &(ref key, ref expr) in attrs.iter() {
                            let mut expr_str = String::new();
                            self.write_computed_expr_value(&mut expr_str, &expr)?;
                            write!(attrs_str, " {}=\"{}\"", key, expr_str)?;
                        }
                    }

                    // For now, assume these are HTML nodes
                    write!(w, "<{}{}>",
                        element_tag,
                        attrs_str
                    )?;

                    if let Some(ref children) = element_data.children {
                        for ref child in children {
                            self.write_html_content(w, child)?;
                        }
                    }

                    write!(w, "</{}>", element_tag)?;
                },
                // Ignore other node types
                _ => {}
            }
            Ok(())
        }

        pub fn write_html_document(&self, w : &mut fmt::Write, ast: &Template) -> fmt::Result {
            writeln!(w, "<!doctype HTML>")?;
            writeln!(w, "<html>")?;
            writeln!(w, "<body>")?;
            writeln!(w, "<div id=\"root\">")?;
            
            for ref loc in ast.children.iter() {
                match &loc.inner {
                    &NodeType::ContentNode(ref content) => {
                        self.write_html_content(w, &content)?
                    },
                    _ => {}
                }
            }

            writeln!(w, "</div>")?;
            writeln!(w, "</body>")?;
            writeln!(w, "</html>")?;
            Ok(())
        }
    }
}

use self::format_html::FormatHtml;

pub type Result = io::Result<fmt::Result>;

pub struct ClientOutput {
}

impl ClientOutput {
    pub fn new() -> ClientOutput {
        ClientOutput {}
    }

    pub fn write_html(&self, w : &mut io::Write, ast: &Template) -> Result {
        let format = FormatHtml::new();

        let mut doc_str = String::new();
        if let Err(e) = format.write_html_document(&mut doc_str, ast) {
            return Ok(Err(e));
        }

        if let Err(e) = w.write_fmt(format_args!("{}", doc_str)) {
            return Err(e);
        }

        Ok(Ok(()))
    }
}