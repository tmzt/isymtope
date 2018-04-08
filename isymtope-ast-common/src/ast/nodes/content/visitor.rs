use error::*;
use super::*;

pub trait ContentNodeVisitor<T> {
    fn visit_nodes<F: FnMut(&ContentNode<T>) -> DocumentProcessingResult<()>>(&mut self, n: &ContentNode<T>, f: &mut F) -> DocumentProcessingResult<()>;
    fn visit_elements<F: FnMut(&ElementNode<T>) -> DocumentProcessingResult<()>>(&mut self, n: &ContentNode<T>, f: &mut F) -> DocumentProcessingResult<()>;
    fn visit_value_bindings<F: FnMut(&ElementNode<T>, &ElementValueBinding<T>) -> DocumentProcessingResult<()>>(&mut self, n: &ContentNode<T>, f: &mut F) -> DocumentProcessingResult<()>;
}

#[derive(Debug, Default)]
pub struct DefaultContentNodeVisitor;

impl<T> ContentNodeVisitor<T> for DefaultContentNodeVisitor {
    fn visit_nodes<F: FnMut(&ContentNode<T>) -> DocumentProcessingResult<()>>(&mut self, n: &ContentNode<T>, f: &mut F) -> DocumentProcessingResult<()> {
        // Visit node first
        f(n)?;

        // Visit children
        if let ContentNode::Element(ref element, _) = n {
            if let Some(children) = element.children() {
                for child in children {
                    self.visit_nodes(child, f)?;
                }
            }
        };

        Ok(())
    }

    fn visit_elements<F: FnMut(&ElementNode<T>) -> DocumentProcessingResult<()>>(&mut self, n: &ContentNode<T>, f: &mut F) -> DocumentProcessingResult<()> {
        if let ContentNode::Element(ref element, _) = n {
            // Visit node first
            f(element)?;

            // Visit children
            if let Some(children) = element.children() {
                for child in children {
                    self.visit_elements(child, f)?;
                }
            };
        };

        Ok(())
    }

    fn visit_value_bindings<F: FnMut(&ElementNode<T>, &ElementValueBinding<T>) -> DocumentProcessingResult<()>>(&mut self, n: &ContentNode<T>, f: &mut F) -> DocumentProcessingResult<()> {
        self.visit_elements(n, &mut |element| {
            if let Some(bindings) = element.bindings() {
                if let Some(value_binding) = bindings
                    .filter_map(|b| match *b {
                        ElementBinding::Value(ref b, _) => Some(b),
                        _ => None,
                    })
                    .nth(0)
                {
                    f(element, value_binding)?;
                };
            };

            Ok(())
        })
    }
}
