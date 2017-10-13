
use std::io;

use model::*;
use parser::*;
use scope::*;
use processing::*;
use output::*;


pub trait PathWriter {
    fn write_path(&mut self, w: &mut io::Write, ctx: &mut Context) -> Result;
    fn write_path_with<T: AsExpr>(&mut self, w: &mut io::Write, ctx: &mut Context, suffix: &T) -> Result;
}

//#[derive(Debug, Default)]
//pub struct PathWriterJs {}

impl<W: ExprWriter> PathWriter for W {
    default fn write_path(&mut self, w: &mut io::Write, ctx: &mut Context) -> Result {
    	Ok(())
    }

    default fn write_path_with<T: AsExpr>(&mut self, w: &mut io::Write, ctx: &mut Context, suffix: &T) -> Result {
    	Ok(())
    }
}


#[cfg(test)]
mod tests {
    use super::*;
    use std::io::Write;
    use std::str;
    use processing::*;
    use scope::*;
    use output::*;

    #[macro_use]
    use output::tests;


    #[test]
    pub fn test_output_path_writer_js() {
        let template = Template::new(vec![]);

        let mut s: Vec<u8> = Default::default();
	let mut writer = DefaultOutputWriterJs::default();

        test_writing_to!(
            template,
	    move |_, _, _| Ok(()),
            |doc, ctx: &mut Context, bindings| -> Result {
                ctx.append_path_str("prefix");

		writer.write_path(&mut s, ctx)?;
		writeln!(&mut s, "")?;
		writer.write_path_with(&mut s, ctx, &"suffix")?;

                Ok(())
            }
        );

        assert_eq!(str::from_utf8(&s), Ok("prefix\nprefix.suffix".into()));
    }

}
