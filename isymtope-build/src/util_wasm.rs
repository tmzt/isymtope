
use std::sync::Mutex;
use rand::{ChaChaRng, Rng};

lazy_static!(
    static ref RNG: Mutex<ChaChaRng> = Mutex::new(ChaChaRng::new_unseeded());
);

pub fn allocate_element_key() -> String {
    // use itertools::Itertools;
    // use rand::{ChaChaRng, Rng};
    // let mut rng = ChaChaRng::new_unseeded();

    // static SYMBOLS: &'static [char]  = &['0', '1', '2', '3', '4', '5', '6', '7', '8', '9', 'a', 'b', 'c', 'd', 'e', 'f'];

    // ::std::iter::repeat(5)
    //     .map(|_| { let idx = rng.gen_range(0, 15); SYMBOLS[idx] })
    //     .join("")

    let mut rng = RNG.lock().unwrap();
    let s = format!("{}", rng.gen::<i32>()).replace("-", "");
    eprintln!("random: {}", s);
    s
}
