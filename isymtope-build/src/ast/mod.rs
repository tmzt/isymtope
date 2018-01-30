pub mod loc;
pub mod nodes;

pub use self::loc::*;
pub use self::nodes::*;

use expressions::*;


// #[cfg(feature="parser")]
// pub type LocToken<Inner> = Loc<Inner, (usize, usize)>;

#[derive(Debug, Default, Clone, PartialEq, Eq, Hash)]
pub struct Template(Vec<TemplateNode<SourceExpression>>);

impl Template {
    pub fn new(children: Vec<TemplateNode<SourceExpression>>) -> Self {
        Template(children)
    }

    pub fn children<'a>(&'a self) -> impl Iterator<Item = &'a TemplateNode<SourceExpression>> {
        self.0.iter()
    }
}