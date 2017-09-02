pub mod element_ops_writer;

pub use self::element_ops_writer::*;

use std::io;
use std::iter;

use parser::ast::*;
use processing::structs::*;
use scope::context::*;
use scope::bindings::*;
use output::writers::*;


pub trait ElementOpsWriter {
    type E: ExpressionWriter;
    type S: ElementOpsStreamWriter<E = Self::E>;

    fn write_element_op(&mut self, w: &mut io::Write, ctx: &mut Context, bindings: &BindingContext, op: &ElementOp) -> Result;
    fn write_element_ops<'a, I: IntoIterator<Item = &'a ElementOp>>(&mut self, w: &mut io::Write, ctx: &mut Context, bindings: &BindingContext, ops: I) -> Result;
}
