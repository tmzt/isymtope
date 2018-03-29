use std::rc::Rc;

use regex::RegexSet;

use isymtope_build::*;

#[derive(Debug, Default)]
pub struct Router {
    routes: Option<RegexSet>,
}

impl Router {
    pub fn with_document_provider(document_provider: Rc<DocumentProvider>) -> Self {
        let doc = document_provider.doc();
        let routes = doc.routes();

        let route_patterns: Vec<_> = routes.map(|r| r.pattern()).collect();
        let routes = RegexSet::new(route_patterns.into_iter()).unwrap();

        Router {
            routes: Some(routes),
        }
    }
}
