pub mod element_ops_writer;

pub use self::element_ops_writer::*;

use std::io;

use parser::*;
use processing::*;
use scope::*;


pub trait EventCollector {
    fn event<'a, I: IntoIterator<Item = &'a PropRef<'a>>>(&mut self, instance_key: &str, event: &EventsItem, props: I) -> Result;
}

pub trait ElementOpsWriter {
    // type O: OutputWriter;
    // type E: ExpressionWriter;
    // type S: ElementOpsStreamWriter<E = Self::E>;

    fn write_element_op(&mut self, w: &mut io::Write, doc: &Document, ctx: &mut Context, bindings: &BindingContext, op: &ElementOp) -> Result;
    fn write_element_ops<'a, 'e, I: IntoIterator<Item = &'a ElementOp>>(&mut self, w: &mut io::Write, doc: &Document, ctx: &mut Context, bindings: &BindingContext, ops: I) -> Result;
}
