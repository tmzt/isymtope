
use std::fmt::Debug;
use std::iter;

use linked_hash_map::LinkedHashMap;

use itertools::Itertools;
use itertools::FoldWhile::{Continue, Done};

use error::*;

pub trait ScopeParentId {
    fn parent_id(&self) -> Option<&str>;
}

pub fn find_entry<S: ScopeParentId, K, V, F: FnMut(&mut S) -> Option<V>>(scopes: &mut LinkedHashMap<String, S>, scope_id: &str, key: K, mut f: F) -> DocumentProcessingResult<Option<V>> where K: Debug, V: Debug {
    assert!(scopes.len() > 0);
    let scope_id = scope_id.to_owned();

    let res = iter::repeat(0)
        .fold_while((scope_id, None), |acc, x| {
            let (ref scope_id, _) = acc;

            let scope = scopes.get_mut(scope_id).expect("scope_id from previous iteration expected to exist.");

            eprintln!("[entries]  Looking for entry for key [{:?}] in scope [{}] ", key, scope_id);
            let entry = f(scope);

            if let Some(entry) = entry {
                eprintln!("[entries] Found entry for key [{:?}]: {:?}", key, entry);

                Done((scope_id.to_owned(), Some(entry)))

            } else {
                eprintln!("[entries] Did not find entry for key [{:?}] in scope {:?}.", key, scope_id);

                let parent_id = scope.parent_id();
                eprintln!("[entries] parent_id: {:?}", parent_id);

                if parent_id.is_none() {
                    eprintln!("[entries] no parent_id, returning Done(None) to end fold_while.");
                    return Done((scope_id.to_owned(), None));
                }

                let scope_id = parent_id.unwrap().to_owned();
                eprintln!("[entries] continuing to search in parent_id: {:?}", parent_id);
                Continue((scope_id, None))
            }

        }).into_inner().1;

    eprintln!("[entries] find entry res: {:?}", res);

    Ok(res)
}

pub fn must_find_entry<S: ScopeParentId, K, V, F: FnMut(&mut S) -> Option<V>>(scopes: &mut LinkedHashMap<String, S>, scope_id: &str, key: K, f: F) -> DocumentProcessingResult<V> where K: Debug + Clone, V: Debug {
    let res = find_entry(scopes, scope_id, key.clone(), f)?;

    match res {
        Some(value) => Ok(value),
        _ => Err(try_eval_from_err!(format!("Could not find entry for key [{:?}]", key)))
    }
}
