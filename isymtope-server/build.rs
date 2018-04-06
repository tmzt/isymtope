
#[cfg(feature = "copy_wasm")]
fn main() {
    use std::str;
    use std::env;
    use std::path::PathBuf;
    use std::process::Command;

    // Copy compiled wasm application
    let in_wasm = PathBuf::from(env::var("CARGO_MANIFEST_DIR").unwrap()).join("../wasm-build/target/wasm32-unknown-unknown/release/isymtope_wasm.wasm");
    let out_wasm = PathBuf::from(env::var("CARGO_MANIFEST_DIR").unwrap()).join("res/tests/app/playground/isymtope-small.wasm");

    let wasm_gc = Command::new("wasm-gc")
        .arg(in_wasm.clone())
        .arg(out_wasm)
        .output()
        .expect("wasm-gc failed");

    println!("[wasm-gc stdout]");
    println!("{}",  str::from_utf8(wasm_gc.stdout.as_slice()).unwrap());

    println!("[wasm-gc stderr]");
    println!("{}",  str::from_utf8(wasm_gc.stderr.as_slice()).unwrap());

    println!("cargo:rerun-if-changed=\"{}\"", in_wasm.to_str().unwrap());
}

#[cfg(not(feature = "copy_wasm"))]
fn main() {
}
