use std::env;
use std::path::{Path, PathBuf};

fn main() {
    // build the FFI wrapper
    cc::Build::new()
        .cpp(true)
        .file("include/wrapper.cc")
        .include("..") // needed for cpp-example/Counter.h
        .emit_rerun_if_env_changed(true)
        .compile("wrapper");

    // generate the FFI bindings
    let bindings = bindgen::Builder::default()
        .header("include/wrapper.h")
        .parse_callbacks(Box::new(bindgen::CargoCallbacks::new()))
        .merge_extern_blocks(true)
        .size_t_is_usize(true)
        .use_core()
        .generate()
        .expect("Unable to generate bindings");

    let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());
    bindings
        .write_to_file(out_path.join("bindings.rs"))
        .expect("Couldn't write bindings!");

    // Dynamically link to libexample.so in the artifacts folder
    println!("cargo:rustc-link-lib=dylib=counter");

    let dir = std::env::var("CARGO_MANIFEST_DIR").unwrap();
    println!(
        "cargo:rustc-link-search=native={}",
        Path::new(&dir).join("..").join("artifacts").display()
    );
}
