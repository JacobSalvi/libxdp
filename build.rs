use std::{env, path};

fn main() {
    let src_dir = path::PathBuf::from(env::var_os("CARGO_MANIFEST_DIR").unwrap());

    let bindings = bindgen::Builder::default()
        .header("wrapper.h")
        .generate_inline_functions(true)
        .blocklist_type("[u64; 4]")
        .blocklist_type("[u64; 4usize]")
        .clang_arg("-I/usr/include/xdp")
        .generate()
        .expect("Unable to generate bindings");

    bindings
        .write_to_file(src_dir.join("src/bindings.rs"))
        .expect("Couldn't write bindings!");
}
