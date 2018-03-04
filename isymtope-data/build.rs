extern crate ructe;

use std::env;
use std::path::PathBuf;

use ructe::compile_templates;

fn main() {
    // Build internal (html) templates
    let out_dir = PathBuf::from(env::var("OUT_DIR").unwrap());
    let in_dir = PathBuf::from(env::var("CARGO_MANIFEST_DIR").unwrap()).join("res/templates/page");
    compile_templates(&in_dir, &out_dir).expect("foo");

    println!("cargo:rerun-if-changed=\"{}/*\"", in_dir.to_str().unwrap());
}
