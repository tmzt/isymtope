#![allow(dead_code)]

// Scope: store/{scope*}/api

#[derive(Debug, Clone)]
pub enum ApiNodeType {
    ResourceNode(ApiResourceData),
    MethodsNode(Vec<MethodType>),
    BareMethodNode(MethodType)
}

#[derive(Debug, Clone)]
pub enum MethodType {
    Get,
    Post,
    Put,
    Delete,
    Patch
}

#[derive(Debug, Clone)]
pub struct ApiResourceData {
    pub resource_name: String,
    pub children: Option<Vec<ApiNodeType>>
}
