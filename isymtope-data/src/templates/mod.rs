pub mod data;
pub use self::data::*;
include!(concat!(env!("OUT_DIR"), "/templates.rs"));
use self::templates::*;