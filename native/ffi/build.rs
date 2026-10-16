use std::collections::BTreeMap;
use std::env;
use std::fs;
use std::path::{Path, PathBuf};
use std::process::Command;

fn main() {
    println!("cargo:rerun-if-env-changed=TACO_LLAMA_SRC");
    println!("cargo:rerun-if-env-changed=TACO_WHISPER_SRC");
    println!("cargo:rerun-if-changed=../CMakeLists.txt");
    println!("cargo:rerun-if-changed=../taco_engines.cpp");
    println!("cargo:rerun-if-changed=../taco_engines.h");
    println!("cargo:rustc-check-cfg=cfg(taco_engines_static)");

    if !apple_silicon() {
        if env::var_os("TACO_LLAMA_SRC").is_some() || env::var_os("TACO_WHISPER_SRC").is_some() {
            println!(
                "cargo:warning=TACO_LLAMA_SRC/TACO_WHISPER_SRC ignored off macOS aarch64; engines stay unlinked"
            );
        }
        return;
    }

    match (env::var("TACO_LLAMA_SRC"), env::var("TACO_WHISPER_SRC")) {
        (Err(_), Err(_)) => {}
        (Ok(llama), Ok(whisper)) => link(&llama, &whisper),
        _ => panic!("set both TACO_LLAMA_SRC and TACO_WHISPER_SRC, or neither"),
    }
}

fn apple_silicon() -> bool {
    cfg!(all(target_os = "macos", target_arch = "aarch64"))
        && env::var("CARGO_CFG_TARGET_OS").ok().as_deref() == Some("macos")
        && env::var("CARGO_CFG_TARGET_ARCH").ok().as_deref() == Some("aarch64")
}

fn link(llama: &str, whisper: &str) {
    let llama = checkout("TACO_LLAMA_SRC", llama);
    let whisper = checkout("TACO_WHISPER_SRC", whisper);
    let source = PathBuf::from(env::var("CARGO_MANIFEST_DIR").expect("manifest"))
        .parent()
        .expect("native/")
        .to_path_buf();
    let out = PathBuf::from(env::var("OUT_DIR").expect("OUT_DIR")).join("cmake");
    fs::create_dir_all(&out).expect("cmake build dir");

    run(
        "cmake",
        &[
            "-S".to_string(),
            source.to_str().expect("utf-8").to_string(),
            "-B".to_string(),
            out.to_str().expect("utf-8").to_string(),
            "-DCMAKE_BUILD_TYPE=Release".to_string(),
            format!("-DLLAMA_SRC={}", llama.display()),
            format!("-DWHISPER_SRC={}", whisper.display()),
        ],
    );
    run(
        "cmake",
        &[
            "--build".to_string(),
            out.to_str().expect("utf-8").to_string(),
            "--config".to_string(),
            "Release".to_string(),
        ],
    );

    let mut archives = Vec::new();
    collect_archives(&out, &mut archives);
    let mut by_stem: BTreeMap<String, PathBuf> = BTreeMap::new();
    for path in archives {
        let stem = stem_of(&path);
        if !keep(&stem) {
            continue;
        }
        if let Some(prev) = by_stem.insert(stem.clone(), path.clone()) {
            panic!(
                "duplicate static archive {stem}: {} and {}",
                prev.display(),
                path.display()
            );
        }
    }
    for required in ["taco_engines", "whisper", "llama"] {
        if !by_stem.contains_key(required) {
            panic!(
                "cmake did not produce lib{required}.a under {}",
                out.display()
            );
        }
    }
    if !by_stem.keys().any(|stem| stem.starts_with("ggml")) {
        panic!("cmake did not produce a ggml static archive");
    }

    let mut stems: Vec<&String> = by_stem.keys().collect();
    stems.sort_by_key(|stem| (rank(stem), *stem));
    for stem in stems {
        let path = &by_stem[stem];
        println!(
            "cargo:rustc-link-search=native={}",
            path.parent().expect("archive dir").display()
        );
        // ponytail: whole-archive keeps embedded Metal objects. Drop it if one
        // ggml tree was compiled twice and the link reports duplicate symbols.
        println!("cargo:rustc-link-lib=static:+whole-archive={stem}");
    }
    for framework in [
        "Accelerate",
        "Foundation",
        "Metal",
        "MetalKit",
        "CoreFoundation",
    ] {
        println!("cargo:rustc-link-lib=framework={framework}");
    }
    println!("cargo:rustc-link-lib=c++");
    println!("cargo:rustc-link-lib=objc");
    println!("cargo:rustc-cfg=taco_engines_static");
}

fn checkout(label: &str, raw: &str) -> PathBuf {
    let path = PathBuf::from(raw);
    let path = path.canonicalize().unwrap_or_else(|err| {
        panic!("{label} ({raw}) is not a directory: {err}");
    });
    if !path.join("CMakeLists.txt").is_file() {
        panic!("{label} has no CMakeLists.txt: {}", path.display());
    }
    path
}

fn keep(stem: &str) -> bool {
    stem == "taco_engines" || stem == "whisper" || stem == "llama" || stem.starts_with("ggml")
}

fn rank(stem: &str) -> usize {
    const ORDER: &[&str] = &[
        "taco_engines",
        "whisper",
        "llama",
        "ggml-metal",
        "ggml-blas",
        "ggml-cpu",
        "ggml",
        "ggml-base",
    ];
    ORDER
        .iter()
        .position(|name| *name == stem)
        .unwrap_or(ORDER.len())
}

fn stem_of(path: &Path) -> String {
    let stem = path
        .file_stem()
        .and_then(|stem| stem.to_str())
        .expect("archive name")
        .to_string();
    stem.strip_prefix("lib").unwrap_or(&stem).to_string()
}

fn collect_archives(dir: &Path, out: &mut Vec<PathBuf>) {
    let entries = match fs::read_dir(dir) {
        Ok(entries) => entries,
        Err(_) => return,
    };
    for entry in entries.flatten() {
        let path = entry.path();
        if path.is_dir() {
            let name = path
                .file_name()
                .and_then(|name| name.to_str())
                .unwrap_or("");
            if name == "CMakeFiles" || name == "CMakeScratch" || name == "Testing" {
                continue;
            }
            collect_archives(&path, out);
            continue;
        }
        if path.extension().and_then(|ext| ext.to_str()) == Some("a") {
            out.push(path);
        }
    }
}

fn run(program: &str, args: &[String]) {
    let output = Command::new(program)
        .args(args)
        .output()
        .unwrap_or_else(|err| {
            panic!("failed to start {program}: {err}");
        });
    if output.status.success() {
        return;
    }
    let err = String::from_utf8_lossy(&output.stderr);
    let out = String::from_utf8_lossy(&output.stdout);
    panic!(
        "{program} {args:?} failed ({})\n{err}\n{out}",
        output.status
    );
}
