
use std::collections::hash_map::{HashMap, Entry};

use time::{Timespec, Duration, get_time};

use isymtope_build::error::*;
use isymtope_build::ast::*;
use isymtope_build::traits::*;
use isymtope_build::expressions::*;


#[derive(Debug)]
pub struct MemorySession {
    created: Timespec,
    expires: Option<Timespec>,
    data: HashMap<String, ExpressionValue<OutputExpression>>
}

impl Default for MemorySession {
    fn default() -> Self {
        MemorySession::new(get_time(), None)
    }
}

impl MemorySession {
    pub fn new(created: Timespec, expires: Option<Duration>) -> Self {
        let expires = expires.map(|dur| created + dur);

        MemorySession {
            created: created,
            expires: expires,
            data: Default::default()
        }
    }
}

impl Session for MemorySession {
    fn set_value(&mut self, key: &str, value: ExpressionValue<OutputExpression>, update: bool) -> SessionResult<()> {
        self.data.insert(key.to_owned(), value);

        // let entry = self.data.entry(key);

        // match self.data.entry(key.to_owned()) {
        //     Entry::Occupied(mut o) => {
        //         let item = o.get_mut();

        //         // Set modified timestamp
        //         item.2 = Some(ts.clone());

        //         println!("Replacing existing value of [{}] with [{:?}] (was [{:?}])", key, expr, item.0);

        //         Ok(())
        //     }

        //     Entry::Vacant(v) => {
        //         // let initial_ty = match mode { Some(DataItemMode::InitialType(ref ty) ) => Some(ty.to_owned()), _ => None };
        //         // let item = SessionDataItem::new(expr, initial_ty, ts);
        //         let item = SessionDataItem::new(expr, ts);

        //         v.insert(item);

        //         Ok(())
        //     }
        // }
        Ok(())
    }

    fn remove_value(&mut self, key: &str) -> SessionResult<()> {
        self.data.remove(key);
        Ok(())
    }

    fn get_value(&self, key: &str) -> SessionResult<Option<&ExpressionValue<OutputExpression>>> {
        Ok(self.data.get(key))
    }

    fn created(&self) -> &Timespec { &self.created }
    fn expires(&self) -> Option<&Timespec> { self.expires.as_ref() }

    fn execute_action(&mut self, action_op: &ActionOp<ProcessedExpression>) -> SessionResult<()> {
        Ok(())
    }

    #[cfg(feature = "types")]
    fn set_with_type(&mut self, key: &str, value: ExpressionValue<OutputExpression>) -> SessionResult<()> {
        let ty = value.peek_ty();

        let mode = ty.map(|ty| if initial { DataItemMode::InitialType(ty) } else { DataItemMode::ReplaceType(ty) });

        self.set_with_type_mode(key, value, mode)?;
    }
}

impl ReducerStateProvider for MemorySession {
    fn get(&self, key: &str) -> SessionResult<Option<&ExpressionValue<OutputExpression>>> {
        eprintln!("Requested reducer state key {}", key);
        let expr = self.get_value(key)?;
        eprintln!("Got value for reducer state key {}: {:?}", key, expr);
        Ok(expr)
    }
}
