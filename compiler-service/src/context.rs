use isymtope_generate::*;
use super::*;

pub trait CompilerContext {
    fn handle_msg(&mut self, msg: CompilerRequestMsg) -> IsymtopeGenerateResult<CompilerResponseMsg>;
}

#[derive(Debug)]
pub struct DefaultCompilerContext {
}

impl DefaultCompilerContext {
    pub fn new() -> Self {
        DefaultCompilerContext {}
    }
}

impl CompilerContext for DefaultCompilerContext {
    fn handle_msg(&mut self, msg: CompilerRequestMsg) -> IsymtopeGenerateResult<CompilerResponseMsg> {
        match msg {
            CompilerRequestMsg::CompileSource(ref src, ref base_url) => {
                let body = compile_template(src, base_url)?;
                Ok(CompilerResponseMsg::CompileComplete(body))
            }
        }
    }
}
