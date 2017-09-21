
use std::hash::Hash;
use scope::*;


// pub enum MapWalkReturn<'a, T> {
//     NoMatch,
//     InterimMatch(&'a T),
//     FinalMatch(&'a T)
// }

pub enum MapWalkState<'a, K: 'a, T: 'a> {
    NoMatch,
    InterimMatch(K, Option<&'a T>),
    FinalMatch(&'a T)
}

#[allow(dead_code)]
pub struct MapWalkIter<'a, K: 'a, F>
{
    ctx: &'a mut Context,
    scope_id: String,
    key: K,
    func: F
}

impl<'a, K: Hash + Clone + 'a, T: 'a, F> MapWalkIter<'a, K, F>
    where F: FnMut(&Symbols, K) -> MapWalkState<'a, K, T>
{
    pub fn new(ctx: &'a mut Context, key: K, func: F) -> Self {
        let scope_id = ctx.scope_ref().expect("Context must have valid scope").id().to_owned();
        // let scope_id = scope.id().to_owned();

        MapWalkIter {
            ctx: ctx,
            scope_id: scope_id,
            key: key,
            func: func
        }
    }
}

impl<'a, K: Hash + Clone + 'a, T: 'a, F> Iterator for MapWalkIter<'a, K, F>
    where F: FnMut(&Symbols, K) -> MapWalkState<'a, K, T>
{
    type Item = MapWalkState<'a, K, T>;

    fn next(&mut self) -> Option<MapWalkState<'a, K, T>> {
        let (map_id, parent_id) = {
            let scope = self.ctx.get_scope(&self.scope_id);
            if scope.is_none() { return None; }
            let scope = scope.unwrap();
            (scope.map_id().to_owned(), scope.parent_id().map(|parent_id| parent_id.to_owned()))
        };

        let map_id = map_id.to_owned();
        // let map_ref = self.ctx.get_map(map_id).expect("Map must exist for scope");

        // let state = (self.func)(&*map_ref, self.key.to_owned());

        let map = self.ctx.get_map_move_id(map_id);
        if let Some(map) = map {
            let state = (self.func)(map, self.key.to_owned());

            match state {
                MapWalkState::InterimMatch(ref next_key, ref value) => {
                    if parent_id.is_none() { return None; }
                    self.scope_id = parent_id.unwrap();
                    self.key = next_key.to_owned();
                    return Some(MapWalkState::InterimMatch(next_key.to_owned(), value.to_owned()));
                }

                MapWalkState::FinalMatch(value) => { return Some(MapWalkState::FinalMatch(value.to_owned())); },
                _ if parent_id.is_some() => { return Some(MapWalkState::NoMatch); },

                _ => {}
            };
        };

        // match map.as_ref().map(|map| (self.func)(map, self.key.to_owned())) {
        //     _ => None
        // }

        // match state {
        //     &MapWalkState::InterimMatch(ref next_key, _) => {
        //         if parent_id.is_none() { return None; }
        //         self.scope_id = parent_id.unwrap();
        //         self.key = next_key.to_owned();
        //         Some(state)
        //     }

        //     &MapWalkState::FinalMatch(..) => Some(state),
        //     _ if parent_id.is_some() => Some(&MapWalkState::NoMatch),
        //     _ => None
        // }

        None
    }
}
