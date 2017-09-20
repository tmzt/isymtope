
use scope::*;


// pub enum MapWalkReturn<'a, T> {
//     NoMatch,
//     InterimMatch(&'a T),
//     FinalMatch(&'a T)
// }

pub enum MapWalkState<'a, K: 'a, T: 'a> {
    NoMatch,
    InterimMatch(&'a K, Option<&'a T>),
    FinalMatch(&'a T)
}

#[allow(dead_code)]
pub struct MapWalkIter<'a, K: 'a, T: 'a>
{
    ctx: &'a mut Context,
    scope_id: String,
    key: K,
    next_key: Option<K>,
    func: FnMut(&Symbols, &K) -> MapWalkState<'a, K, T>
}

impl<'a, K, T> Iterator for MapWalkIter<'a, K, T>
{
    type Item = MapWalkState<'a, K, T>;

    fn next(&mut self) -> Option<Self::Item> {
        let (map_id, parent_id) = {
            let scope = self.ctx.get_scope(&self.scope_id);
            if scope.is_none() { return None; }
            let scope = scope.unwrap();
            (scope.map_id().to_owned(), scope.parent_id().map(|parent_id| parent_id.to_owned()))
        };

        let map_ref = self.ctx.get_map(&map_id).expect("Map must exist for scope");
        let state = (self.func)(map_ref, &self.key);

        // match state {
        //     Some(MapWalkState::InterimMatch(ref next_key, ref v)) => {
        //         self.next_key = Some(next_key);
        //         return state;
        //     }

        //     Some(MapWalkState::FinalMatch(ref v)) => { return state; }

        //     _ => {}
        // };

        // // Prepare next iteration
        // if parent_id.is_none() { return None; }
        // self.next_scope_id = parent_id.map(|s| s.to_owned());

        // Some(MapWalkState::NoMatch)
        None
    }
}
