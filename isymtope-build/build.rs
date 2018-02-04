extern crate ructe;

use ructe::compile_templates;
use std::env;
use std::path::PathBuf;

fn main() {
    let out_dir = PathBuf::from(env::var("OUT_DIR").unwrap());
    let in_dir = PathBuf::from(env::var("CARGO_MANIFEST_DIR").unwrap())
        .join("res/templates/page");
    compile_templates(&in_dir, &out_dir).expect("foo");

    // lalrpop::Configuration::new()
    //     .emit_comments(true)
    //     .process_dir("./src/input/parser")
    //     .unwrap();
}
