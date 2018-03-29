use std::collections::{HashMap, HashSet};

#[derive(Debug)]
pub struct InternalTemplateData {
    pub base_url: String,
    pub library_names: HashSet<String>,
    pub route_keys: Vec<String>,
    pub route_func_keys: HashMap<String, String>,
    pub route_bodies: HashMap<String, String>,
    pub event_keys: Vec<String>,
    pub event_enterkeyflags: HashMap<String, bool>,
    pub event_action_keys: HashMap<String, Vec<String>>,
    pub event_action_bodies: HashMap<String, HashMap<String, String>>,
    pub reducer_keys: Vec<String>,
    pub reducer_action_keys: HashMap<String, Vec<String>>,
    pub reducer_bodies: HashMap<String, HashMap<String, String>>,
    pub reducer_defaults: HashMap<String, String>,
    pub extern_reducer_keys: Vec<String>,
    pub query_names: Vec<String>,
    pub query_params: HashMap<String, Vec<String>>,
    pub query_bodies: HashMap<String, String>,
    pub component_names: Vec<String>,
    pub component_bodies: HashMap<String, String>,
    pub page_render_func_body: String,
    pub page_body_key: String,
    pub page_body_html: String,
}
