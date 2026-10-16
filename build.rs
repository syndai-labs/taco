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
    // Apple: -sectcreate __TEXT __info_plist <path>, with CFBundleIdentifier and
    // CFBundleName in that plist. Without those keys, otool can show the section
    // while codesign still prints Info.plist=not bound.
    println!(
        "cargo:rustc-link-arg=-Wl,-sectcreate,__TEXT,__info_plist,{}",
        plist.display()
    );
}
