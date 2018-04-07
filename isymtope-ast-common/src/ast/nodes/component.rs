// use std::iter;
// use itertools::Itertools;
// use itertools::FoldWhile::{Continue, Done};

use std::collections::hash_map::{HashMap, Entry};

use error::*;
use expressions::*;
use ast::*;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct ComponentDefinition<T>(String, FormalParams<T>, Option<Vec<ContentNode<T>>>);

impl<T: Clone> ComponentDefinition<T> {
    pub fn new(
        name: String,
        params: FormalParams<T>,
        children: Option<Vec<ContentNode<T>>>,
    ) -> Self {
        ComponentDefinition(name, params, children)
    }

    pub fn name(&self) -> &str {
        self.0.as_str()
    }

    pub fn params<'a>(&'a self) -> Option<impl Iterator<Item = &'a str>> {
        self.1.params()
    }

    pub fn children<'a>(&'a self) -> Option<impl Iterator<Item = &'a ContentNode<T>>> {
        self.2.as_ref().map(|v| v.iter())
    }

    pub fn gather_value_binding_mappings(&self) -> DocumentProcessingResult<HashMap<String, String>> {
        let mut mappings: HashMap<String, String> = Default::default();
        let mut visitor = DefaultContentNodeVisitor::default();

        if let Some(children) = self.children() {
            for child in children {
                visitor.visit_value_bindings(child, &mut |element, value_binding| {
                    if let Some(alias) = value_binding.ident() {
                        match mappings.entry(alias.to_owned()) {
                            Entry::Occupied(_) => {
                                return Err(try_process_from_err!(format!("Duplicate value binding (as): {}", alias)));
                            }

                            Entry::Vacant(v) => {
                                v.insert(element.key().to_owned())
                            }
                        };
                    };

                    Ok(())
                })?;
            }
        };

        Ok(mappings)
    }

    // pub fn gather_value_binding_keys(&self) -> Option<Vec<String>> {
    //         let mut keys: Vec<String> = Vec::with_capacity(32);
    //         let mut current: Option<&ContentNode> = None;

    //         if let Some(children) = self.children() {
    //             for child in children {
    //                 current = child;
    //                 iter::repeat(0).fold_while((&mut keys, &mut current), |keys, _| {
    //                     match current.children() {
    //                         Some(children) => {

    //                         }
    //                     }

    //                 })
    //             }
    // }
}
