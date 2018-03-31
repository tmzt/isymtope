use isymtope_generate::*;

#[derive(Debug)]
pub enum CompilerRequestMsg {
    CompileSource(String, String)
}

#[derive(Debug)]
pub enum CompilerResponseMsg {
    CompileComplete(IsymtopeGenerateResult<String>)
}
