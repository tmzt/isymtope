
#[derive(Debug)]
pub enum CompilerRequestMsg {
    CompileSource(String, String)
}

#[derive(Debug)]
pub enum CompilerResponseMsg {
    CompileComplete(String)
}
