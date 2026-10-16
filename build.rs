use std::env;
use std::path::PathBuf;

fn main() {
    // Engine cmake lives in native/ffi/build.rs. It runs only for
    // `--features native` on macOS aarch64 when TACO_LLAMA_SRC and
    // TACO_WHISPER_SRC are local checkouts. This file still only embeds
    // Info.plist. Default builds and Linux do not configure cmake.
    println!("cargo:rerun-if-changed=assets/Info.plist");
    let target_os = env::var("CARGO_CFG_TARGET_OS").unwrap_or_default();
    if target_os != "macos" {
        return;
    }
    let manifest = PathBuf::from(env::var("CARGO_MANIFEST_DIR").unwrap());
    let plist = manifest.join("assets/Info.plist");
    // Apple Code Signing Guide: -sectcreate __TEXT __info_plist <path>.
    // Pass via -Xlinker so rustc/cc does not mangle the commas; CFBundleIdentifier
    // and CFBundleName must already be in the plist or codesign stays unbound.
    for arg in [
        "-Xlinker",
        "-sectcreate",
        "-Xlinker",
        "__TEXT",
        "-Xlinker",
        "__info_plist",
        "-Xlinker",
    ] {
        println!("cargo:rustc-link-arg={arg}");
    }
    println!("cargo:rustc-link-arg={}", plist.display());
}
