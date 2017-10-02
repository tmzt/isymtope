
use std::hash::Hash;
use scope::*;


// pub enum MapWalkReturn<'a, T> {
//     NoMatch,
//     InterimMatch(&'a T),
//     FinalMatch(&'a T)
// }

// pub enum MapWalkState<'a: 'map + 'k, 'map, 'k, K: 'k, T: 'map> {

#[allow(dead_code)]
#[derive(Debug, PartialEq)]
pub enum MapWalkState<K, T> {
    NoMatch,
    InterimMatch(K, Option<T>),
    FinalMatch(T)
}

#[allow(dead_code)]
pub struct MapWalkIter<'ctx, K, F>
{
    ctx: &'ctx mut Context,
    scope_id: Option<String>,
    key: K,
    func: F,
}

impl<'ctx, K, T, F> MapWalkIter<'ctx, K, F>
    where F: FnMut(&Symbols, &K) -> MapWalkState<K, T>
{
    pub fn new(ctx: &'ctx mut Context, key: K, func: F) -> Self {
        let scope_id = ctx.scope_ref().expect("Context must have valid scope").id().to_owned();

        MapWalkIter {
            ctx: ctx,
            scope_id: Some(scope_id),
            key: key,
            func: func,
        }
    }
}

impl<'ctx, K: Hash + Clone, T, F> Iterator for MapWalkIter<'ctx, K, F>
    where F: FnMut(&Symbols, &K) -> MapWalkState<K, T>
{
    type Item = MapWalkState<K, T>;

    fn next(&mut self) -> Option<Self::Item> {
        if self.scope_id.is_none() { return None; }
        // let scope_id = self.scope_id.as_ref().map(|s| s.to_owned());
        let scope_id = self.scope_id.to_owned();

        let values = scope_id.as_ref()
            .and_then(|scope_id| self.ctx.get_scope(scope_id))
            .map(|scope| (scope.map_id().to_owned(), scope.parent_id().map(|s| s.to_owned())));
        if values.is_none() { return None; }

        let (map_id, parent_id) = values.unwrap();

        let state = (self.func)(self.ctx.get_map(&map_id).unwrap(), &self.key);

        match &state {
            &MapWalkState::FinalMatch(..) => { },

            &MapWalkState::InterimMatch(ref next_key, _) => {
                self.key = next_key.to_owned();
            }

            _ if parent_id.is_none() => { return None; }

            _ => {}
        };

        // Set up next iteration
        self.scope_id = parent_id.map(|s| s.to_owned());
        Some(state)


        // if let MapWalkState::FinalMatch(..) = state { return Some(state); }

        // if parent_id.is_none() { return None; }
        // self.scope_id = parent_id;

        // if let MapWalkState::InterimMatch(ref next_key, _) = state { self.key = next_key.to_owned(); };

        // if let MapWalkState::InterimMatch(..) = state {
        //     return Some(state);
        // };

        // Some(MapWalkState::NoMatch)
    }
}


#[cfg(test)]
mod tests  {
    use super::*;
    use model::*;
    use parser::*;


    #[test]
    pub fn test_walk_maps() {
        let mut ctx = Context::default();
        ctx.push_child_scope();
        ctx.push_child_scope();
        ctx.add_binding_value(&BindingType::ComponentKeyBinding, ExprValue::LiteralString("a13".into()));

        let mut iter = MapWalkIter::new(&mut ctx, BindingType::ComponentKeyBinding, move |ref map, binding| map.get_binding_value(binding).map_or(MapWalkState::NoMatch, |b| MapWalkState::FinalMatch(b.to_owned())));

        assert_eq!(iter.next(), Some(MapWalkState::FinalMatch(ExprValue::LiteralString("a13".into()))));
    }

    #[test]
    pub fn test_walk_maps_no_match() {
        let mut ctx = Context::default();
        ctx.push_child_scope();
        ctx.push_child_scope();
        // We aren't adding the binding value

        let mut iter = MapWalkIter::new(&mut ctx, BindingType::ComponentKeyBinding, move |ref map, binding| map.get_binding_value(binding).map_or(MapWalkState::NoMatch, |b| MapWalkState::FinalMatch(b.to_owned())));

        assert_eq!(iter.next(), Some(MapWalkState::NoMatch));
        assert_eq!(iter.next(), Some(MapWalkState::NoMatch));
        assert_eq!(iter.next(), None);
    }
}