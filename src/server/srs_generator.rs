
use std::iter::repeat;
use std::fmt::Debug;

use rand::{Rng, OsRng};
use data_encoding::BASE32HEX;

use server::*;


#[derive(Debug)]
pub struct DefaultSecureRandomStringGenerator {
    gen: OsRng
}

pub trait SecureRandomStringGenerator: Debug {
    fn generate_secure_string(&mut self, bytes: usize) -> IsymtopeServerResult<String>;
}

impl Default for DefaultSecureRandomStringGenerator {
    fn default() -> Self {
        let gen = OsRng::new().expect("Failed to get OS random number generator");

        DefaultSecureRandomStringGenerator { gen: gen }
    }
}

impl SecureRandomStringGenerator for DefaultSecureRandomStringGenerator {
    fn generate_secure_string(&mut self, bytes: usize) -> IsymtopeServerResult<String> {
        let mut res: Vec<u8> = repeat(0u8).take(bytes).collect();
        self.gen.fill_bytes(&mut res[..]);
        Ok(BASE32HEX.encode(&res[..]))
    }
}