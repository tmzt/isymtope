use isymtope_generate::*;
use super::*;

#[derive(Debug)]
pub enum AppRequestMsg {
    TemplateRequest(TemplateRequestMsg),
}

#[derive(Debug)]
pub enum AppResponseMsg {
    TemplateResponse(TemplateResponseMsg),
}
