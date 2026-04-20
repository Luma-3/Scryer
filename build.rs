use std::{env, path::PathBuf};

fn main() {
    println!("cargo:rerun-if-changed=./commun/src/memory_event.h");

    let bindings = bindgen::Builder::default()
        .header("./commun/src/memory_event.h")
        .parse_callbacks(Box::new(bindgen::CargoCallbacks::new()))
        .generate()
        .expect("Failed to generate binding");

    let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());
    bindings
        .write_to_file(out_path.join("bindings.rs"))
        .expect("Impossible d'écrire les bindings");
}
