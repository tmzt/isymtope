
use std::hash::Hash;
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
    func: Box<FnMut(&Symbols, &K) -> MapWalkState<'a, K, T>>
}

impl<'a, K: Hash + Clone, T> MapWalkIter<'a, K, T> {
    pub fn new(ctx: &'a mut Context, key: K, func: Box<FnMut(&Symbols, &K) -> MapWalkState<'a, K, T>>) -> Self {
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

impl<'a, K: Hash + Clone, T> Iterator for MapWalkIter<'a, K, T>
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
        // let box ref func = func;
        let state = (self.func.as_mut())(map_ref, &self.key);

        match state {
            MapWalkState::InterimMatch(next_key, v) => {
                if parent_id.is_none() { return None; }
                self.scope_id = parent_id.unwrap();
                self.key = next_key.to_owned();
                Some(state)
            }

            MapWalkState::FinalMatch(..) => Some(state),
            _ if parent_id.is_some() => Some(MapWalkState::NoMatch),
            _ => None
        }
    }
}
