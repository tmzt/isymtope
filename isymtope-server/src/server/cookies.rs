use std::rc::Rc;
use std::fmt::Debug;

#[cfg(feature = "session_time")]
use time::{get_time, Timespec};

use server::*;

pub const SESSION_COOKIES_RANDOM_STRING_BYTES: usize = 128;

#[cfg(feature = "session_time")]
#[derive(Debug)]
pub struct ReturnedCookie(String, Timespec, Timespec);

#[cfg(not(feature = "session_time"))]
#[derive(Debug)]
pub struct ReturnedCookie(String);

#[derive(Debug, Default)]
pub struct Cookies {
    cookie_secure_prefix: Option<String>,
    // cookie_name: Option<(String, Timespec)>
}

impl Cookies {
    fn create<'s>(&'s mut self, cookie: &str) -> IsymtopeServerResult<&'s ReturnedCookie> {
        unimplemented!()
    }

    fn refresh_cookie<'s>(
        &'s mut self,
        old_cookie: &str,
        cookie: &str,
    ) -> IsymtopeServerResult<&'s ReturnedCookie> {
        unimplemented!()
    }

    // fn cookie_name(&mut self) -> &str {
    //     // TODO: Handle expiration and retaining old values for session rotation
    //     if self.cookie_secure_prefix.is_none() {
    //         // Initialized at server startup
    //         self.cookie_secure_prefix = Some(self.srs.generate_secure_string(SESSION_COOKIES_RANDOM_STRING_BYTES));
    //     };

    //     if self.cookie_name.is_none() {
    //         let ts = get_time();
    //         let prefix = self.cookie_secure_prefix.as_ref().unwrap();
    //         let name = self.srs.generate_secure_string(self::SESSION_COOKIES_RANDOM_STRING_BYTES);

    //         self.cookie_name = Some((
    //             format!("{}_{}_{}_{}", &prefix, &name, ts.sec, ts.nsec),
    //             ts
    //         ));
    //     };

    //     self.cookie_name.as_ref().map(|s| s.0.as_str())
    //         .expect("Cookie name should have been populated above. This should not happen.")
    // }
}
