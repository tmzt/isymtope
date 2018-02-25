#[macro_use]
extern crate log;

use std::panic;
use log::{Level, Metadata, Record};

extern "C" {
    pub fn log_to_js(s: *const ::std::os::raw::c_char);
}

pub fn do_log_to_js(msg: String) {
    unsafe {
        log_to_js(::std::ffi::CString::new(msg).unwrap().into_raw());
    }
}

struct WasmLogger;

impl log::Log for WasmLogger {
    fn enabled(&self, metadata: &Metadata) -> bool {
        metadata.level() <= Level::Info
    }

    fn log(&self, record: &Record) {
        if self.enabled(record.metadata()) {
            let msg = format!("{} - {}", record.level(), record.args());
            unsafe {
                log_to_js(::std::ffi::CString::new(msg).unwrap().into_raw());
            }
        }
    }

    fn flush(&self) {}
}

pub fn wasm_log_init() {
    unsafe {
        log_to_js(
            ::std::ffi::CString::new("[wasm logger] init")
                .unwrap()
                .into_raw(),
        );
    }
    log::set_boxed_logger(Box::new(WasmLogger)).unwrap();
}

pub fn wasm_install_panic_hook() {
    panic::set_hook(Box::new(|panic_info| {
        let msg = format!(
            "panic occurred: {:?}",
            panic_info.payload().downcast_ref::<&str>().unwrap()
        );
        unsafe {
            log_to_js(::std::ffi::CString::new(msg).unwrap().into_raw());
        }
    }));
}

#[macro_export]
macro_rules! eprint {
    ($($arg:tt)*) => ({ ::wasm_log::do_log_to_js(format!($($arg)*)); });
}

#[macro_export]
macro_rules! eprintln {
    () => (eprint!("\n"));
    ($fmt:expr) => (eprint!(concat!($fmt, "\n")));
    ($fmt:expr, $($arg:tt)*) => (eprint!(concat!($fmt, "\n"), $($arg)*));
}

// pub fn do_log_to_js(s: String) {
//     unsafe {
//         log_to_js(::std::ffi::CString::new(s).unwrap().into_raw());
//     }
// }

// mod wasm_logging {
//     use std::panic;
//     use super::do_log_to_js;

//     pub fn install_panic_hook() {
//         panic::set_hook(Box::new(|panic_info| {
//             let s = format!(
//                 "panic occurred: {:?}",
//                 panic_info.payload().downcast_ref::<&str>().unwrap()
//             );
//             self::do_log_to_js(s);
//         }));
//     }
// }
// pub use self::wasm_logging::install_panic_hook;

// #[macro_export]
// macro_rules! log {
//     ($lvl:expr, $($arg:tt)+) => ({
//         static LOC: ::log::LogLocation = ::log::LogLocation {
//             line: line!(),
//             file: file!(),
//             module_path: module_path!(),
//         };
//         let lvl = $lvl;
//         if log_enabled!(lvl) {
//             ::log::log(lvl, &LOC, format_args!($($arg)+))
//         }
//     })
// }

// #[macro_export]
// macro_rules! eprint {
//     ($($arg:tt)*) => ({ ::log::do_log_to_js(format!($($arg)*)); });
// }

// #[macro_export]
// macro_rules! eprintln {
//     () => (eprint!("\n"));
//     ($fmt:expr) => (eprint!(concat!($fmt, "\n")));
//     ($fmt:expr, $($arg:tt)*) => (eprint!(concat!($fmt, "\n"), $($arg)*));
// }

// #[macro_export]
// macro_rules! debug  {
//     () => (eprint!("\n"));
//     ($fmt:expr) => (eprint!(concat!($fmt, "\n")));
//     ($fmt:expr, $($arg:tt)*) => (eprint!(concat!($fmt, "\n"), $($arg)*));
// }

// #[macro_export]
// macro_rules! info  {
//     () => (eprint!("\n"));
//     ($fmt:expr) => (eprint!(concat!($fmt, "\n")));
//     ($fmt:expr, $($arg:tt)*) => (eprint!(concat!($fmt, "\n"), $($arg)*));
// }

// #[macro_export]
// macro_rules! log  {
//     () => (eprint!("\n"));
//     ($fmt:expr) => (eprint!(concat!($fmt, "\n")));
//     ($fmt:expr, $($arg:tt)*) => (eprint!(concat!($fmt, "\n"), $($arg)*));
// }
