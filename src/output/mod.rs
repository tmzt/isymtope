#[cfg(test)]
#[macro_use]
pub mod tests;

extern crate uuid;
extern crate itertools;

pub mod writers;
pub mod path_writer;
pub mod page_writer;
pub mod store_writer;
pub mod events_writer;

pub use processing::structs::Result;
pub use self::writers::*;
pub use self::path_writer::*;
pub use self::page_writer::*;
pub use self::store_writer::*;
pub use self::events_writer::*;