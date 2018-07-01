extern crate lalrpop;

use lalrpop::Configuration;

fn main() {
    // Build parser
    Configuration::new()
        .emit_comments(true)
        .use_cargo_dir_conventions()
        .process()
        .ok()
        .unwrap();
}
