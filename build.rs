use std::env;
use std::path::PathBuf;

fn main() {
    let go_dir = PathBuf::from("go_methods");

    println!("cargo:rustc-link-search=native={}", go_dir.display());
    println!("cargo:rustc-link-lib=dylib=relation");

    println!(
        "cargo:rustc-link-arg=-Wl,-rpath,$ORIGIN/go_methods"
    );

    let bindings = bindgen::Builder::default()
        .header("go_methods/librelation.h")
        .parse_callbacks(Box::new(bindgen::CargoCallbacks))
        .generate()
        .expect("bindgen failed");

    let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());
    bindings
        .write_to_file(out_path.join("relation_bindings.rs"))
        .expect("could not write bindings");
}
