
use std::panic;
use std::ffi::CString;
use std::os::raw::c_char;


extern {
    pub fn log_to_js(s: *const ::std::os::raw::c_char);
}

pub fn install_panic_hook() {
    panic::set_hook(Box::new(|panic_info| {
        let s = format!("panic occurred: {:?}", panic_info.payload().downcast_ref::<&str>().unwrap());
        unsafe { ::log::log_to_js(::std::ffi::CString::new(s).unwrap().into_raw()); }
    }));
}

#[macro_export]
macro_rules! eprint {
    ($($arg:tt)*) => ( unsafe { ::log::log_to_js(::std::ffi::CString::new(format!($($arg)*)).unwrap().into_raw()); });
}

#[macro_export]
macro_rules! eprintln {
    () => (eprint!("\n"));
    ($fmt:expr) => (eprint!(concat!($fmt, "\n")));
    ($fmt:expr, $($arg:tt)*) => (eprint!(concat!($fmt, "\n"), $($arg)*));
}

#[macro_export]
macro_rules! debug  {
    () => (eprint!("\n"));
    ($fmt:expr) => (eprint!(concat!($fmt, "\n")));
    ($fmt:expr, $($arg:tt)*) => (eprint!(concat!($fmt, "\n"), $($arg)*));
}

#[macro_export]
macro_rules! info  {
    () => (eprint!("\n"));
    ($fmt:expr) => (eprint!(concat!($fmt, "\n")));
    ($fmt:expr, $($arg:tt)*) => (eprint!(concat!($fmt, "\n"), $($arg)*));
}
