#[cfg(feature="extern_rand")]
use std::iter;

#[cfg(feature="uuid_v4")]
use uuid::Uuid;

#[cfg(feature="extern_rand")]
use itertools::Itertools;


#[cfg(feature="uuid_v4")]
pub fn allocate_element_key() -> String {
    let uuid_ = format!("{}", Uuid::new_v4());
    format!("{0:.5}", uuid_)
}

#[cfg(feature="extern_rand")]
extern {
    fn rand_value(n: usize) -> usize;
}

#[cfg(feature="extern_rand")]
pub fn allocate_element_key() -> String {
    static SYMBOLS: &'static [char]  = &['0', '1', '2', '3', '4', '5', '6', '7', '8', '9', 'a', 'b', 'c', 'd', 'e', 'f'];

    iter::repeat(5)
        .map(|_| { let idx = unsafe { rand_value(16) }; SYMBOLS[idx] })
        .join("")

    // "abc".into()
}
