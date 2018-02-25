#[cfg(all(target_arch = "wasm32", target_os = "unknown", not(feature = "uuid_v4")))]
pub mod util_wasm;

#[cfg(feature = "uuid_v4")]
pub mod util_uuid;

#[cfg(all(target_arch = "wasm32", target_os = "unknown", not(feature = "uuid_v4")))]
pub use self::util_wasm as util;

#[cfg(feature = "uuid_v4")]
pub use self::util_uuid as util;

pub use self::util::*;
