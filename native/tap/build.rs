use std::env;
use std::path::PathBuf;

fn main() {
    let manifest = PathBuf::from(env::var("CARGO_MANIFEST_DIR").unwrap());
    let source = manifest.join("../process_tap.m");
    println!("cargo:rerun-if-changed={}", source.display());
    if env::var("CARGO_CFG_TARGET_OS").unwrap_or_default() != "macos" {
        return;
    }
    cc::Build::new()
        .file(&source)
        .flag("-fobjc-arc")
        .compile("taco_process_tap");
    println!("cargo:rustc-link-lib=framework=Foundation");
    println!("cargo:rustc-link-lib=framework=CoreFoundation");
    println!("cargo:rustc-link-lib=framework=CoreAudio");
}
