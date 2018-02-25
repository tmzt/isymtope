#[derive(Debug)]
pub struct RenderResponse(String);

impl RenderResponse {
    pub fn new(body: String) -> Self {
        RenderResponse(body)
    }

    pub fn into_inner(self) -> String {
        self.0
    }
}

#[derive(Debug)]
pub enum TemplateRequestMsg {
    RenderAppRoute(String, String, String, String),
}

#[derive(Debug)]
pub enum TemplateResponseMsg {
    RenderComplete(RenderResponse),
}
