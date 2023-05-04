extern crate bindgen;
extern crate cmake;

use cmake::Config;

fn main() {
    let dst = Config::new("backend").build();

    println!("cargo:rustc-link-search=native={}", dst.display());
    println!("cargo:rustc-link-lib=dylib=backend");

    println!("cargo:rerun-if-changed=backend/bindings.h");

    let bindings = bindgen::Builder::default()
        .header("backend/bindings.h")
        .generate()
        .expect("Unable to generate bindings");
    let out_path = std::path::PathBuf::from("src");
    bindings
        .write_to_file(out_path.join("bindings.rs"))
        .expect("Couldn't write bindings!");
}
