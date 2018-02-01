#[cfg(all(target_arch = "wasm32", target_os = "unknown"))]
pub mod util_wasm;

#[cfg(not(all(target_arch = "wasm32", target_os = "unknown")))]
pub mod util_uuid;

#[cfg(all(target_arch = "wasm32", target_os = "unknown"))]
pub use self::util_wasm as util;

#[cfg(not(all(target_arch = "wasm32", target_os = "unknown")))]
pub use self::util_uuid as util;

pub use self::util::*;
