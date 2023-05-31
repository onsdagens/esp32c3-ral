use std::env;
use std::fs::File;
use std::io::Write;
use std::path::PathBuf;
fn main() {
        let out = &PathBuf::from(env::var_os("OUT_DIR").unwrap());
        File::create(out.join("device.x"))
            .unwrap()
            .write_all(include_bytes!("Esp32c3.x"))
            .unwrap();
        println!("cargo:rustc-link-search={}", out.display());
        println!("cargo:rerun-if-changed=Esp32c3.x");
    println!("cargo:rerun-if-changed=build.rs");
}