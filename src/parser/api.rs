#![allow(dead_code)]

// Scope: store/{scope*}/api

#[derive(Debug)]
pub enum ApiNodeType {
    ResourceNode(ApiResourceData),
    MethodsNode(Vec<MethodType>),
    BareMethodNode(MethodType)
}

#[derive(Debug)]
pub enum MethodType {
    Get,
    Post,
    Put,
    Delete,
    Patch
}

#[derive(Debug)]
pub struct ApiResourceData {
    pub resource_name: String,
    pub children: Option<Vec<ApiNodeType>>
}
