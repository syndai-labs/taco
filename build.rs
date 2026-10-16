use std::env;
use std::path::PathBuf;

fn main() {
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
