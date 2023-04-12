extern crate bindgen;
extern crate cc;

use std::env;
use std::path::PathBuf;

fn main() {
    let libdir_path = PathBuf::from("lib")
        .canonicalize()
        .expect("cannot canonicalize path");

    let headers_path = libdir_path.join("hello.h");
    let source_path = libdir_path.join("hello.c");
    let headers_path_str = headers_path.to_str().expect("Path is not a valid string");
    let source_path_str = source_path.to_str().expect("Path is not a valid string");

    // Tell cargo to invalidate the built crate whenever the header changes
    println!("cargo:rerun-if-changed={}", headers_path_str);
    // Tell cargo to invalidate the built crate whenever the source file changes
    println!("cargo:rerun-if-changed={}", source_path_str);


    // Compile the C library
    cc::Build::new()
        .file(source_path)
        .include(libdir_path)
        .compile("hello");

    let bindings = bindgen::Builder::default()
        .header(headers_path_str)
        .parse_callbacks(Box::new(bindgen::CargoCallbacks))
        .generate()
        .expect("Unable to generate bindings");

    // Write the bindings to the $OUT_DIR/bindings.rs file.
    let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());
    println!("OUT_DIR = {}", out_path.display());
    bindings
        .write_to_file(out_path.join("bindings.rs"))
        .expect("Couldn't write bindings!")
}