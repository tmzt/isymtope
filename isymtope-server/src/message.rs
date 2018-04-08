use isymtope_generate::*;

#[derive(Debug)]
pub enum AppRequestMsg {
    TemplateRequest(TemplateRequestMsg),
}

#[derive(Debug)]
pub enum AppResponseMsg {
    TemplateResponse(TemplateResponseMsg),
}
