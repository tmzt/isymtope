
use std::io;
use std::slice::Iter;

use parser::ast::*;
use processing::structs::*;
use processing::scope::*;
use scope::bindings::*;


#[derive(Debug)]
pub struct ValueWriterJs {}

impl ValueWriter for ValueWriterJs {
    fn write_binding(&mut self, w: &mut io::Writer, ctx: &mut Context, binding: &BindingType) -> Result;
}


#[cfg(test)]
mod tests {
    use super::*;
    use scope::bindings::*;


    #[test]
    fn test_stream_writers_value_writer_write_binding1() {
        let mut s = String::new();
        let mut value_writer = ValueWriterJs::default();

        let mut ctx = Context::default();
        let binding = BindingType::ReducerPathBinding("todo", None);

        let res = value_writer.write_binding(&mut s, &mut ctx, binding);
        assert_eq!(res, Ok(()));
        assert_eq!(s, "".into());
    }
}