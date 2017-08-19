#![allow(dead_code)]

use parser::util::allocate_element_key;


#[derive(Debug, Clone, Default)]
pub struct Scope {
    scope_id: String,
    map_id: String
}

impl Scope {
    pub fn with_map_id(map_id: &str) -> Scope {
        let scope_id = allocate_element_key();
        Scope { scope_id: scope_id, map_id: map_id.to_owned() }
    }

    pub fn id(&self) -> &str { &self.scope_id }
    pub fn map_id(&self) -> &str { &self.map_id }
}