use std::fmt::Debug;
use std::iter;

use linked_hash_map::LinkedHashMap;

use itertools::Itertools;
use itertools::FoldWhile::{Continue, Done};

use traits::*;
use error::*;

pub fn find_match<S: ScopeParentId + Debug, V, F: FnMut(&mut S) -> Option<V>>(
    scopes: &mut LinkedHashMap<String, S>,
    scope_id: &str,
    mut f: F,
) -> DocumentProcessingResult<Option<V>>
where
    V: Debug,
{
    assert!(scopes.len() > 0);
    let scope_id = scope_id.to_owned();

    let res = iter::repeat(0)
        .fold_while((scope_id, None), |acc, _| {
            let (ref scope_id, _) = acc;

            let scope = scopes
                .get_mut(scope_id)
                .expect("scope_id from previous iteration expected to exist.");

            debug!(
                "[find_match]  Looking for match in scope [{:?}] ",
                scope
            );
            let result = f(scope);

            if let Some(result) = result {
                debug!("[find_match] Found match in scope [{:?}]: {:?}", scope, result);

                Done((scope_id.to_owned(), Some(result)))
            } else {
                debug!(
                    "[find_match] Did not find match in scope {:?}.",
                    scope
                );

                let parent_id = scope.parent_id();
                // debug!("[find_match] parent_id: {:?}", parent_id);

                if parent_id.is_none() {
                    debug!("[find_match] no parent_id, returning Done(None) to end fold_while.");
                    return Done((scope_id.to_owned(), None));
                }

                let scope_id = parent_id.unwrap().to_owned();
                debug!(
                    "[find_match] continuing to search in parent_id: {:?}",
                    parent_id
                );
                Continue((scope_id, None))
            }
        })
        .into_inner()
        .1;

    debug!("[find_match] find entry res: {:?}", res);

    Ok(res)
}

pub fn find_entry<S: ScopeParentId + Debug, K, V, F: FnMut(&mut S) -> Option<V>>(
    scopes: &mut LinkedHashMap<String, S>,
    scope_id: &str,
    key: K,
    mut f: F,
) -> DocumentProcessingResult<Option<V>>
where
    K: Debug,
    V: Debug,
{
    assert!(scopes.len() > 0);
    let scope_id = scope_id.to_owned();

    let res = iter::repeat(0)
        .fold_while((scope_id, None), |acc, _| {
            let (ref scope_id, _) = acc;

            let scope = scopes
                .get_mut(scope_id)
                .expect("scope_id from previous iteration expected to exist.");

            debug!(
                "[entries]  Looking for entry for key [{:?}] in scope [{:?}] ",
                key, scope
            );
            let entry = f(scope);

            if let Some(entry) = entry {
                eprintln!("[entries] Found entry for key [{:?}]: {:?}", key, entry);

                Done((scope_id.to_owned(), Some(entry)))
            } else {
                debug!(
                    "[entries] Did not find entry for key [{:?}] in scope [{:?}].",
                    key, scope
                );

                let parent_id = scope.parent_id();
                // debug!("[entries] parent_id: {:?}", parent_id);

                if parent_id.is_none() {
                    debug!("[entries] no parent_id, returning Done(None) to end fold_while.");
                    return Done((scope_id.to_owned(), None));
                }

                let scope_id = parent_id.unwrap().to_owned();
                debug!(
                    "[entries] continuing to search in parent_id: {:?}",
                    parent_id
                );
                Continue((scope_id, None))
            }
        })
        .into_inner()
        .1;

    eprintln!("[entries] find entry res: {:?}", res);

    Ok(res)
}

pub fn must_find_entry<S: ScopeParentId + Debug, K, V, F: FnMut(&mut S) -> Option<V>>(
    scopes: &mut LinkedHashMap<String, S>,
    scope_id: &str,
    key: K,
    f: F,
) -> DocumentProcessingResult<V>
where
    K: Debug + Clone,
    V: Debug,
{
    let res = find_entry(scopes, scope_id, key.clone(), f)?;

    match res {
        Some(value) => Ok(value),
        _ => {
            eprintln!(
                "Could not find entry for key [{:?}]",
                key
            );

            Err(try_eval_from_err!(format!(
                "Could not find entry for key [{:?}]",
                key
            )))
        },
    }
}
